use proc_macro::TokenStream;
use syn::DeriveInput;
use quote::quote;

mod schema;
use schema::*;

#[proc_macro_derive(Decode, attributes(byten))]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let ident = &input.ident;

    let schema = match &input.data {
        syn::Data::Struct(_) => interpret_struct_schema(&input),
        syn::Data::Enum(_) => interpret_enum_schema(&input),
        _ => panic!("Decode can only be derived for structs and enums"),
    };

    let decoded = schema.decode(&DecodeContext {
        encoded: quote! { encoded },
        offset: quote! { offset },
    });

    quote! {
        impl ::byten::Decode for #ident {
            fn decode(encoded: &[u8], offset: &mut usize) -> Result<Self, ::byten::DecodeError> {
                Ok(#decoded)
            }
        }
    }.into()
}

#[proc_macro_derive(Encode, attributes(byten))]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let ident = &input.ident;

    let schema = match &input.data {
        syn::Data::Struct(_) => interpret_struct_schema(&input),
        syn::Data::Enum(_) => interpret_enum_schema(&input),
        _ => panic!("Encode can only be derived for structs and enums"),
    };

    let encoded = schema.encode(&EncodeContext {
        wrapper: quote! { Self },
        decoded: quote! { self },
        encoded: quote! { encoded },
        offset: quote! { offset },
    });

    quote! {
        impl ::byten::Encode for #ident {
            fn encode(&self, encoded: &mut [u8], offset: &mut usize) -> Result<(), ::byten::EncodeError> {
                #encoded
                Ok(())
            }
        }
    }.into()
}

#[proc_macro_derive(FixedMeasure, attributes(byten))]
pub fn derive_measure_fixed(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let ident = &input.ident;

    let schema = match &input.data {
        syn::Data::Struct(_) => interpret_struct_schema(&input),
        syn::Data::Enum(_) => interpret_enum_schema(&input),
        _ => panic!("FixedMeasure can only be derived for structs and enums"),
    };

    let measured = schema.fixed_measure();

    quote! {
        impl ::byten::FixedMeasure for #ident {
            fn fixed_measure() -> usize {
                #measured
            }
        }
    }.into()
}

#[proc_macro_derive(Measure, attributes(byten))]
pub fn derive_measure(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let ident = &input.ident;

    let schema = match &input.data {
        syn::Data::Struct(_) => interpret_struct_schema(&input),
        syn::Data::Enum(_) => interpret_enum_schema(&input),
        _ => panic!("Measure can only be derived for structs and enums"),
    };

    let measured = schema.measure(&MeasureContext {
        wrapper: quote! { Self },
        decoded: quote! { self },
    });

    quote! {
        impl ::byten::Measure for #ident {
            fn measure(&self) -> usize {
                #measured
            }
        }
    }.into()
}
