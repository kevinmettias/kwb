//! The standard-library implementation of [`kwb_platform`]'s port traits.
//!
//! [`DirectoryContentStore`] implements [`kwb_platform::ContentStoreStrategy`] over a
//! directory, one file per address, writing through a temporary name and a rename so that a
//! reader never sees a partial value under a complete address.

#![forbid(unsafe_code)]

mod directory_content_store;

pub use directory_content_store::DirectoryContentStore;
