#![allow(non_snake_case)]

extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;

// #[proc_macro_derive(SelfId)]
// pub fn derive_sid(input: TokenStream) -> TokenStream {
//     let item: syn::DeriveInput = syn::parse(input).unwrap();
//     let name = &item.ident;
//     let code = quote! {
//         ImplISId!(#name);
//     };
//     code.into()
// }

//---------------------vclForm----------------------------

#[proc_macro_derive(VclForm)]
pub fn derive_vclForm(input: TokenStream) -> TokenStream {
    let item: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &item.ident;
    let code = quote! {
        ImplForm!(#name);
    };
    code.into()
}

//---------------------vclApp----------------------------

#[proc_macro_derive(VclApp)]
pub fn derive_vclApp(input: TokenStream) -> TokenStream {
    let item: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &item.ident;
    let code = quote! {
        // ImplISId!(#name);
        ImplIApplication!(#name);
        // 有需要再添加
    };
    code.into()
}
