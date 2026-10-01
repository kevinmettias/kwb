//! Reading a crate's own source, for the guard tests built on it.
//!
//! Three crates assert a structural property of themselves by reading their own source.
//! `kwb-store` that it exposes exactly one mutating method, `kwb-retrieval` that it exposes none,
//! and `kwb-domain` that it defines the liveness rule exactly once. The *property* differs between
//! them and stays with each crate. The *reading* does not, so it lives here once.
//!
//! Written twice it was two copies a fix had to reach, which is what `check-interfile-duplication`
//! names as the hazard rather than the line count -- and that cost was paid: `KWB-108` and
//! `KWB-110` each repaired the same one-level `read_dir`, as two items, in two commits, in two
//! files. `KWB-138` removed the second copy.
//!
//! A crate reaches this through a `dev-dependency`, so nothing here is linked into a shipped
//! binary and no production crate gains an edge to it.

#![forbid(unsafe_code)]

use std::path::Path;

/// Every `.rs` file under `<manifest_directory>/src`, at any depth, with the path it sits at
/// relative to `src` and its text.
///
/// The caller passes its own `CARGO_MANIFEST_DIR` rather than this crate reading it directly,
/// because `env!` is expanded where it is written: read here, it would name *this* crate and
/// every guard built on it would scan the wrong tree and pass for the wrong reason.
///
/// The path is relative to `src`, with `/` separators, because with recursion a file's name alone
/// is no longer an identity: two subdirectories may each hold a `mod.rs`, and a finding has to say
/// which one it came from. A guard that must name the file a rule was found in needs this -- the
/// liveness guard asserts its one definition came out of `epistemic/standing.rs`. A guard that
/// only counts rules across the whole tree does not, and takes [`Crate_Sources`] below.
///
/// The result is the evidence the caller asserts against, so dropping it would discard the
/// only thing this function produces.
///
/// # Why it descends, and why the depth matters more here than elsewhere
///
/// It read one level until `KWB-110`. Two of the guards built on it assert the *absence* of a
/// mutating public method — `kwb-store` that exactly one exists and `kwb-retrieval` that none does
/// — and an absence is the polarity that fails where it cannot be seen: losing sight of part of
/// the tree makes such a guard **pass**, where a guard whose subject is a count it must find fails
/// loudly. `kwb-domain`'s liveness guard is that other polarity, and `KWB-108` is it reporting zero
/// where it required one, because the rule it looks for moved into a subdirectory.
///
/// The assert below catches total blindness and cannot catch partial blindness, which is exactly
/// what splitting a file into a subdirectory produces. `kwb-domain`, `kwb-ingest` and
/// `kwb-retrieval` already keep subdirectories under `src`, so the arrangement a one-level reader
/// assumes is one this workspace has already stopped keeping.
///
/// # Panics
///
/// Panics when the tree cannot be read, which is every way this function has of failing:
/// `<manifest_directory>/src` not existing, a directory under it not being readable, a directory
/// entry not being readable, or a source file not being readable as text. It also panics when no
/// `.rs` file was found at all, because a scan that read nothing would let a guard built on it
/// pass for the wrong reason rather than report that it never looked.
#[must_use]
pub fn Crate_Sources_With_Paths(manifest_directory: &str) -> Vec<(String, String)>
{
    let root = Path::new(manifest_directory).join("src");
    let mut sources = Vec::new();
    Collect_Sources(&root, &root, &mut sources);

    assert!(!sources.is_empty(), "no sources were scanned, so this test proves nothing");
    return sources;
}

/// The text of every `.rs` file under `<manifest_directory>/src`, at any depth.
///
/// Expressed through [`Crate_Sources_With_Paths`] rather than beside it, so the directory walk
/// exists once and a repair to it reaches every guard built on this crate.
///
/// # Panics
///
/// Panics on everything [`Crate_Sources_With_Paths`] panics on, which is the tree not being
/// readable and the scan finding nothing.
#[must_use]
pub fn Crate_Sources(manifest_directory: &str) -> Vec<String>
{
    return Crate_Sources_With_Paths(manifest_directory)
        .into_iter()
        .map(|(_, source)| return source)
        .collect();
}

/// Add every `.rs` file under `directory`, at any depth, named relative to `root`.
fn Collect_Sources(root: &Path, directory: &Path, sources: &mut Vec<(String, String)>)
{
    let entries = std::fs::read_dir(directory).expect("a readable directory under the crate's src");

    for entry in entries
    {
        let path = entry.expect("a readable directory entry").path();
        if path.is_dir()
        {
            Collect_Sources(root, &path, sources);
            continue;
        }
        if path.extension().is_some_and(|extension| return extension == "rs")
        {
            let name = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            sources.push((name, std::fs::read_to_string(&path).expect("a readable source file")));
        }
    }
}

/// The name of every `pub fn` in `source` whose parameter list takes `&mut self`.
///
/// Whitespace is collapsed first, so a signature broken across lines cannot slip past, and the
/// parameter list is what is inspected, so a doc comment mentioning `&mut self` is not a
/// finding.
///
/// The result is the evidence the caller asserts against, so dropping it would discard the
/// only thing this function produces.
#[must_use]
pub fn Mutating_Public_Methods(source: &str) -> Vec<String>
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

#[cfg(test)]
mod tests
{
    use std::path::PathBuf;

    use super::Crate_Sources_With_Paths;

    /// A directory tree built for one test and removed when that test ends, panic or not.
    ///
    /// The reader is pointed at a directory rather than reaching for a `CARGO_MANIFEST_DIR` of its
    /// own, which is what makes a fixture possible. It cannot be a tree inside this repository: the
    /// case below is a source file *below* the top level, and committing one to prove the reader
    /// descends would make the assertion about a path somebody could move.
    struct Fixture
    {
        root: PathBuf,
    }

    impl Fixture
    {
        /// A tree under the system temporary directory, named so two runs cannot collide.
        ///
        /// The removal first is for the run after a test that panicked before its own cleanup
        /// could run on a platform where `Drop` is skipped; it is not needed on the usual path.
        #[must_use]
        fn New(name: &str) -> Self
        {
            let root = std::env::temp_dir().join(format!("kwb-source-guards-{name}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&root);

            let fixture = Self { root };
            fixture.Write("src/top.rs", "pub fn Top_Level() { }");
            fixture.Write("src/below/nested.rs", "pub fn Below_The_Top_Level() { }");

            return fixture;
        }

        /// Write one file into the tree, creating the directories above it.
        fn Write(&self, relative: &str, body: &str)
        {
            let path = self.root.join(relative);
            if let Some(parent) = path.parent()
            {
                std::fs::create_dir_all(parent).expect("a writable temporary directory");
            }
            std::fs::write(&path, body).expect("a writable temporary file");
        }

        /// The path the reader under test is pointed at.
        fn Manifest_Directory(&self) -> String
        {
            return self.root.to_string_lossy().into_owned();
        }
    }

    impl Drop for Fixture
    {
        fn drop(&mut self)
        {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    /// The reader must return a source below the top level of `src`, and the top-level one too.
    ///
    /// The first assertion is the item's subject: a read that never descends returns `top.rs` and
    /// not the nested source, and it rules out exactly that -- a guard built on this reader would
    /// stop seeing a file the day somebody split it into a subdirectory.
    ///
    /// The second is keyed on the *path*, and that is the whole of `KWB-138`. It read
    /// `source.contains("Top_Level")` until then, and the nested fixture's `Below_The_Top_Level`
    /// carries that string, so it held whenever the first assertion did and the case its
    /// documentation claimed to cover -- a reader that descends but skips the top level -- was
    /// never checked at all. A file's path is not something the other file can carry, so the two
    /// assertions are now independent, and this one also fixes the separators: a raw path on
    /// Windows would read `below\nested.rs`.
    #[test]
    fn Test_The_Source_Reader_Should_Read_Below_The_Top_Level()
    {
        let fixture = Fixture::New("recursion");
        let sources = Crate_Sources_With_Paths(&fixture.Manifest_Directory());

        assert!(
            sources
                .iter()
                .any(|(path, source)| return path == "below/nested.rs"
                    && source.contains("Below_The_Top_Level")),
            "a source below the top level was not read, so a guard built on this reader would \
             stop seeing a file the day somebody split it into a subdirectory: {sources:?}"
        );
        assert!(
            sources.iter().any(|(path, _)| return path == "top.rs"),
            "the fixture's top-level source was not read, so a reader that descended but skipped \
             the files beside `src/` would pass this case: {sources:?}"
        );
    }
}
