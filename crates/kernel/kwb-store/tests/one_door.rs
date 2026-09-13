//! The properties the write door exists to have, exercised from outside the crate.
//!
//! From outside deliberately. A door is only one door if it is the only one a caller can
//! reach, and a test living inside the crate can reach what a caller cannot.

use kwb_store::Admission;
use kwb_store::Document;
use kwb_store::DocumentStore;
use kwb_store::StoreError;

fn Passage(text: &str) -> Document
{
    return Document::Of(text.as_bytes().to_vec());
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

    for source in Crate_Sources()
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

/// Every `.rs` file in this crate's `src`, read as text.
fn Crate_Sources() -> Vec<String>
{
    let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let entries = std::fs::read_dir(&directory).expect("the crate has a src directory");

    let mut sources = Vec::new();
    for entry in entries
    {
        let path = entry.expect("a readable directory entry").path();
        if path.extension().is_some_and(|extension| return extension == "rs")
        {
            sources.push(std::fs::read_to_string(&path).expect("a readable source file"));
        }
    }

    assert!(!sources.is_empty(), "no sources were scanned, so this test proves nothing");
    return sources;
}

/// The name of every `pub fn` in `source` whose parameter list takes `&mut self`.
fn Mutating_Public_Methods(source: &str) -> Vec<String>
{
    let collapsed = source.split_whitespace().collect::<Vec<&str>>().join(" ");

    let mut found = Vec::new();
    for declaration in collapsed.split("pub fn ").skip(1)
    {
        let Some(signature) = declaration.split(')').next()
        else
        {
            continue;
        };

        if signature.contains("&mut self")
        {
            let name = signature.split('(').next().unwrap_or_default().trim();
            found.push(name.to_owned());
        }
    }

    return found;
}

// ---- D19: a success value cannot be produced on a path that did no work ----

#[test]
fn Test_A_Write_Should_Return_The_Address_Its_Content_Has()
{
    let document = Passage("Entropy is non-decreasing in an isolated system.");
    let identity = document.Identity();
    let mut store = DocumentStore::Empty();

    let written = store.Write(document).expect("writes");

    assert_eq!(written.Identity(), identity);
    assert_eq!(written.Admission(), Admission::Stored);
    assert!(written.Was_Stored());
    assert_eq!(written.Length(), 48);
}

#[test]
fn Test_A_Written_Document_Should_Reread_Byte_Identical()
{
    let content = b"fn main() {}\n\n  indented\t\x00\xFF".to_vec();
    let mut store = DocumentStore::Empty();

    let written = store.Write(Document::Of(content.clone())).expect("writes");

    assert_eq!(
        store.Read(written.Identity()).expect("reads").Content(),
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

    let first = store.Write(Passage("a passage")).expect("writes");
    let second = store.Write(Passage("a passage")).expect("writes again");

    assert_eq!(first.Identity(), second.Identity(), "content addressing produced two addresses");
    assert_eq!(first.Admission(), Admission::Stored);
    assert_eq!(
        second.Admission(),
        Admission::AlreadyPresent,
        "a re-offered document reported as newly stored is a count of admissions that \
         over-reports, which is the reporting half of D19"
    );
    assert!(!second.Was_Stored());
    assert_eq!(store.Length(), 1, "the second write duplicated the document");
}

#[test]
fn Test_Two_Documents_Differing_Only_By_Whitespace_Should_Both_Be_Held()
{
    let mut store = DocumentStore::Empty();

    let spaced = store.Write(Passage("a  passage")).expect("writes");
    let tight = store.Write(Passage("a passage")).expect("writes");

    assert_ne!(
        spaced.Identity(),
        tight.Identity(),
        "a document is its octets; deriving one address for both is how the second one's \
         bytes would be lost to a write that reported success"
    );
    assert_eq!(store.Length(), 2);
}

#[test]
fn Test_An_Admission_Should_Have_No_Default_To_Fall_Into()
{
    // `PipelineOutcome` read `Clean` on a dead model because `Clean` was the enum's zero
    // value and the field was settable. There is no expression this test could write that
    // produces an `Admission` without a write having happened, and that is the assertion:
    // the line below is the only shape available, and it goes through the door.
    let mut store = DocumentStore::Empty();

    let admission = store.Write(Passage("a passage")).expect("writes").Admission();

    assert_eq!(admission, Admission::Stored);
}

// ---- D17: a write has no destruction to authorise ----

#[test]
fn Test_A_Held_Document_Should_Not_Change_When_Another_Is_Written()
{
    let mut store = DocumentStore::Empty();
    let held = store.Write(Passage("the first passage")).expect("writes");
    let before = store.Read(held.Identity()).expect("reads").clone();

    assert!(store.Write(Passage("an unrelated passage")).expect("writes").Was_Stored());

    assert_eq!(&before, store.Read(held.Identity()).expect("reads"));
    assert_eq!(store.Length(), 2);
}

#[test]
fn Test_Every_Address_The_Store_Reports_Should_Read_Back()
{
    let mut store = DocumentStore::Empty();
    for passage in ["one", "two", "three"]
    {
        assert!(store.Write(Passage(passage)).expect("writes").Was_Stored());
    }

    let identities: Vec<_> = store.Identities().collect();

    assert_eq!(identities.len(), 3);
    for identity in identities
    {
        assert!(store.Holds(identity));
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
        assert!(forwards.Write(Passage(passage)).expect("writes").Was_Stored());
    }
    for passage in ["three", "two", "one"]
    {
        assert!(backwards.Write(Passage(passage)).expect("writes").Was_Stored());
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
    let never_written = Passage("never written").Identity();

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
    let identity = Passage("anything").Identity();

    let refusal = StoreError::Collision { document: identity };

    assert!(format!("{refusal}").contains(&identity.Render()));
}

// ---- D-014: a write that reaches memory and not the medium is not a write ----

#[test]
fn Test_A_Backed_Store_Should_Keep_What_It_Wrote_Beyond_Its_Own_Lifetime()
{
    let root = std::env::temp_dir().join("kwb-store-test-survives");
    let _ = std::fs::remove_dir_all(&root);
    let content = b"a passage worth keeping".to_vec();

    let identity = {
        let backing = kwb_platform_std::DirectoryContentStore::Under(&root).expect("creates");
        let mut store = DocumentStore::Backed_By(Box::new(backing));
        assert!(store.Is_Durable());
        store.Write(Document::Of(content.clone())).expect("writes").Identity()
    };

    // The store is gone. The bytes are not.
    let survivor = kwb_platform_std::DirectoryContentStore::Under(&root).expect("reopens");
    assert_eq!(
        kwb_platform::ContentStoreStrategy::Get(&survivor, &identity.Render()).expect("reads"),
        content,
        "the document did not survive the store that wrote it"
    );
}

#[test]
fn Test_An_Unbacked_Store_Should_Still_Work_And_Say_It_Keeps_Nothing()
{
    // A test suite that needed a filesystem to test a type would be testing the filesystem.
    let mut store = DocumentStore::Empty();

    let written = store.Write(Passage("a passage")).expect("writes");

    assert!(!store.Is_Durable(), "an unbacked store must not claim to keep anything");
    assert_eq!(store.Read(written.Identity()).expect("reads").Content(), b"a passage");
}

#[test]
fn Test_A_Durable_Write_That_Cannot_Complete_Should_Refuse_The_Whole_Call()
{
    // D19: the report must not outrun the work. A document in memory and not on the medium
    // exists until the process ends, and a caller told nothing would not know which of its
    // documents were real.
    let root = std::env::temp_dir().join("kwb-store-test-refuses");
    let _ = std::fs::remove_dir_all(&root);
    let backing = kwb_platform_std::DirectoryContentStore::Under(&root).expect("creates");
    let document = Passage("a passage");
    std::fs::create_dir_all(root.join(document.Identity().Render())).expect("obstructs");

    let mut store = DocumentStore::Backed_By(Box::new(backing));
    let refusal = store.Write(document).expect_err("must refuse");

    assert!(matches!(refusal, StoreError::NotStored { .. }), "{refusal}");
    assert!(
        store.Is_Empty(),
        "the document was kept in memory after the durable write failed, so the store now \
         holds something the next process will not"
    );
}
