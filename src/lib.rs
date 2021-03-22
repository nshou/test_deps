use proc_macro::TokenStream;
use quote;
use syn::{self, Attribute, Block, ItemFn};

#[proc_macro_attribute]
pub fn deps(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = proc_macro2::TokenStream::from(args);
    let input = proc_macro2::TokenStream::from(input);

    let arg_tokens = verify_args(args);

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

// TODO: Support more special characters e.g. '-' and '.' for target and prereq name
//       As of Mar 2021, Span::{start, end} are nightly and they return always 0 when called through proc_macro2.
//       Due to the restriction, there is no way to distinguish an isolated punctuation from one consisting of a
//       word. For example, today's TokenStream presents "abc.def" and "abc .def" in the same way. This TODO will
//       be revisited once the functions get supported (https://doc.rust-lang.org/proc_macro/struct.Span.html).
fn verify_args(args: proc_macro2::TokenStream) -> Vec<String> {
    let mut illegal_str = None;
    let mut tokens = Vec::new();

    for arg in args.into_iter() {
        tokens.push(arg.to_string());
        match arg {
            proc_macro2::TokenTree::Ident(_) => {}
            proc_macro2::TokenTree::Punct(pt) => {
                if pt.as_char() != ':' {
                    if illegal_str.is_none() {
                        illegal_str = Some(pt.to_string());
                    }
                }
            }
            _ => {
                if illegal_str.is_none() {
                    illegal_str = Some(arg.to_string());
                }
            }
        }
    }

    if let Some(x) = illegal_str {
        panic!("Illegal format: {}", x);
    }

    tokens
}

fn should_retain(attr: &Attribute) -> bool {
    if let Ok(m) = attr.parse_meta() {
        let p = m.path();
        !(p.is_ident("ignore") || p.is_ident("should_panic"))
    } else {
        true
    }
}
