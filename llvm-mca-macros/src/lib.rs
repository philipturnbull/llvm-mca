#![doc = include_str!("../README.md")]
#![feature(proc_macro_quote)]
use proc_macro::{Span, TokenStream};
use quote::{quote, ToTokens as _};
use syn::{
    parse::{Parse, ParseStream},
    parse_quote, Item, ItemFn,
};

struct MacroArgs {
    allow_inline: bool,
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self {
                allow_inline: false,
            });
        }

        // TODO: Allow specifying regions names in arguments, to match
        // `llvm_mca_begin!(...)` and `llvm_mca_end!(...)`
        let arg = input.parse::<syn::Ident>()?;
        if arg == "allow_inline" {
            Ok(Self { allow_inline: true })
        } else {
            Err(syn::Error::new(arg.span(), "expected `allow_inline`"))
        }
    }
}

/// Wrap the body of a function with `LLVM-MCA-BEGIN` and `LLVM-MCA-END` markers.
///
/// The markers are inserted as inline assembly, after the function prologue and
/// before the function epilogue.
///
/// # Examples
///
/// This:
/// ```
/// use llvm_mca_macros::llvm_mca;
/// #[llvm_mca]
/// fn quadruple(x: u32) -> u32 {
///     let doubled = x + x;
///     doubled + doubled
/// }
/// ```
///
/// is equivalent to:
/// ```
/// #[inline(never)]
/// fn quadruple(x: u32) -> u32 {
///     // emit LLVM-MCA-BEGIN marker
///     let ret = {
///         let doubled = x + x;
///         doubled + doubled
///     };
///     // emit LLVM-MCA-END marker
///     ret
/// }
/// ```
///
/// If inlining is desired, specify the `allow_inline` argument:
/// ```
/// use llvm_mca_macros::llvm_mca;
/// #[llvm_mca(allow_inline)]
/// fn quadruple(x: u32) -> u32 {
///     let doubled = x + x;
///     doubled + doubled
/// }
/// ```
///
/// which is equivalent to:
/// ```
/// fn quadruple(x: u32) -> u32 {
///     // emit LLVM-MCA-BEGIN marker
///     let ret = {
///         let doubled = x + x;
///         doubled + doubled
///     };
///     // emit LLVM-MCA-END marker
///     ret
/// }
/// ```
#[proc_macro_attribute]
pub fn llvm_mca(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let function = match syn::parse(input) {
        Ok(Item::Fn(function)) => function,
        _ => {
            return syn::Error::new(
                Span::call_site().into(),
                "`llvm_mca` can only be applied to functions",
            )
            .to_compile_error()
            .into()
        }
    };

    // Take the original block and wedge it between the two markers. By default,
    // `rustc` assumes that an `asm!(..)` block requires a stack frame so it
    // includes stack-frame setup/teardown in the function prologue/epilogue. We
    // can avoid this by adding the `nostack` attribute to the `asm!(..)` block.
    //
    // Similarly, `rustc` assumes that an `asm!(..)` block can read/write memory
    // and affect processor flags, so we specify the `nomem` and `preserve_flags`
    // attributes too.
    let original_block = function.block;
    let block = syn::parse(
        quote! {{
            unsafe {
                std::arch::asm!(
                    ";# LLVM-MCA-BEGIN",
                    options(nostack, nomem, preserves_flags)
                );
            }
            let ret = #original_block;
            unsafe {
                std::arch::asm!(
                    ";# LLVM-MCA-END",
                    options(nostack, nomem, preserves_flags)
                );
            }
            ret
        }}
        .into(),
    )
    .unwrap();

    let args = match syn::parse::<MacroArgs>(attrs) {
        Ok(args) => args,
        Err(err) => return err.to_compile_error().into(),
    };

    // Add `#[inline(never)]` to the function attributes if `allow_inline` is
    // _not_ specified
    let attrs = if args.allow_inline {
        function.attrs
    } else {
        function
            .attrs
            .into_iter()
            .chain(std::iter::once(parse_quote! {
                #[inline(never)]
            }))
            .collect()
    };

    let result = ItemFn {
        attrs,
        block,
        ..function
    };

    result.into_token_stream().into()
}
