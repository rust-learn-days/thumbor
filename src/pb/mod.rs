use prost::Message;
pub use abi::*;
use base64::{encode_config, URL_SAFE_NO_PAD};
mod abi;

impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self { specs }
    }
}

impl From<&ImageSpec> for String{
    fn from(spec: &ImageSpec) -> Self {
        let data = spec.encode_to_vec();
        encode_config(data, URL_SAFE_NO_PAD)
    }
}
