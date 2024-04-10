use super::Sx126xVariant;

/// Sx1261 implements the Sx126xVariant trait
pub struct Sx1261;
impl Sx126xVariant for Sx1261 {
    fn get_device_sel(&self, _output_power: i32) -> super::DeviceSel {
        super::DeviceSel::LowPowerPA
    }
}
