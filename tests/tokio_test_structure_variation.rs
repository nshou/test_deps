use test_deps::deps;
use tokio::time::{self, Duration};

static mut COUNTER_RET_RESULT: usize = 0;

#[deps(RET_RESULT_000)]
#[tokio::test]
async fn tokio_ret_result_000() -> Result<(), ()> {
    time::sleep(Duration::from_secs_f64(0.1)).await;
    unsafe {
        assert_eq!(0, COUNTER_RET_RESULT);
        COUNTER_RET_RESULT = COUNTER_RET_RESULT + 1;
    }
    Ok(())
}

#[deps(RET_RESULT_001: RET_RESULT_000)]
#[tokio::test]
async fn tokio_ret_result_001() -> Result<(), ()> {
    time::sleep(Duration::from_secs_f64(0.05)).await;
    unsafe {
        assert_eq!(1, COUNTER_RET_RESULT);
        COUNTER_RET_RESULT = COUNTER_RET_RESULT + 1;
    }
    Ok(())
}

#[deps(RET_RESULT_002: RET_RESULT_001)]
#[tokio::test]
async fn tokio_ret_result_002() -> Result<(), ()> {
    time::sleep(Duration::from_secs_f64(0.025)).await;
    unsafe {
        assert_eq!(2, COUNTER_RET_RESULT);
        COUNTER_RET_RESULT = COUNTER_RET_RESULT + 1;
    }
    Ok(())
}

static mut COUNTER_MT_RUNTIME: usize = 0;

#[deps(MT_RUNTIME_000)]
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn tokio_mt_runtime_000() {
    time::sleep(Duration::from_secs_f64(0.1)).await;
    unsafe {
        assert_eq!(0, COUNTER_MT_RUNTIME);
        COUNTER_MT_RUNTIME = COUNTER_MT_RUNTIME + 1;
    }
}

#[deps(MT_RUNTIME_001: MT_RUNTIME_000)]
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn tokio_mt_runtime_001() {
    time::sleep(Duration::from_secs_f64(0.05)).await;
    unsafe {
        assert_eq!(1, COUNTER_MT_RUNTIME);
        COUNTER_MT_RUNTIME = COUNTER_MT_RUNTIME + 1;
    }
}

#[deps(MT_RUNTIME_002: MT_RUNTIME_001)]
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn tokio_mt_runtime_002() {
    time::sleep(Duration::from_secs_f64(0.025)).await;
    unsafe {
        assert_eq!(2, COUNTER_MT_RUNTIME);
        COUNTER_MT_RUNTIME = COUNTER_MT_RUNTIME + 1;
    }
}
