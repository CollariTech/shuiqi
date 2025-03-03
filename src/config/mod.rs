#[derive(Clone, Debug)]
pub struct ShuiqiOptions {
    pub resize_interval: u32,
    pub resize_interval_accumulates: bool
}

impl Default for ShuiqiOptions {
    fn default() -> Self {
        ShuiqiOptions {
            resize_interval: 250,
            resize_interval_accumulates: false
        }
    }
}