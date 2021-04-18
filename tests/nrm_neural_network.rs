use test_deps::deps;
static mut N: [usize; 40] = [0; 40];

#[test]
#[deps(T_000: T_081 T_082 T_083 T_084 T_085 T_086 T_087 T_088 T_089 T_090 T_091 T_092 T_093 T_094 T_095 T_096 T_097 T_098 T_099 T_100)]
fn neural_network_000() {
    let mut input = 0;
    let pos = 5 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(3200000, input);
}

#[test]
#[deps(T_001)]
fn neural_network_001() {
    unsafe {
        for n in &N[..20] {
            assert_eq!(0, *n);
        }
        N[20 + 0] = 1;
    }
}

#[test]
#[deps(T_002)]
fn neural_network_002() {
    unsafe {
        for n in &N[..20] {
            assert_eq!(0, *n);
        }
        N[20 + 1] = 1;
    }
}

#[test]
#[deps(T_003)]
fn neural_network_003() {
    unsafe {
        for n in &N[..20] {
            assert_eq!(0, *n);
        }
        N[20 + 2] = 1;
    }
}

#[test]
#[deps(T_004)]
fn neural_network_004() {
    unsafe {
        for n in &N[..20] {
            assert_eq!(0, *n);
        }
        N[20 + 3] = 1;
    }
}

#[test]
#[deps(T_005)]
fn neural_network_005() {
    unsafe {
        for n in &N[..20] {
            assert_eq!(0, *n);
        }
        N[20 + 4] = 1;
    }
}

#[test]
#[deps(T_006)]
fn neural_network_006() {
    unsafe {
        for n in &N[..20] {
            assert_eq!(0, *n);
        }
        N[20 + 5] = 1;
    }
}

#[test]
#[deps(T_007)]
fn neural_network_007() {
    unsafe {
        for n in &N[..20] {
            assert_eq!(0, *n);
        }
        N[20 + 6] = 1;
    }
}

#[test]
#[deps(T_008)]
fn neural_network_008() {
    unsafe {
        for n in &N[..20] {
            assert_eq!(0, *n);
        }
        N[20 + 7] = 1;
    }
}

#[test]
#[deps(T_009)]
fn neural_network_009() {
    unsafe {
        for n in &N[..20] {
            assert_eq!(0, *n);
        }
        N[20 + 8] = 1;
    }
}

#[test]
#[deps(T_010)]
fn neural_network_010() {
    unsafe {
        for n in &N[..20] {
            assert_eq!(0, *n);
        }
        N[20 + 9] = 1;
    }
}

#[test]
#[deps(T_011)]
fn neural_network_011() {
    unsafe {
        for n in &N[..20] {
            assert_eq!(0, *n);
        }
        N[20 + 10] = 1;
    }
}

#[test]
#[deps(T_012)]
fn neural_network_012() {
    unsafe {
        for n in &N[..20] {
            assert_eq!(0, *n);
        }
        N[20 + 11] = 1;
    }
}

#[test]
#[deps(T_013)]
fn neural_network_013() {
    unsafe {
        for n in &N[..20] {
            assert_eq!(0, *n);
        }
        N[20 + 12] = 1;
    }
}

#[test]
#[deps(T_014)]
fn neural_network_014() {
    unsafe {
        for n in &N[..20] {
            assert_eq!(0, *n);
        }
        N[20 + 13] = 1;
    }
}

#[test]
#[deps(T_015)]
fn neural_network_015() {
    unsafe {
        for n in &N[..20] {
            assert_eq!(0, *n);
        }
        N[20 + 14] = 1;
    }
}

#[test]
#[deps(T_016)]
fn neural_network_016() {
    unsafe {
        for n in &N[..20] {
            assert_eq!(0, *n);
        }
        N[20 + 15] = 1;
    }
}

#[test]
#[deps(T_017)]
fn neural_network_017() {
    unsafe {
        for n in &N[..20] {
            assert_eq!(0, *n);
        }
        N[20 + 16] = 1;
    }
}

#[test]
#[deps(T_018)]
fn neural_network_018() {
    unsafe {
        for n in &N[..20] {
            assert_eq!(0, *n);
        }
        N[20 + 17] = 1;
    }
}

#[test]
#[deps(T_019)]
fn neural_network_019() {
    unsafe {
        for n in &N[..20] {
            assert_eq!(0, *n);
        }
        N[20 + 18] = 1;
    }
}

#[test]
#[deps(T_020)]
fn neural_network_020() {
    unsafe {
        for n in &N[..20] {
            assert_eq!(0, *n);
        }
        N[20 + 19] = 1;
    }
}

#[test]
#[deps(T_021: T_001 T_002 T_003 T_004 T_005 T_006 T_007 T_008 T_009 T_010 T_011 T_012 T_013 T_014 T_015 T_016 T_017 T_018 T_019 T_020)]
fn neural_network_021() {
    let mut input = 0;
    let pos = 1 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(20, input);
    unsafe {
        N[(pos ^ 1) * 20 + 0] = 20;
    }
}

#[test]
#[deps(T_022: T_001 T_002 T_003 T_004 T_005 T_006 T_007 T_008 T_009 T_010 T_011 T_012 T_013 T_014 T_015 T_016 T_017 T_018 T_019 T_020)]
fn neural_network_022() {
    let mut input = 0;
    let pos = 1 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(20, input);
    unsafe {
        N[(pos ^ 1) * 20 + 1] = 20;
    }
}

#[test]
#[deps(T_023: T_001 T_002 T_003 T_004 T_005 T_006 T_007 T_008 T_009 T_010 T_011 T_012 T_013 T_014 T_015 T_016 T_017 T_018 T_019 T_020)]
fn neural_network_023() {
    let mut input = 0;
    let pos = 1 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(20, input);
    unsafe {
        N[(pos ^ 1) * 20 + 2] = 20;
    }
}

#[test]
#[deps(T_024: T_001 T_002 T_003 T_004 T_005 T_006 T_007 T_008 T_009 T_010 T_011 T_012 T_013 T_014 T_015 T_016 T_017 T_018 T_019 T_020)]
fn neural_network_024() {
    let mut input = 0;
    let pos = 1 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(20, input);
    unsafe {
        N[(pos ^ 1) * 20 + 3] = 20;
    }
}

#[test]
#[deps(T_025: T_001 T_002 T_003 T_004 T_005 T_006 T_007 T_008 T_009 T_010 T_011 T_012 T_013 T_014 T_015 T_016 T_017 T_018 T_019 T_020)]
fn neural_network_025() {
    let mut input = 0;
    let pos = 1 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(20, input);
    unsafe {
        N[(pos ^ 1) * 20 + 4] = 20;
    }
}

#[test]
#[deps(T_026: T_001 T_002 T_003 T_004 T_005 T_006 T_007 T_008 T_009 T_010 T_011 T_012 T_013 T_014 T_015 T_016 T_017 T_018 T_019 T_020)]
fn neural_network_026() {
    let mut input = 0;
    let pos = 1 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(20, input);
    unsafe {
        N[(pos ^ 1) * 20 + 5] = 20;
    }
}

#[test]
#[deps(T_027: T_001 T_002 T_003 T_004 T_005 T_006 T_007 T_008 T_009 T_010 T_011 T_012 T_013 T_014 T_015 T_016 T_017 T_018 T_019 T_020)]
fn neural_network_027() {
    let mut input = 0;
    let pos = 1 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(20, input);
    unsafe {
        N[(pos ^ 1) * 20 + 6] = 20;
    }
}

#[test]
#[deps(T_028: T_001 T_002 T_003 T_004 T_005 T_006 T_007 T_008 T_009 T_010 T_011 T_012 T_013 T_014 T_015 T_016 T_017 T_018 T_019 T_020)]
fn neural_network_028() {
    let mut input = 0;
    let pos = 1 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(20, input);
    unsafe {
        N[(pos ^ 1) * 20 + 7] = 20;
    }
}

#[test]
#[deps(T_029: T_001 T_002 T_003 T_004 T_005 T_006 T_007 T_008 T_009 T_010 T_011 T_012 T_013 T_014 T_015 T_016 T_017 T_018 T_019 T_020)]
fn neural_network_029() {
    let mut input = 0;
    let pos = 1 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(20, input);
    unsafe {
        N[(pos ^ 1) * 20 + 8] = 20;
    }
}

#[test]
#[deps(T_030: T_001 T_002 T_003 T_004 T_005 T_006 T_007 T_008 T_009 T_010 T_011 T_012 T_013 T_014 T_015 T_016 T_017 T_018 T_019 T_020)]
fn neural_network_030() {
    let mut input = 0;
    let pos = 1 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(20, input);
    unsafe {
        N[(pos ^ 1) * 20 + 9] = 20;
    }
}

#[test]
#[deps(T_031: T_001 T_002 T_003 T_004 T_005 T_006 T_007 T_008 T_009 T_010 T_011 T_012 T_013 T_014 T_015 T_016 T_017 T_018 T_019 T_020)]
fn neural_network_031() {
    let mut input = 0;
    let pos = 1 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(20, input);
    unsafe {
        N[(pos ^ 1) * 20 + 10] = 20;
    }
}

#[test]
#[deps(T_032: T_001 T_002 T_003 T_004 T_005 T_006 T_007 T_008 T_009 T_010 T_011 T_012 T_013 T_014 T_015 T_016 T_017 T_018 T_019 T_020)]
fn neural_network_032() {
    let mut input = 0;
    let pos = 1 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(20, input);
    unsafe {
        N[(pos ^ 1) * 20 + 11] = 20;
    }
}

#[test]
#[deps(T_033: T_001 T_002 T_003 T_004 T_005 T_006 T_007 T_008 T_009 T_010 T_011 T_012 T_013 T_014 T_015 T_016 T_017 T_018 T_019 T_020)]
fn neural_network_033() {
    let mut input = 0;
    let pos = 1 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(20, input);
    unsafe {
        N[(pos ^ 1) * 20 + 12] = 20;
    }
}

#[test]
#[deps(T_034: T_001 T_002 T_003 T_004 T_005 T_006 T_007 T_008 T_009 T_010 T_011 T_012 T_013 T_014 T_015 T_016 T_017 T_018 T_019 T_020)]
fn neural_network_034() {
    let mut input = 0;
    let pos = 1 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(20, input);
    unsafe {
        N[(pos ^ 1) * 20 + 13] = 20;
    }
}

#[test]
#[deps(T_035: T_001 T_002 T_003 T_004 T_005 T_006 T_007 T_008 T_009 T_010 T_011 T_012 T_013 T_014 T_015 T_016 T_017 T_018 T_019 T_020)]
fn neural_network_035() {
    let mut input = 0;
    let pos = 1 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(20, input);
    unsafe {
        N[(pos ^ 1) * 20 + 14] = 20;
    }
}

#[test]
#[deps(T_036: T_001 T_002 T_003 T_004 T_005 T_006 T_007 T_008 T_009 T_010 T_011 T_012 T_013 T_014 T_015 T_016 T_017 T_018 T_019 T_020)]
fn neural_network_036() {
    let mut input = 0;
    let pos = 1 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(20, input);
    unsafe {
        N[(pos ^ 1) * 20 + 15] = 20;
    }
}

#[test]
#[deps(T_037: T_001 T_002 T_003 T_004 T_005 T_006 T_007 T_008 T_009 T_010 T_011 T_012 T_013 T_014 T_015 T_016 T_017 T_018 T_019 T_020)]
fn neural_network_037() {
    let mut input = 0;
    let pos = 1 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(20, input);
    unsafe {
        N[(pos ^ 1) * 20 + 16] = 20;
    }
}

#[test]
#[deps(T_038: T_001 T_002 T_003 T_004 T_005 T_006 T_007 T_008 T_009 T_010 T_011 T_012 T_013 T_014 T_015 T_016 T_017 T_018 T_019 T_020)]
fn neural_network_038() {
    let mut input = 0;
    let pos = 1 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(20, input);
    unsafe {
        N[(pos ^ 1) * 20 + 17] = 20;
    }
}

#[test]
#[deps(T_039: T_001 T_002 T_003 T_004 T_005 T_006 T_007 T_008 T_009 T_010 T_011 T_012 T_013 T_014 T_015 T_016 T_017 T_018 T_019 T_020)]
fn neural_network_039() {
    let mut input = 0;
    let pos = 1 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(20, input);
    unsafe {
        N[(pos ^ 1) * 20 + 18] = 20;
    }
}

#[test]
#[deps(T_040: T_001 T_002 T_003 T_004 T_005 T_006 T_007 T_008 T_009 T_010 T_011 T_012 T_013 T_014 T_015 T_016 T_017 T_018 T_019 T_020)]
fn neural_network_040() {
    let mut input = 0;
    let pos = 1 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(20, input);
    unsafe {
        N[(pos ^ 1) * 20 + 19] = 20;
    }
}

#[test]
#[deps(T_041: T_021 T_022 T_023 T_024 T_025 T_026 T_027 T_028 T_029 T_030 T_031 T_032 T_033 T_034 T_035 T_036 T_037 T_038 T_039 T_040)]
fn neural_network_041() {
    let mut input = 0;
    let pos = 2 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(400, input);
    unsafe {
        N[(pos ^ 1) * 20 + 0] = 400;
    }
}

#[test]
#[deps(T_042: T_021 T_022 T_023 T_024 T_025 T_026 T_027 T_028 T_029 T_030 T_031 T_032 T_033 T_034 T_035 T_036 T_037 T_038 T_039 T_040)]
fn neural_network_042() {
    let mut input = 0;
    let pos = 2 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(400, input);
    unsafe {
        N[(pos ^ 1) * 20 + 1] = 400;
    }
}

#[test]
#[deps(T_043: T_021 T_022 T_023 T_024 T_025 T_026 T_027 T_028 T_029 T_030 T_031 T_032 T_033 T_034 T_035 T_036 T_037 T_038 T_039 T_040)]
fn neural_network_043() {
    let mut input = 0;
    let pos = 2 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(400, input);
    unsafe {
        N[(pos ^ 1) * 20 + 2] = 400;
    }
}

#[test]
#[deps(T_044: T_021 T_022 T_023 T_024 T_025 T_026 T_027 T_028 T_029 T_030 T_031 T_032 T_033 T_034 T_035 T_036 T_037 T_038 T_039 T_040)]
fn neural_network_044() {
    let mut input = 0;
    let pos = 2 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(400, input);
    unsafe {
        N[(pos ^ 1) * 20 + 3] = 400;
    }
}

#[test]
#[deps(T_045: T_021 T_022 T_023 T_024 T_025 T_026 T_027 T_028 T_029 T_030 T_031 T_032 T_033 T_034 T_035 T_036 T_037 T_038 T_039 T_040)]
fn neural_network_045() {
    let mut input = 0;
    let pos = 2 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(400, input);
    unsafe {
        N[(pos ^ 1) * 20 + 4] = 400;
    }
}

#[test]
#[deps(T_046: T_021 T_022 T_023 T_024 T_025 T_026 T_027 T_028 T_029 T_030 T_031 T_032 T_033 T_034 T_035 T_036 T_037 T_038 T_039 T_040)]
fn neural_network_046() {
    let mut input = 0;
    let pos = 2 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(400, input);
    unsafe {
        N[(pos ^ 1) * 20 + 5] = 400;
    }
}

#[test]
#[deps(T_047: T_021 T_022 T_023 T_024 T_025 T_026 T_027 T_028 T_029 T_030 T_031 T_032 T_033 T_034 T_035 T_036 T_037 T_038 T_039 T_040)]
fn neural_network_047() {
    let mut input = 0;
    let pos = 2 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(400, input);
    unsafe {
        N[(pos ^ 1) * 20 + 6] = 400;
    }
}

#[test]
#[deps(T_048: T_021 T_022 T_023 T_024 T_025 T_026 T_027 T_028 T_029 T_030 T_031 T_032 T_033 T_034 T_035 T_036 T_037 T_038 T_039 T_040)]
fn neural_network_048() {
    let mut input = 0;
    let pos = 2 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(400, input);
    unsafe {
        N[(pos ^ 1) * 20 + 7] = 400;
    }
}

#[test]
#[deps(T_049: T_021 T_022 T_023 T_024 T_025 T_026 T_027 T_028 T_029 T_030 T_031 T_032 T_033 T_034 T_035 T_036 T_037 T_038 T_039 T_040)]
fn neural_network_049() {
    let mut input = 0;
    let pos = 2 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(400, input);
    unsafe {
        N[(pos ^ 1) * 20 + 8] = 400;
    }
}

#[test]
#[deps(T_050: T_021 T_022 T_023 T_024 T_025 T_026 T_027 T_028 T_029 T_030 T_031 T_032 T_033 T_034 T_035 T_036 T_037 T_038 T_039 T_040)]
fn neural_network_050() {
    let mut input = 0;
    let pos = 2 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(400, input);
    unsafe {
        N[(pos ^ 1) * 20 + 9] = 400;
    }
}

#[test]
#[deps(T_051: T_021 T_022 T_023 T_024 T_025 T_026 T_027 T_028 T_029 T_030 T_031 T_032 T_033 T_034 T_035 T_036 T_037 T_038 T_039 T_040)]
fn neural_network_051() {
    let mut input = 0;
    let pos = 2 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(400, input);
    unsafe {
        N[(pos ^ 1) * 20 + 10] = 400;
    }
}

#[test]
#[deps(T_052: T_021 T_022 T_023 T_024 T_025 T_026 T_027 T_028 T_029 T_030 T_031 T_032 T_033 T_034 T_035 T_036 T_037 T_038 T_039 T_040)]
fn neural_network_052() {
    let mut input = 0;
    let pos = 2 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(400, input);
    unsafe {
        N[(pos ^ 1) * 20 + 11] = 400;
    }
}

#[test]
#[deps(T_053: T_021 T_022 T_023 T_024 T_025 T_026 T_027 T_028 T_029 T_030 T_031 T_032 T_033 T_034 T_035 T_036 T_037 T_038 T_039 T_040)]
fn neural_network_053() {
    let mut input = 0;
    let pos = 2 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(400, input);
    unsafe {
        N[(pos ^ 1) * 20 + 12] = 400;
    }
}

#[test]
#[deps(T_054: T_021 T_022 T_023 T_024 T_025 T_026 T_027 T_028 T_029 T_030 T_031 T_032 T_033 T_034 T_035 T_036 T_037 T_038 T_039 T_040)]
fn neural_network_054() {
    let mut input = 0;
    let pos = 2 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(400, input);
    unsafe {
        N[(pos ^ 1) * 20 + 13] = 400;
    }
}

#[test]
#[deps(T_055: T_021 T_022 T_023 T_024 T_025 T_026 T_027 T_028 T_029 T_030 T_031 T_032 T_033 T_034 T_035 T_036 T_037 T_038 T_039 T_040)]
fn neural_network_055() {
    let mut input = 0;
    let pos = 2 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(400, input);
    unsafe {
        N[(pos ^ 1) * 20 + 14] = 400;
    }
}

#[test]
#[deps(T_056: T_021 T_022 T_023 T_024 T_025 T_026 T_027 T_028 T_029 T_030 T_031 T_032 T_033 T_034 T_035 T_036 T_037 T_038 T_039 T_040)]
fn neural_network_056() {
    let mut input = 0;
    let pos = 2 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(400, input);
    unsafe {
        N[(pos ^ 1) * 20 + 15] = 400;
    }
}

#[test]
#[deps(T_057: T_021 T_022 T_023 T_024 T_025 T_026 T_027 T_028 T_029 T_030 T_031 T_032 T_033 T_034 T_035 T_036 T_037 T_038 T_039 T_040)]
fn neural_network_057() {
    let mut input = 0;
    let pos = 2 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(400, input);
    unsafe {
        N[(pos ^ 1) * 20 + 16] = 400;
    }
}

#[test]
#[deps(T_058: T_021 T_022 T_023 T_024 T_025 T_026 T_027 T_028 T_029 T_030 T_031 T_032 T_033 T_034 T_035 T_036 T_037 T_038 T_039 T_040)]
fn neural_network_058() {
    let mut input = 0;
    let pos = 2 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(400, input);
    unsafe {
        N[(pos ^ 1) * 20 + 17] = 400;
    }
}

#[test]
#[deps(T_059: T_021 T_022 T_023 T_024 T_025 T_026 T_027 T_028 T_029 T_030 T_031 T_032 T_033 T_034 T_035 T_036 T_037 T_038 T_039 T_040)]
fn neural_network_059() {
    let mut input = 0;
    let pos = 2 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(400, input);
    unsafe {
        N[(pos ^ 1) * 20 + 18] = 400;
    }
}

#[test]
#[deps(T_060: T_021 T_022 T_023 T_024 T_025 T_026 T_027 T_028 T_029 T_030 T_031 T_032 T_033 T_034 T_035 T_036 T_037 T_038 T_039 T_040)]
fn neural_network_060() {
    let mut input = 0;
    let pos = 2 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(400, input);
    unsafe {
        N[(pos ^ 1) * 20 + 19] = 400;
    }
}

#[test]
#[deps(T_061: T_041 T_042 T_043 T_044 T_045 T_046 T_047 T_048 T_049 T_050 T_051 T_052 T_053 T_054 T_055 T_056 T_057 T_058 T_059 T_060)]
fn neural_network_061() {
    let mut input = 0;
    let pos = 3 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(8000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 0] = 8000;
    }
}

#[test]
#[deps(T_062: T_041 T_042 T_043 T_044 T_045 T_046 T_047 T_048 T_049 T_050 T_051 T_052 T_053 T_054 T_055 T_056 T_057 T_058 T_059 T_060)]
fn neural_network_062() {
    let mut input = 0;
    let pos = 3 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(8000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 1] = 8000;
    }
}

#[test]
#[deps(T_063: T_041 T_042 T_043 T_044 T_045 T_046 T_047 T_048 T_049 T_050 T_051 T_052 T_053 T_054 T_055 T_056 T_057 T_058 T_059 T_060)]
fn neural_network_063() {
    let mut input = 0;
    let pos = 3 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(8000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 2] = 8000;
    }
}

#[test]
#[deps(T_064: T_041 T_042 T_043 T_044 T_045 T_046 T_047 T_048 T_049 T_050 T_051 T_052 T_053 T_054 T_055 T_056 T_057 T_058 T_059 T_060)]
fn neural_network_064() {
    let mut input = 0;
    let pos = 3 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(8000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 3] = 8000;
    }
}

#[test]
#[deps(T_065: T_041 T_042 T_043 T_044 T_045 T_046 T_047 T_048 T_049 T_050 T_051 T_052 T_053 T_054 T_055 T_056 T_057 T_058 T_059 T_060)]
fn neural_network_065() {
    let mut input = 0;
    let pos = 3 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(8000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 4] = 8000;
    }
}

#[test]
#[deps(T_066: T_041 T_042 T_043 T_044 T_045 T_046 T_047 T_048 T_049 T_050 T_051 T_052 T_053 T_054 T_055 T_056 T_057 T_058 T_059 T_060)]
fn neural_network_066() {
    let mut input = 0;
    let pos = 3 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(8000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 5] = 8000;
    }
}

#[test]
#[deps(T_067: T_041 T_042 T_043 T_044 T_045 T_046 T_047 T_048 T_049 T_050 T_051 T_052 T_053 T_054 T_055 T_056 T_057 T_058 T_059 T_060)]
fn neural_network_067() {
    let mut input = 0;
    let pos = 3 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(8000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 6] = 8000;
    }
}

#[test]
#[deps(T_068: T_041 T_042 T_043 T_044 T_045 T_046 T_047 T_048 T_049 T_050 T_051 T_052 T_053 T_054 T_055 T_056 T_057 T_058 T_059 T_060)]
fn neural_network_068() {
    let mut input = 0;
    let pos = 3 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(8000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 7] = 8000;
    }
}

#[test]
#[deps(T_069: T_041 T_042 T_043 T_044 T_045 T_046 T_047 T_048 T_049 T_050 T_051 T_052 T_053 T_054 T_055 T_056 T_057 T_058 T_059 T_060)]
fn neural_network_069() {
    let mut input = 0;
    let pos = 3 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(8000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 8] = 8000;
    }
}

#[test]
#[deps(T_070: T_041 T_042 T_043 T_044 T_045 T_046 T_047 T_048 T_049 T_050 T_051 T_052 T_053 T_054 T_055 T_056 T_057 T_058 T_059 T_060)]
fn neural_network_070() {
    let mut input = 0;
    let pos = 3 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(8000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 9] = 8000;
    }
}

#[test]
#[deps(T_071: T_041 T_042 T_043 T_044 T_045 T_046 T_047 T_048 T_049 T_050 T_051 T_052 T_053 T_054 T_055 T_056 T_057 T_058 T_059 T_060)]
fn neural_network_071() {
    let mut input = 0;
    let pos = 3 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(8000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 10] = 8000;
    }
}

#[test]
#[deps(T_072: T_041 T_042 T_043 T_044 T_045 T_046 T_047 T_048 T_049 T_050 T_051 T_052 T_053 T_054 T_055 T_056 T_057 T_058 T_059 T_060)]
fn neural_network_072() {
    let mut input = 0;
    let pos = 3 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(8000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 11] = 8000;
    }
}

#[test]
#[deps(T_073: T_041 T_042 T_043 T_044 T_045 T_046 T_047 T_048 T_049 T_050 T_051 T_052 T_053 T_054 T_055 T_056 T_057 T_058 T_059 T_060)]
fn neural_network_073() {
    let mut input = 0;
    let pos = 3 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(8000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 12] = 8000;
    }
}

#[test]
#[deps(T_074: T_041 T_042 T_043 T_044 T_045 T_046 T_047 T_048 T_049 T_050 T_051 T_052 T_053 T_054 T_055 T_056 T_057 T_058 T_059 T_060)]
fn neural_network_074() {
    let mut input = 0;
    let pos = 3 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(8000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 13] = 8000;
    }
}

#[test]
#[deps(T_075: T_041 T_042 T_043 T_044 T_045 T_046 T_047 T_048 T_049 T_050 T_051 T_052 T_053 T_054 T_055 T_056 T_057 T_058 T_059 T_060)]
fn neural_network_075() {
    let mut input = 0;
    let pos = 3 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(8000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 14] = 8000;
    }
}

#[test]
#[deps(T_076: T_041 T_042 T_043 T_044 T_045 T_046 T_047 T_048 T_049 T_050 T_051 T_052 T_053 T_054 T_055 T_056 T_057 T_058 T_059 T_060)]
fn neural_network_076() {
    let mut input = 0;
    let pos = 3 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(8000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 15] = 8000;
    }
}

#[test]
#[deps(T_077: T_041 T_042 T_043 T_044 T_045 T_046 T_047 T_048 T_049 T_050 T_051 T_052 T_053 T_054 T_055 T_056 T_057 T_058 T_059 T_060)]
fn neural_network_077() {
    let mut input = 0;
    let pos = 3 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(8000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 16] = 8000;
    }
}

#[test]
#[deps(T_078: T_041 T_042 T_043 T_044 T_045 T_046 T_047 T_048 T_049 T_050 T_051 T_052 T_053 T_054 T_055 T_056 T_057 T_058 T_059 T_060)]
fn neural_network_078() {
    let mut input = 0;
    let pos = 3 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(8000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 17] = 8000;
    }
}

#[test]
#[deps(T_079: T_041 T_042 T_043 T_044 T_045 T_046 T_047 T_048 T_049 T_050 T_051 T_052 T_053 T_054 T_055 T_056 T_057 T_058 T_059 T_060)]
fn neural_network_079() {
    let mut input = 0;
    let pos = 3 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(8000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 18] = 8000;
    }
}

#[test]
#[deps(T_080: T_041 T_042 T_043 T_044 T_045 T_046 T_047 T_048 T_049 T_050 T_051 T_052 T_053 T_054 T_055 T_056 T_057 T_058 T_059 T_060)]
fn neural_network_080() {
    let mut input = 0;
    let pos = 3 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(8000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 19] = 8000;
    }
}

#[test]
#[deps(T_081: T_061 T_062 T_063 T_064 T_065 T_066 T_067 T_068 T_069 T_070 T_071 T_072 T_073 T_074 T_075 T_076 T_077 T_078 T_079 T_080)]
fn neural_network_081() {
    let mut input = 0;
    let pos = 4 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(160000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 0] = 160000;
    }
}

#[test]
#[deps(T_082: T_061 T_062 T_063 T_064 T_065 T_066 T_067 T_068 T_069 T_070 T_071 T_072 T_073 T_074 T_075 T_076 T_077 T_078 T_079 T_080)]
fn neural_network_082() {
    let mut input = 0;
    let pos = 4 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(160000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 1] = 160000;
    }
}

#[test]
#[deps(T_083: T_061 T_062 T_063 T_064 T_065 T_066 T_067 T_068 T_069 T_070 T_071 T_072 T_073 T_074 T_075 T_076 T_077 T_078 T_079 T_080)]
fn neural_network_083() {
    let mut input = 0;
    let pos = 4 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(160000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 2] = 160000;
    }
}

#[test]
#[deps(T_084: T_061 T_062 T_063 T_064 T_065 T_066 T_067 T_068 T_069 T_070 T_071 T_072 T_073 T_074 T_075 T_076 T_077 T_078 T_079 T_080)]
fn neural_network_084() {
    let mut input = 0;
    let pos = 4 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(160000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 3] = 160000;
    }
}

#[test]
#[deps(T_085: T_061 T_062 T_063 T_064 T_065 T_066 T_067 T_068 T_069 T_070 T_071 T_072 T_073 T_074 T_075 T_076 T_077 T_078 T_079 T_080)]
fn neural_network_085() {
    let mut input = 0;
    let pos = 4 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(160000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 4] = 160000;
    }
}

#[test]
#[deps(T_086: T_061 T_062 T_063 T_064 T_065 T_066 T_067 T_068 T_069 T_070 T_071 T_072 T_073 T_074 T_075 T_076 T_077 T_078 T_079 T_080)]
fn neural_network_086() {
    let mut input = 0;
    let pos = 4 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(160000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 5] = 160000;
    }
}

#[test]
#[deps(T_087: T_061 T_062 T_063 T_064 T_065 T_066 T_067 T_068 T_069 T_070 T_071 T_072 T_073 T_074 T_075 T_076 T_077 T_078 T_079 T_080)]
fn neural_network_087() {
    let mut input = 0;
    let pos = 4 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(160000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 6] = 160000;
    }
}

#[test]
#[deps(T_088: T_061 T_062 T_063 T_064 T_065 T_066 T_067 T_068 T_069 T_070 T_071 T_072 T_073 T_074 T_075 T_076 T_077 T_078 T_079 T_080)]
fn neural_network_088() {
    let mut input = 0;
    let pos = 4 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(160000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 7] = 160000;
    }
}

#[test]
#[deps(T_089: T_061 T_062 T_063 T_064 T_065 T_066 T_067 T_068 T_069 T_070 T_071 T_072 T_073 T_074 T_075 T_076 T_077 T_078 T_079 T_080)]
fn neural_network_089() {
    let mut input = 0;
    let pos = 4 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(160000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 8] = 160000;
    }
}

#[test]
#[deps(T_090: T_061 T_062 T_063 T_064 T_065 T_066 T_067 T_068 T_069 T_070 T_071 T_072 T_073 T_074 T_075 T_076 T_077 T_078 T_079 T_080)]
fn neural_network_090() {
    let mut input = 0;
    let pos = 4 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(160000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 9] = 160000;
    }
}

#[test]
#[deps(T_091: T_061 T_062 T_063 T_064 T_065 T_066 T_067 T_068 T_069 T_070 T_071 T_072 T_073 T_074 T_075 T_076 T_077 T_078 T_079 T_080)]
fn neural_network_091() {
    let mut input = 0;
    let pos = 4 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(160000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 10] = 160000;
    }
}

#[test]
#[deps(T_092: T_061 T_062 T_063 T_064 T_065 T_066 T_067 T_068 T_069 T_070 T_071 T_072 T_073 T_074 T_075 T_076 T_077 T_078 T_079 T_080)]
fn neural_network_092() {
    let mut input = 0;
    let pos = 4 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(160000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 11] = 160000;
    }
}

#[test]
#[deps(T_093: T_061 T_062 T_063 T_064 T_065 T_066 T_067 T_068 T_069 T_070 T_071 T_072 T_073 T_074 T_075 T_076 T_077 T_078 T_079 T_080)]
fn neural_network_093() {
    let mut input = 0;
    let pos = 4 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(160000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 12] = 160000;
    }
}

#[test]
#[deps(T_094: T_061 T_062 T_063 T_064 T_065 T_066 T_067 T_068 T_069 T_070 T_071 T_072 T_073 T_074 T_075 T_076 T_077 T_078 T_079 T_080)]
fn neural_network_094() {
    let mut input = 0;
    let pos = 4 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(160000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 13] = 160000;
    }
}

#[test]
#[deps(T_095: T_061 T_062 T_063 T_064 T_065 T_066 T_067 T_068 T_069 T_070 T_071 T_072 T_073 T_074 T_075 T_076 T_077 T_078 T_079 T_080)]
fn neural_network_095() {
    let mut input = 0;
    let pos = 4 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(160000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 14] = 160000;
    }
}

#[test]
#[deps(T_096: T_061 T_062 T_063 T_064 T_065 T_066 T_067 T_068 T_069 T_070 T_071 T_072 T_073 T_074 T_075 T_076 T_077 T_078 T_079 T_080)]
fn neural_network_096() {
    let mut input = 0;
    let pos = 4 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(160000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 15] = 160000;
    }
}

#[test]
#[deps(T_097: T_061 T_062 T_063 T_064 T_065 T_066 T_067 T_068 T_069 T_070 T_071 T_072 T_073 T_074 T_075 T_076 T_077 T_078 T_079 T_080)]
fn neural_network_097() {
    let mut input = 0;
    let pos = 4 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(160000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 16] = 160000;
    }
}

#[test]
#[deps(T_098: T_061 T_062 T_063 T_064 T_065 T_066 T_067 T_068 T_069 T_070 T_071 T_072 T_073 T_074 T_075 T_076 T_077 T_078 T_079 T_080)]
fn neural_network_098() {
    let mut input = 0;
    let pos = 4 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(160000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 17] = 160000;
    }
}

#[test]
#[deps(T_099: T_061 T_062 T_063 T_064 T_065 T_066 T_067 T_068 T_069 T_070 T_071 T_072 T_073 T_074 T_075 T_076 T_077 T_078 T_079 T_080)]
fn neural_network_099() {
    let mut input = 0;
    let pos = 4 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(160000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 18] = 160000;
    }
}

#[test]
#[deps(T_100: T_061 T_062 T_063 T_064 T_065 T_066 T_067 T_068 T_069 T_070 T_071 T_072 T_073 T_074 T_075 T_076 T_077 T_078 T_079 T_080)]
fn neural_network_100() {
    let mut input = 0;
    let pos = 4 % 2;
    unsafe {
        for n in &N[(pos * 20)..((pos + 1) * 20)] {
            input = input + n;
        }
    }
    assert_eq!(160000, input);
    unsafe {
        N[(pos ^ 1) * 20 + 19] = 160000;
    }
}

