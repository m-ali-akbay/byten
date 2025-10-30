use proc_macro2::Span;
use syn::{Fields, FieldsNamed, Ident};
use quote::{ToTokens, quote};

use crate::{interpret_codec_schema, parse_byten_attribute};

use super::{BinarySchema, DecodeContext, EncodeContext, MeasureContext};

pub trait FieldsSchema: BinarySchema {
    fn wildcard_pattern(&self) -> proc_macro2::TokenStream;
}

pub fn interpret_fields_schema(fields: &Fields) -> Box<dyn FieldsSchema> {
    match fields {
        Fields::Named(fields) => Box::new(NamedFieldsSchema::interpret(fields)),
        Fields::Unnamed(fields) => Box::new(UnnamedFieldsSchema::interpret(fields)),
        Fields::Unit => Box::new(UnitFieldsSchema {}),
    }
}

struct NamedFieldsSchema {
    fields: Vec<(Ident, Box<dyn BinarySchema>)>,
}

impl FieldsSchema for NamedFieldsSchema {
    fn wildcard_pattern(&self) -> proc_macro2::TokenStream {
        quote! { { .. } }
    }
}

impl NamedFieldsSchema {
    fn interpret(fields: &FieldsNamed) -> NamedFieldsSchema {
        let fields = fields.named.iter().map(|field| {
            let ident = field.ident.clone().expect("Named field must have an identifier");
            let ty = &field.ty;
            let codec_path = parse_byten_attribute(&field.attrs).unwrap_or_else(|| syn::parse_quote!{
                ::byten::SelfCodec::<#ty>::default()
            });
            let codec = interpret_codec_schema(&codec_path);
            (ident, codec)
        }).collect();
        NamedFieldsSchema {
            fields,
        }
    }
}

impl BinarySchema for NamedFieldsSchema {
    fn decode(&self, ctx: &DecodeContext) -> proc_macro2::TokenStream {
        let fields = self.fields.iter().map(|(ident, schema)| {
            let decode = schema.decode(&ctx.clone());
            quote! { #ident: #decode }
        });
        quote! { { #(#fields),* } }
    }

    fn encode(&self, ctx: &EncodeContext) -> proc_macro2::TokenStream {
        let wrapper = &ctx.decoded;
        let type_path = &ctx.wrapper;
        let idents = self.fields.iter().map(|(ident, _)| ident).collect::<Vec<_>>();
        let variables = idents.iter()
            .map(|ident| Ident::new(format!("variant_{}", ident.to_string()).as_str(), ident.span()))
            .collect::<Vec<_>>();
        let encodes = self.fields.iter().zip(variables.iter()).map(|((_, schema), variable)| {
            let encode = schema.encode(&EncodeContext {
                wrapper: quote! {},
                decoded: variable.into_token_stream(),
                encoded: ctx.encoded.clone(),
                offset: ctx.offset.clone(),
            });
            encode
        });
        quote! { 
            let #type_path { #(#idents: #variables,)* } = #wrapper else { unreachable!() };
            #(#encodes;)*
        }
    }

    fn measure(&self, ctx: &MeasureContext) -> proc_macro2::TokenStream {
        let wrapper = &ctx.decoded;
        let type_path = &ctx.wrapper;
        let idents = self.fields.iter().map(|(ident, _)| ident).collect::<Vec<_>>();
        let variables = idents.iter()
            .map(|ident| Ident::new(format!("variant_{}", ident.to_string()).as_str(), ident.span()))
            .collect::<Vec<_>>();
        let measures = self.fields.iter().zip(variables.iter()).map(|((_, schema), variable)| {
            let measure = schema.measure(&MeasureContext {
                wrapper: quote! {},
                decoded: variable.into_token_stream(),
            });
            measure
        });
        quote! { {
            let #type_path { #(#idents: #variables,)* } = #wrapper else { unreachable!() };
            0 #( + #measures )*
        } }
    }
}

struct UnnamedFieldsSchema {
    fields: Vec<Box<dyn BinarySchema>>,
}

impl FieldsSchema for UnnamedFieldsSchema {
    fn wildcard_pattern(&self) -> proc_macro2::TokenStream {
        quote! { ( .. ) }
    }
}

impl UnnamedFieldsSchema {
    fn interpret(fields: &syn::FieldsUnnamed) -> UnnamedFieldsSchema {
        let fields = fields.unnamed.iter().map(|field| {
            if field.ident.is_some() { panic!("Unnamed field must not have an identifier"); }
            let ty = &field.ty;
            let codec_path = parse_byten_attribute(&field.attrs).unwrap_or_else(|| syn::parse_quote!{
                ::byten::SelfCodec::<#ty>::default()
            });
            interpret_codec_schema(&codec_path)
        }).collect();
        UnnamedFieldsSchema { fields }
    }
}

impl BinarySchema for UnnamedFieldsSchema {
    fn decode(&self, ctx: &DecodeContext) -> proc_macro2::TokenStream {
        let fields = self.fields.iter().map(|schema| {
            schema.decode(&ctx.clone())
        });
        quote! { ( #(#fields),* ) }
    }

    fn encode(&self, ctx: &EncodeContext) -> proc_macro2::TokenStream {
        let decoded = &ctx.decoded;
        let wrapper = &ctx.wrapper;
        let variables = self.fields.iter()
            .enumerate()
            .map(|(index, _)| Ident::new(format!("variant_{}", index).as_str(), Span::call_site()))
            .collect::<Vec<_>>();
        let encodes = self.fields.iter().zip(variables.iter()).map(|(schema, variable)| {
            let encode = schema.encode(&EncodeContext {
                wrapper: quote! {},
                decoded: variable.into_token_stream(),
                encoded: ctx.encoded.clone(),
                offset: ctx.offset.clone(),
            });
            encode
        });
        quote! { 
            let #wrapper ( #(#variables),* ) = #decoded else { unreachable!() };
            #(#encodes;)*
        }
    }

    fn measure(&self, ctx: &MeasureContext) -> proc_macro2::TokenStream {
        let decoded = &ctx.decoded;
        let wrapper = &ctx.wrapper;
        let variables = self.fields.iter()
            .enumerate()
            .map(|(index, _)| Ident::new(format!("variant_{}", index).as_str(), Span::call_site()))
            .collect::<Vec<_>>();
        let measures = self.fields.iter().zip(variables.iter()).map(|(schema, variable)| {
            let measure = schema.measure(&MeasureContext {
                wrapper: quote! {},
                decoded: variable.into_token_stream(),
            });
            measure
        });
        quote! { {
            let #wrapper ( #(#variables),* ) = #decoded else { unreachable!() };
            0 #( + #measures )*
        } }
    }
}

struct UnitFieldsSchema {}

impl FieldsSchema for UnitFieldsSchema {
    fn wildcard_pattern(&self) -> proc_macro2::TokenStream {
        quote! {}
    }
}

impl BinarySchema for UnitFieldsSchema {
    fn decode(&self, _ctx: &DecodeContext) -> proc_macro2::TokenStream {
        quote! {}
    }

    fn encode(&self, _ctx: &EncodeContext) -> proc_macro2::TokenStream {
        quote! {}
    }

    fn measure(&self, _ctx: &MeasureContext) -> proc_macro2::TokenStream {
        quote! { 0 }
    }
}

