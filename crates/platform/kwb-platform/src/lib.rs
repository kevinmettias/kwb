//! Platform port traits, mirroring `nomos-platform` in `f:/repos/nomos`.
//!
//! `kwb-platform-xvpe` is deliberately absent, for the same reason
//! `nomos-platform-xvpe` is: XVPE's foundations tier does not currently compile
//! (`xvpe-dataflow` fails at 62 errors as of the last check from the Nomos side), and a
//! `path` dependency on a workspace mid-refactor would make this repository's
//! buildability a function of another product's refactor. When XVPE stabilizes, a
//! second implementation of these traits is a new crate behind an existing seam, never
//! a change to the traits themselves.
//!
//! Nothing is implemented yet.

#![forbid(unsafe_code)]
