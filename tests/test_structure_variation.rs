use std::{thread, time};
use test_deps::deps;

static mut COUNTER_RET_RESULT: usize = 0;

#[deps(RET_RESULT_000)]
#[test]
fn ret_result_000() -> Result<(), ()> {
    thread::sleep(time::Duration::from_secs_f64(0.1));
    unsafe {
        assert_eq!(0, COUNTER_RET_RESULT);
        COUNTER_RET_RESULT = COUNTER_RET_RESULT + 1;
    }
    Ok(())
}

#[deps(RET_RESULT_001: RET_RESULT_000)]
#[test]
fn ret_result_001() -> Result<(), ()> {
    thread::sleep(time::Duration::from_secs_f64(0.05));
    unsafe {
        assert_eq!(1, COUNTER_RET_RESULT);
        COUNTER_RET_RESULT = COUNTER_RET_RESULT + 1;
    }
    Ok(())
}

#[deps(RET_RESULT_002: RET_RESULT_001)]
#[test]
fn ret_result_002() -> Result<(), ()> {
    thread::sleep(time::Duration::from_secs_f64(0.05));
    unsafe {
        assert_eq!(2, COUNTER_RET_RESULT);
        COUNTER_RET_RESULT = COUNTER_RET_RESULT + 1;
    }
    Ok(())
}
