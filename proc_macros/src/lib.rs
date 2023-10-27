use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data, DataStruct, Fields};

#[proc_macro_derive(From)]
pub fn derive_from(input: TokenStream) -> TokenStream {
    let tree = syn::parse(input).unwrap();
    impl_from(&tree)
}

fn impl_from(tree: &syn::DeriveInput) -> TokenStream {
    let name = &tree.ident;
    let fields = match &tree.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let field_name = fields.iter().map(|field| &field.ident);

    let mut return_code = quote!();

    for name in field_name {
        return_code.extend(quote!(
            #name,

        ));
    }

    let mut fields_fn = quote!();

    for field in fields {
        let name = &field.ident;
        fields_fn.extend(quote!(#name: ));

        let name = &field.ty;
        fields_fn.extend(quote!(#name, ));
    }

    let gen = quote! {
        impl #name {
            fn from(#fields_fn) -> Self {
                Self {
                #return_code
                }
            }
        }
    };
    gen.into()
}
