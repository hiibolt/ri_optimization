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
RUSTFLAGS="-C target-cpu=native" cargo build --release && ./target/release/ri_optimization 1000000000 -100000 100000 600
```
...where the first argument is the sample size, 2nd/3rd are the lower/upper bound, and the fourth is the number of threads to allocate for computation.