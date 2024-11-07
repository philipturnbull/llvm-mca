# `llvm-mca-macros`

Procedural macros to generate marker comments for [LLVM's Machine Code Analyzer](https://llvm.org/docs/CommandGuide/llvm-mca.html).

These macros generate markers after the function epilogue and before the function prologue. If more granularity is needed, you can use the `llvm-mca` crate instead.

## Usage

By default, `llvm_mca` will disable inlining. For example, this:

```rust
use llvm_mca_macros::llvm_mca;

#[llvm_mca]
fn quadruple(x: u32) -> u32 {
    let doubled = x + x;
    doubled + doubled
}
```

will generate the equivalent of:

```rust
#[inline(never)]
fn quadruple(x: u32) -> u32 {
    // emit `LLVM-MCA-BEGIN` marker
    let ret = {
        let doubled = x + x;
        doubled + doubled
    };
    // emit `LLVM-MCA-END` marker
    ret
}
```

If inlining is desired, the `allow_inline` attribute can be specified:

```rust
use llvm_mca_macros::llvm_mca;

#[llvm_mca(allow_inline)]
fn quadruple(x: u32) -> u32 {
    let doubled = x + x;
    doubled + doubled
}
```

This will generate the equivalent of:

```rust
fn quadruple(x: u32) -> u32 {
    // emit `LLVM-MCA-BEGIN` marker
    let ret = {
        let doubled = x + x;
        doubled + doubled
    };
    // emit `LLVM-MCA-END` marker
    ret
}
```

# Generating assembly

You must set the `RUSTFLAGS="--emit asm"` option when building your project with `cargo`. For example:

```sh
RUSTFLAGS="--emit asm" cargo build --release
```

This will output assembly files in `target/*/deps`