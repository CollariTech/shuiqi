#[derive(Clone, Debug)]
pub struct ShuiqiOptions {
    pub resize_interval: u32
}

impl Default for ShuiqiOptions {
    fn default() -> Self {
        ShuiqiOptions {
            resize_interval: 250
        }
    }
}