#![feature(generic_associated_types)]
#![allow(incomplete_features)]

/// The generated ICE changes between:
///
///     rustc 1.48.0-nightly (e599b53e6 2020-09-24)k
///
/// ```sh
/// : cargo +nightly run --bin issue-71176
///    Compiling gat v0.1.0 (/Users/rodarmor/src/gat-ice)
/// warning: field is never read: `inner`
///   --> src/bin/issue-71176.rs:68:5
///    |
/// 68 |     inner: Box<dyn Provider<A = B>>,
///    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
///    |
///    = note: `#[warn(dead_code)]` on by default
///
/// error: internal compiler error: compiler/rustc_codegen_ssa/src/debuginfo/type_names.rs:219:13: debuginfo: Trying to create type name for unexpected type: [type error]
///
/// thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:945:9
/// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
///
/// note: the compiler unexpectedly panicked. this is a bug.
///
/// note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
///
/// note: rustc 1.48.0-nightly (e599b53e6 2020-09-24) running on x86_64-apple-darwin
///
/// note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin
///
/// note: some of the compiler flags provided by cargo are hidden
///
/// error: aborting due to previous error; 1 warning emitted
///
/// error: could not compile `gat`
///
/// To learn more, run the command again with --verbose.
/// ```
///
/// And a stage1 rustc built from:
///
///     commit 255cb9f3bd8dd9785d0482222308680225cc0130 (matthewjasper/projection-bounds-2)
///     Author: Matthew Jasper <mjjasper1@gmail.com>
///     Date:   Mon Sep 7 10:01:45 2020 +0100
///
///     Fix tests from rebase
///
/// ```sh
/// · cargo +stage1 run --bin issue-71176
///    Compiling gat v0.1.0 (/Users/rodarmor/src/gat-ice)
/// error: internal compiler error: compiler/rustc_middle/src/ty/subst.rs:476:25: Region parameter out of range when substituting in region 'a (index=1)
///
/// thread 'rustc' panicked at 'Box<Any>', /Users/rodarmor/src/rust/compiler/rustc_errors/src/lib.rs:891:9
/// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
///
/// note: the compiler unexpectedly panicked. this is a bug.
///
/// note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
///
/// note: rustc 1.48.0-dev running on x86_64-apple-darwin
///
/// note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin
///
/// note: some of the compiler flags provided by cargo are hidden
///
/// error: aborting due to previous error
///
/// error: could not compile `gat`
///
/// To learn more, run the command again with --verbose.
/// ```

trait Provider {
    type A<'a>;
}

impl Provider for () {
    type A<'a> = ();
}

struct Holder<B> {
    inner: Box<dyn Provider<A = B>>,
}

fn main() {
    Holder {
        inner: Box::new(()),
    };
}
