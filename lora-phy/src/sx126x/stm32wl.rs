use super::{DeviceSel, Sx126xVariant};

const HIGH_POWER_PA_THRESHOLD: i32 = 15;

/// Stm32wl implements the Sx126xVariant trait
pub struct Stm32wl;
impl Sx126xVariant for Stm32wl {
    fn get_device_sel(&self, output_power: i32) -> super::DeviceSel {
        if output_power <= HIGH_POWER_PA_THRESHOLD {
            DeviceSel::LowPowerPA
        } else {
            DeviceSel::HighPowerPA
        }
    }
}
