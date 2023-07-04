# blog_os
An operating system built in Rust!

## Building
First, install the Rust nightly toolchain (optional, it will download
automatically just by building):

```sh
rustup toolchain install nightly
```

Download the Rust source code:

```sh
rustup component add rust-src
```

Install the `bootimage` bootloader:

```sh
cargo install bootimage
```

Add the `llvm-tools-preview` rustup component:

```sh
rustup component add llvm-tools-preview
```