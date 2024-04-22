use super::{DeviceSel, Sx126xVariant};

/// Stm32wl variant.
pub struct Stm32wl {
    /// select which output to use. (Switching is not supported)
    pub use_high_power_pa: bool,
}
impl Sx126xVariant for Stm32wl {
    fn get_device_sel(&self) -> super::DeviceSel {
        if self.use_high_power_pa {
            DeviceSel::HighPowerPA
        } else {
            DeviceSel::LowPowerPA
        }
    }
    fn use_dio2_as_rfswitch(&self) -> bool {
        false
    }
}
