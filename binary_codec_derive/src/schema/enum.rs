use syn::{Data, DeriveInput, Expr, Ident, Meta, TypePath};
use quote::quote;

use super::{BinarySchema, DecodeContext, EncodeContext, FieldsSchema, MeasureContext, interpret_codec_schema, interpret_type_path_schema, parse_binary_codec_attribute, interpret_fields_schema};

pub fn interpret_enum_schema(input: &DeriveInput) -> Box<dyn BinarySchema> {
    let Data::Enum(ref data) = input.data else {
        panic!("EnumSchema can only be created from enum data");
    };

    let repr = input.attrs.iter().find(
        |attr| attr.path().is_ident("repr")
    ).expect("Enum must have a repr attribute");
    let repr = match &repr.meta {
        Meta::List(meta) => {
            meta.parse_args::<TypePath>().expect("Invalid repr attribute")
        },
        _ => panic!("Invalid repr attribute format"),
    };

    let codec = parse_binary_codec_attribute(&input.attrs);
    let discriminator = match codec {
        Some(codec_path) => interpret_codec_schema(codec_path),
        None => interpret_type_path_schema(&repr),
    };

    let variants = data.variants.iter().map(|variant| {
        let ident = variant.ident.clone();
        let schema = interpret_fields_schema(&variant.fields);
        let discriminant = match &variant.discriminant {
            Some((_, expr)) => expr.clone(),
            None => panic!("Enum variants must have discriminants"),
        };
        (ident, schema, discriminant)
    }).collect();
    Box::new(EnumSchema {
        ident: input.ident.clone(),
        discriminator,
        variants,
    })
}

struct EnumSchema {
    ident: Ident,
    discriminator: Box<dyn BinarySchema>,
    variants: Vec<(Ident, Box<dyn FieldsSchema>, Expr)>,
}

impl BinarySchema for EnumSchema {
    fn decode(&self, ctx: &DecodeContext) -> proc_macro2::TokenStream {
        let ident = &self.ident;
        let variants = self.variants.iter().map(|(variant_ident, schema, discriminant)| {
            let decode = schema.decode(&ctx.clone());
            quote! {
                #discriminant => {
                    Ok(#ident::#variant_ident #decode)
                }
            }
        });
        let encoded = &ctx.encoded;
        let offset = &ctx.offset;
        let decode_discriminant = self.discriminator.decode(&DecodeContext {
            encoded: encoded.clone(),
            offset: offset.clone(),
        });
        quote! { {
            let discriminant = #decode_discriminant;
            match discriminant {
                #(#variants),*,
                _ => Err(::binary_codec::DecodeError::InvalidDiscriminant),
            }
        }? }
    }

    fn encode(&self, ctx: &EncodeContext) -> proc_macro2::TokenStream {
        let ident = &self.ident;
        let decoded = ctx.decoded.clone();
        let encoded = ctx.encoded.clone();
        let offset = ctx.offset.clone();
        let variants = self.variants.iter().map(|(variant_ident, schema, discriminant)| {
            let encoder_discriminant = self.discriminator.encode(&EncodeContext {
                wrapper: quote! {},
                decoded: quote! { (&#discriminant) },
                encoded: encoded.clone(),
                offset: offset.clone(),
            });
            let encode = schema.encode(&EncodeContext {
                wrapper: quote! { #ident::#variant_ident },
                decoded: quote! { variant },
                encoded: encoded.clone(),
                offset: offset.clone(),
            });
            let wildcard_pattern = schema.wildcard_pattern();
            quote! {
                variant @ #ident::#variant_ident #wildcard_pattern => {
                    #encoder_discriminant;
                    #encode
                }
            }
        });
        quote! {
            match #decoded {
                #(#variants),*
            }
        }
    }

    fn measure(&self, ctx: &MeasureContext) -> proc_macro2::TokenStream {
        let ident = &self.ident;
        let decoded = ctx.decoded.clone();
        let variants = self.variants.iter().map(|(variant_ident, schema, discriminant)| {
            let measure_discriminant = self.discriminator.measure(&MeasureContext {
                wrapper: quote! {},
                decoded: quote! { (&#discriminant) },
            });

            let measure = schema.measure(&MeasureContext {
                wrapper: quote! { #ident::#variant_ident },
                decoded: quote! { variant },
            });
            let wildcard_pattern = schema.wildcard_pattern();
            quote! {
                variant @ #ident::#variant_ident #wildcard_pattern => {
                    #measure_discriminant + #measure
                }
            }
        });
        quote! {
            match #decoded {
                #(#variants),*
            }
        }
    }
}

