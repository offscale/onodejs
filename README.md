onodejs
=======

`onodejs` is a cross-platform (`nvm`)[https://github.com/creationix/nvm]/[`n`](https://github.com/tj/n) alternative.

## RFCs
Of interest are our RFCs:
https://github.com/offscale/offscale-rfcs

## Developer guide

Install the latest version of [Rust](https://www.rust-lang.org). We tend to use nightly versions. [CLI tool for installing Rust](https://rustup.rs).

We use [rust-clippy](https://github.com/rust-lang-nursery/rust-clippy) linters to improve code quality.

There are plenty of [IDEs](https://areweideyet.com) and other [Rust development tools to consider](https://github.com/rust-unofficial/awesome-rust#development-tools).

### Step-by-step guide
```bash
# Install Rust (nightly)
$ curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly
# Install cargo-make (cross-platform feature-rich reimplementation of Make)
$ cargo install --force cargo-make
# Install rustfmt (Rust formatter)
$ rustup component add rustfmt
# Clone this repo
$ git clone "$this_org>/$this_repo" && cd "$this_repo"
# Run tests
$ cargo test
# Format, build and test
$ cargo make
```
