# gat-ice

This repo contains two binaries `src/bin/issue-68648.rs` (from [#68648](https://github.com/rust-lang/rust/issues/68648)) and `src/bin/gat-bound.rs`, which generate ICEs when built with rustc 1.48.0-nightly (e599b53e6 2020-09-24), and with a rustc built from [PR #73905](https://github.com/rust-lang/rust/pull/73905).

I thought that the two examples might be generating the same ICE, but gat-bound
actually changes error messages between nightly rustc, and PR #73905 rustc.

See [issue-68648.rs](src/bin/issue-68648.rs) and [gat-bound.rs](src/bin/gat-bound.rs) for more details.

