use proc_macro::TokenStream;
use quote::{format_ident, quote, IdentFragment};
use syn::{parse_macro_input, Data, DeriveInput, Field, Index};

#[cfg(not(tarpaulin_include))]
#[proc_macro_derive(Config)]
pub fn make_config(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let methods = match input.data {
        Data::Struct(ref data) => data
            .fields
            .iter()
            .enumerate()
            .map(|(i, f)| match &f.ident {
                Some(name) => build_fns(&f, name),
                None => build_fns(&f, &Index::from(i)),
            })
            .collect::<Vec<_>>(),
        _ => panic!("Config only supports struct"),
    };

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #(#methods)*
        }
    };

    TokenStream::from(expanded)
}

#[cfg(not(tarpaulin_include))]
fn build_fns<T>(f: &Field, name: &T) -> quote::__private::TokenStream
where
    T: IdentFragment + quote::ToTokens,
{
    let set_fn_name = format_ident!("set_{}", name);
    let map_fn_name = format_ident!("map_{}", name);
    let mut_fn_name = format_ident!("mut_{}", name);
    let ty = &f.ty;

    quote! {
        pub fn #set_fn_name(mut self, new_val: #ty) -> Self {
            self.#name = new_val;
            self
        }

        pub fn #map_fn_name<F: FnOnce(#ty) -> #ty>(mut self, f: F) -> Self {
            self.#name = f(self.#name);
            self
        }

        pub fn #mut_fn_name<F: FnOnce(&mut #ty)>(mut self, f: F) -> Self {
            f(&mut self.#name);
            self
        }
    }
}
