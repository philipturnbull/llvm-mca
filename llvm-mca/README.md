# `llvm-mca`

Macros to generate marker comments for [LLVM's Machine Code Analyzer](https://llvm.org/docs/CommandGuide/llvm-mca.html).

The `llvm_mca_begin!` and `llvm_mca_end!` macros will emit `LLVM-MCA-BEGIN` and `LLVM-MCA-END` markers, respectively.

If you want to automatically add markers to the beginning and end of a function, you can use the `llvm-mca-macros` crate instead.

# Usage

For example, this:

```rust
use llvm_mca::{llvm_mca_begin, llvm_mca_end};

fn quadruple(x: u32) -> u32 {
    llvm_mca_begin!();
    let doubled = x + x;
    llvm_mca_end!();
    doubled + doubled
}
```

will generate the equivalent of:

```rust
fn quadruple(x: u32) -> u32 {
    // emit `LLVM-MCA-BEGIN` marker
    let doubled = x + x;
    // emit `LLVM-MCA-END` marker
    doubled + doubled
}
```

# Naming regions

Regions can also be named. This:

```rust
use llvm_mca::{llvm_mca_begin, llvm_mca_end};

fn quadruple(x: u32) -> u32 {
    llvm_mca_begin!("double");
    let doubled = x + x;
    llvm_mca_end!("double");
    doubled + doubled
}
```

will generated the equivalent of:

```rust
fn quadruple(x: u32) -> u32 {
    // emit `LLVM-MCA-BEGIN double` marker
    let doubled = x + x;
    // emit `LLVM-MCA-END double` marker
    doubled + doubled
}
```

See the [Using Markers to Analyze Specific Code Blocks](https://llvm.org/docs/CommandGuide/llvm-mca.html#using-markers-to-analyze-specific-code-blocks) section of the LLVM docs for details about naming regions and how to nest them.

# Generating assembly

You must set the `RUSTFLAGS="--emit asm"` option when building your project with `cargo`. For example:

```sh
RUSTFLAGS="--emit asm" cargo build --release
```

This will output assembly files in `target/*/deps`.