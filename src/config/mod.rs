#[derive(Clone, Debug)]
pub struct ShuiqiOptions {
    pub(crate) resize_interval: u128
}

impl Default for ShuiqiOptions {
    fn default() -> Self {
        ShuiqiOptions {
            resize_interval: 250
        }
    }
}