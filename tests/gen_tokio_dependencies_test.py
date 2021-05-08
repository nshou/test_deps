#!/usr/bin/env python3

import textwrap

TEST_DELAY_SEC = 0.5

def common_imports():
    print(textwrap.dedent("""\
        use rand::Rng;
        use test_deps::deps;
        use tokio::time::{self, Duration};
    """))

def gen_serial_test(n="5"):
    base_sleep_sec = TEST_DELAY_SEC / int(n)
    print(textwrap.dedent("""\

        static mut COUNTER_SERIAL: usize = 0;

        #[deps(SERIAL_000)]
        #[tokio::test]
        async fn tokio_serial_000() {
            time::sleep(Duration::from_secs_f64(%f * rand::thread_rng().gen::<f64>())).await;
            unsafe {
                assert_eq!(0, COUNTER_SERIAL);
                COUNTER_SERIAL = COUNTER_SERIAL + 1;
            }
        }
    """ % base_sleep_sec))
    for i in range(int(n)):
        print(textwrap.dedent(f"""\
            #[deps(SERIAL_{i+1:03d}: SERIAL_{i:03d})]
            #[tokio::test]
            async fn tokio_serial_{i+1:03d}() {{
                time::sleep(Duration::from_secs_f64({base_sleep_sec:f} * rand::thread_rng().gen::<f64>())).await;
                unsafe {{
                    assert_eq!({i+1}, COUNTER_SERIAL);
                    COUNTER_SERIAL = COUNTER_SERIAL + 1;
                }}
            }}
        """))

def gen_fork_test(n="5"):
    print(textwrap.dedent("""\

        static mut LEAF_FORK: [bool; {n}] = [false; {n}];

        #[deps(FORK_000)]
        #[tokio::test]
        async fn tokio_fork_000() {{
            time::sleep(Duration::from_secs_f64({s})).await;
            unsafe {{
                for l in &LEAF_FORK[1..] {{
                    assert!(!l);
                }}
                LEAF_FORK[0] = true;
            }}
        }}
    """.format(n=int(n) + 1, s=TEST_DELAY_SEC)))
    for i in range(int(n)):
        print(textwrap.dedent(f"""\
            #[deps(FORK_{i+1:03d}: FORK_000)]
            #[tokio::test]
            async fn tokio_fork_{i+1:03d}() {{
                unsafe {{
                    assert!(LEAF_FORK[0]);
                    LEAF_FORK[{i+1}] = true;
                }}
            }}
        """))

def gen_merge_test(n="5"):
    base_sleep_sec = TEST_DELAY_SEC / int(n)
    print(textwrap.dedent("""\

        static mut LEAF_MERGE: [bool; {n}] = [false; {n}];

        #[deps(MERGE_000: {d})]
        #[tokio::test]
        async fn tokio_merge_000() {{
            unsafe {{
                for l in &LEAF_MERGE[1..] {{
                    assert!(l);
                }}
                LEAF_MERGE[0] = true;
            }}
        }}
    """.format(n=int(n) + 1, d=" ".join(["MERGE_%03d" % (i + 1) for i in range(int(n))]))))
    for i in range(int(n)):
        print(textwrap.dedent(f"""\
            #[deps(MERGE_{i+1:03d})]
            #[tokio::test]
            async fn tokio_merge_{i+1:03d}() {{
                time::sleep(Duration::from_secs_f64({base_sleep_sec:f} * rand::thread_rng().gen::<f64>())).await;
                unsafe {{
                    assert!(!LEAF_MERGE[0]);
                    LEAF_MERGE[{i+1}] = true;
                }}
            }}
        """))

def gen_neural_network(f="5", d="4"):
    f = int(f)
    d = int(d)
    base_sleep_sec = TEST_DELAY_SEC / (f * d)
    print(textwrap.dedent("""\

        static mut N: [usize; {f_2}] = [0; {f_2}];

        #[deps(NN_000: {dep})]
        #[tokio::test]
        async fn tokio_neural_network_000() {{
            let mut input = 0;
            let pos = {d} % 2;
            time::sleep(Duration::from_secs_f64({s:f} * rand::thread_rng().gen::<f64>())).await;
            unsafe {{
                for n in &N[(pos * {f})..((pos + 1) * {f})] {{
                    input = input + n;
                }}
            }}
            assert_eq!({o}, input);
        }}
    """.format(f=f, f_2=f*2, d=d, dep=" ".join(["NN_%03d" % i for i in range((d - 1) * f + 1, d * f + 1)]), o=f**d, s=base_sleep_sec)))
    for _f in range(f):
        print(textwrap.dedent("""\
            #[deps(NN_{me})]
            #[tokio::test]
            async fn tokio_neural_network_{me}() {{
                time::sleep(Duration::from_secs_f64({s:f} * rand::thread_rng().gen::<f64>())).await;
                unsafe {{
                    for n in &N[..{f}] {{
                        assert_eq!(0, *n);
                    }}
                    N[{f} + {_f}] = 1;
                }}
            }}
        """.format(me="%03d" % (_f + 1), f=f, _f=_f, s=base_sleep_sec)))
    for _d in range(1, d):
        for _f in range(f):
            print(textwrap.dedent("""\
                #[deps(NN_{me}: {dep})]
                #[tokio::test]
                async fn tokio_neural_network_{me}() {{
                    let mut input = 0;
                    let pos = {_d} % 2;
                    time::sleep(Duration::from_secs_f64({s:f} * rand::thread_rng().gen::<f64>())).await;
                    unsafe {{
                        for n in &N[(pos * {f})..((pos + 1) * {f})] {{
                            input = input + n;
                        }}
                    }}
                    assert_eq!({o}, input);
                    unsafe {{
                        N[(pos ^ 1) * {f} + {_f}] = {o};
                    }}
                }}
            """.format(me="%03d" % (_d * f + _f + 1), dep=" ".join(["NN_%03d" % i for i in range((_d - 1) * f + 1, _d * f + 1)]), _d=_d, f=f, _f=_f, o=f**_d, s=base_sleep_sec)))

if __name__ == "__main__":
    common_imports()
    fs = [x[0] for x in globals().items() if callable(x[1]) and x[0].startswith("gen_")]
    for f in fs:
        globals()[f]()
