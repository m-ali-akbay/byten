use quote::quote;
use syn::{Type, TypePath};

use super::{BinarySchema, DecodeContext, EncodeContext, MeasureContext};

pub fn interpret_type_schema(ty: &Type) -> Box<dyn BinarySchema> {
    match ty {
        Type::Path(ty) => interpret_type_path_schema(ty),
        _ => panic!("Unsupported type"),
    }
}

pub fn interpret_type_path_schema(ty: &TypePath) -> Box<dyn BinarySchema> {
    Box::new(TypePathSchema::from(ty))
}

struct TypePathSchema {
    type_path: TypePath,
}

impl BinarySchema for TypePathSchema {
    fn decode(&self, ctx: &DecodeContext) -> proc_macro2::TokenStream {
        let type_path = &self.type_path;
        let encoded = &ctx.encoded;
        let offset = &ctx.offset;
        quote! { <#type_path as ::binary_codec::Decode>::decode(#encoded, #offset)? }
    }

    fn encode(&self, ctx: &EncodeContext) -> proc_macro2::TokenStream {
        let type_path = &self.type_path;
        let decoded = &ctx.decoded;
        let encoded = &ctx.encoded;
        let offset = &ctx.offset;
        quote! { <#type_path as ::binary_codec::Encode>::encode(#decoded, #encoded, #offset)? }
    }

    fn measure(&self, ctx: &MeasureContext) -> proc_macro2::TokenStream {
        let type_path = &self.type_path;
        let decoded = &ctx.decoded;
        quote! { <#type_path as ::binary_codec::Measure>::measure(#decoded) }
    }
}

impl From<&TypePath> for TypePathSchema {
    fn from(ty: &TypePath) -> Self {
        TypePathSchema {
            type_path: ty.clone(),
        }
    }
}
