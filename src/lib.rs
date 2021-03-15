use proc_macro::TokenStream;
use quote;
use syn::{self, Attribute, Block, ItemFn};

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

    let mut attrs = ast.attrs;
    attrs.retain(should_retain);
    ast.attrs = attrs;

    let gen = quote::quote! {
        #ast
    };
    gen.into()
}

fn should_retain(attr: &Attribute) -> bool {
    if let Ok(m) = attr.parse_meta() {
        let p = m.path();
        !(p.is_ident("ignore") || p.is_ident("should_panic"))
    } else {
        true
    }
}
