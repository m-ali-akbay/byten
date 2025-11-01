use quote::quote;
use syn::{Attribute, Expr, Meta};

use super::{BinarySchema, DecodeContext, EncodeContext, MeasureContext};

pub fn interpret_codec_schema(expr: &Expr) -> Box<dyn BinarySchema> {
    Box::new(CodecSchema {
        expr: expr.clone(),
    })
}

struct CodecSchema {
    expr: Expr,
}

impl BinarySchema for CodecSchema {
    fn decode(&self, ctx: &DecodeContext) -> proc_macro2::TokenStream {
        let expr = &self.expr;
        let encoded = &ctx.encoded;
        let offset = &ctx.offset;
        quote! { ::byten::Decoder::decode(&#expr, #encoded, #offset)? }
    }

    fn encode(&self, ctx: &EncodeContext) -> proc_macro2::TokenStream {
        let expr = &self.expr;
        let decoded = &ctx.decoded;
        let encoded = &ctx.encoded;
        let offset = &ctx.offset;
        quote! { ::byten::Encoder::encode(&#expr, #decoded, #encoded, #offset)? }
    }

    fn measure_fixed(&self) -> proc_macro2::TokenStream {
        let expr = &self.expr;
        quote! { ::byten::FixedMeasurer::measure_fixed(&#expr) }
    }

    fn measure(&self, ctx: &MeasureContext) -> proc_macro2::TokenStream {
        let expr = &self.expr;
        let decoded = &ctx.decoded;
        quote! { ::byten::Measurer::measure(&#expr, #decoded)? }
    }
}

pub fn parse_byten_attribute(attr: &Vec<Attribute>) -> Option<Expr> {
    for attribute in attr {
        if attribute.path().is_ident("byten") {
            match &attribute.meta {
                Meta::List(meta) => {
                    return Some(meta.parse_args::<Expr>().expect("Invalid byten attribute"));
                },
                _ => panic!("Invalid byten attribute format"),
            }
        }
    }
    None
}
