# Libmypaint's Rust port

This is heavily in development. Only use this if you like bugs.

## Why port libmypaint to Rust?

Why not? Honestly, it's not a great library. Mypaint suffers from it being too generic, and nothing else uses it (to my knowledge) because it's not generic enough. It being written in C means there's likely dozens of major bugs that haven't been discovered yet. In the end, libmypaint really needs a fresh start.

## What are your goals?

* Step 1: Have a fully ABI-compatible libmypaint.so generated entirely in Rust with no C inter-op outside of FFI.
* Step 2: Clean this shit up. A direct port of C code to Rust does *not* look pretty.
* Step 3?: Possibly try and move some of the C++ in Mypaint to Rust. [rust-cpython](https://github.com/dgrunwald/rust-cpython) will help with this.
* Ultimate goal: Fully eliminate all unsafe code from Mypaint.

There's a few side goals as well, that I'm sure are manageable. For example, leveraging [Rust's excellent concurrency tools](https://github.com/nikomatsakis/rayon/) to fully parallelize brush stroke painting would be pretty nice. Also a fully-featured test suite will make lives much easier.

## So, where are you at now?

Around 70% ported. Got a few bugs that need hunting down, which is annoying because of the lack of testing beyond "open Mypaint and draw a bit". Most functions are ported over to equivalent Rust, and some areas that aren't publicly exposed have been cleaned up a lot. Progress is slow but steady, as I tend to be busy, so chances are if you find something broken, a PR would really help out.

# Compiling and running

The current layout of the project is:

1. This crate is generating a .so
2. The .so is dynamically linked with C libmypaint's .so
3. Functions are removed from the C code, causing the linker to automatically use the Rust functions instead
4. Running Mypaint just involves telling it where our Rust-augmented libmypaint is

Steps:

1. Install Rust if you haven't already and run `cargo build --release` in this project.
2. Clone the [libmypaint repo](https://github.com/mypaint/libmypaint)
3. Checkout f052590 (just in case changes make incompatibilities pop up, which is unlikely as libmypaint development is slow)
4. Follow libmypaint's compilation guide, but replace `./configure` with `./configure LDFLAGS="-L/path/to/rustport/target/release/ -lrustport"`
5. It *should* make properly without complaining. If it doesn't then uh, make an issue?
6. Clone [Mypaint](https://github.com/mypaint/mypaint) and follow its installation instructions.
7. Export `LD_PRELOAD` as `/path/to/our/libmypaint/.libs/libmypaint.so` and `LD_LIBRARY_PATH` as `/path/to/rustport/target/release/`
8. Run Mypaint and pray that it works.
