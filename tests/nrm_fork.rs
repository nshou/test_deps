use test_deps::deps;
static mut LEAF: [bool; 101] = [false; 101];

#[test]
#[deps(T_000)]
fn fork_000() {
    unsafe {
        for l in &LEAF[1..] {
            assert!(!l);
        }
        LEAF[0] = true;
    }
}

#[test]
#[deps(T_001: T_000)]
fn fork_001() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[1] = true;
    }
}

#[test]
#[deps(T_002: T_000)]
fn fork_002() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[2] = true;
    }
}

#[test]
#[deps(T_003: T_000)]
fn fork_003() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[3] = true;
    }
}

#[test]
#[deps(T_004: T_000)]
fn fork_004() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[4] = true;
    }
}

#[test]
#[deps(T_005: T_000)]
fn fork_005() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[5] = true;
    }
}

#[test]
#[deps(T_006: T_000)]
fn fork_006() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[6] = true;
    }
}

#[test]
#[deps(T_007: T_000)]
fn fork_007() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[7] = true;
    }
}

#[test]
#[deps(T_008: T_000)]
fn fork_008() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[8] = true;
    }
}

#[test]
#[deps(T_009: T_000)]
fn fork_009() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[9] = true;
    }
}

#[test]
#[deps(T_010: T_000)]
fn fork_010() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[10] = true;
    }
}

#[test]
#[deps(T_011: T_000)]
fn fork_011() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[11] = true;
    }
}

#[test]
#[deps(T_012: T_000)]
fn fork_012() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[12] = true;
    }
}

#[test]
#[deps(T_013: T_000)]
fn fork_013() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[13] = true;
    }
}

#[test]
#[deps(T_014: T_000)]
fn fork_014() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[14] = true;
    }
}

#[test]
#[deps(T_015: T_000)]
fn fork_015() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[15] = true;
    }
}

#[test]
#[deps(T_016: T_000)]
fn fork_016() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[16] = true;
    }
}

#[test]
#[deps(T_017: T_000)]
fn fork_017() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[17] = true;
    }
}

#[test]
#[deps(T_018: T_000)]
fn fork_018() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[18] = true;
    }
}

#[test]
#[deps(T_019: T_000)]
fn fork_019() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[19] = true;
    }
}

#[test]
#[deps(T_020: T_000)]
fn fork_020() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[20] = true;
    }
}

#[test]
#[deps(T_021: T_000)]
fn fork_021() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[21] = true;
    }
}

#[test]
#[deps(T_022: T_000)]
fn fork_022() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[22] = true;
    }
}

#[test]
#[deps(T_023: T_000)]
fn fork_023() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[23] = true;
    }
}

#[test]
#[deps(T_024: T_000)]
fn fork_024() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[24] = true;
    }
}

#[test]
#[deps(T_025: T_000)]
fn fork_025() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[25] = true;
    }
}

#[test]
#[deps(T_026: T_000)]
fn fork_026() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[26] = true;
    }
}

#[test]
#[deps(T_027: T_000)]
fn fork_027() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[27] = true;
    }
}

#[test]
#[deps(T_028: T_000)]
fn fork_028() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[28] = true;
    }
}

#[test]
#[deps(T_029: T_000)]
fn fork_029() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[29] = true;
    }
}

#[test]
#[deps(T_030: T_000)]
fn fork_030() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[30] = true;
    }
}

#[test]
#[deps(T_031: T_000)]
fn fork_031() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[31] = true;
    }
}

#[test]
#[deps(T_032: T_000)]
fn fork_032() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[32] = true;
    }
}

#[test]
#[deps(T_033: T_000)]
fn fork_033() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[33] = true;
    }
}

#[test]
#[deps(T_034: T_000)]
fn fork_034() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[34] = true;
    }
}

#[test]
#[deps(T_035: T_000)]
fn fork_035() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[35] = true;
    }
}

#[test]
#[deps(T_036: T_000)]
fn fork_036() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[36] = true;
    }
}

#[test]
#[deps(T_037: T_000)]
fn fork_037() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[37] = true;
    }
}

#[test]
#[deps(T_038: T_000)]
fn fork_038() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[38] = true;
    }
}

#[test]
#[deps(T_039: T_000)]
fn fork_039() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[39] = true;
    }
}

#[test]
#[deps(T_040: T_000)]
fn fork_040() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[40] = true;
    }
}

#[test]
#[deps(T_041: T_000)]
fn fork_041() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[41] = true;
    }
}

#[test]
#[deps(T_042: T_000)]
fn fork_042() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[42] = true;
    }
}

#[test]
#[deps(T_043: T_000)]
fn fork_043() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[43] = true;
    }
}

#[test]
#[deps(T_044: T_000)]
fn fork_044() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[44] = true;
    }
}

#[test]
#[deps(T_045: T_000)]
fn fork_045() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[45] = true;
    }
}

#[test]
#[deps(T_046: T_000)]
fn fork_046() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[46] = true;
    }
}

#[test]
#[deps(T_047: T_000)]
fn fork_047() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[47] = true;
    }
}

#[test]
#[deps(T_048: T_000)]
fn fork_048() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[48] = true;
    }
}

#[test]
#[deps(T_049: T_000)]
fn fork_049() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[49] = true;
    }
}

#[test]
#[deps(T_050: T_000)]
fn fork_050() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[50] = true;
    }
}

#[test]
#[deps(T_051: T_000)]
fn fork_051() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[51] = true;
    }
}

#[test]
#[deps(T_052: T_000)]
fn fork_052() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[52] = true;
    }
}

#[test]
#[deps(T_053: T_000)]
fn fork_053() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[53] = true;
    }
}

#[test]
#[deps(T_054: T_000)]
fn fork_054() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[54] = true;
    }
}

#[test]
#[deps(T_055: T_000)]
fn fork_055() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[55] = true;
    }
}

#[test]
#[deps(T_056: T_000)]
fn fork_056() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[56] = true;
    }
}

#[test]
#[deps(T_057: T_000)]
fn fork_057() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[57] = true;
    }
}

#[test]
#[deps(T_058: T_000)]
fn fork_058() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[58] = true;
    }
}

#[test]
#[deps(T_059: T_000)]
fn fork_059() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[59] = true;
    }
}

#[test]
#[deps(T_060: T_000)]
fn fork_060() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[60] = true;
    }
}

#[test]
#[deps(T_061: T_000)]
fn fork_061() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[61] = true;
    }
}

#[test]
#[deps(T_062: T_000)]
fn fork_062() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[62] = true;
    }
}

#[test]
#[deps(T_063: T_000)]
fn fork_063() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[63] = true;
    }
}

#[test]
#[deps(T_064: T_000)]
fn fork_064() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[64] = true;
    }
}

#[test]
#[deps(T_065: T_000)]
fn fork_065() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[65] = true;
    }
}

#[test]
#[deps(T_066: T_000)]
fn fork_066() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[66] = true;
    }
}

#[test]
#[deps(T_067: T_000)]
fn fork_067() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[67] = true;
    }
}

#[test]
#[deps(T_068: T_000)]
fn fork_068() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[68] = true;
    }
}

#[test]
#[deps(T_069: T_000)]
fn fork_069() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[69] = true;
    }
}

#[test]
#[deps(T_070: T_000)]
fn fork_070() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[70] = true;
    }
}

#[test]
#[deps(T_071: T_000)]
fn fork_071() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[71] = true;
    }
}

#[test]
#[deps(T_072: T_000)]
fn fork_072() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[72] = true;
    }
}

#[test]
#[deps(T_073: T_000)]
fn fork_073() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[73] = true;
    }
}

#[test]
#[deps(T_074: T_000)]
fn fork_074() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[74] = true;
    }
}

#[test]
#[deps(T_075: T_000)]
fn fork_075() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[75] = true;
    }
}

#[test]
#[deps(T_076: T_000)]
fn fork_076() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[76] = true;
    }
}

#[test]
#[deps(T_077: T_000)]
fn fork_077() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[77] = true;
    }
}

#[test]
#[deps(T_078: T_000)]
fn fork_078() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[78] = true;
    }
}

#[test]
#[deps(T_079: T_000)]
fn fork_079() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[79] = true;
    }
}

#[test]
#[deps(T_080: T_000)]
fn fork_080() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[80] = true;
    }
}

#[test]
#[deps(T_081: T_000)]
fn fork_081() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[81] = true;
    }
}

#[test]
#[deps(T_082: T_000)]
fn fork_082() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[82] = true;
    }
}

#[test]
#[deps(T_083: T_000)]
fn fork_083() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[83] = true;
    }
}

#[test]
#[deps(T_084: T_000)]
fn fork_084() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[84] = true;
    }
}

#[test]
#[deps(T_085: T_000)]
fn fork_085() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[85] = true;
    }
}

#[test]
#[deps(T_086: T_000)]
fn fork_086() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[86] = true;
    }
}

#[test]
#[deps(T_087: T_000)]
fn fork_087() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[87] = true;
    }
}

#[test]
#[deps(T_088: T_000)]
fn fork_088() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[88] = true;
    }
}

#[test]
#[deps(T_089: T_000)]
fn fork_089() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[89] = true;
    }
}

#[test]
#[deps(T_090: T_000)]
fn fork_090() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[90] = true;
    }
}

#[test]
#[deps(T_091: T_000)]
fn fork_091() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[91] = true;
    }
}

#[test]
#[deps(T_092: T_000)]
fn fork_092() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[92] = true;
    }
}

#[test]
#[deps(T_093: T_000)]
fn fork_093() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[93] = true;
    }
}

#[test]
#[deps(T_094: T_000)]
fn fork_094() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[94] = true;
    }
}

#[test]
#[deps(T_095: T_000)]
fn fork_095() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[95] = true;
    }
}

#[test]
#[deps(T_096: T_000)]
fn fork_096() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[96] = true;
    }
}

#[test]
#[deps(T_097: T_000)]
fn fork_097() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[97] = true;
    }
}

#[test]
#[deps(T_098: T_000)]
fn fork_098() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[98] = true;
    }
}

#[test]
#[deps(T_099: T_000)]
fn fork_099() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[99] = true;
    }
}

#[test]
#[deps(T_100: T_000)]
fn fork_100() {
    unsafe {
        assert!(LEAF[0]);
        LEAF[100] = true;
    }
}

