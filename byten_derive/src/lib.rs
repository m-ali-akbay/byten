use proc_macro::TokenStream;
use syn::DeriveInput;
use quote::quote;

mod schema;
use schema::*;

#[proc_macro_derive(DecodeOwned, attributes(byten))]
pub fn derive_decode_owned(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let ident = &input.ident;
    let generics = &input.generics;

    let schema = match &input.data {
        syn::Data::Struct(_) => interpret_struct_schema(&input),
        syn::Data::Enum(_) => interpret_enum_schema(&input),
        _ => panic!("DecodeOwned can only be derived for structs and enums"),
    };

    let decoded = schema.decode(&DecodeContext {
        encoded: quote! { encoded },
        offset: quote! { offset },
    });

    quote! {
        impl #generics ::byten::Decode<'_> for #ident #generics {
            fn decode(encoded: &'_ [u8], offset: &mut usize) -> Result<Self, ::byten::DecodeError> {
                Ok(#decoded)
            }
        }
    }.into()
}

#[proc_macro_derive(Decode, attributes(byten))]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let ident = &input.ident;
    let generics = &input.generics;

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
        impl #generics ::byten::Decode<'encoded> for #ident #generics {
            fn decode(encoded: &'encoded [u8], offset: &mut usize) -> Result<Self, ::byten::DecodeError> {
                Ok(#decoded)
            }
        }
    }.into()
}

#[proc_macro_derive(Encode, attributes(byten))]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let ident = &input.ident;
    let generics = &input.generics;

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
        impl #generics ::byten::Encode for #ident #generics {
            fn encode(&self, encoded: &mut [u8], offset: &mut usize) -> Result<(), ::byten::EncodeError> {
                #encoded
                Ok(())
            }
        }
    }.into()
}

#[proc_macro_derive(MeasureFixed, attributes(byten))]
pub fn derive_measure_fixed(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let ident = &input.ident;
    let generics = &input.generics;

    let schema = match &input.data {
        syn::Data::Struct(_) => interpret_struct_schema(&input),
        syn::Data::Enum(_) => interpret_enum_schema(&input),
        _ => panic!("MeasureFixed can only be derived for structs and enums"),
    };

    let measured = schema.measure_fixed();

    quote! {
        impl #generics ::byten::MeasureFixed for #ident #generics {
            fn measure_fixed() -> usize {
                #measured
            }
        }

        impl #generics ::byten::Measure for #ident #generics {
            fn measure(&self) -> Result<usize, ::byten::EncodeError> {
                Ok(Self::measure_fixed())
            }
        }
    }.into()
}

#[proc_macro_derive(Measure, attributes(byten))]
pub fn derive_measure(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let ident = &input.ident;
    let generics = &input.generics;

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
        impl #generics ::byten::Measure for #ident #generics {
            fn measure(&self) -> Result<usize, ::byten::EncodeError> {
                Ok(#measured)
            }
        }
    }.into()
}
