use rand::Rng;
use std::{thread, time};
use test_deps::deps;


static mut COUNTER_SERIAL: usize = 0;

#[deps(SERIAL_000)]
#[test]
fn serial_000() {
    thread::sleep(time::Duration::from_secs_f64(0.100000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        assert_eq!(0, COUNTER_SERIAL);
        COUNTER_SERIAL = COUNTER_SERIAL + 1;
    }
}

#[deps(SERIAL_001: SERIAL_000)]
#[test]
fn serial_001() {
    thread::sleep(time::Duration::from_secs_f64(0.100000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        assert_eq!(1, COUNTER_SERIAL);
        COUNTER_SERIAL = COUNTER_SERIAL + 1;
    }
}

#[deps(SERIAL_002: SERIAL_001)]
#[test]
fn serial_002() {
    thread::sleep(time::Duration::from_secs_f64(0.100000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        assert_eq!(2, COUNTER_SERIAL);
        COUNTER_SERIAL = COUNTER_SERIAL + 1;
    }
}

#[deps(SERIAL_003: SERIAL_002)]
#[test]
fn serial_003() {
    thread::sleep(time::Duration::from_secs_f64(0.100000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        assert_eq!(3, COUNTER_SERIAL);
        COUNTER_SERIAL = COUNTER_SERIAL + 1;
    }
}

#[deps(SERIAL_004: SERIAL_003)]
#[test]
fn serial_004() {
    thread::sleep(time::Duration::from_secs_f64(0.100000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        assert_eq!(4, COUNTER_SERIAL);
        COUNTER_SERIAL = COUNTER_SERIAL + 1;
    }
}

#[deps(SERIAL_005: SERIAL_004)]
#[test]
fn serial_005() {
    thread::sleep(time::Duration::from_secs_f64(0.100000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        assert_eq!(5, COUNTER_SERIAL);
        COUNTER_SERIAL = COUNTER_SERIAL + 1;
    }
}


static mut LEAF_FORK: [bool; 6] = [false; 6];

#[deps(FORK_000)]
#[test]
fn fork_000() {
    thread::sleep(time::Duration::from_secs_f64(0.5));
    unsafe {
        for l in &LEAF_FORK[1..] {
            assert!(!l);
        }
        LEAF_FORK[0] = true;
    }
}

#[deps(FORK_001: FORK_000)]
#[test]
fn fork_001() {
    unsafe {
        assert!(LEAF_FORK[0]);
        LEAF_FORK[1] = true;
    }
}

#[deps(FORK_002: FORK_000)]
#[test]
fn fork_002() {
    unsafe {
        assert!(LEAF_FORK[0]);
        LEAF_FORK[2] = true;
    }
}

#[deps(FORK_003: FORK_000)]
#[test]
fn fork_003() {
    unsafe {
        assert!(LEAF_FORK[0]);
        LEAF_FORK[3] = true;
    }
}

#[deps(FORK_004: FORK_000)]
#[test]
fn fork_004() {
    unsafe {
        assert!(LEAF_FORK[0]);
        LEAF_FORK[4] = true;
    }
}

#[deps(FORK_005: FORK_000)]
#[test]
fn fork_005() {
    unsafe {
        assert!(LEAF_FORK[0]);
        LEAF_FORK[5] = true;
    }
}


static mut LEAF_MERGE: [bool; 6] = [false; 6];

#[deps(MERGE_000: MERGE_001 MERGE_002 MERGE_003 MERGE_004 MERGE_005)]
#[test]
fn merge_000() {
    unsafe {
        for l in &LEAF_MERGE[1..] {
            assert!(l);
        }
        LEAF_MERGE[0] = true;
    }
}

#[deps(MERGE_001)]
#[test]
fn merge_001() {
    thread::sleep(time::Duration::from_secs_f64(0.100000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        assert!(!LEAF_MERGE[0]);
        LEAF_MERGE[1] = true;
    }
}

#[deps(MERGE_002)]
#[test]
fn merge_002() {
    thread::sleep(time::Duration::from_secs_f64(0.100000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        assert!(!LEAF_MERGE[0]);
        LEAF_MERGE[2] = true;
    }
}

#[deps(MERGE_003)]
#[test]
fn merge_003() {
    thread::sleep(time::Duration::from_secs_f64(0.100000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        assert!(!LEAF_MERGE[0]);
        LEAF_MERGE[3] = true;
    }
}

#[deps(MERGE_004)]
#[test]
fn merge_004() {
    thread::sleep(time::Duration::from_secs_f64(0.100000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        assert!(!LEAF_MERGE[0]);
        LEAF_MERGE[4] = true;
    }
}

#[deps(MERGE_005)]
#[test]
fn merge_005() {
    thread::sleep(time::Duration::from_secs_f64(0.100000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        assert!(!LEAF_MERGE[0]);
        LEAF_MERGE[5] = true;
    }
}


static mut N: [usize; 10] = [0; 10];

#[deps(NN_000: NN_016 NN_017 NN_018 NN_019 NN_020)]
#[test]
fn neural_network_000() {
    let mut input = 0;
    let pos = 4 % 2;
    thread::sleep(time::Duration::from_secs_f64(0.025000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        for n in &N[(pos * 5)..((pos + 1) * 5)] {
            input = input + n;
        }
    }
    assert_eq!(625, input);
}

#[deps(NN_001)]
#[test]
fn neural_network_001() {
    thread::sleep(time::Duration::from_secs_f64(0.025000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        for n in &N[..5] {
            assert_eq!(0, *n);
        }
        N[5 + 0] = 1;
    }
}

#[deps(NN_002)]
#[test]
fn neural_network_002() {
    thread::sleep(time::Duration::from_secs_f64(0.025000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        for n in &N[..5] {
            assert_eq!(0, *n);
        }
        N[5 + 1] = 1;
    }
}

#[deps(NN_003)]
#[test]
fn neural_network_003() {
    thread::sleep(time::Duration::from_secs_f64(0.025000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        for n in &N[..5] {
            assert_eq!(0, *n);
        }
        N[5 + 2] = 1;
    }
}

#[deps(NN_004)]
#[test]
fn neural_network_004() {
    thread::sleep(time::Duration::from_secs_f64(0.025000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        for n in &N[..5] {
            assert_eq!(0, *n);
        }
        N[5 + 3] = 1;
    }
}

#[deps(NN_005)]
#[test]
fn neural_network_005() {
    thread::sleep(time::Duration::from_secs_f64(0.025000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        for n in &N[..5] {
            assert_eq!(0, *n);
        }
        N[5 + 4] = 1;
    }
}

#[deps(NN_006: NN_001 NN_002 NN_003 NN_004 NN_005)]
#[test]
fn neural_network_006() {
    let mut input = 0;
    let pos = 1 % 2;
    thread::sleep(time::Duration::from_secs_f64(0.025000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        for n in &N[(pos * 5)..((pos + 1) * 5)] {
            input = input + n;
        }
    }
    assert_eq!(5, input);
    unsafe {
        N[(pos ^ 1) * 5 + 0] = 5;
    }
}

#[deps(NN_007: NN_001 NN_002 NN_003 NN_004 NN_005)]
#[test]
fn neural_network_007() {
    let mut input = 0;
    let pos = 1 % 2;
    thread::sleep(time::Duration::from_secs_f64(0.025000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        for n in &N[(pos * 5)..((pos + 1) * 5)] {
            input = input + n;
        }
    }
    assert_eq!(5, input);
    unsafe {
        N[(pos ^ 1) * 5 + 1] = 5;
    }
}

#[deps(NN_008: NN_001 NN_002 NN_003 NN_004 NN_005)]
#[test]
fn neural_network_008() {
    let mut input = 0;
    let pos = 1 % 2;
    thread::sleep(time::Duration::from_secs_f64(0.025000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        for n in &N[(pos * 5)..((pos + 1) * 5)] {
            input = input + n;
        }
    }
    assert_eq!(5, input);
    unsafe {
        N[(pos ^ 1) * 5 + 2] = 5;
    }
}

#[deps(NN_009: NN_001 NN_002 NN_003 NN_004 NN_005)]
#[test]
fn neural_network_009() {
    let mut input = 0;
    let pos = 1 % 2;
    thread::sleep(time::Duration::from_secs_f64(0.025000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        for n in &N[(pos * 5)..((pos + 1) * 5)] {
            input = input + n;
        }
    }
    assert_eq!(5, input);
    unsafe {
        N[(pos ^ 1) * 5 + 3] = 5;
    }
}

#[deps(NN_010: NN_001 NN_002 NN_003 NN_004 NN_005)]
#[test]
fn neural_network_010() {
    let mut input = 0;
    let pos = 1 % 2;
    thread::sleep(time::Duration::from_secs_f64(0.025000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        for n in &N[(pos * 5)..((pos + 1) * 5)] {
            input = input + n;
        }
    }
    assert_eq!(5, input);
    unsafe {
        N[(pos ^ 1) * 5 + 4] = 5;
    }
}

#[deps(NN_011: NN_006 NN_007 NN_008 NN_009 NN_010)]
#[test]
fn neural_network_011() {
    let mut input = 0;
    let pos = 2 % 2;
    thread::sleep(time::Duration::from_secs_f64(0.025000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        for n in &N[(pos * 5)..((pos + 1) * 5)] {
            input = input + n;
        }
    }
    assert_eq!(25, input);
    unsafe {
        N[(pos ^ 1) * 5 + 0] = 25;
    }
}

#[deps(NN_012: NN_006 NN_007 NN_008 NN_009 NN_010)]
#[test]
fn neural_network_012() {
    let mut input = 0;
    let pos = 2 % 2;
    thread::sleep(time::Duration::from_secs_f64(0.025000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        for n in &N[(pos * 5)..((pos + 1) * 5)] {
            input = input + n;
        }
    }
    assert_eq!(25, input);
    unsafe {
        N[(pos ^ 1) * 5 + 1] = 25;
    }
}

#[deps(NN_013: NN_006 NN_007 NN_008 NN_009 NN_010)]
#[test]
fn neural_network_013() {
    let mut input = 0;
    let pos = 2 % 2;
    thread::sleep(time::Duration::from_secs_f64(0.025000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        for n in &N[(pos * 5)..((pos + 1) * 5)] {
            input = input + n;
        }
    }
    assert_eq!(25, input);
    unsafe {
        N[(pos ^ 1) * 5 + 2] = 25;
    }
}

#[deps(NN_014: NN_006 NN_007 NN_008 NN_009 NN_010)]
#[test]
fn neural_network_014() {
    let mut input = 0;
    let pos = 2 % 2;
    thread::sleep(time::Duration::from_secs_f64(0.025000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        for n in &N[(pos * 5)..((pos + 1) * 5)] {
            input = input + n;
        }
    }
    assert_eq!(25, input);
    unsafe {
        N[(pos ^ 1) * 5 + 3] = 25;
    }
}

#[deps(NN_015: NN_006 NN_007 NN_008 NN_009 NN_010)]
#[test]
fn neural_network_015() {
    let mut input = 0;
    let pos = 2 % 2;
    thread::sleep(time::Duration::from_secs_f64(0.025000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        for n in &N[(pos * 5)..((pos + 1) * 5)] {
            input = input + n;
        }
    }
    assert_eq!(25, input);
    unsafe {
        N[(pos ^ 1) * 5 + 4] = 25;
    }
}

#[deps(NN_016: NN_011 NN_012 NN_013 NN_014 NN_015)]
#[test]
fn neural_network_016() {
    let mut input = 0;
    let pos = 3 % 2;
    thread::sleep(time::Duration::from_secs_f64(0.025000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        for n in &N[(pos * 5)..((pos + 1) * 5)] {
            input = input + n;
        }
    }
    assert_eq!(125, input);
    unsafe {
        N[(pos ^ 1) * 5 + 0] = 125;
    }
}

#[deps(NN_017: NN_011 NN_012 NN_013 NN_014 NN_015)]
#[test]
fn neural_network_017() {
    let mut input = 0;
    let pos = 3 % 2;
    thread::sleep(time::Duration::from_secs_f64(0.025000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        for n in &N[(pos * 5)..((pos + 1) * 5)] {
            input = input + n;
        }
    }
    assert_eq!(125, input);
    unsafe {
        N[(pos ^ 1) * 5 + 1] = 125;
    }
}

#[deps(NN_018: NN_011 NN_012 NN_013 NN_014 NN_015)]
#[test]
fn neural_network_018() {
    let mut input = 0;
    let pos = 3 % 2;
    thread::sleep(time::Duration::from_secs_f64(0.025000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        for n in &N[(pos * 5)..((pos + 1) * 5)] {
            input = input + n;
        }
    }
    assert_eq!(125, input);
    unsafe {
        N[(pos ^ 1) * 5 + 2] = 125;
    }
}

#[deps(NN_019: NN_011 NN_012 NN_013 NN_014 NN_015)]
#[test]
fn neural_network_019() {
    let mut input = 0;
    let pos = 3 % 2;
    thread::sleep(time::Duration::from_secs_f64(0.025000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        for n in &N[(pos * 5)..((pos + 1) * 5)] {
            input = input + n;
        }
    }
    assert_eq!(125, input);
    unsafe {
        N[(pos ^ 1) * 5 + 3] = 125;
    }
}

#[deps(NN_020: NN_011 NN_012 NN_013 NN_014 NN_015)]
#[test]
fn neural_network_020() {
    let mut input = 0;
    let pos = 3 % 2;
    thread::sleep(time::Duration::from_secs_f64(0.025000 * rand::thread_rng().gen::<f64>()));
    unsafe {
        for n in &N[(pos * 5)..((pos + 1) * 5)] {
            input = input + n;
        }
    }
    assert_eq!(125, input);
    unsafe {
        N[(pos ^ 1) * 5 + 4] = 125;
    }
}

