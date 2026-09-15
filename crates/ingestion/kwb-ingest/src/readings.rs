//! The vocabulary a reading is described in.
//!
//! A reading names who or what took it ([`ReaderName`]), what kind of reading the source needs
//! ([`ReadingKind`]), the protocol it was taken under ([`ReadingProtocol`]) and where in the
//! source it was found ([`SourceLocation`]). [`ClaimText`] is the text it read, carried
//! unnormalized so that what a reader said is still recoverable after everything downstream has
//! acted on it.
//!
//! [`ReaderName`]: reader_name::ReaderName
//! [`ReadingKind`]: reading_kind::ReadingKind
//! [`ReadingProtocol`]: reading_protocol::ReadingProtocol
//! [`SourceLocation`]: source_location::SourceLocation
//! [`ClaimText`]: claim_text::ClaimText

pub(crate) mod claim_text;
pub(crate) mod reader_name;
pub(crate) mod reading_kind;
pub(crate) mod reading_protocol;
pub(crate) mod source_location;
