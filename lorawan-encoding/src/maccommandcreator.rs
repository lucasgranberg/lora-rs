use super::maccommands::{mac_commands_len, SerializableMacCommand};
use crate::types::{ChannelMask, DLSettings, DataRateRange, Frequency, Redundancy};

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Error {
    InvalidDataRate,
    InvalidTxPower,
    MarginOutOfRange,
    DelayOutOfRange,
    MaxEirpOutOfRange,
    NanoSecondsOutOfRange,
    BufferTooShort,
}

/// LinkCheckReqCreator serves for creating LinkCheckReq MacCommand.
///
/// # Examples
///
/// ```
/// let mut creator = lorawan::maccommandcreator::LinkCheckReqCreator::new();
/// let res = creator.build();
/// ```
#[doc(inline)]
pub use crate::maccommands::LinkCheckReqCreator;

/// LinkCheckAnsCreator serves for creating LinkCheckAns MacCommand.
///
/// # Examples
///
/// ```
/// let mut creator = lorawan::maccommandcreator::LinkCheckAnsCreator::new();
/// let res = creator.set_margin(253).set_gateway_count(254).build();
/// ```
#[doc(inline)]
pub use crate::maccommands::LinkCheckAnsCreator;

impl LinkCheckAnsCreator {
    /// Sets the margin of the LinkCheckAns to the provided value.
    ///
    /// # Argument
    ///
    /// * margin - margin in dB. The value is relative to the demodulation
    ///   floor. The value 255 is reserved.
    pub fn set_margin(&mut self, margin: u8) -> &mut Self {
        self.data[1] = margin;

        self
    }

    /// Sets the gateway count of the LinkCheckAns to the provided value.
    ///
    /// # Argument
    ///
    /// * gateway_count - the number of gateways that received the LinkCheckReq.
    pub fn set_gateway_count(&mut self, gateway_count: u8) -> &mut Self {
        self.data[2] = gateway_count;

        self
    }
}

/// LinkADRReqCreator serves for creating LinkADRReq MacCommand.
///
/// # Examples
///
/// ```
/// let mut creator = lorawan::maccommandcreator::LinkADRReqCreator::new();
/// let channel_mask_bytes = [0xc7, 0x0b];
/// let res = creator
///     .set_data_rate(0x05)
///     .unwrap()
///     .set_tx_power(0x03)
///     .unwrap()
///     .set_channel_mask(channel_mask_bytes)
///     .set_redundancy(0x37)
///     .build();
/// ```
#[doc(inline)]
pub use crate::maccommands::LinkADRReqCreator;

impl LinkADRReqCreator {
    /// Sets the data rate of the LinkADRReq to the provided value.
    ///
    /// # Argument
    ///
    /// * data_rate - data rate index of the ADR request. The value must be between 0 and 15.
    pub fn set_data_rate(&mut self, data_rate: u8) -> Result<&mut Self, Error> {
        if data_rate > 0x0f {
            return Err(Error::InvalidDataRate);
        }
        self.data[1] &= 0x0f;
        self.data[1] |= data_rate << 4;

        Ok(self)
    }

    /// Sets the TX power of the LinkADRReq to the provided value.
    ///
    /// # Argument
    ///
    /// * tx_power - TX power index. The value must be between 0 and 15.
    pub fn set_tx_power(&mut self, tx_power: u8) -> Result<&mut Self, Error> {
        if tx_power > 0x0f {
            return Err(Error::InvalidTxPower);
        }
        self.data[1] &= 0xf0;
        self.data[1] |= tx_power & 0x0f;

        Ok(self)
    }

    /// Sets the channel mask of the LinkADRReq to the provided value.
    ///
    /// # Argument
    ///
    /// * channel_mask - instance of maccommands::ChannelMask or anything that
    ///   can be converted into it.
    pub fn set_channel_mask<T: Into<ChannelMask<2>>>(&mut self, channel_mask: T) -> &mut Self {
        let converted = channel_mask.into();
        self.data[2] = converted.as_ref()[0];
        self.data[3] = converted.as_ref()[1];

        self
    }

    /// Sets the redundancy of the LinkADRReq to the provided value.
    ///
    /// # Argument
    ///
    /// * redundancy - instance of types::Redundancy or anything that can
    ///   be converted into it.
    pub fn set_redundancy<T: Into<Redundancy>>(&mut self, redundancy: T) -> &mut Self {
        let converted = redundancy.into();
        self.data[4] = converted.raw_value();

        self
    }
}

/// LinkADRAnsCreator serves for creating LinkADRAns MacCommand.
///
/// # Examples
///
/// ```
/// let mut creator = lorawan::maccommandcreator::LinkADRAnsCreator::new();
/// let res =
///     creator.set_channel_mask_ack(true).set_data_rate_ack(true).set_tx_power_ack(true).build();
/// ```
#[doc(inline)]
pub use crate::maccommands::LinkADRAnsCreator;

impl LinkADRAnsCreator {
    /// Sets the channel mask acknowledgement of the LinkADRAns to the provided value.
    ///
    /// # Argument
    ///
    /// * ack - true when channel mask was acceptable or false otherwise.
    pub fn set_channel_mask_ack(&mut self, ack: bool) -> &mut Self {
        self.data[1] &= 0xfe;
        self.data[1] |= ack as u8;

        self
    }

    /// Sets the data rate acknowledgement of the LinkADRAns to the provided value.
    ///
    /// # Argument
    ///
    /// * ack - true when data rate was acceptable or false otherwise.
    pub fn set_data_rate_ack(&mut self, ack: bool) -> &mut Self {
        self.data[1] &= 0xfd;
        self.data[1] |= (ack as u8) << 1;

        self
    }

    /// Sets the TX power acknowledgement of the LinkADRAns to the provided value.
    ///
    /// # Argument
    ///
    /// * ack - true when TX power was acceptable or false otherwise.
    pub fn set_tx_power_ack(&mut self, ack: bool) -> &mut Self {
        self.data[1] &= 0xfb;
        self.data[1] |= (ack as u8) << 2;

        self
    }
}

/// DutyCycleReqCreator serves for creating DutyCycleReq MacCommand.
///
/// # Examples
///
/// ```
/// let mut creator = lorawan::maccommandcreator::DutyCycleReqCreator::new();
/// let res = creator.set_max_duty_cycle(0x0f).unwrap().build();
/// ```
#[doc(inline)]
pub use crate::maccommands::DutyCycleReqCreator;

impl DutyCycleReqCreator {
    /// Sets the max duty cycle of the DutyCycleReq to the provided value.
    ///
    /// # Argument
    ///
    /// * max_duty_cycle - the value used to determine the aggregated duty cycle
    ///   using the formula `1 / (2 ** max_duty_cycle)`.
    pub fn set_max_duty_cycle(&mut self, max_duty_cycle: u8) -> Result<&mut Self, Error> {
        self.data[1] = max_duty_cycle;

        Ok(self)
    }
}

/// DutyCycleAnsCreator serves for creating DutyCycleAns MacCommand.
///
/// # Examples
///
/// ```
/// let creator = lorawan::maccommandcreator::DutyCycleAnsCreator::new();
/// let res = creator.build();
/// ```
#[doc(inline)]
pub use crate::maccommands::DutyCycleAnsCreator;

/// RXParamSetupReqCreator serves for creating RXParamSetupReq MacCommand.
///
/// # Examples
///
/// ```
/// let mut creator = lorawan::maccommandcreator::RXParamSetupReqCreator::new();
/// let res = creator.set_dl_settings(0xcd).set_frequency(&[0x12, 0x34, 0x56]).build();
/// ```
#[doc(inline)]
pub use crate::maccommands::RXParamSetupReqCreator;

impl RXParamSetupReqCreator {
    /// Sets the DLSettings of the RXParamSetupReq to the provided value.
    ///
    /// # Argument
    ///
    /// * dl_settings - instance of maccommands::DLSettings or anything that can
    ///   be converted into it.
    pub fn set_dl_settings<T: Into<DLSettings>>(&mut self, dl_settings: T) -> &mut Self {
        let converted = dl_settings.into();
        self.data[1] = converted.raw_value();

        self
    }

    /// Sets the frequency of the RXParamSetupReq to the provided value.
    ///
    /// # Argument
    ///
    /// * frequency - instance of maccommands::Frequency or anything that can be
    ///   converted into it.
    pub fn set_frequency<'a, T: Into<Frequency<'a>>>(&mut self, frequency: T) -> &mut Self {
        let converted = frequency.into();
        self.data[2..5].copy_from_slice(converted.as_ref());

        self
    }
}

/// RXParamSetupAnsCreator serves for creating RXParamSetupAns MacCommand.
///
/// # Examples
///
/// ```
/// let mut creator = lorawan::maccommandcreator::RXParamSetupAnsCreator::new();
/// let res = creator
///     .set_channel_ack(true)
///     .set_rx2_data_rate_ack(true)
///     .set_rx1_data_rate_offset_ack(true)
///     .build();
/// ```
#[doc(inline)]
pub use crate::maccommands::RXParamSetupAnsCreator;

impl RXParamSetupAnsCreator {
    /// Sets the channel acknowledgement of the RXParamSetupAns to the provided value.
    ///
    /// # Argument
    ///
    /// * ack - true when channel was acceptable or false otherwise.
    pub fn set_channel_ack(&mut self, ack: bool) -> &mut Self {
        self.data[1] &= 0xfe;
        self.data[1] |= ack as u8;

        self
    }

    /// Sets the rx2 data rate acknowledgement of the RXParamSetupAns to the provided value.
    ///
    /// # Argument
    ///
    /// * ack - true when RX2 data rate was acceptable or false otherwise.
    pub fn set_rx2_data_rate_ack(&mut self, ack: bool) -> &mut Self {
        self.data[1] &= 0xfd;
        self.data[1] |= (ack as u8) << 1;

        self
    }

    /// Sets the rx1 data rate offset acknowledgement of the RXParamSetupAns to the provided value.
    ///
    /// # Argument
    ///
    /// * ack - true when RX1 data rate offset was acceptable or false otherwise.
    pub fn set_rx1_data_rate_offset_ack(&mut self, ack: bool) -> &mut Self {
        self.data[1] &= 0xfb;
        self.data[1] |= (ack as u8) << 2;

        self
    }
}

/// DevStatusReqCreator serves for creating DevStatusReq MacCommand.
///
/// # Examples
///
/// ```
/// let creator = lorawan::maccommandcreator::DevStatusReqCreator::new();
/// let res = creator.build();
/// ```
#[doc(inline)]
pub use crate::maccommands::DevStatusReqCreator;

/// DevStatusAnsCreator serves for creating DevStatusAns MacCommand.
///
/// # Examples
///
/// ```
/// let mut creator = lorawan::maccommandcreator::DevStatusAnsCreator::new();
/// let res = creator.set_battery(0xfe).set_margin(-32).unwrap().build();
/// ```
#[doc(inline)]
pub use crate::maccommands::DevStatusAnsCreator;

impl DevStatusAnsCreator {
    /// Sets the battery of the DevStatusAns to the provided value.
    ///
    /// # Argument
    ///
    /// * battery - value to be used as the battery level. 0 means external
    ///   energy source, 1 and 254 are the minimum and maximum values of normal
    ///   battery reading, while 255 indicates that the device failed to measure
    ///   its battery level.
    pub fn set_battery(&mut self, battery: u8) -> &mut Self {
        self.data[1] = battery;

        self
    }

    /// Sets the margin of the DevStatusAns to the provided value.
    ///
    /// # Argument
    ///
    /// * margin - the value to be used as margin.
    pub fn set_margin(&mut self, margin: i8) -> Result<&mut Self, Error> {
        if !(-32..=31).contains(&margin) {
            return Err(Error::MarginOutOfRange);
        }
        self.data[2] = ((margin << 2) as u8) >> 2;

        Ok(self)
    }
}

/// NewChannelReqCreator serves for creating NewChannelReq MacCommand.
///
/// # Examples
///
/// ```
/// let mut creator = lorawan::maccommandcreator::NewChannelReqCreator::new();
/// let res = creator
///     .set_channel_index(0x0f)
///     .set_frequency(&[0x12, 0x34, 0x56])
///     .set_data_rate_range(0x53)
///     .build();
/// ```
#[doc(inline)]
pub use crate::maccommands::NewChannelReqCreator;

impl NewChannelReqCreator {
    /// Sets the channel index of the NewChannelReq to the provided value.
    ///
    /// # Argument
    ///
    /// * channel_index - the value to be used as channel_index.
    pub fn set_channel_index(&mut self, channel_index: u8) -> &mut Self {
        self.data[1] = channel_index;

        self
    }

    /// Sets the frequency of the NewChannelReq to the provided value.
    ///
    /// # Argument
    ///
    /// * frequency - instance of maccommands::Frequency or anything that can
    ///   be converted into it.
    pub fn set_frequency<'a, T: Into<Frequency<'a>>>(&mut self, frequency: T) -> &mut Self {
        let converted = frequency.into();
        self.data[2..5].copy_from_slice(converted.as_ref());

        self
    }

    /// Sets the data rate range of the NewChannelReq to the provided value.
    ///
    /// # Argument
    ///
    /// * data_rate_range - instance of maccommands::DataRateRange or anything
    ///   that can be converted into it.
    pub fn set_data_rate_range<T: Into<DataRateRange>>(&mut self, data_rate_range: T) -> &mut Self {
        self.data[5] = data_rate_range.into().raw_value();

        self
    }
}

/// NewChannelAnsCreator serves for creating NewChannelAns MacCommand.
///
/// # Examples
///
/// ```
/// let mut creator = lorawan::maccommandcreator::NewChannelAnsCreator::new();
/// let res = creator.set_channel_frequency_ack(true).set_data_rate_range_ack(true).build();
/// ```
#[doc(inline)]
pub use crate::maccommands::NewChannelAnsCreator;

impl NewChannelAnsCreator {
    /// Sets the channel frequency acknowledgement of the NewChannelAns to the provided value.
    ///
    /// # Argument
    ///
    /// * ack - true when channel frequency was acceptable or false otherwise.
    pub fn set_channel_frequency_ack(&mut self, ack: bool) -> &mut Self {
        self.data[1] &= 0xfe;
        self.data[1] |= ack as u8;

        self
    }

    /// Sets the data rate range acknowledgement of the NewChannelAns to the provided value.
    ///
    /// # Argument
    ///
    /// * ack - true when data rate range was acceptable or false otherwise.
    pub fn set_data_rate_range_ack(&mut self, ack: bool) -> &mut Self {
        self.data[1] &= 0xfd;
        self.data[1] |= (ack as u8) << 1;

        self
    }
}

/// RXTimingSetupReqCreator serves for creating RXTimingSetupReq MacCommand.
///
/// # Examples
///
/// ```
/// let mut creator = lorawan::maccommandcreator::RXTimingSetupReqCreator::new();
/// let res = creator.set_delay(0x0f).unwrap().build();
/// ```
#[doc(inline)]
pub use crate::maccommands::RXTimingSetupReqCreator;

impl RXTimingSetupReqCreator {
    /// Sets the delay of the RXTimingSetupReq to the provided value.
    ///
    /// # Argument
    ///
    /// * delay - the value to be used as delay.
    pub fn set_delay(&mut self, delay: u8) -> Result<&mut Self, Error> {
        if delay > 0x0f {
            return Err(Error::DelayOutOfRange);
        }
        self.data[1] &= 0xf0;
        self.data[1] |= delay;

        Ok(self)
    }
}

/// RXTimingSetupAnsCreator serves for creating RXTimingSetupAns MacCommand.
///
/// # Examples
///
/// ```
/// let creator = lorawan::maccommandcreator::RXTimingSetupAnsCreator::new();
/// let res = creator.build();
/// ```
#[doc(inline)]
pub use crate::maccommands::RXTimingSetupAnsCreator;

#[doc(inline)]
pub use crate::maccommands::TXParamSetupReqCreator;

impl TXParamSetupReqCreator {
    pub fn set_downlink_dwell_time(&mut self) -> &mut Self {
        self.data[1] &= 0xfe;
        self.data[1] |= (1 << 5) as u8;
        self
    }
    pub fn set_uplink_dwell_time(&mut self) -> &mut Self {
        self.data[1] &= 0xfe;
        self.data[1] |= (1 << 4) as u8;
        self
    }
    pub fn set_max_eirp(&mut self, max_eirp: u8) -> Result<&mut Self, Error> {
        if max_eirp > 0x0F {
            return Err(Error::MaxEirpOutOfRange);
        }
        self.data[1] &= 0xf0;
        self.data[1] |= max_eirp;

        Ok(self)
    }
}

#[doc(inline)]
pub use crate::maccommands::TXParamSetupAnsCreator;

#[doc(inline)]
pub use crate::maccommands::DlChannelReqCreator;

impl DlChannelReqCreator {
    pub fn set_channel_index(&mut self, index: u8) -> &mut Self {
        self.data[1] = index;
        self
    }
    pub fn set_frequency<'a, T: Into<Frequency<'a>>>(&mut self, frequency: T) -> &mut Self {
        let converted = frequency.into();
        self.data[2..5].copy_from_slice(converted.as_ref());

        self
    }
}

#[doc(inline)]
pub use crate::maccommands::DlChannelAnsCreator;

impl DlChannelAnsCreator {
    /// Sets the channel frequency acknowledgement of the DlChannelAns to the provided value.
    ///
    /// # Argument
    ///
    /// * ack - true when channel frequency was acceptable or false otherwise.
    pub fn set_channel_frequency_ack(&mut self, ack: bool) -> &mut Self {
        self.data[1] &= 0xfe;
        self.data[1] |= ack as u8;

        self
    }

    /// Sets the uplink frequency exists acknowledgement of the DlChannelAns to the provided value.
    ///
    /// # Argument
    ///
    /// * ack - true when data rate range was acceptable or false otherwise.
    pub fn set_uplink_frequency_exists_ack(&mut self, ack: bool) -> &mut Self {
        self.data[1] &= 0xfd;
        self.data[1] |= (ack as u8) << 1;

        self
    }
}

#[doc(inline)]
pub use crate::maccommands::DeviceTimeAnsCreator;
#[doc(inline)]
pub use crate::maccommands::DeviceTimeReqCreator;

impl DeviceTimeAnsCreator {
    pub fn set_seconds(&mut self, seconds: u32) -> &mut Self {
        self.data[1..5].copy_from_slice(&seconds.to_le_bytes());
        self
    }
    pub fn set_nano_seconds(&mut self, nano_seconds: u32) -> Result<&mut Self, Error> {
        if nano_seconds > 1000000000 {
            return Err(Error::NanoSecondsOutOfRange);
        }
        self.data[5] = (nano_seconds / 3906250) as u8;
        Ok(self)
    }
}

pub fn build_mac_commands<T: AsMut<[u8]>>(
    cmds: &[&dyn SerializableMacCommand],
    mut out: T,
) -> Result<usize, Error> {
    let res = out.as_mut();
    if mac_commands_len(cmds) > res.len() {
        return Err(Error::BufferTooShort);
    }
    let mut i = 0;
    for mc in cmds {
        // prefix the CID
        res[i] = mc.cid();
        let start = i + 1;
        let end = start + mc.payload_len();
        res[start..end].copy_from_slice(mc.payload_bytes());
        i = end;
    }
    Ok(i)
}

#[doc(inline)]
pub use crate::maccommands::DownlinkMacCommandCreator;

#[doc(inline)]
pub use crate::maccommands::UplinkMacCommandCreator;
