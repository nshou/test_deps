use test_deps::deps;
static mut COUNTER: usize = 0;

#[test]
#[deps(T_000)]
fn serial_000() {
    unsafe {
        assert_eq!(0, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_001: T_000)]
fn serial_001() {
    unsafe {
        assert_eq!(1, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_002: T_001)]
fn serial_002() {
    unsafe {
        assert_eq!(2, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_003: T_002)]
fn serial_003() {
    unsafe {
        assert_eq!(3, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_004: T_003)]
fn serial_004() {
    unsafe {
        assert_eq!(4, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_005: T_004)]
fn serial_005() {
    unsafe {
        assert_eq!(5, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_006: T_005)]
fn serial_006() {
    unsafe {
        assert_eq!(6, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_007: T_006)]
fn serial_007() {
    unsafe {
        assert_eq!(7, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_008: T_007)]
fn serial_008() {
    unsafe {
        assert_eq!(8, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_009: T_008)]
fn serial_009() {
    unsafe {
        assert_eq!(9, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_010: T_009)]
fn serial_010() {
    unsafe {
        assert_eq!(10, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_011: T_010)]
fn serial_011() {
    unsafe {
        assert_eq!(11, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_012: T_011)]
fn serial_012() {
    unsafe {
        assert_eq!(12, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_013: T_012)]
fn serial_013() {
    unsafe {
        assert_eq!(13, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_014: T_013)]
fn serial_014() {
    unsafe {
        assert_eq!(14, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_015: T_014)]
fn serial_015() {
    unsafe {
        assert_eq!(15, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_016: T_015)]
fn serial_016() {
    unsafe {
        assert_eq!(16, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_017: T_016)]
fn serial_017() {
    unsafe {
        assert_eq!(17, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_018: T_017)]
fn serial_018() {
    unsafe {
        assert_eq!(18, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_019: T_018)]
fn serial_019() {
    unsafe {
        assert_eq!(19, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_020: T_019)]
fn serial_020() {
    unsafe {
        assert_eq!(20, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_021: T_020)]
fn serial_021() {
    unsafe {
        assert_eq!(21, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_022: T_021)]
fn serial_022() {
    unsafe {
        assert_eq!(22, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_023: T_022)]
fn serial_023() {
    unsafe {
        assert_eq!(23, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_024: T_023)]
fn serial_024() {
    unsafe {
        assert_eq!(24, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_025: T_024)]
fn serial_025() {
    unsafe {
        assert_eq!(25, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_026: T_025)]
fn serial_026() {
    unsafe {
        assert_eq!(26, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_027: T_026)]
fn serial_027() {
    unsafe {
        assert_eq!(27, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_028: T_027)]
fn serial_028() {
    unsafe {
        assert_eq!(28, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_029: T_028)]
fn serial_029() {
    unsafe {
        assert_eq!(29, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_030: T_029)]
fn serial_030() {
    unsafe {
        assert_eq!(30, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_031: T_030)]
fn serial_031() {
    unsafe {
        assert_eq!(31, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_032: T_031)]
fn serial_032() {
    unsafe {
        assert_eq!(32, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_033: T_032)]
fn serial_033() {
    unsafe {
        assert_eq!(33, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_034: T_033)]
fn serial_034() {
    unsafe {
        assert_eq!(34, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_035: T_034)]
fn serial_035() {
    unsafe {
        assert_eq!(35, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_036: T_035)]
fn serial_036() {
    unsafe {
        assert_eq!(36, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_037: T_036)]
fn serial_037() {
    unsafe {
        assert_eq!(37, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_038: T_037)]
fn serial_038() {
    unsafe {
        assert_eq!(38, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_039: T_038)]
fn serial_039() {
    unsafe {
        assert_eq!(39, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_040: T_039)]
fn serial_040() {
    unsafe {
        assert_eq!(40, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_041: T_040)]
fn serial_041() {
    unsafe {
        assert_eq!(41, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_042: T_041)]
fn serial_042() {
    unsafe {
        assert_eq!(42, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_043: T_042)]
fn serial_043() {
    unsafe {
        assert_eq!(43, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_044: T_043)]
fn serial_044() {
    unsafe {
        assert_eq!(44, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_045: T_044)]
fn serial_045() {
    unsafe {
        assert_eq!(45, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_046: T_045)]
fn serial_046() {
    unsafe {
        assert_eq!(46, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_047: T_046)]
fn serial_047() {
    unsafe {
        assert_eq!(47, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_048: T_047)]
fn serial_048() {
    unsafe {
        assert_eq!(48, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_049: T_048)]
fn serial_049() {
    unsafe {
        assert_eq!(49, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_050: T_049)]
fn serial_050() {
    unsafe {
        assert_eq!(50, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_051: T_050)]
fn serial_051() {
    unsafe {
        assert_eq!(51, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_052: T_051)]
fn serial_052() {
    unsafe {
        assert_eq!(52, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_053: T_052)]
fn serial_053() {
    unsafe {
        assert_eq!(53, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_054: T_053)]
fn serial_054() {
    unsafe {
        assert_eq!(54, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_055: T_054)]
fn serial_055() {
    unsafe {
        assert_eq!(55, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_056: T_055)]
fn serial_056() {
    unsafe {
        assert_eq!(56, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_057: T_056)]
fn serial_057() {
    unsafe {
        assert_eq!(57, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_058: T_057)]
fn serial_058() {
    unsafe {
        assert_eq!(58, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_059: T_058)]
fn serial_059() {
    unsafe {
        assert_eq!(59, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_060: T_059)]
fn serial_060() {
    unsafe {
        assert_eq!(60, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_061: T_060)]
fn serial_061() {
    unsafe {
        assert_eq!(61, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_062: T_061)]
fn serial_062() {
    unsafe {
        assert_eq!(62, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_063: T_062)]
fn serial_063() {
    unsafe {
        assert_eq!(63, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_064: T_063)]
fn serial_064() {
    unsafe {
        assert_eq!(64, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_065: T_064)]
fn serial_065() {
    unsafe {
        assert_eq!(65, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_066: T_065)]
fn serial_066() {
    unsafe {
        assert_eq!(66, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_067: T_066)]
fn serial_067() {
    unsafe {
        assert_eq!(67, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_068: T_067)]
fn serial_068() {
    unsafe {
        assert_eq!(68, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_069: T_068)]
fn serial_069() {
    unsafe {
        assert_eq!(69, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_070: T_069)]
fn serial_070() {
    unsafe {
        assert_eq!(70, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_071: T_070)]
fn serial_071() {
    unsafe {
        assert_eq!(71, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_072: T_071)]
fn serial_072() {
    unsafe {
        assert_eq!(72, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_073: T_072)]
fn serial_073() {
    unsafe {
        assert_eq!(73, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_074: T_073)]
fn serial_074() {
    unsafe {
        assert_eq!(74, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_075: T_074)]
fn serial_075() {
    unsafe {
        assert_eq!(75, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_076: T_075)]
fn serial_076() {
    unsafe {
        assert_eq!(76, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_077: T_076)]
fn serial_077() {
    unsafe {
        assert_eq!(77, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_078: T_077)]
fn serial_078() {
    unsafe {
        assert_eq!(78, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_079: T_078)]
fn serial_079() {
    unsafe {
        assert_eq!(79, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_080: T_079)]
fn serial_080() {
    unsafe {
        assert_eq!(80, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_081: T_080)]
fn serial_081() {
    unsafe {
        assert_eq!(81, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_082: T_081)]
fn serial_082() {
    unsafe {
        assert_eq!(82, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_083: T_082)]
fn serial_083() {
    unsafe {
        assert_eq!(83, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_084: T_083)]
fn serial_084() {
    unsafe {
        assert_eq!(84, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_085: T_084)]
fn serial_085() {
    unsafe {
        assert_eq!(85, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_086: T_085)]
fn serial_086() {
    unsafe {
        assert_eq!(86, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_087: T_086)]
fn serial_087() {
    unsafe {
        assert_eq!(87, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_088: T_087)]
fn serial_088() {
    unsafe {
        assert_eq!(88, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_089: T_088)]
fn serial_089() {
    unsafe {
        assert_eq!(89, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_090: T_089)]
fn serial_090() {
    unsafe {
        assert_eq!(90, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_091: T_090)]
fn serial_091() {
    unsafe {
        assert_eq!(91, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_092: T_091)]
fn serial_092() {
    unsafe {
        assert_eq!(92, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_093: T_092)]
fn serial_093() {
    unsafe {
        assert_eq!(93, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_094: T_093)]
fn serial_094() {
    unsafe {
        assert_eq!(94, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_095: T_094)]
fn serial_095() {
    unsafe {
        assert_eq!(95, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_096: T_095)]
fn serial_096() {
    unsafe {
        assert_eq!(96, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_097: T_096)]
fn serial_097() {
    unsafe {
        assert_eq!(97, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_098: T_097)]
fn serial_098() {
    unsafe {
        assert_eq!(98, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_099: T_098)]
fn serial_099() {
    unsafe {
        assert_eq!(99, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

#[test]
#[deps(T_100: T_099)]
fn serial_100() {
    unsafe {
        assert_eq!(100, COUNTER);
        COUNTER = COUNTER + 1;
    }
}

