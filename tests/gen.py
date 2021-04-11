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

if __name__ == "__main__":
    if len(sys.argv) == 1:
        fs = [x[0] for x in globals().items() if callable(x[1]) and x[0].startswith("gen_")]
        for f in sorted(fs):
            print(f[len("gen_"):])
    else:
        globals()["gen_%s" % sys.argv[1]](*sys.argv[2:])
