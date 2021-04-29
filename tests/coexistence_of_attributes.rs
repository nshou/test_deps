use std::{thread, time};
use test_deps::deps;

static mut BOOL_IGN_OK: bool = false;

#[deps(IGN_OK_000)]
#[test]
fn ignore_attribute_satisfies_target_immediately_000() {
    thread::sleep(time::Duration::from_millis(250));
    unsafe {
        assert!(!BOOL_IGN_OK);
        BOOL_IGN_OK = true;
    }
}

#[deps(IGN_OK_001: IGN_OK_000)]
#[ignore]
#[test]
fn ignore_attribute_satisfies_target_immediately_001() {
    panic!("you should not come here");
}

#[deps(IGN_OK_002: IGN_OK_001)]
#[test]
fn ignore_attribute_satisfies_target_immediately_002() {
    unsafe {
        assert!(BOOL_IGN_OK);
        BOOL_IGN_OK = false;
    }
}
