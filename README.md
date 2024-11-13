# ri-optimization

## Pre-requisites
- [Rust](https://www.rust-lang.org/)
- [Git](https://git-scm.com/downloads)

## Setup
Clone the repository:
```bash
git clone https://github.com/hiibolt/ri_optimization.git
```

Change into the directory of the project:
```bash
cd ri_optimization
```

Start the project:
```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release && ./target/release/ri_optimization 1000000000 600
```
...where the first argument is the sample size and the second is the number of threads to allocate.