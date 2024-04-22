use super::Sx126xVariant;

/// Sx1261 uses only LowPowerPA
pub struct Sx1261;
impl Sx126xVariant for Sx1261 {
    fn get_device_sel(&self) -> super::DeviceSel {
        super::DeviceSel::LowPowerPA
    }
}
