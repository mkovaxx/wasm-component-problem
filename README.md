# wasm-component-problem

Demonstrates a problem with cargo-component and wit-bindgen

## Version Info

```
$ cargo component --version
cargo-component-component 0.9.0 (wasi:ab5a448)

$ cat rust-toolchain
1.75.0
```

## Steps to Reproduce

```
cargo component build --package parser
cargo component build --package cli
```

## Error

```
$ RUST_BACKTRACE=1 cargo component build --package cli   
  Generating bindings for cli (components/cli/src/bindings.rs)
thread 'main' panicked at /Users/mate/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wit-bindgen-rust-0.20.0/src/interface.rs:1764:73:
no entry found for key
stack backtrace:
   0: _rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::option::expect_failed
   3: wit_bindgen_rust::interface::InterfaceGenerator::path_to_interface
   4: wit_bindgen_rust::interface::InterfaceGenerator::type_path_with_name
   5: wit_bindgen_rust::interface::InterfaceGenerator::print_ty
   6: wit_bindgen_core::InterfaceGenerator::define_type
   7: <wit_bindgen_rust::RustWasm as wit_bindgen_core::WorldGenerator>::import_interface
   8: wit_bindgen_core::WorldGenerator::generate
   9: cargo_component::run_cargo_command::{{closure}}
  10: cargo_component::main::{{closure}}
  11: cargo_component::main
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
