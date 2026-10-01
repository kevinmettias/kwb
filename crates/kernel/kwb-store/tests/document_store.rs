//! The store's own contract, method by method.
//!
//! # Why this file sits beside `tests/one_door.rs`
//!
//! `one_door.rs` asserts the properties the store exists to have — one write door, a write that
//! cannot destroy, an admission decided from the medium rather than from this process — and it
//! reaches every method here on the way. What it does not do is name any of them. The unit this
//! check reads is the test file's own stem, so `src/document_store.rs` is covered by a file called
//! `document_store.rs` and by nothing else, and a failure in `Identities` arrived under the name of
//! a test about a door.
//!
//! Each test below is one method, asserting the contract of that method, so a break arrives under
//! the name of the thing that broke.

use std::path::Path;

use kwb_store::Document;
use kwb_store::DocumentStore;
use kwb_store::StoreError;

/// How many passages the provider below yields: the whole sample every case here is stated over.
///
/// The number belongs to the sample and not to the type, so it is named here rather than written
/// into the array type: another passage is a change to this line and to the cases, and the tests
/// that compare against `Passages().len()` are comparing against the sample rather than against a
/// count that happens to agree with it today.
const PASSAGE_COUNT: usize = 3;

/// Three passages, differing only in being three.
///
/// A provider rather than a literal in the tests that need more than one document: the cases live
/// here, the loops live there, and adding a fourth is a change in one place.
fn Passages() -> [&'static str; PASSAGE_COUNT]
{
    return ["one", "two", "three"];
}

/// Clears a directory this test owns, and fails on anything but its absence.
///
/// Not finding the root is the ordinary first run rather than a fault. Any other failure leaves the
/// store pointed at a tree this test cannot reason about, which is worth stopping on rather than
/// writing past.
fn Clear_Root(root: &Path)
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

/// A store under a directory this test owns, cleared so that what is read back belongs to this run.
fn With_A_Medium(name: &str) -> DocumentStore
{
    let root = std::env::temp_dir().join(format!("kwb-store-document-store-{name}"));
    Clear_Root(&root);

    let backing = kwb_platform_std::DirectoryContentStore::Under(&root)
        .expect("Under creates the root it was given");

    return DocumentStore::Backed_By(Box::new(backing));
}

#[test]
fn Test_Empty_Should_Start_A_Store_That_Holds_Nothing()
{
    // The configuration every test in this workspace runs in, and the one a report has to be able
    // to tell from the durable one. It must start genuinely empty rather than merely unread: a
    // count, a listing and the emptiness question all have to answer for nothing.
    let store = DocumentStore::Empty();

    assert!(store.Is_Empty(), "a store just made holds something");
    assert_eq!(store.Length(), 0, "a store just made counts documents");
    assert_eq!(store.Identities().count(), 0, "a store just made lists an address");
}

#[test]
fn Test_Backed_By_Should_Give_The_Store_A_Medium_To_Write_Through()
{
    // The constructor's whole job. `kwb-store` names `ContentStoreStrategy` and never an
    // implementation of it, so somebody has to hand one over, and a store given one must come out
    // as the durable configuration — and no fuller than the other one, because durability is where
    // the bytes go and not something already in the store.
    let store = With_A_Medium("backed-by");

    assert!(
        store.Is_Durable(),
        "a store handed a medium is not the durable configuration, so --store keeps nothing"
    );
    assert!(store.Is_Empty(), "a store handed a medium started with something in it");
}

#[test]
fn Test_Is_Durable_Should_Tell_The_Two_Ways_Of_Making_A_Store_Apart()
{
    // The field is an `Option` rather than a null implementation so that *lost on exit* and
    // *written to disk* stay distinguishable, and this is the function that distinguishes them: it
    // is what the binary reads to tell a person whether their bytes were kept. A predicate
    // answering `true` for both would make the two identical at every call site, which is the state
    // the `Option` was chosen to avoid.
    assert!(
        With_A_Medium("is-durable").Is_Durable(),
        "a store that was given a medium does not report one"
    );
    assert!(
        !DocumentStore::Empty().Is_Durable(),
        "a store told nowhere reports that it keeps its bytes somewhere"
    );
}

#[test]
fn Test_Write_Should_Refuse_A_Document_That_Carries_No_Bytes()
{
    // The one refusal the door makes before it looks at anything else. `StoreError::Vacuous`
    // rather than a stored empty document, because an admission reported for no bytes is `D19`'s
    // *empty-but-valid reply* — a success value on a path that did no work — and the store has to
    // be as empty afterwards as it was before.
    let mut store = DocumentStore::Empty();

    let refusal = store
        .Write(Document::Of(Vec::new()))
        .expect_err("a document of no octets is refused");

    assert_eq!(
        refusal,
        StoreError::Vacuous,
        "an empty document was refused for a reason other than being empty"
    );
    assert!(store.Is_Empty(), "the refused write stored something anyway");
}

#[test]
fn Test_Read_Should_Refuse_An_Address_Nothing_Was_Written_Under()
{
    // `D19-B`: absent is not empty. A read that answered an empty document would be a read
    // answering for a world the caller did not ask about, and the address is carried into the
    // refusal so the complaint names what was looked for rather than only that it was not there.
    let store = DocumentStore::Empty();
    let never_written = Document::Of(b"never written".to_vec()).Identity();

    let refusal = store
        .Read(never_written)
        .expect_err("an address nothing was written under is refused");

    assert_eq!(refusal, StoreError::NoSuchDocument { document: never_written });
}

#[test]
fn Test_Has_Document_Should_Say_No_Before_A_Write_And_Yes_After()
{
    // The question asked without asking for the document, which is what lets a caller avoid paying
    // for bytes it does not want. Both answers are asserted, because a predicate that always
    // answered `true` would satisfy the second half on its own.
    let mut store = DocumentStore::Empty();
    let document = Document::Of(b"a passage".to_vec());
    let identity = document.Identity();

    assert!(!store.Has_Document(identity), "a store just made claims to hold a document");

    assert!(
        store
            .Write(document)
            .expect("the document carries bytes, which is all an unbacked store requires")
            .Has_Stored(),
        "the store answered for a document it did not store"
    );

    assert!(store.Has_Document(identity), "the store does not hold the document it just wrote");
}

#[test]
fn Test_Length_Should_Count_Each_Distinct_Document_Once()
{
    // The count a report prints, and the one a second identical write must not move. Content
    // addressing makes the same octets one document however many times they are offered, so this
    // counts what the store holds rather than how often it was asked — and the three offered twice
    // is what tells the two apart.
    let mut store = DocumentStore::Empty();
    Store_Every_Passage_Once(&mut store);
    Re_Offer_Every_Passage(&mut store);

    assert_eq!(
        store.Length(),
        Passages().len(),
        "the store is counting writes rather than the documents it holds"
    );
}

/// Stores each of the three passages, asserting that each offer was the write that stored it.
///
/// The verdict is asserted here rather than left to the count below, because the count rests on
/// this premise: a passage the store did not store is one it is right to leave out of its length,
/// so a length of three reads the same whether the write happened or was quietly dropped.
fn Store_Every_Passage_Once(store: &mut DocumentStore)
{
    for passage in Passages()
    {
        assert!(
            store
                .Write(Document::Of(passage.as_bytes().to_vec()))
                .expect("the document carries bytes, which is all an unbacked store requires")
                .Has_Stored(),
            "the store did not store a passage it had not seen before"
        );
    }
}

/// Offers each of the three passages a second time, asserting that none was counted as newly stored.
///
/// This is the other half of the premise the count needs: content addressing makes the same octets
/// one document however many times they are offered, so a store that counted writes would answer
/// `Stored` again and report six. Offering them twice is what tells the two counts apart.
fn Re_Offer_Every_Passage(store: &mut DocumentStore)
{
    for passage in Passages()
    {
        assert!(
            !store
                .Write(Document::Of(passage.as_bytes().to_vec()))
                .expect("a re-offered document is answered, never refused")
                .Has_Stored(),
            "a passage the store already held was counted as newly stored"
        );
    }
}

#[test]
fn Test_Is_Empty_Should_Turn_Over_At_The_First_Document_The_Store_Keeps()
{
    // The complement of `Length`, and the question a caller asks before deciding whether there is
    // anything to read at all. Asserted across the transition rather than in either state, because
    // a predicate answering the same thing forever satisfies either half on its own.
    let mut store = DocumentStore::Empty();
    assert!(store.Is_Empty(), "a store just made holds something");

    assert!(
        store
            .Write(Document::Of(b"a passage".to_vec()))
            .expect("the document carries bytes, which is all an unbacked store requires")
            .Has_Stored(),
        "the store holds a document and does not say so"
    );

    assert!(!store.Is_Empty(), "the store holds a document and does not say so");
}

#[test]
fn Test_Identities_Should_List_Every_Address_The_Store_Holds_And_No_Other()
{
    // What a caller walks to enumerate the store without reading it. The listing has to be the set
    // of documents held and nothing else — an iterator that yielded nothing, or yielded twice,
    // would satisfy neither half — and every address on it has to be one the store answers for.
    let mut store = DocumentStore::Empty();

    Assert_Listing_Is_Exactly_What_Was_Written(&mut store);
    Assert_Every_Address_Listed_Is_Held(&store);
}

/// Writes every passage, collecting each address as it goes, and asserts that the store's listing
/// is exactly that set.
///
/// The addresses are collected from the documents as they are written rather than read back out of
/// the store, because comparing the listing against a second walk of the store would assert only
/// that its iterator agrees with itself — which every iterator does, including one that yields
/// nothing.
///
/// The write's own verdict is asserted alongside, because the equality below depends on it: a
/// passage the store never stored is an address the listing is right to omit, so a store that
/// dropped the write and one that kept it would pass the comparison equally.
fn Assert_Listing_Is_Exactly_What_Was_Written(store: &mut DocumentStore)
{
    let mut written = Vec::new();
    for passage in Passages()
    {
        let document = Document::Of(passage.as_bytes().to_vec());
        written.push(document.Identity());
        assert!(
            store
                .Write(document)
                .expect("the document carries bytes, which is all an unbacked store requires")
                .Has_Stored(),
            "a passage was not stored, so the listing below is not of what was written"
        );
    }
    // Sorted before comparing, because the order is the map's and not the order they arrived in;
    // that independence is `one_door.rs`'s to assert and not this test's.
    written.sort();
    let mut listed: Vec<_> = store.Identities().collect();
    listed.sort();

    assert_eq!(listed, written, "the listing is not the set of documents the store holds");
}

/// Asserts that every address the store lists is one the store answers for.
///
/// The set being the right size and its members being addresses the store can answer for are two
/// different claims: an iterator that yielded the right number of addresses belonging to something
/// else would satisfy the comparison above and fail this one, which is what `Has_Document` is for.
fn Assert_Every_Address_Listed_Is_Held(store: &DocumentStore)
{
    for identity in store.Identities()
    {
        assert!(store.Has_Document(identity), "the listing names an address the store does not hold");
    }
}
