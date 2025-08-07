use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(FromTuple)]
pub fn from_tuple_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("FromTuple can only be derived for structs with named fields"),
        },
        _ => panic!("FromTuple can only be derived for structs"),
    };

    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();

    let mut iter = fields.into_iter();

    let first = iter.next().expect("Struct should have at least 1 field");
    let first_name = first.ident.as_ref().unwrap();
    let first_ty = &first.ty;

    let mut unnested_names = quote! { #first_name };
    let mut tuple_type = quote! { #first_ty };

    for this in iter {
        let this_name = this.ident.as_ref().unwrap();
        let this_ty = &this.ty;
        tuple_type = quote! { (#tuple_type, #this_ty) };
        unnested_names = quote! { (#unnested_names, #this_name) };
    }

    let expanded = quote! {
        impl #impl_generics from_nested_tuple::FromTuple for #name #ty_generics #where_clause {
            type Tuple = #tuple_type;

            fn from_tuple(tuple: Self::Tuple) -> Self {
                let #unnested_names = tuple;
                Self {
                    #(#field_names,)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
