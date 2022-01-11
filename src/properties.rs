use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Properties {
    pub(crate) quote: String,
    pub(crate) tag: String,
    pub(crate) quote_position: u32,
    pub(crate) tag_position: u32,
    pub(crate) crop_position: (u32, u32),
    pub(crate) rgba: [u8; 4],
}
