#!/usr/bin/env python3

import textwrap
import sys

def gen_nrm_serial(n="100"):
    print(textwrap.dedent("""\
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
    """))
    for i in range(int(n)):
        print(textwrap.dedent(f"""\
            #[test]
            #[deps(T_{i+1:03d}: T_{i:03d})]
            fn serial_{i+1:03d}() {{
                unsafe {{
                    assert_eq!({i+1}, COUNTER);
                    COUNTER = COUNTER + 1;
                }}
            }}
        """))

def gen_nrm_fork(n="100"):
    print(textwrap.dedent("""\
        use test_deps::deps;
        static mut LEAF: [bool; {n}] = [false; {n}];

        #[test]
        #[deps(T_000)]
        fn fork_000() {{
            unsafe {{
                for l in &LEAF[1..] {{
                    assert!(!l);
                }}
                LEAF[0] = true;
            }}
        }}
    """.format(n=int(n) + 1)))
    for i in range(int(n)):
        print(textwrap.dedent(f"""\
            #[test]
            #[deps(T_{i+1:03d}: T_000)]
            fn fork_{i+1:03d}() {{
                unsafe {{
                    assert!(LEAF[0]);
                    LEAF[{i+1}] = true;
                }}
            }}
        """))

def gen_nrm_merge(n="100"):
    print(textwrap.dedent("""\
        use test_deps::deps;
        static mut LEAF: [bool; {n}] = [false; {n}];

        #[test]
        #[deps(T_000: {d})]
        fn merge_000() {{
            unsafe {{
                for l in &LEAF[1..] {{
                    assert!(l);
                }}
                LEAF[0] = true;
            }}
        }}
    """.format(n=int(n) + 1, d=" ".join(["T_%03d" % (i + 1) for i in range(int(n))]))))
    for i in range(int(n)):
        print(textwrap.dedent(f"""\
            #[test]
            #[deps(T_{i+1:03d})]
            fn merge_{i+1:03d}() {{
                unsafe {{
                    assert!(!LEAF[0]);
                    LEAF[{i+1}] = true;
                }}
            }}
        """))

if __name__ == "__main__":
    if len(sys.argv) == 1:
        fs = [x[0] for x in globals().items() if callable(x[1]) and x[0].startswith("gen_")]
        for f in sorted(fs):
            print(f[len("gen_"):])
    else:
        globals()["gen_%s" % sys.argv[1]](*sys.argv[2:])
