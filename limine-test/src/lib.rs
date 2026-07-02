use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::Fields;
use syn::Ident;
use syn::ItemStruct;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn test_layout(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(item as ItemStruct);
    let c_type = parse_macro_input!(attr as Ident);
    let rust_type = &input_struct.ident;
    let test_name = format_ident!("{}Test", rust_type);

    let Fields::Named(fields) = &input_struct.fields else {
        panic!("test_layout only supports structs with named fields");
    };

    let field_checks = fields.named.iter().filter_map(|field| {
        let field = field.ident.as_ref()?;

        (field != "header").then(|| {
            quote! {
                assert_eq!(
                    core::mem::offset_of!(#rust_type, #field),
                    core::mem::offset_of!(crate::bindings::#c_type, #field),
                );

                assert_eq!(
                    crate::field_size!(#rust_type, #field),
                    crate::field_size!(crate::bindings::#c_type, #field),
                );
            }
        })
    });

    let expanded = quote! {
        #input_struct

        #[cfg(test)]
        #[test]
        fn #test_name() {
            use super::*;

            assert_eq!(
                core::mem::size_of::<#rust_type>(),
                core::mem::size_of::<crate::bindings::#c_type>(),
            );

            assert_eq!(
                core::mem::align_of::<#rust_type>(),
                core::mem::align_of::<crate::bindings::#c_type>(),
            );

            #(#field_checks)*
        }
    };

    TokenStream::from(expanded)
}
