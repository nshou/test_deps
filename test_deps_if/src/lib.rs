//! A helper crate for `test_deps`

use proc_macro::TokenStream;
use quote;
use std::collections::HashMap;
use syn::{self, Attribute, Block, ItemFn};

/// Defines the test dependency
///
/// ## Argument
/// This macro takes GNU Make-like syntax as its argument that is described as:
/// ```notrust
/// target [: prereq_0 [prereq_1 ... prereq_n] ]
/// ```
/// *Target* is the alias of the test that the macro is applied to. *Prereq_k* are the aliases that the
/// *target* waits for. It is guaranteed that the *target* only begins after all the *prereq_k* finish.
/// For example, `#[deps(A)]` defines a test alias *A* that has no prerequisites, which means *A* immediately
/// starts when you hit `cargo test`. `#[deps(A: B C)]` is another example where *A* waits until *B* and *C*
/// complete. Note that this macro doesn't care whether the tests succeeded or failed. In the example of
/// `#[deps(A: B C)]`, *A* will begin even if *B* and *C* failed in their tests.
///
/// The available characters for *target* and *prereq* are `a-zA-Z0-9` and underscore (`_`). It may not
/// start with a digit.
///
/// ## Panic
/// ### At compile time
/// - Unsupported character is used for *target* or *prereq*
/// - Argument is not legally formatted
/// - Duplicated *prereq* alias is specified
/// - Same alias appears in both *target* and *prereq*
/// ### At run time
/// - *Target* with same alias completed twice
#[proc_macro_attribute]
pub fn deps(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = proc_macro2::TokenStream::from(args);
    let input = proc_macro2::TokenStream::from(input);

    // TODO: proc_macro2::TokenStream impls only into_iter(self).
    //       Not clone but borrow it once iter(&self) implemented.
    let arg_tokens = verify_args_text(args.clone());
    let (target, prereqs) = verify_args_format(&arg_tokens);

    let mut ast: ItemFn = syn::parse2(input).unwrap();

    let body_orig = ast.block.as_ref();
    let body_new: Block = syn::parse_quote! {{
        struct Ticket;
        impl Drop for Ticket {
            fn drop(&mut self) {
                let target = String::from(#target);
                test_deps::target_completed(&target).unwrap();
            }
        }
        let t = Ticket;
        {
            let prereqs: Vec<String> = vec![#(String::from(#prereqs)),*];
            test_deps::ensure_prereqs(&prereqs).unwrap();
        }
        #body_orig
    }};
    *ast.block = body_new;

    let mut gen = quote::quote! {
        #ast
    };

    if is_ignored_test(&ast.attrs) {
        let dummy_fn = format!("__dummy__{}", ast.sig.ident);
        let dummy_fn_ident = proc_macro2::Ident::new(&dummy_fn, proc_macro2::Span::call_site());
        gen = quote::quote! {
            #[deps(#args)]
            #[test]
            fn #dummy_fn_ident(){}

            #gen
        };
    }

    gen.into()
}

// TODO: Support more special characters e.g. '-' and '.' for target and prereq name
//       As of Mar 2021, Span::{start, end} are nightly and they return always 0 when called through proc_macro2.
//       Due to the restriction, there is no way to distinguish an isolated punctuation from one consisting of a
//       word. For example, today's TokenStream presents "abc.def" and "abc .def" in the same way. This TODO will
//       be revisited once the functions get supported (https://doc.rust-lang.org/proc_macro/struct.Span.html).
fn verify_args_text(args: proc_macro2::TokenStream) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut illegal_str = None;
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
        panic!("Illegal string: {}", x);
    }

    tokens
}

fn verify_args_format(tokens: &Vec<String>) -> (&String, &[String]) {
    if tokens.len() == 0 {
        panic!("Illegal format: Missing target name");
    } else {
        let mut tokiter = tokens.iter();
        let icolon = tokiter.position(|x| x == ":");
        if let Some(i) = icolon {
            if tokiter.position(|x| x == ":").is_some() {
                panic!("Illegal format: Separator ':' should appear at most once");
            }
            if i == 0 {
                panic!("Illegal format: Missing target name");
            } else if i == tokens.len() - 1 {
                panic!("Illegal format: Missing prereq names");
            } else if i != 1 {
                panic!("Illegal format: Target name should appear only once");
            }
        } else {
            if tokens.len() != 1 {
                panic!("Illegal format: Target name should appear only once");
            }
        }
    }
    let mut counts = HashMap::new();
    for i in 1..tokens.len() {
        if counts.insert(&tokens[i], ()).is_some() {
            panic!("Illegal format: Duplicated prereq {}", tokens[i]);
        }
    }
    if counts.contains_key(&tokens[0]) {
        panic!(
            "Illegal format: {} appears in both target and prereq",
            tokens[0]
        );
    }

    let mut sepiter = tokens.split(|s| s == ":");
    let target = &sepiter.next().unwrap()[0];
    let prereqs = sepiter.next().unwrap_or(&[]);
    (target, prereqs)
}

fn is_ignored_test(attrs: &Vec<Attribute>) -> bool {
    for attr in attrs {
        if let Ok(m) = attr.parse_meta() {
            if m.path().is_ident("ignore") {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn valid_names() {
        let names = [
            "a",
            "ab",
            "_",
            "__",
            "a_",
            "_a",
            "_a_",
            "a_a",
            "a0",
            "a0a",
            "_0",
            "_0_",
            "a0_",
            "a_0",
            "_a0",
            "_0a",
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_",
        ];
        for name in &names {
            assert_eq!(
                vec![String::from(*name)],
                verify_args_text(proc_macro2::TokenStream::from_str(name).unwrap())
            );
        }
    }

    #[test]
    #[should_panic(expected = "Illegal string: 0a")]
    fn invalid_name_starts_with_digit() {
        verify_args_text(proc_macro2::TokenStream::from_str("0a").unwrap());
    }

    #[test]
    #[should_panic(expected = "Illegal string: !")]
    fn invalid_name_contains_special_char() {
        verify_args_text(proc_macro2::TokenStream::from_str("a!").unwrap());
    }

    #[test]
    fn valid_tokens() {
        let one_tgt = vec![String::from("a")];
        let one_tgt_one_prq = vec![String::from("a"), String::from(":"), String::from("b")];
        let one_tgt_two_prqs = vec![
            String::from("a"),
            String::from(":"),
            String::from("b"),
            String::from("c"),
        ];
        assert_eq!(verify_args_format(&one_tgt), (&one_tgt[0], &one_tgt[1..]));
        assert_eq!(
            verify_args_format(&one_tgt_one_prq),
            (&one_tgt_one_prq[0], &one_tgt_one_prq[2..])
        );
        assert_eq!(
            verify_args_format(&one_tgt_two_prqs),
            (&one_tgt_two_prqs[0], &one_tgt_two_prqs[2..])
        );
    }

    #[test]
    #[should_panic(expected = "Illegal format: Missing target name")]
    fn invalid_tokens_empty() {
        verify_args_format(&vec![]);
    }

    #[test]
    #[should_panic(expected = "Illegal format: Missing target name")]
    fn invalid_tokens_single_colon() {
        verify_args_format(&vec![String::from(":")]);
    }

    #[test]
    #[should_panic(expected = "Illegal format: Missing prereq names")]
    fn invalid_tokens_no_prereq() {
        verify_args_format(&vec![String::from("a"), String::from(":")]);
    }

    #[test]
    #[should_panic(expected = "Illegal format: Missing target name")]
    fn invalid_tokens_no_target() {
        verify_args_format(&vec![String::from(":"), String::from("a")]);
    }

    #[test]
    #[should_panic(expected = "Illegal format: Target name should appear only once")]
    fn invalid_tokens_double_targets() {
        verify_args_format(&vec![String::from("a"), String::from("b")]);
    }

    #[test]
    #[should_panic(expected = "Illegal format: Separator ':' should appear at most once")]
    fn invalid_tokens_double_colons() {
        verify_args_format(&vec![String::from(":"), String::from(":")]);
    }

    #[test]
    #[should_panic(expected = "Illegal format: Target name should appear only once")]
    fn invalid_tokens_double_targets_with_prereq() {
        verify_args_format(&vec![
            String::from("a"),
            String::from("b"),
            String::from(":"),
            String::from("c"),
        ]);
    }

    #[test]
    #[should_panic(expected = "Illegal format: Separator ':' should appear at most once")]
    fn invalid_tokens_two_colons() {
        verify_args_format(&vec![
            String::from("a"),
            String::from(":"),
            String::from("b"),
            String::from(":"),
        ]);
    }

    #[test]
    #[should_panic(expected = "Illegal format: Duplicated prereq b")]
    fn invalid_tokens_dup_prereqs() {
        verify_args_format(&vec![
            String::from("a"),
            String::from(":"),
            String::from("b"),
            String::from("b"),
        ]);
    }

    #[test]
    #[should_panic(expected = "Illegal format: a appears in both target and prereq")]
    fn invalid_tokens_deps_loop() {
        verify_args_format(&vec![
            String::from("a"),
            String::from(":"),
            String::from("b"),
            String::from("a"),
        ]);
    }
}
