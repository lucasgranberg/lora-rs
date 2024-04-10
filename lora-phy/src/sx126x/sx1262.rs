use super::Sx126xVariant;

/// Sx1262 implements the Sx126xVariant trait
pub struct Sx1262;
impl Sx126xVariant for Sx1262 {
    fn get_device_sel(&self, _output_power: i32) -> super::DeviceSel {
        super::DeviceSel::HighPowerPA
    }
}
