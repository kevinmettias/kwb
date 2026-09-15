//! The properties the write door exists to have, exercised from outside the crate.
//!
//! From outside deliberately. A door is only one door if it is the only one a caller can
//! reach, and a test living inside the crate can reach what a caller cannot.

use kwb_source_guards::Crate_Sources;
use kwb_source_guards::Mutating_Public_Methods;
use kwb_store::Admission;
use kwb_store::Document;
use kwb_store::DocumentStore;
use kwb_store::StoreError;
use kwb_store::Written;

fn Document_From_Text(text: &str) -> Document
{
    return Document::Of(text.as_bytes().to_vec());
}

/// Clears a directory this test owns, and fails the test on anything but its absence.
///
/// Each root below is one property's own name under the temp directory, and a run begins by
/// clearing it. Not finding it is therefore the ordinary first run rather than a fault. Any
/// other failure leaves the store pointed at a tree this test cannot reason about, which is
/// worth stopping on rather than writing past.
fn Clear_Root(root: &std::path::Path)
{
    if let Err(error) = std::fs::remove_dir_all(root)
    {
        assert_eq!(
            error.kind(),
            std::io::ErrorKind::NotFound,
            "the test root could not be cleared: {error}"
        );
    }
}

// ---- the property KWB-2 exists for ----

/// The claim is structural, so the test is structural: it reads this crate's own source
/// and counts the public methods that take `&mut self`.
///
/// # Why this is not a comment
///
/// "One write door" is a property of the whole crate at every future revision, not of the
/// function that happens to be written today. A doc comment saying so is checked by
/// whoever reads it; this is checked by whoever runs the tests. The prototype's own
/// `ReplaceAllAsync` was a second write path that arrived as a private helper and stayed,
/// and nothing in that repository was in a position to notice.
///
/// The scan collapses whitespace first, so a signature broken across lines cannot slip
/// past it, and it looks at the parameter list rather than the line, so a doc comment
/// mentioning `&mut self` is not a finding.
#[test]
fn Test_The_Crate_Should_Expose_Exactly_One_Mutating_Method()
{
    let mut doors: Vec<String> = Vec::new();

    for source in Crate_Sources(env!("CARGO_MANIFEST_DIR"))
    {
        for name in Mutating_Public_Methods(&source)
        {
            doors.push(name);
        }
    }

    assert_eq!(
        doors,
        vec!["Write".to_owned()],
        "kwb-store must expose exactly one method that can change it, and it must be \
         Write. Anything else here is a second route into the store, which is the shape \
         of every incident D17 through D21 records"
    );
}

// ---- D19: a success value cannot be produced on a path that did no work ----

#[test]
fn Test_A_Write_Should_Return_The_Address_Its_Content_Has()
{
    let text = "Entropy is non-decreasing in an isolated system.";
    let document = Document_From_Text(text);
    let identity = document.Identity();
    let mut store = DocumentStore::Empty();

    let written = store.Write(document).expect("the document carries bytes, which is all an unbacked store requires");

    assert_eq!(written.Identity(), identity);
    assert_eq!(written.Admission(), Admission::Stored);
    assert!(written.Has_Stored());
    assert_eq!(written.Length(), text.len());
}

#[test]
fn Test_A_Written_Document_Should_Reread_Byte_Identical()
{
    let content = b"fn main() {}\n\n  indented\t\x00\xFF".to_vec();
    let mut store = DocumentStore::Empty();

    let written = store.Write(Document::Of(content.clone())).expect("the document carries bytes, which is all an unbacked store requires");

    assert_eq!(
        store.Read(written.Identity()).expect("the identity came from a write this store accepted").Content(),
        content,
        "the store holds something other than the octets it was given"
    );
}

#[test]
fn Test_A_Write_Offering_No_Bytes_Should_Be_Refused()
{
    let mut store = DocumentStore::Empty();

    let refusal = store.Write(Document::Of(Vec::new())).expect_err("must refuse");

    assert_eq!(refusal, StoreError::Vacuous);
    assert!(store.Is_Empty(), "the refused write stored something anyway");
    assert!(
        format!("{refusal}").contains("empty-but-valid reply"),
        "the refusal should carry the incident that justifies it: {refusal}"
    );
}

// ---- D20: an outcome is derived from the evidence, never assigned ----

#[test]
fn Test_Writing_The_Same_Document_Twice_Should_Say_So_Rather_Than_Report_Two_Admissions()
{
    let mut store = DocumentStore::Empty();

    let first = store.Write(Document_From_Text("a passage")).expect("the document carries bytes, which is all an unbacked store requires");
    let second = store.Write(Document_From_Text("a passage")).expect("a re-offered document is answered, never refused");

    assert_eq!(first.Identity(), second.Identity(), "content addressing produced two addresses");
    assert_eq!(first.Admission(), Admission::Stored);
    assert_eq!(
        second.Admission(),
        Admission::AlreadyPresent,
        "a re-offered document reported as newly stored is a count of admissions that \
         over-reports, which is the reporting half of D19"
    );
    assert!(!second.Has_Stored());
    assert_eq!(store.Length(), 1, "the second write duplicated the document");
}

#[test]
fn Test_Two_Documents_Differing_Only_By_Whitespace_Should_Both_Be_Held()
{
    let mut store = DocumentStore::Empty();

    let spaced = store.Write(Document_From_Text("a  passage")).expect("the document carries bytes, which is all an unbacked store requires");
    let tight = store.Write(Document_From_Text("a passage")).expect("the document carries bytes, which is all an unbacked store requires");

    assert_ne!(
        spaced.Identity(),
        tight.Identity(),
        "a document is its octets; deriving one address for both is how the second one's \
         bytes would be lost to a write that reported success"
    );
    assert_eq!(store.Length(), [spaced, tight].len());
}

#[test]
fn Test_An_Admission_Should_Have_No_Default_To_Fall_Into()
{
    // `PipelineOutcome` read `Clean` on a dead model because `Clean` was the enum's zero
    // value and the field was settable. There is no expression this test could write that
    // produces an `Admission` without a write having happened, and that is the assertion:
    // the line below is the only shape available, and it goes through the door.
    let mut store = DocumentStore::Empty();

    let admission = store.Write(Document_From_Text("a passage")).expect("the document carries bytes, which is all an unbacked store requires").Admission();

    assert_eq!(admission, Admission::Stored);
}

// ---- D17: a write has no destruction to authorise ----

#[test]
fn Test_A_Held_Document_Should_Not_Change_When_Another_Is_Written()
{
    let mut store = DocumentStore::Empty();
    let held = store.Write(Document_From_Text("the first passage")).expect("the document carries bytes, which is all an unbacked store requires");
    let before = store.Read(held.Identity()).expect("the identity came from a write this store accepted").clone();

    let unrelated = store.Write(Document_From_Text("an unrelated passage")).expect("the document carries bytes, which is all an unbacked store requires");
    assert!(unrelated.Has_Stored());

    assert_eq!(&before, store.Read(held.Identity()).expect("the identity came from a write this store accepted"));
    assert_eq!(store.Length(), [held, unrelated].len());
}

#[test]
fn Test_Every_Address_The_Store_Reports_Should_Read_Back()
{
    let mut store = DocumentStore::Empty();
    let passages = ["one", "two", "three"];
    for passage in passages
    {
        assert!(store.Write(Document_From_Text(passage)).expect("the document carries bytes, which is all an unbacked store requires").Has_Stored());
    }

    let identities: Vec<_> = store.Identities().collect();

    assert_eq!(identities.len(), passages.len());
    for identity in identities
    {
        assert!(store.Has_Document(identity));
        store.Read(identity).expect("an address the store reports must read back");
    }
}

#[test]
fn Test_The_Listing_Should_Not_Depend_On_The_Order_Documents_Arrived_In()
{
    let mut forwards = DocumentStore::Empty();
    let mut backwards = DocumentStore::Empty();
    for passage in ["one", "two", "three"]
    {
        assert!(forwards.Write(Document_From_Text(passage)).expect("the document carries bytes, which is all an unbacked store requires").Has_Stored());
    }
    for passage in ["three", "two", "one"]
    {
        assert!(backwards.Write(Document_From_Text(passage)).expect("the document carries bytes, which is all an unbacked store requires").Has_Stored());
    }

    assert_eq!(
        forwards.Identities().collect::<Vec<_>>(),
        backwards.Identities().collect::<Vec<_>>(),
        "anything derived from the listing would change for no reason anybody could account for"
    );
}

// ---- D19-B: absent is not empty ----

#[test]
fn Test_Reading_An_Address_Nothing_Wrote_Should_Be_Refused_Rather_Than_Empty()
{
    let store = DocumentStore::Empty();
    let never_written = Document_From_Text("never written").Identity();

    let refusal = store.Read(never_written).expect_err("must refuse");

    assert_eq!(refusal, StoreError::NoSuchDocument { document: never_written });
    assert!(format!("{refusal}").contains("Absent is not empty"), "{refusal}");
}

// ---- the guard that cannot be reached, and is written anyway ----

/// [`StoreError::Collision`] has no test, and this is the note that says so on purpose.
///
/// Reaching it requires two byte strings whose full SHA-256 derivations agree, which is a
/// preimage break rather than a case a test can construct. What *is* tested is the
/// comparison that guards it: every `AlreadyPresent` above runs it, so the branch is live
/// code on the path this crate actually takes, and only the refusal arm is unreachable.
///
/// It is recorded here rather than deleted because a store that assumed its addressing was
/// sound could not tell anybody when it was not, and because an untested branch that
/// nobody has written down is indistinguishable from one that was forgotten.
#[test]
fn Test_The_Collision_Refusal_Should_Render_Its_Address()
{
    let identity = Document_From_Text("anything").Identity();

    let refusal = StoreError::Collision { document: identity };

    assert!(format!("{refusal}").contains(&identity.Render()));
}

// ---- D-014: a write that reaches memory and not the medium is not a write ----

#[test]
fn Test_A_Backed_Store_Should_Keep_What_It_Wrote_Beyond_Its_Own_Lifetime()
{
    let root = std::env::temp_dir().join("kwb-store-test-survives");
    Clear_Root(&root);
    let content = b"a passage worth keeping".to_vec();

    let identity = {
        let backing = kwb_platform_std::DirectoryContentStore::Under(&root).expect("Under creates the root");
        let mut store = DocumentStore::Backed_By(Box::new(backing));
        assert!(store.Is_Durable());
        store.Write(Document::Of(content.clone())).expect("the root exists and the document is not empty").Identity()
    };

    // The store is gone. The bytes are not.
    let survivor = kwb_platform_std::DirectoryContentStore::Under(&root).expect("the root exists from above");
    assert_eq!(
        kwb_platform::ContentStoreStrategy::Get(&survivor, &identity.Render()).expect("the identity came from a write this store accepted"),
        content,
        "the document did not survive the store that wrote it"
    );
}

#[test]
fn Test_An_Unbacked_Store_Should_Still_Work_And_Say_It_Keeps_Nothing()
{
    // A test suite that needed a filesystem to test a type would be testing the filesystem.
    let mut store = DocumentStore::Empty();

    let written = store.Write(Document_From_Text("a passage")).expect("the document carries bytes, which is all an unbacked store requires");

    assert!(!store.Is_Durable(), "an unbacked store must not claim to keep anything");
    assert_eq!(store.Read(written.Identity()).expect("the identity came from a write this store accepted").Content(), b"a passage");
}

#[test]
fn Test_A_Durable_Write_That_Cannot_Complete_Should_Refuse_The_Whole_Call()
{
    // D19: the report must not outrun the work. A document in memory and not on the medium
    // exists until the process ends, and a caller told nothing would not know which of its
    // documents were real.
    let root = std::env::temp_dir().join("kwb-store-test-refuses");
    Clear_Root(&root);
    let backing = kwb_platform_std::DirectoryContentStore::Under(&root).expect("Under creates the root");
    let document = Document_From_Text("a passage");
    std::fs::create_dir_all(root.join(document.Identity().Render())).expect("the root exists to create it under");

    let mut store = DocumentStore::Backed_By(Box::new(backing));
    let refusal = store.Write(document).expect_err("must refuse");

    assert!(matches!(refusal, StoreError::NotStored { .. }), "{refusal}");
    assert!(
        store.Is_Empty(),
        "the document was kept in memory after the durable write failed, so the store now \
         holds something the next process will not"
    );
}

// ---- KWB-35: an admission is decided from what the store holds, not from this process ----

/// One durable write through a store that is dropped on the way out.
///
/// The drop is the point: a second call over the same root shares no memory with the first,
/// which is what makes it a new process as far as the store can tell.
fn Write_And_Drop_The_Store(root: &std::path::Path, content: &[u8]) -> Written
{
    let backing = kwb_platform_std::DirectoryContentStore::Under(root).expect("opens the root");
    let mut store = DocumentStore::Backed_By(Box::new(backing));
    return store.Write(Document::Of(content.to_vec())).expect("the root exists and the document is not empty");
}

#[test]
fn Test_Re_Admitting_A_Durably_Held_Document_Should_Report_It_As_Already_Present()
{
    // The defect this replaced: memory is empty at the start of every process and the medium
    // is not, so deciding from memory alone made every process report storing what it already
    // had. That is a success value produced on a path that did no work, in the crate whose one
    // job is not lying about writes.
    let root = std::env::temp_dir().join("kwb-store-test-across-processes");
    Clear_Root(&root);
    let content = b"a passage".to_vec();

    let first = Write_And_Drop_The_Store(&root, &content);
    assert!(first.Has_Stored(), "the first write is what put the bytes there");

    // A second store over the same directory: a new process, as far as memory is concerned.
    let second = Write_And_Drop_The_Store(&root, &content);

    assert!(
        !second.Has_Stored(),
        "the bytes were already on the medium, so this write is not what put them there"
    );
    assert_eq!(second.Admission(), Admission::AlreadyPresent);
    assert_eq!(first.Identity(), second.Identity());
}

#[test]
fn Test_A_Document_Held_Only_On_The_Medium_Should_Still_Read_Back()
{
    // The regression the fix above could have introduced: an AlreadyPresent document that
    // never entered memory would be one the store holds and cannot answer for.
    let root = std::env::temp_dir().join("kwb-store-test-reads-across");
    Clear_Root(&root);
    let content = b"a passage worth reading".to_vec();

    let identity = {
        let backing = kwb_platform_std::DirectoryContentStore::Under(&root).expect("Under creates the root");
        let mut store = DocumentStore::Backed_By(Box::new(backing));
        store.Write(Document::Of(content.clone())).expect("the root exists and the document is not empty").Identity()
    };

    let backing = kwb_platform_std::DirectoryContentStore::Under(&root).expect("the root exists from above");
    let mut store = DocumentStore::Backed_By(Box::new(backing));
    let written = store.Write(Document::Of(content.clone())).expect("the root exists and the document is not empty");

    assert!(!written.Has_Stored());
    assert_eq!(
        store.Read(identity).expect("a document the store holds must read back").Content(),
        content
    );
}

#[test]
fn Test_An_Unbacked_Store_Should_Decide_Its_Admissions_Exactly_As_Before()
{
    let mut store = DocumentStore::Empty();

    let first = store.Write(Document_From_Text("a passage")).expect("the document carries bytes, which is all an unbacked store requires");
    let second = store.Write(Document_From_Text("a passage")).expect("a re-offered document is answered, never refused");

    assert!(first.Has_Stored());
    assert_eq!(second.Admission(), Admission::AlreadyPresent);
    assert_eq!(store.Length(), 1);
}

// ---- KWB-44: the detector is shown to detect ----

/// A detector that always returned nothing would pass every run against the real tree.
///
/// The guard above only ever scans this crate's own source, which has one mutating method and
/// is expected to. So a passing guard is consistent with two very different worlds: a crate
/// with one write door, or a parser that has stopped parsing. These cases separate them.
///
/// Synthetic input rather than a mutation of the tree. A test that edits source to prove a
/// point can leave the tree broken when it fails partway, and this repository would rather not
/// introduce that failure mode to guard against another one.
#[test]
fn Test_The_Mutating_Method_Detector_Should_Detect()
{
    assert_eq!(Mutating_Public_Methods("pub fn Write(&mut self) -> bool { }"), ["Write"]);
    assert_eq!(
        Mutating_Public_Methods("pub fn Insert(\n    &mut self,\n    value: usize,\n) { }"),
        ["Insert"],
        "a signature broken across lines must not slip past; whitespace is collapsed first"
    );
    assert_eq!(
        Mutating_Public_Methods("pub fn A(&mut self){} pub fn B(&mut self){}"),
        ["A", "B"],
        "it must find every one, not stop at the first"
    );
}

/// Sources that must produce no mutating method: a shared reader, a private writer, a doc
/// comment that reads like a signature, a const constructor, and no declaration at all.
const NON_MUTATING_SOURCES: &[&str] = &[
    "pub fn Read(&self) -> bool { }",
    "fn Private(&mut self) { }",
    "/// A doc comment mentioning &mut self, which is not a signature.",
    "pub const fn Empty() -> Self { }",
    "",
];

#[test]
fn Test_The_Mutating_Method_Detector_Should_Not_Report_What_Is_Not_One()
{
    for source in NON_MUTATING_SOURCES
    {
        assert!(
            Mutating_Public_Methods(source).is_empty(),
            "reported a mutating method in: {source:?}"
        );
    }
}
