#![no_std]

use quote::quote;
use syn::parse_macro_input;
use syn::Token;

#[proc_macro]
pub fn defr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_macro_input!(input as Tts).0.into()
}

struct Tts(proc_macro2::TokenStream);

impl syn::parse::Parse for Tts {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let exprs = syn::punctuated::Punctuated::<syn::Expr, Token![;]>::parse_terminated(input)?
            .into_iter();
        let b = proc_macro2::Ident::new("__body", proc_macro2::Span::mixed_site());
        let d = proc_macro2::Ident::new("__defr", proc_macro2::Span::mixed_site());
        let tts = quote! {
            let #b = || { #(#exprs;)* };
            let #d = {
                #[doc(hidden)]
                struct __Defr<F: std::ops::FnOnce()>(std::option::Option<F>);
                impl<F: std::ops::FnOnce()> std::ops::Drop for __Defr<F> {
                    fn drop(&mut self) {
                        self.0.take().unwrap()();
                    }
                }
                __Defr(std::option::Option::Some(#b))
            };
        };
        Ok(Tts(tts))
    }
}
