#![feature(generic_associated_types)]
#![allow(incomplete_features)]

/// This code generates an ICE that is unchanged between:
///
///     rustc 1.48.0-nightly (e599b53e6 2020-09-24)
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
/// : cargo +stage1 run --bin issue-68648
///    Compiling gat v0.1.0 (/Users/rodarmor/src/gat-ice)
/// warning: Error finalizing incremental compilation session directory `/Users/rodarmor/src/gat-ice/target/debug/incremental/issue_68648-390cc8dy6oiu0/s-frig4plgwi-vjm3rt-working`: No such file or directory (os error 2)
///
/// warning: 1 warning emitted
///
/// error: internal compiler error: impl item and trait item have different parameter counts
///   --> src/bin/issue-68648.rs:35:5
///    |
/// 35 |     bug(x);
///    |     ^^^
///    |
///    = note: delayed at compiler/rustc_trait_selection/src/traits/project.rs:1474:23
///
/// error: internal compiler error: impl item and trait item have different parameter counts
///   |
///   = note: delayed at compiler/rustc_trait_selection/src/traits/project.rs:1474:23
///
/// error: internal compiler error: TyKind::Error constructed but no error reported
///   |
///   = note: delayed at /Users/rodarmor/src/rust/compiler/rustc_middle/src/ty/relate.rs:338:59
///
/// thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:961:13
/// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
///
/// error: internal compiler error: unexpected panic
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
/// error: could not compile `gat`
///
/// To learn more, run the command again with --verbose.
/// ```

trait Fun {
    type F<'a>;

    fn identity<'a>(t: Self::F<'a>) -> Self::F<'a> {
        t
    }
}

impl<T> Fun for T {
    type F<'a> = Self;
}

fn bug<'a, T: Fun<F = T>>(t: T) -> T::F<'a> {
    T::identity(t)
}

fn main() {
    let x = 10;

    bug(x);
}
