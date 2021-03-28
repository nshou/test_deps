use lazy_static::lazy_static;
use proc_macro::TokenStream;
use quote;
use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};
use syn::{self, Attribute, Block, ItemFn};

lazy_static! {
    static ref TEST_DEPS_REG: Mutex<(
        HashMap<String, ()>,
        HashMap<String, (Vec<String>, Arc<Condvar>)>,
        HashMap<String, Vec<String>>
    )> = {
        let completed = HashMap::new();
        let waitlist = HashMap::new();
        let r_waitlist = HashMap::new();
        Mutex::new((completed, waitlist, r_waitlist))
    };
}

#[proc_macro_attribute]
pub fn deps(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = proc_macro2::TokenStream::from(args);
    let input = proc_macro2::TokenStream::from(input);

    let arg_tokens = verify_args_text(args);
    let (target, prereqs) = verify_args_format(&arg_tokens);

    let mut ast: ItemFn = syn::parse2(input).unwrap();

    let body_orig = ast.block.as_ref();
    let body_new: Block = syn::parse_quote! {{
        struct Ticket;
        impl Drop for Ticket {
            fn drop(&mut self) {
                let target = String::from(#target);
                let mut reg = TEST_DEPS_REG.lock().unwrap();
                if reg.0.insert(target, ()).is_some() { //reg.0 = completed
                    panic!("Illegal dependencies: Duplicated target {}", target);
                }
                if let Some(wl) = reg.2.get(target) { //reg.2 = r_waitlist
                    for w in &wl {
                        if let Some((prq, cv)) = reg.1.get_mut(w) { //reg.1 = waitlist
                            let curlen = prq.len();
                            prq.retain(|&n| n != target);
                            if curlen - prq.len() != 1 {
                                panic!("Runtime error: Discrepancy between waitlist and reverse waitlist on {} that depends on {}", w, target);
                            }
                            if prq.is_empty() {
                                cv.notify_one();
                            }
                        } else {
                            panic!("Runtime error: Discrepancy between waitlist and reverse waitlist on {} that depends on {}", w, target);
                        }
                    }
                }
            }
        }
        let t = Ticket;
        {
            let target = String::from(#target);
            let mut prereqs: Vec<String> = vec![#(String::from(#prereqs)),*];
            let mut reg = TEST_DEPS_REG.lock().unwrap();
            prereqs.retain(|&n| !reg.0.contains_key(n)); //reg.0 = completed
            if !prereqs.is_empty() {
                let cv = std::sync::Arc::new(std::sync::Condvar::new());
                let cv2 = cv.clone();
                if reg.1.insert(target, (prereqs, cv2)).is_some() { //reg.1 = waitlist
                    panic!("Illegal dependencies: Duplicated target {}", target);
                }
                for prereq in &prereqs {
                    if let Some(n) = reg.2.get_mut(prereq) { //reg.2 = r_waitlist
                        // Skip duplication check here bc it's already done above
                        (*n).push(target);
                    } else {
                        reg.2.insert(prereq, vec![target]);
                    }
                }
                //TODO: no unwrap here
                while !(*reg).1.get(target).unwrap().0.is_empty() { //~~.0 = prereqs vec
                    reg = cv.wait(reg).unwrap();
                }
            }
        }
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

fn should_retain(attr: &Attribute) -> bool {
    if let Ok(m) = attr.parse_meta() {
        let p = m.path();
        !(p.is_ident("ignore") || p.is_ident("should_panic"))
    } else {
        true
    }
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
}
