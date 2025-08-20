
# Tools
    About tools



<!--------------------------------------------------------------------------------- rustup -->
<br><br>

## rustup
```bash
rustup --version
rustup --help
rustup install nightly
rustup toolchain list
rustup default nightly
rustup default stable
rustup component add rustfmt
```



<!--------------------------------------------------------------------------------- cargo -->
<br><br>

## cargo
General
```bash
cargo --help
cargo --version
```

Create a package as a binary
```bash
    cargo new my_project_1 
    cargo new my_project_1 --bin
```

Create a package as a library
```bash
    cargo new my_library_1 --lib
```

Add external package
```bash
    cargo add random
```

Install a Rust binary
```bash
    cargo install diesel_cli --no-default-features --features sqlite
```


Other
```bash
cargo run
cargo run -q
cargo run --release
cargo build
cargo build --release
cargo fmt
```



<!--------------------------------------------------------------------------------- rust-analyzer -->
<br><br>

## rust-analyzer



<!--------------------------------------------------------------------------------- rust-gdb -->
<br><br>

## rust-gdb



<!--------------------------------------------------------------------------------- rust-gdbgui -->
<br><br>

## rust-gdbgui



<!--------------------------------------------------------------------------------- rust-lldb -->
<br><br>

## rust-lldb



<!--------------------------------------------------------------------------------- rustc -->
<br><br>

## rustc



<!--------------------------------------------------------------------------------- rustdoc -->
<br><br>

## rustdoc



<!--------------------------------------------------------------------------------- rustfmt -->
<br><br>

## rustfmt
