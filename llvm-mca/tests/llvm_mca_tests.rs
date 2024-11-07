use llvm_mca::{llvm_mca_begin, llvm_mca_end};

#[test]
fn compiles_begin_end() {
    // Region name are optional
    llvm_mca_begin!();
    llvm_mca_end!();
}

#[test]
fn compiles_begin_end_name() {
    llvm_mca_begin!("name");
    llvm_mca_end!("name");

    // There are no checks if region names are duplicated.
    llvm_mca_begin!("name");
    // There are no checks if region names do not match.
    llvm_mca_end!("other-name");

    // Region names do not have to be string literals.
    llvm_mca_begin!(1234);
    llvm_mca_end!(1235);
}
