
pub mod r#struct;
pub mod r#enum;
pub mod field;
pub mod codec;

pub use r#struct::*;
pub use r#enum::*;
pub use field::*;
pub use codec::*;

pub trait BinarySchema {
    fn decode(&self, _ctx: &DecodeContext) -> proc_macro2::TokenStream { unimplemented!() }
    fn encode(&self, _ctx: &EncodeContext) -> proc_macro2::TokenStream { unimplemented!() }
    fn fixed_measure(&self) -> proc_macro2::TokenStream { unimplemented!() }
    fn measure(&self, _ctx: &MeasureContext) -> proc_macro2::TokenStream { unimplemented!() }
}

#[derive(Clone)]
pub struct DecodeContext {
    pub encoded: proc_macro2::TokenStream,
    pub offset: proc_macro2::TokenStream,
}

#[derive(Clone)]
pub struct EncodeContext {
    pub wrapper: proc_macro2::TokenStream,
    pub decoded: proc_macro2::TokenStream,
    pub encoded: proc_macro2::TokenStream,
    pub offset: proc_macro2::TokenStream,
}

#[derive(Clone)]
pub struct MeasureContext {
    pub wrapper: proc_macro2::TokenStream,
    pub decoded: proc_macro2::TokenStream,
}
