This is an attempt to run [C2Rust](https://c2rust.com/) on [OpenConnect](https://www.infradead.org/openconnect/) on my MacBook. If you clone this repo, you will get some [errors linking](https://github.com/ctaggart/openconnect-rust/issues).

## regenerate source code using c2rust
It requires being able to build OpenConnect. Try out [build-openconnect-osx.sh](build-openconnect-osx.sh). Make sure that works. It is used by [run-c2rust.sh](run-c2rust.sh) to capture the build and translate the files. The result it committed. If you try to 