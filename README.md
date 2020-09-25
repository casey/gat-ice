# gat-ice

This repo contains three binaries, which generate ICEs when built with rustc 1.48.0-nightly (e599b53e6 2020-09-24), and with a rustc built from [PR #73905](https://github.com/rust-lang/rust/pull/73905).

I thought that the examples might be generating the same ICE, but `gat-bound.rs` and `issue-71176.rs` actually changes error ICEs between nightly rustc, and PR #73905 rustc, whereas `issue-68648.rs` is unchanged.

See [issue-68648.rs](src/bin/issue-68648.rs), [issue-71176.rs](src/bin/issue-71176.rs), and [gat-bound.rs](src/bin/gat-bound.rs) for more details.
