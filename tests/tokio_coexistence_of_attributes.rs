use test_deps::deps;
use tokio::time::{self, Duration};

static mut BOOL_IGN_OK: bool = false;

#[deps(IGN_OK_000)]
#[tokio::test]
async fn tokio_with_ignore_attribute_000() {
    time::sleep(Duration::from_millis(250)).await;
    unsafe {
        assert!(!BOOL_IGN_OK);
        BOOL_IGN_OK = true;
    }
}

#[deps(IGN_OK_001: IGN_OK_000)]
#[ignore]
#[tokio::test]
async fn tokio_with_ignore_attribute_001() {
    unsafe {
        BOOL_IGN_OK = false;
    }
    panic!("you should not come here");
}

#[deps(IGN_OK_002: IGN_OK_001)]
#[tokio::test]
async fn tokio_with_ignore_attribute_002() {
    unsafe {
        assert!(BOOL_IGN_OK);
        BOOL_IGN_OK = false;
    }
}

static mut COUNTER_SHOULD_PANIC_OK: usize = 0;

#[deps(SHOULD_PANIC_OK_000)]
#[tokio::test]
async fn tokio_with_should_panic_attribute_000() {
    unsafe {
        assert_eq!(0, COUNTER_SHOULD_PANIC_OK);
        COUNTER_SHOULD_PANIC_OK = COUNTER_SHOULD_PANIC_OK + 1;
    }
}

#[deps(SHOULD_PANIC_OK_001: SHOULD_PANIC_OK_000)]
#[should_panic]
#[tokio::test]
async fn tokio_with_should_panic_attribute_001() {
    time::sleep(Duration::from_millis(250)).await;
    unsafe {
        assert_eq!(1, COUNTER_SHOULD_PANIC_OK);
        COUNTER_SHOULD_PANIC_OK = COUNTER_SHOULD_PANIC_OK + 1;
    }
    panic!("this is fine");
    #[allow(unreachable_code)]
    unsafe {
        COUNTER_SHOULD_PANIC_OK = COUNTER_SHOULD_PANIC_OK + 1;
    }
}

#[deps(SOULD_PANIC_OK_002: SHOULD_PANIC_OK_001)]
#[tokio::test]
async fn tokio_with_should_panic_attribute_002() {
    unsafe {
        assert_eq!(2, COUNTER_SHOULD_PANIC_OK);
        COUNTER_SHOULD_PANIC_OK = COUNTER_SHOULD_PANIC_OK + 1;
    }
}

static mut BOOL_IGN_SHOULD_PANIC_OK: bool = false;

#[deps(IGN_SHOULD_PANIC_OK_000)]
#[tokio::test]
async fn tokio_with_ignore_and_should_panic_attribute_000() {
    time::sleep(Duration::from_millis(250)).await;
    unsafe {
        assert!(!BOOL_IGN_SHOULD_PANIC_OK);
        BOOL_IGN_SHOULD_PANIC_OK = true;
    }
}

#[deps(IGN_SHOULD_PANIC_OK_001: IGN_SHOULD_PANIC_OK_000)]
#[ignore]
#[should_panic]
#[tokio::test]
async fn tokio_with_ignore_and_should_panic_attribute_001() {
    unsafe {
        BOOL_IGN_SHOULD_PANIC_OK = false;
    }
}

#[deps(IGN_SHOULD_PANIC_OK_002: IGN_SHOULD_PANIC_OK_001)]
#[tokio::test]
async fn tokio_with_ignore_and_should_panic_attribute_002() {
    unsafe {
        assert!(BOOL_IGN_SHOULD_PANIC_OK);
        BOOL_IGN_SHOULD_PANIC_OK = false;
    }
}
