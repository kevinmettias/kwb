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
