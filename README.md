# test_deps

`test_deps` allows developers to define dependencies among tests.

## Quick Examples

```rust
// Serial
// A -> B -> C

#[deps(A)]
#[test]
fn test_a() {}

#[deps(B: A)]
#[test]
fn test_b() {}

#[deps(C: B)]
#[test]
fn test_c() {}
```

```rust
// Fork
// A -+-> B
//    `-> C

#[deps(A)]
#[test]
fn test_a() {}

#[deps(B: A)]
#[test]
fn test_b() {}

#[deps(C: A)]
#[test]
fn test_c() {}
```

```rust
// Merge
// A --\
// B --+-> C

#[deps(A)]
#[test]
fn test_a() {}

#[deps(B)]
#[test]
fn test_b() {}

#[deps(C: A B)]
#[test]
fn test_c() {}
```

## Usage

Add

```toml
[dev-dependencies]
test_deps = "1.0"
```

to your Cargo.toml and add

```rust
use test_deps::deps;
```

to your test module.

## Detailed Spec

See [docs.rs](https://docs.rs/test_deps).

## License

MIT. See COPYING.

## Donation

[Buy me a coffee](https://buymeacoffee.com/nshou) ☕
