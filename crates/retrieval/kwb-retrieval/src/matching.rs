//! What a query's words are compared against, decided once for both worlds.
//!
//! It is filed apart from either query type because both answer *which text contains every word
//! of the query*, and a second copy of that rule is where the current and historical searches
//! would come to disagree about what a match is — the same drift `D-012` describes between two
//! liveness rules, one level down.
//!
//! Crate-private, so this is a rule about how a question is asked rather than a surface a caller
//! may build against.

use kwb_model::Normalize_Text;

/// The query's words, normalized the way a claim's own text was.
pub(crate) fn Words_Of(query: &str) -> Vec<String>
{
    return Normalize_Text(query)
        .split_whitespace()
        .map(str::to_owned)
        .collect();
}

/// Whether `text` contains every word, compared after the same normalization.
pub(crate) fn Has_All_Words(text: &str, words: &[String]) -> bool
{
    let normalized = Normalize_Text(text);
    return words.iter().all(|word| return normalized.contains(word.as_str()));
}

/// The two rules above, asserted where nothing else can assert them.
///
/// # Why these are in the file rather than beside it
///
/// Both functions are `pub(crate)`, so no `tests/` suite can reach them — a test file under
/// `tests/` is a separate package and sees only the library's `pub` surface. They are the
/// rules the current and historical searches *share*, which is the reason this module exists,
/// and the reason to test them here is that a divergence between the two searches would
/// otherwise be invisible: each query type has its own suite, and a rule both of them call has
/// none.
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn Test_Words_Of_Should_Split_A_Query_Into_The_Words_A_Claim_Would_Keep()
    {
        // The normalization is `kwb-model`'s and this file does not own it. What this file
        // owns is the split, so a caller relying on "two spaces are one word" is relying on
        // this call rather than on the normalizer underneath it.
        assert_eq!(Words_Of("  isolated   system  "), ["isolated", "system"]);
    }

    #[test]
    fn Test_Words_Of_Should_Answer_Nothing_For_A_Query_That_Is_Only_Space()
    {
        // This empty list is the whole of how an empty query is refused: both query types read
        // it here and return before matching. It must stay empty rather than become one empty
        // word, because one empty word matches every text there is.
        assert!(Words_Of("").is_empty());
        assert!(Words_Of("   ").is_empty());
    }

    #[test]
    fn Test_Has_All_Words_Should_Require_Every_Word_Rather_Than_Any_Of_Them()
    {
        let words = Words_Of("isolated system");

        assert!(Has_All_Words("It is non-decreasing in an isolated system.", &words));
        assert!(
            !Has_All_Words("It is isolated.", &words),
            "a text holding one word of the query was reported as holding all of them"
        );
    }

    #[test]
    fn Test_Has_All_Words_Should_Answer_True_For_No_Words_At_All()
    {
        // Vacuously true, and pinned deliberately. `Claims_Matching` and `Concepts_Matching`
        // each refuse an empty query *before* reaching here, and this is why they must: a
        // caller that moved that refusal down here, on the reasonable-sounding belief that the
        // matcher is where emptiness belongs, would match every text in the graph.
        assert!(Has_All_Words("anything at all", &[]));
        assert!(!Has_All_Words("", &Words_Of("isolated")));
        assert!(Has_All_Words("isolated", &Words_Of("isolated")));
    }
}
