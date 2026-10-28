//! Housekeeping: the trash and the hidden vault.
//!
//! `domain` decides what a trashed file is called, when it expires, and where a restored copy
//! goes; `infrastructure` is the only place that touches the disk or the phone's screen lock.

pub mod domain;
pub mod infrastructure;
