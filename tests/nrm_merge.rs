use test_deps::deps;
static mut LEAF: [bool; 101] = [false; 101];

#[test]
#[deps(T_000: T_001 T_002 T_003 T_004 T_005 T_006 T_007 T_008 T_009 T_010 T_011 T_012 T_013 T_014 T_015 T_016 T_017 T_018 T_019 T_020 T_021 T_022 T_023 T_024 T_025 T_026 T_027 T_028 T_029 T_030 T_031 T_032 T_033 T_034 T_035 T_036 T_037 T_038 T_039 T_040 T_041 T_042 T_043 T_044 T_045 T_046 T_047 T_048 T_049 T_050 T_051 T_052 T_053 T_054 T_055 T_056 T_057 T_058 T_059 T_060 T_061 T_062 T_063 T_064 T_065 T_066 T_067 T_068 T_069 T_070 T_071 T_072 T_073 T_074 T_075 T_076 T_077 T_078 T_079 T_080 T_081 T_082 T_083 T_084 T_085 T_086 T_087 T_088 T_089 T_090 T_091 T_092 T_093 T_094 T_095 T_096 T_097 T_098 T_099 T_100)]
fn merge_000() {
    unsafe {
        for l in &LEAF[1..] {
            assert!(l);
        }
        LEAF[0] = true;
    }
}

#[test]
#[deps(T_001)]
fn merge_001() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[1] = true;
    }
}

#[test]
#[deps(T_002)]
fn merge_002() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[2] = true;
    }
}

#[test]
#[deps(T_003)]
fn merge_003() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[3] = true;
    }
}

#[test]
#[deps(T_004)]
fn merge_004() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[4] = true;
    }
}

#[test]
#[deps(T_005)]
fn merge_005() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[5] = true;
    }
}

#[test]
#[deps(T_006)]
fn merge_006() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[6] = true;
    }
}

#[test]
#[deps(T_007)]
fn merge_007() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[7] = true;
    }
}

#[test]
#[deps(T_008)]
fn merge_008() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[8] = true;
    }
}

#[test]
#[deps(T_009)]
fn merge_009() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[9] = true;
    }
}

#[test]
#[deps(T_010)]
fn merge_010() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[10] = true;
    }
}

#[test]
#[deps(T_011)]
fn merge_011() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[11] = true;
    }
}

#[test]
#[deps(T_012)]
fn merge_012() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[12] = true;
    }
}

#[test]
#[deps(T_013)]
fn merge_013() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[13] = true;
    }
}

#[test]
#[deps(T_014)]
fn merge_014() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[14] = true;
    }
}

#[test]
#[deps(T_015)]
fn merge_015() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[15] = true;
    }
}

#[test]
#[deps(T_016)]
fn merge_016() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[16] = true;
    }
}

#[test]
#[deps(T_017)]
fn merge_017() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[17] = true;
    }
}

#[test]
#[deps(T_018)]
fn merge_018() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[18] = true;
    }
}

#[test]
#[deps(T_019)]
fn merge_019() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[19] = true;
    }
}

#[test]
#[deps(T_020)]
fn merge_020() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[20] = true;
    }
}

#[test]
#[deps(T_021)]
fn merge_021() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[21] = true;
    }
}

#[test]
#[deps(T_022)]
fn merge_022() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[22] = true;
    }
}

#[test]
#[deps(T_023)]
fn merge_023() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[23] = true;
    }
}

#[test]
#[deps(T_024)]
fn merge_024() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[24] = true;
    }
}

#[test]
#[deps(T_025)]
fn merge_025() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[25] = true;
    }
}

#[test]
#[deps(T_026)]
fn merge_026() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[26] = true;
    }
}

#[test]
#[deps(T_027)]
fn merge_027() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[27] = true;
    }
}

#[test]
#[deps(T_028)]
fn merge_028() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[28] = true;
    }
}

#[test]
#[deps(T_029)]
fn merge_029() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[29] = true;
    }
}

#[test]
#[deps(T_030)]
fn merge_030() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[30] = true;
    }
}

#[test]
#[deps(T_031)]
fn merge_031() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[31] = true;
    }
}

#[test]
#[deps(T_032)]
fn merge_032() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[32] = true;
    }
}

#[test]
#[deps(T_033)]
fn merge_033() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[33] = true;
    }
}

#[test]
#[deps(T_034)]
fn merge_034() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[34] = true;
    }
}

#[test]
#[deps(T_035)]
fn merge_035() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[35] = true;
    }
}

#[test]
#[deps(T_036)]
fn merge_036() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[36] = true;
    }
}

#[test]
#[deps(T_037)]
fn merge_037() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[37] = true;
    }
}

#[test]
#[deps(T_038)]
fn merge_038() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[38] = true;
    }
}

#[test]
#[deps(T_039)]
fn merge_039() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[39] = true;
    }
}

#[test]
#[deps(T_040)]
fn merge_040() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[40] = true;
    }
}

#[test]
#[deps(T_041)]
fn merge_041() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[41] = true;
    }
}

#[test]
#[deps(T_042)]
fn merge_042() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[42] = true;
    }
}

#[test]
#[deps(T_043)]
fn merge_043() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[43] = true;
    }
}

#[test]
#[deps(T_044)]
fn merge_044() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[44] = true;
    }
}

#[test]
#[deps(T_045)]
fn merge_045() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[45] = true;
    }
}

#[test]
#[deps(T_046)]
fn merge_046() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[46] = true;
    }
}

#[test]
#[deps(T_047)]
fn merge_047() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[47] = true;
    }
}

#[test]
#[deps(T_048)]
fn merge_048() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[48] = true;
    }
}

#[test]
#[deps(T_049)]
fn merge_049() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[49] = true;
    }
}

#[test]
#[deps(T_050)]
fn merge_050() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[50] = true;
    }
}

#[test]
#[deps(T_051)]
fn merge_051() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[51] = true;
    }
}

#[test]
#[deps(T_052)]
fn merge_052() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[52] = true;
    }
}

#[test]
#[deps(T_053)]
fn merge_053() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[53] = true;
    }
}

#[test]
#[deps(T_054)]
fn merge_054() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[54] = true;
    }
}

#[test]
#[deps(T_055)]
fn merge_055() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[55] = true;
    }
}

#[test]
#[deps(T_056)]
fn merge_056() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[56] = true;
    }
}

#[test]
#[deps(T_057)]
fn merge_057() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[57] = true;
    }
}

#[test]
#[deps(T_058)]
fn merge_058() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[58] = true;
    }
}

#[test]
#[deps(T_059)]
fn merge_059() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[59] = true;
    }
}

#[test]
#[deps(T_060)]
fn merge_060() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[60] = true;
    }
}

#[test]
#[deps(T_061)]
fn merge_061() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[61] = true;
    }
}

#[test]
#[deps(T_062)]
fn merge_062() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[62] = true;
    }
}

#[test]
#[deps(T_063)]
fn merge_063() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[63] = true;
    }
}

#[test]
#[deps(T_064)]
fn merge_064() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[64] = true;
    }
}

#[test]
#[deps(T_065)]
fn merge_065() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[65] = true;
    }
}

#[test]
#[deps(T_066)]
fn merge_066() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[66] = true;
    }
}

#[test]
#[deps(T_067)]
fn merge_067() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[67] = true;
    }
}

#[test]
#[deps(T_068)]
fn merge_068() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[68] = true;
    }
}

#[test]
#[deps(T_069)]
fn merge_069() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[69] = true;
    }
}

#[test]
#[deps(T_070)]
fn merge_070() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[70] = true;
    }
}

#[test]
#[deps(T_071)]
fn merge_071() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[71] = true;
    }
}

#[test]
#[deps(T_072)]
fn merge_072() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[72] = true;
    }
}

#[test]
#[deps(T_073)]
fn merge_073() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[73] = true;
    }
}

#[test]
#[deps(T_074)]
fn merge_074() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[74] = true;
    }
}

#[test]
#[deps(T_075)]
fn merge_075() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[75] = true;
    }
}

#[test]
#[deps(T_076)]
fn merge_076() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[76] = true;
    }
}

#[test]
#[deps(T_077)]
fn merge_077() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[77] = true;
    }
}

#[test]
#[deps(T_078)]
fn merge_078() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[78] = true;
    }
}

#[test]
#[deps(T_079)]
fn merge_079() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[79] = true;
    }
}

#[test]
#[deps(T_080)]
fn merge_080() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[80] = true;
    }
}

#[test]
#[deps(T_081)]
fn merge_081() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[81] = true;
    }
}

#[test]
#[deps(T_082)]
fn merge_082() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[82] = true;
    }
}

#[test]
#[deps(T_083)]
fn merge_083() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[83] = true;
    }
}

#[test]
#[deps(T_084)]
fn merge_084() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[84] = true;
    }
}

#[test]
#[deps(T_085)]
fn merge_085() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[85] = true;
    }
}

#[test]
#[deps(T_086)]
fn merge_086() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[86] = true;
    }
}

#[test]
#[deps(T_087)]
fn merge_087() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[87] = true;
    }
}

#[test]
#[deps(T_088)]
fn merge_088() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[88] = true;
    }
}

#[test]
#[deps(T_089)]
fn merge_089() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[89] = true;
    }
}

#[test]
#[deps(T_090)]
fn merge_090() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[90] = true;
    }
}

#[test]
#[deps(T_091)]
fn merge_091() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[91] = true;
    }
}

#[test]
#[deps(T_092)]
fn merge_092() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[92] = true;
    }
}

#[test]
#[deps(T_093)]
fn merge_093() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[93] = true;
    }
}

#[test]
#[deps(T_094)]
fn merge_094() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[94] = true;
    }
}

#[test]
#[deps(T_095)]
fn merge_095() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[95] = true;
    }
}

#[test]
#[deps(T_096)]
fn merge_096() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[96] = true;
    }
}

#[test]
#[deps(T_097)]
fn merge_097() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[97] = true;
    }
}

#[test]
#[deps(T_098)]
fn merge_098() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[98] = true;
    }
}

#[test]
#[deps(T_099)]
fn merge_099() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[99] = true;
    }
}

#[test]
#[deps(T_100)]
fn merge_100() {
    unsafe {
        assert!(!LEAF[0]);
        LEAF[100] = true;
    }
}

