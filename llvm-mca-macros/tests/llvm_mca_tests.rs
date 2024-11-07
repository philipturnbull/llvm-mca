#[macro_use]
extern crate llvm_mca_macros;

#[test]
#[llvm_mca]
fn compiles_attribute() {}

#[test]
#[llvm_mca(allow_inline)]
fn compiles_attribute_allow_inline() {}
