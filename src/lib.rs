use proc_macro::TokenStream;
use quote;
use syn::{self, Attribute, Block, ItemFn};

#[proc_macro_attribute]
pub fn deps(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = proc_macro2::TokenStream::from(args);
    let input = proc_macro2::TokenStream::from(input);

    let mut ast: ItemFn = syn::parse2(input).unwrap();
    let ident = "fake_name";

    let body_orig = ast.block.as_ref();
    let body_new: Block = syn::parse_quote! {{
        struct Ticket;
        impl Drop for Ticket {
            fn drop(&mut self) {
                println!("out: {}", stringify!(#ident));
            }
        }
        let t = Ticket;
        println!("in: {}", stringify!(#ident));
        #body_orig
    }};
    *ast.block = body_new;

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
