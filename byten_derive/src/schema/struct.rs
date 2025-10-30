use syn::{Data, DeriveInput, Ident};
use quote::quote;

use super::{BinarySchema, DecodeContext, EncodeContext, MeasureContext, interpret_fields_schema};

pub fn interpret_struct_schema(input: &DeriveInput) -> Box<dyn BinarySchema> {
    let Data::Struct(ref data) = input.data else {
        panic!("StructSchema can only be created from struct data");
    };
    Box::new(StructSchema {
        ident: input.ident.clone(),
        fields: interpret_fields_schema(&data.fields),
    })
}

struct StructSchema {
    ident: Ident,
    fields: Box<dyn BinarySchema>,
}

impl BinarySchema for StructSchema {
    fn decode(&self, ctx: &DecodeContext) -> proc_macro2::TokenStream {
        let ident = &self.ident;
        let fields = self.fields.decode(&ctx.clone());
        quote! { #ident #fields }
    }

    fn encode(&self, ctx: &EncodeContext) -> proc_macro2::TokenStream {
        self.fields.encode(&ctx.clone())
    }

    fn measure(&self, ctx: &MeasureContext) -> proc_macro2::TokenStream {
        self.fields.measure(&ctx.clone())
    }

    fn fixed_measure(&self) -> proc_macro2::TokenStream {
        self.fields.fixed_measure()
    }
}

