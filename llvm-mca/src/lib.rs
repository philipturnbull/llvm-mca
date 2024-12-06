#![doc = include_str!("../README.md")]

/// Emit an `LLVM-MCA-BEGIN` marker, with an optional name.
///
/// # Examples
///
/// ```
/// use llvm_mca::llvm_mca_begin;
///
/// // An unnamed region: "LLVM-MCA-BEGIN"
/// llvm_mca_begin!();
/// // A named region: "LLVM-MCA-BEGIN my-region-name"
/// llvm_mca_begin!("my-region-name");
/// ```
#[macro_export]
macro_rules! llvm_mca_begin {
    () => {
        unsafe {
            std::arch::asm!(
                ";# LLVM-MCA-BEGIN",
                options(nostack, nomem, preserves_flags)
            );
        }
    };
    ($name:literal) => {
        unsafe {
            std::arch::asm!(
                concat!(";# LLVM-MCA-BEGIN ", $name),
                options(nostack, nomem, preserves_flags)
            );
        }
    };
}

/// Emit an `LLVM-MCA-END` marker, with an optional name.
///
/// # Examples
///
/// ```
/// use llvm_mca::llvm_mca_end;
/// // An unnamed region: "LLVM-MCA-END"
/// llvm_mca_end!();
/// // A named region: "LLVM-MCA-END my-region-name"
/// llvm_mca_end!("my-region-name");
/// ```
#[macro_export]
macro_rules! llvm_mca_end {
    () => {
        unsafe {
            std::arch::asm!(
                concat!(";# LLVM-MCA-END"),
                options(nostack, nomem, preserves_flags)
            );
        }
    };
    ($name:literal) => {
        unsafe {
            std::arch::asm!(
                concat!(";# LLVM-MCA-END ", $name),
                options(nostack, nomem, preserves_flags)
            );
        }
    };
}
