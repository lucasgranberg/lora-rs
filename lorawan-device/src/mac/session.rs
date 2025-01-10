use crate::{AppSKey, NwkSKey};
use heapless::Vec;
use lorawan::keys::CryptoFactory;
use lorawan::parser::FCtrl;
use lorawan::{
    creator::DataPayloadCreator,
    maccommands::SerializableMacCommand,
    parser::{DecryptedJoinAcceptPayload, DevAddr},
};

use crate::radio::RadioBuffer;

use super::{
    otaa::{DevNonce, NetworkCredentials},
    uplink, FcntUp, Response, SendData,
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Session {
    pub uplink: uplink::Uplink,
    pub confirmed: bool,
    pub nwkskey: NwkSKey,
    pub appskey: AppSKey,
    pub devaddr: DevAddr<[u8; 4]>,
    pub fcnt_up: u32,
    pub fcnt_down: u32,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct SessionKeys {
    pub nwkskey: NwkSKey,
    pub appskey: AppSKey,
    pub devaddr: DevAddr<[u8; 4]>,
}

impl From<Session> for SessionKeys {
    fn from(session: Session) -> Self {
        Self { nwkskey: session.nwkskey, appskey: session.appskey, devaddr: session.devaddr }
    }
}

impl Session {
    pub fn derive_new<T: AsRef<[u8]>, F: CryptoFactory>(
        decrypt: &DecryptedJoinAcceptPayload<T, F>,
        devnonce: DevNonce,
        credentials: &NetworkCredentials,
    ) -> Self {
        Self::new(
            decrypt.derive_nwkskey(&devnonce, credentials.appkey()),
            decrypt.derive_appskey(&devnonce, credentials.appkey()),
            DevAddr::new([
                decrypt.dev_addr().as_ref()[0],
                decrypt.dev_addr().as_ref()[1],
                decrypt.dev_addr().as_ref()[2],
                decrypt.dev_addr().as_ref()[3],
            ])
            .unwrap(),
        )
    }

    pub fn new(nwkskey: NwkSKey, appskey: AppSKey, devaddr: DevAddr<[u8; 4]>) -> Self {
        Self {
            nwkskey,
            appskey,
            devaddr,
            confirmed: false,
            fcnt_down: 0,
            fcnt_up: 0,
            uplink: uplink::Uplink::default(),
        }
    }

    pub fn devaddr(&self) -> &DevAddr<[u8; 4]> {
        &self.devaddr
    }
    pub fn appskey(&self) -> &AppSKey {
        &self.appskey
    }
    #[deprecated(since = "0.12.2", note = "Please use `self.nwkskey` instead")]
    pub fn newskey(&self) -> &NwkSKey {
        &self.nwkskey
    }

    pub fn nwkskey(&self) -> &NwkSKey {
        &self.nwkskey
    }

    pub fn get_session_keys(&self) -> Option<SessionKeys> {
        Some(SessionKeys { nwkskey: self.nwkskey, appskey: self.appskey, devaddr: self.devaddr })
    }
}

impl Session {
    pub(crate) fn rx2_complete(&mut self) -> Response {
        // Until we handle NbTrans, there is no case where we should not increment FCntUp.
        if self.fcnt_up == 0xFFFF_FFFF {
            // if the FCnt is used up, the session has expired
            return Response::SessionExpired;
        } else {
            self.fcnt_up += 1;
        }
        if self.confirmed {
            Response::NoAck
        } else {
            Response::RxComplete
        }
    }

    pub(crate) fn prepare_buffer<C: CryptoFactory + Default, const N: usize>(
        &mut self,
        data: &SendData<'_>,
        tx_buffer: &mut RadioBuffer<N>,
    ) -> FcntUp {
        tx_buffer.clear();
        let fcnt = self.fcnt_up;
        let mut buf = [0u8; 256];
        let mut phy = DataPayloadCreator::with_options(&mut buf, C::default()).unwrap();

        let mut fctrl = FCtrl(0x0, true);
        if self.uplink.confirms_downlink() {
            fctrl.set_ack();
            self.uplink.clear_downlink_confirmation();
        }

        self.confirmed = data.confirmed;

        phy.set_confirmed(data.confirmed)
            .set_fctrl(&fctrl)
            .set_f_port(data.fport)
            .set_dev_addr(self.devaddr)
            .set_fcnt(fcnt);

        let mut cmds = Vec::new();
        self.uplink.get_cmds(&mut cmds);
        let mut dyn_cmds: Vec<&dyn SerializableMacCommand, 8> = Vec::new();

        for cmd in &cmds {
            if let Err(_e) = dyn_cmds.push(cmd) {
                panic!("dyn_cmds too small compared to cmds")
            }
        }

        match phy.build(data.data, dyn_cmds.as_slice(), &self.nwkskey, &self.appskey) {
            Ok(packet) => {
                tx_buffer.clear();
                tx_buffer.extend_from_slice(packet).unwrap();
            }
            Err(e) => panic!("Error assembling packet! {:?} ", e),
        }
        fcnt
    }
}
