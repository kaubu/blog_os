# blog_os
An operating system built in Rust!

## Building
### Setup
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

### Building the image
Once the setup is done, you can build the image by simply running
`cargo bootimage`.

## Booting the OS

### cargo run
If you have *qemu* installed, you can type `cargo run` to quickly compile the
OS and then boot into it with qemu.

### qemu
If you don't want to recompile the OS from scratch, you can boot in using qemu
with this command:

```sh
qemu-system-x86_64 -drive format=raw,file=path/to/bootimage-blog_os.bin
```

### Writing the image to a USB stick
**Be careful** to choose the correct device name, because
**everything on that device is overwritten.**

```sh
dd if=path/to/bootimage-blog_os.bin of=/dev/sdX && sync
```

Where `sdX` is the name of your USB device.