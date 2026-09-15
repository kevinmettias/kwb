//! The store, and its one door in.

use std::collections::BTreeMap;

use kwb_model::ContentIdentity;
use kwb_platform::ContentStoreStrategy;

use crate::Admission;
use crate::Document;
use crate::StoreError;
use crate::Written;

/// A content-addressed document store.
///
/// # One write path
///
/// [`Write`] is the only method on this type that takes `&mut self`, and
/// `tests/one_door.rs` asserts that mechanically against this crate's own source rather
/// than trusting the claim. That is the whole discipline: every byte that enters the store
/// enters through one function, so every invariant the store has is checked in one place
/// and a future `Insert`, `Upsert` or `Replace` cannot quietly appear beside it.
///
/// The prototype is the argument. Its `ReplaceAllAsync` deleted unconditionally and wrote
/// conditionally; its scheduler marked unhandled work `Succeeded`; seven of its stages
/// destroyed data on a belief nobody had checked. None of those are one function doing the
/// wrong thing. They are writes that reached storage by routes the code that cared about
/// the invariants never saw.
///
/// # A write never destroys
///
/// An address is derived from content, so an address is only ever offered the content it
/// names. There is therefore no overwrite to perform: a document already held is reported
/// [`Admission::AlreadyPresent`] and the bytes the store holds are the bytes that were
/// already there. `D17` says destruction requires evidence; this store has no destruction
/// to authorise, which is the strongest form of that rule available to it and the reason
/// there is no delete path here to review.
///
/// # What this store does not claim
///
/// **Nothing about durability, atomicity, temporal reads, or ranked retrieval.** This
/// holds documents in memory for the length of a process. `KWB-10` is the open item that
/// asks which storage semantics the prototype proved are actually required, separating
/// those from what happened to be Postgres, and this type must not pre-answer it by
/// growing a file path or a connection string before that record exists.
///
/// [`Write`]: Self::Write
#[derive(Default)]
pub struct DocumentStore
{
    documents: BTreeMap<ContentIdentity, Document>,

    /// Where writes are also kept, when the store was given somewhere.
    ///
    /// `Option` rather than a null implementation: a store with no durable backing is a real
    /// configuration — every test in this workspace is one — and making that state a variant of
    /// the type says so, where a do-nothing strategy would have made *lost on exit* look
    /// identical to *written to disk* at every call site.
    durable: Option<Box<dyn ContentStoreStrategy>>,
}

impl DocumentStore
{
    /// A store holding nothing, keeping nothing beyond this process.
    #[must_use]
    pub fn Empty() -> Self
    {
        return Self::default();
    }

    /// A store that also writes through to somewhere durable.
    ///
    /// This is a **constructor, not a second door**. Durability is reached through
    /// [`Write`][Self::Write] like everything else, which is what keeps the one-write-path
    /// property true rather than true-except-for-persistence.
    #[must_use]
    pub fn Backed_By(durable: Box<dyn ContentStoreStrategy>) -> Self
    {
        return Self {
            documents: BTreeMap::new(),
            durable: Some(durable),
        };
    }

    /// Whether this store keeps anything beyond the process.
    #[must_use]
    pub const fn Is_Durable(&self) -> bool
    {
        return self.durable.is_some();
    }

    /// Write a document. This is the only way anything enters the store.
    ///
    /// The returned [`Written`] carries the address the content has and whether this call
    /// is what put it there. Offering a document the store already holds is idempotent and
    /// says so, rather than reporting a bare success a caller would have to count as new.
    ///
    /// # Errors
    ///
    /// [`StoreError::Vacuous`] if the document carries no bytes, and
    /// [`StoreError::Collision`] if two different byte strings derive one address.
    pub fn Write(&mut self, document: Document) -> Result<Written, StoreError>
    {
        if document.Is_Empty()
        {
            return Err(StoreError::Vacuous);
        }

        let identity = document.Identity();
        let admission = self.Admission_Of(&document, identity)?;
        let written = Written::For(&document, admission);

        if written.Was_Stored()
        {
            self.Persist(identity, &document)?;
        }

        // Held either way, because memory is a cache of what the *store* holds and not a
        // record of what this process wrote. A document already on the medium is one this
        // store holds, so `Read` must answer for it -- and before the admission consulted the
        // medium, that happened by accident, because every re-admission looked new.
        self.documents.insert(identity, document);

        return Ok(written);
    }

    /// What a document is to this store, given what it already holds under that address.
    ///
    /// The comparison, not the lookup, is what decides the outcome: a store that assumed its
    /// addressing was sound could not tell anybody when it was not. The collision arm is
    /// unreachable without a SHA-256 preimage break, and it is written anyway for the reason
    /// kwb-model writes its own unreachable arm. A total match costs nothing, and an argument
    /// that a branch cannot be taken costs a reader something every time they check it.
    fn Admission_Of(
        &self,
        document: &Document,
        identity: ContentIdentity,
    ) -> Result<Admission, StoreError>
    {
        return match self.documents.get(&identity)
        {
            Some(held) if held.Content() == document.Content() => Ok(Admission::AlreadyPresent),
            Some(_) => Err(StoreError::Collision { document: identity }),
            None => self.Admission_Of_Unheld(identity),
        };
    }

    /// What a document this store does not hold in memory is, which is not always new.
    ///
    /// # The defect this exists to stop
    ///
    /// Memory is empty at the start of every process and the medium is not. Deciding from
    /// memory alone therefore made a backed store report [`Admission::Stored`] for bytes that
    /// were already on disk — a success value produced on a path that did no work, which is
    /// `D19`, in the crate whose one job is not lying about writes.
    ///
    /// `D-008` recorded content-addressed idempotence as **stronger** than an idempotency key,
    /// because a key can be forgotten and content cannot. That is only true if the question is
    /// asked of everything the store holds rather than of the half that happens to be in this
    /// process.
    ///
    /// # Why the bytes are not read back to compare
    ///
    /// The address is the digest, and the write path guarantees a complete value under a
    /// complete address — `D-014`'s atomicity requirement, discharged by a temporary name and a
    /// rename. Re-reading here would be paying on every write for a guarantee the write already
    /// gives, and it would not be a check on the medium so much as a second opinion about the
    /// same digest.
    ///
    /// A medium that cannot answer refuses the write. Guessing would put the lie back with an
    /// extra step in front of it.
    fn Admission_Of_Unheld(&self, identity: ContentIdentity) -> Result<Admission, StoreError>
    {
        let Some(durable) = self.durable.as_ref()
        else
        {
            return Ok(Admission::Stored);
        };

        let held = durable
            .Holds(&identity.Render())
            .map_err(|cause| return StoreError::NotStored { cause })?;

        return Ok(if held { Admission::AlreadyPresent } else { Admission::Stored });
    }

    /// Write the document through to the medium, when the store was given one.
    ///
    /// The medium first. A document recorded in memory and not on the medium is a document that
    /// exists until the process ends, and `D19`'s lesson is that the report must not outrun the
    /// work -- so a failed durable write refuses the whole call rather than succeeding into
    /// memory alone.
    fn Persist(&self, identity: ContentIdentity, document: &Document) -> Result<(), StoreError>
    {
        let Some(durable) = self.durable.as_ref()
        else
        {
            return Ok(());
        };

        return durable
            .Put(&identity.Render(), document.Content())
            .map_err(|cause| return StoreError::NotStored { cause });
    }

    /// The document at an address.
    ///
    /// # Errors
    ///
    /// [`StoreError::NoSuchDocument`] if nothing here was ever written under it. Absent is
    /// an error rather than an empty document, because `D19-B` is what happens when a read
    /// answers for a world the caller did not ask about.
    pub fn Read(&self, identity: ContentIdentity) -> Result<&Document, StoreError>
    {
        return self
            .documents
            .get(&identity)
            .ok_or(StoreError::NoSuchDocument { document: identity });
    }

    /// Whether the store holds a document at this address.
    #[must_use]
    pub fn Holds(&self, identity: ContentIdentity) -> bool
    {
        return self.documents.contains_key(&identity);
    }

    /// How many documents the store holds.
    #[must_use]
    pub fn Length(&self) -> usize
    {
        return self.documents.len();
    }

    /// Whether the store holds nothing.
    #[must_use]
    pub fn Is_Empty(&self) -> bool
    {
        return self.documents.is_empty();
    }

    /// Every address the store holds, in a deterministic order.
    ///
    /// Ordered because the map is ordered, and that is deliberate: a listing that changed
    /// between runs would make anything derived from it, such as a manifest or a digest of
    /// the whole store, change for no reason anybody could account for.
    pub fn Identities(&self) -> impl Iterator<Item = ContentIdentity> + '_
    {
        return self.documents.keys().copied();
    }
}
