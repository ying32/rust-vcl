#![allow(non_snake_case)]

extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;

fn impl_sid(item: &syn::DeriveInput) -> TokenStream {
    let name = &item.ident;
    let code = quote! {
        ImplISId!(#name);
    };
    code.into()
}

#[proc_macro_derive(SelfId)]
pub fn derive_sid(input: TokenStream) -> TokenStream {
    let item = syn::parse(input).unwrap();
    impl_sid(&item)
}

//---------------------vclForm----------------------------

fn impl_vclForm(item: &syn::DeriveInput) -> TokenStream {
    let name = &item.ident;
    let code = quote! {
        ImplForm!(#name);
    };
    code.into()
}

#[proc_macro_derive(VclForm)]
pub fn derive_vclForm(input: TokenStream) -> TokenStream {
    let item = syn::parse(input).unwrap();
    impl_vclForm(&item)
}

//---------------------vclApp----------------------------
fn impl_vclApp(item: &syn::DeriveInput) -> TokenStream {
    let name = &item.ident;
    let code = quote! {
        ImplISId!(#name);
        ImplIApplication!(#name);
        // 有需要再添加
    };
    code.into()
}

#[proc_macro_derive(VclApp)]
pub fn derive_vclApp(input: TokenStream) -> TokenStream {
    let item = syn::parse(input).unwrap();
    impl_vclApp(&item)
}
