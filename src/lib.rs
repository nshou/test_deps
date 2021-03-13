use proc_macro::TokenStream;
use quote;
use syn::{self, Block, ItemFn};

#[proc_macro_attribute]
pub fn trace(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = syn::parse_macro_input!(item as ItemFn);
    //let ident = &ast.ident;
    let ident = "fake_func_name";

    let body = ast.block.as_ref();
    let body: Block = syn::parse_quote! {{
        println!("in: {}", stringify!(#ident));

        let body = || #body;
        let ret = body();

        println!("out: {}", stringify!(#ident));
        ret
    }};

    *ast.block = body;
    let gen = quote::quote! {
        #ast
    };
    gen.into()
}
