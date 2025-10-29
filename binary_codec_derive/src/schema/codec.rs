use quote::quote;
use syn::{Attribute, Meta, Type, TypePath};

use super::{BinarySchema, DecodeContext, EncodeContext, MeasureContext};

pub fn interpret_codec_schema(codec_path: TypePath) -> Box<dyn BinarySchema> {
    Box::new(CodecSchema {
        codec_path,
    })
}

struct CodecSchema {
    codec_path: TypePath,
}

impl BinarySchema for CodecSchema {
    fn decode(&self, ctx: &DecodeContext) -> proc_macro2::TokenStream {
        let codec_path = &self.codec_path;
        let encoded = &ctx.encoded;
        let offset = &ctx.offset;
        quote! { #codec_path::decode(#encoded, #offset)? }
    }

    fn encode(&self, ctx: &EncodeContext) -> proc_macro2::TokenStream {
        let codec_path = &self.codec_path;
        let decoded = &ctx.decoded;
        let encoded = &ctx.encoded;
        let offset = &ctx.offset;
        quote! { #codec_path::encode(#decoded, #encoded, #offset)? }
    }

    fn measure(&self, ctx: &MeasureContext) -> proc_macro2::TokenStream {
        let codec_path = &self.codec_path;
        let decoded = &ctx.decoded;
        quote! { #codec_path::measure(#decoded) }
    }
}

pub fn parse_binary_codec_attribute(attr: &Vec<Attribute>) -> Option<TypePath> {
    for attribute in attr {
        if attribute.path().is_ident("binary_codec") {
            match &attribute.meta {
                Meta::List(meta) => {
                    return Some(meta.parse_args::<TypePath>().expect("Invalid binary_codec attribute"));
                },
                _ => panic!("Invalid binary_codec attribute format"),
            }
        }
    }
    None
}
