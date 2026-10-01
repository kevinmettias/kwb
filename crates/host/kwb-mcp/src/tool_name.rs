//! The name an agent calls a tool by.

/// The name of a tool on [`TOOLS`] — what an agent types to invoke one.
///
/// # Why this is a type and not the `&str` it borrows
///
/// [`Answer_Tool_Call`] took `tool` and `argument` as two adjacent `&str`s, so a caller could hand them
/// over in the wrong order and the compiler would accept it — asking each tool a question
/// named *search* and calling the result the answer to *unicorn*. The two are not
/// interchangeable: one selects from a closed set this workspace declares and the other is
/// whatever the caller wants to know.
///
/// It borrows rather than owning, because a name is either a `&'static str` in [`TOOLS`] or a
/// slice of the command line that outlives the call — and a tool call happens once per
/// question, so owning one would allocate to say what the caller already holds.
///
/// [`TOOLS`]: crate::TOOLS
/// [`Answer_Tool_Call`]: crate::Answer_Tool_Call
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToolName<'name>(&'name str);

impl<'name> ToolName<'name>
{
    /// A name as a caller gives it.
    ///
    /// Nothing is checked here. Whether a name is one a tool declares is [`Answer_Tool_Call`]'s
    /// question, and it answers `None` rather than this type refusing to exist — an unknown
    /// name is a wrong call, not a wrong value.
    ///
    /// [`Answer_Tool_Call`]: crate::Answer_Tool_Call
    #[must_use]
    pub const fn Named(name: &'name str) -> Self
    {
        return Self(name);
    }

    /// The name.
    #[must_use]
    pub fn Text(&self) -> &'name str
    {
        return self.0;
    }
}
