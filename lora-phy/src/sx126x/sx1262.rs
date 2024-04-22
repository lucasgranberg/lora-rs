use super::Sx126xVariant;

/// Sx1261 uses only HighPowerPA
pub struct Sx1262;
impl Sx126xVariant for Sx1262 {
    fn get_device_sel(&self) -> super::DeviceSel {
        super::DeviceSel::HighPowerPA
    }
}
