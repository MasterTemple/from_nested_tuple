use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Ident, parse_macro_input};

struct FieldEntry<'a> {
    ident: &'a syn::Ident,
    ty: &'a syn::Type,
}

#[proc_macro_derive(FromTuple)]
pub fn from_tuple_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                let mut iter = fields.named.iter().map(|f| FieldEntry {
                    ident: f.ident.as_ref().unwrap(),
                    ty: &f.ty,
                });
                let first = iter.next().expect("Struct should have at least 1 field");
                let first_name = first.ident;
                let first_ty = first.ty;

                let mut unnested_names = quote! { #first_name };
                let mut tuple_type = quote! { #first_ty };
                let mut field_names = vec![first_name];

                for this in iter {
                    let this_name = this.ident;
                    let this_ty = &this.ty;
                    tuple_type = quote! { (#tuple_type, #this_ty) };
                    unnested_names = quote! { (#unnested_names, #this_name) };
                    field_names.push(this.ident);
                }

                quote! {
                    impl #impl_generics from_nested_tuple::FromNestedTuple for #name #ty_generics #where_clause {
                        type Tuple = #tuple_type;

                        fn from_nested_tuple(tuple: Self::Tuple) -> Self {
                            let #unnested_names = tuple;
                            Self {
                                #(#field_names,)*
                            }
                        }
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let mut iter = fields.unnamed.iter().map(|f| &f.ty);
                let first_ty = iter.next().expect("Struct should have at least 1 field");
                let first_name = format_ident!("field0");

                let mut unnested_names = quote! { #first_name };
                let mut tuple_type = quote! { #first_ty };
                let mut field_names: Vec<Ident> = vec![first_name];

                for (idx, this) in iter.enumerate() {
                    let this_name = format_ident!("field{}", idx + 1);
                    let this_ty = this;
                    tuple_type = quote! { (#tuple_type, #this_ty) };
                    unnested_names = quote! { (#unnested_names, #this_name) };
                    field_names.push(this_name);
                }

                quote! {
                    impl #impl_generics from_nested_tuple::FromNestedTuple for #name #ty_generics #where_clause {
                        type Tuple = #tuple_type;

                        fn from_nested_tuple(tuple: Self::Tuple) -> Self {
                            let #unnested_names = tuple;
                            Self(#(#field_names,)*)
                        }
                    }
                }
            }
            _ => panic!("FromTuple cannot be derived from Unit Structs"),
        },
        _ => panic!("FromTuple can only be derived for structs"),
    };

    TokenStream::from(expanded)
}
