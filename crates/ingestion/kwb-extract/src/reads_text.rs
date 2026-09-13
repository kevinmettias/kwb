//! Reading born-digital text, one passage at a time.

use kwb_domain::Scope;
use kwb_ingest::Extraction;
use kwb_ingest::ExtractionLineage;
use kwb_ingest::ExtractionRefused;
use kwb_ingest::ExtractionStrategy;
use kwb_ingest::ProposedReading;
use kwb_ingest::ReadingKind;
use kwb_ingest::SourceLocation;
use kwb_model::ContentIdentity;
use kwb_platform_xvpe::inference::AnswerValue;
use kwb_platform_xvpe::inference::ContentBlock;
use kwb_platform_xvpe::inference::InferenceRequest;
use kwb_platform_xvpe::inference::InferenceStrategy;
use kwb_platform_xvpe::inference::ModelIdentifier;
use kwb_platform_xvpe::inference::ModelRole;
use kwb_platform_xvpe::inference::ResponseSchema;
use kwb_platform_xvpe::inference::SchemaField;
use kwb_platform_xvpe::inference::SchemaNode;
use kwb_platform_xvpe::reading::FidelityThresholds;
use kwb_platform_xvpe::reading::PageNumber;
use kwb_platform_xvpe::reading::PageProfile;
use kwb_platform_xvpe::reading::PageText;
use kwb_platform_xvpe::reading::Passage;
use kwb_platform_xvpe::reading::PassageBudget;
use kwb_platform_xvpe::reading::PassageSplitter;
use kwb_platform_xvpe::reading::SectionBoundary;

/// What this reader does, recorded on every reading it produces.
///
/// A version in the name, deliberately. `KWB-47` made lineage travel with the reading so that
/// *changing the model or the prompt does not silently redefine KWB semantic identity*; a
/// protocol that never changed its name when its prompt changed would defeat that by being
/// constant. Whoever alters the instructions below changes this with them.
pub const PROTOCOL: &str = "read-born-digital-text-v1";

/// What the reader is asked for, and the two fields an answer must carry.
const INSTRUCTIONS: &str = "You are reading one passage of a reference. List the propositions \
                            the passage asserts. For each, give the concept it is about and what \
                            the passage says about it, in the passage's own terms. Propose \
                            nothing the passage does not assert.";

/// How large a passage may be before it is split.
///
/// # Why these numbers are arbitrary and say so
///
/// They are not measured, because nothing here has a corpus to measure against: this repository
/// admits handfuls of documents. `D-012`'s warning is the relevant one — *a performance decision
/// taken before anything is slow is a guess with a schema* — so these are a working default and
/// are named as such rather than defended.
///
/// *Settled by:* a corpus where passage size changes what is extracted, which is the kind of
/// question the volume work after this item is for.
const PASSAGE_CHARACTERS: u32 = 4_000;

/// One text file is one page, so a passage never spans more than it.
const PASSAGE_PAGES: u32 = 1;

/// A model-backed reader of born-digital text.
///
/// # What it refuses, and why each refusal is a different fact
///
/// - A source needing a look rather than a read: [`ExtractionRefused::CannotRead`]. It does not
///   return an empty reading, because a page nobody could read is not a page with nothing on it.
/// - Bytes that are not text, or an answer that did not conform: `ReaderFailed`. The reader was
///   asked and did not answer usably, so **nothing was learned about the source** and admission
///   records `Unmet` rather than `Barren`.
/// - A passage read that asserts nothing: **not a refusal**. That is an empty reading and a
///   success, and it is the only outcome that is evidence of absence.
///
/// The middle one is where a model is most dangerous and the least work is needed, because
/// `InferenceStrategy`'s own contract already says *a schema is a constraint, not a suggestion*
/// and returns `Malformed` rather than a non-conforming answer. This maps that to a refusal; it
/// does not re-check what the mechanism already refused.
pub struct ReadsText<Reader>
{
    reader: Reader,
    model: ModelIdentifier,
    scope: Scope,
}

impl<Reader> ReadsText<Reader>
{
    /// A reader, the model it speaks to, and the scope its readings are asserted at.
    ///
    /// The scope is the reader's because `D-010` puts scope on the assertion — *how far a source
    /// claims to reach* is a property of the reading rather than of the proposition — and the
    /// seam already asks every strategy for one.
    pub const fn Over(reader: Reader, model: ModelIdentifier, scope: Scope) -> Self
    {
        return Self {
            reader,
            model,
            scope,
        };
    }
}

impl<Reader: InferenceStrategy> ExtractionStrategy for ReadsText<Reader>
{
    /// Split the source into passages and ask about each.
    ///
    /// # Errors
    ///
    /// [`ExtractionRefused`] when the source needs a kind of reading this does not do, when its
    /// bytes are not text, or when the reader did not answer usably for **any** passage. The
    /// last is all-or-nothing by the seam's own rule: a source two thirds read that reported the
    /// shape of a source fully read is the prototype's 1,367 rows again.
    fn Read(
        &self,
        source: ContentIdentity,
        content: &[u8],
        needed: ReadingKind,
    ) -> Result<Vec<ProposedReading>, ExtractionRefused>
    {
        if needed != ReadingKind::Text
        {
            return Err(ExtractionRefused::CannotRead { needed });
        }

        let text = core::str::from_utf8(content).map_err(|cause| {
            return ExtractionRefused::ReaderFailed {
                cause: format!("the source is not text: {cause}"),
            };
        })?;

        let mut readings = Vec::new();
        for passage in Passages_Of(text)
        {
            // XVPE measured this page and says it needs looking at. That is `PageFidelity`
            // mapping *into* `ReadingKind` rather than replacing it, which is what `KWB-47`
            // said would happen when a reading adapter arrived: the grading stays XVPE's and
            // the capability question stays KWB's.
            if passage.Requires_Page_Images()
            {
                return Err(ExtractionRefused::CannotRead {
                    needed: ReadingKind::Visual,
                });
            }

            readings.push(self.Reading_Of(source, &passage)?);
        }

        return Ok(readings);
    }

    fn Scope(&self) -> Scope
    {
        return self.scope.clone();
    }
}

impl<Reader: InferenceStrategy> ReadsText<Reader>
{
    /// Ask about one passage.
    fn Reading_Of(
        &self,
        source: ContentIdentity,
        passage: &Passage,
    ) -> Result<ProposedReading, ExtractionRefused>
    {
        let request = Request_For(&self.model, passage.Text());

        let answered = self.reader.Infer(&request).map_err(|cause| {
            return ExtractionRefused::ReaderFailed {
                cause: format!("{cause}"),
            };
        })?;

        return Ok(ProposedReading::Of(
            source,
            SourceLocation::Named(&Where_In(passage)),
            Proposed_From(answered.Answer())?,
            ExtractionLineage::Of(PROTOCOL, self.model.As_Str()),
        ));
    }
}

/// The request this reader sends about one passage.
///
/// # Why this is public
///
/// A recording is keyed by a request's fingerprint, so a test that built its own request would
/// be recording against a question this reader never asks — and would go on passing after the
/// instructions or the schema changed underneath it. `KWB-57` is the item where a fixture built
/// from a reconstruction rather than the real instance passed while the real one slipped
/// through; this is the same hazard with a different shape, so the test and the reader ask
/// through one function.
#[must_use]
pub fn Request_For(model: &ModelIdentifier, passage: &str) -> InferenceRequest
{
    return InferenceRequest::New(
        model.clone(),
        ModelRole::Named(0, "extractor"),
        INSTRUCTIONS.to_owned(),
        vec![ContentBlock::Of_Text(passage.to_owned())],
        Some(Propositions_Schema()),
        None,
        2_048,
    );
}

/// The shape an answer must have, or the strategy refuses it.
///
/// # Why the schema is the guard rather than a parser here
///
/// `InferenceStrategy`'s contract is that *a schema is a constraint, not a suggestion*: an answer
/// that does not conform comes back as `Malformed` rather than as data. So the strongest place to
/// say what an extraction *is* is in the request, before anything has been read — and the reading
/// below is then a total function over a value already known to have this shape, rather than a
/// parser deciding at admission time what to make of whatever arrived.
///
/// This is the user-stated invariant *malformed model output never becomes a graph mutation*,
/// enforced one layer below KWB by a mechanism that already had to enforce it.
#[must_use]
pub fn Propositions_Schema() -> ResponseSchema
{
    let proposition = SchemaNode::Record {
        description: "one proposition the passage asserts".to_owned(),
        fields: vec![
            SchemaField::Required(
                "concept".to_owned(),
                SchemaNode::Text {
                    description: "what it is about, as the passage names it".to_owned(),
                },
            ),
            SchemaField::Required(
                "claim".to_owned(),
                SchemaNode::Text {
                    description: "what the passage asserts about it".to_owned(),
                },
            ),
        ],
    };

    return ResponseSchema::New(
        "propositions".to_owned(),
        SchemaNode::Sequence {
            description: "every proposition the passage asserts, and nothing it does not"
                .to_owned(),
            items: Box::new(proposition),
            maximum_length: Some(32),
        },
    );
}

/// The passages of one text file.
///
/// A file is one page, which is the honest description of a born-digital text source: it has no
/// page structure, and inventing one would be this crate deciding something `xvpe-corpus-text`
/// owns. The splitter is XVPE's, so what a passage *is* stays measured there.
fn Passages_Of(text: &str) -> Vec<Passage>
{
    let characters = u32::try_from(text.chars().count()).unwrap_or(u32::MAX);
    let page = PageText::New(
        PageNumber::From_Zero_Based(0),
        PageProfile::New(characters, 0, 0),
        text.to_owned(),
    );

    let Some(budget) = PassageBudget::New(PASSAGE_CHARACTERS, PASSAGE_PAGES)
    else
    {
        return Vec::new();
    };
    // Thresholds decide the fidelity recorded on each passage. A text file is all text, so the
    // floors are the lowest that can be expressed: nothing here should ever be graded as needing
    // a look, and if it is, the refusal above is the right answer rather than a surprise.
    let Some(thresholds) = FidelityThresholds::New(1, 2)
    else
    {
        return Vec::new();
    };

    return SectionBoundary.Chunk_Pages(&[page], budget, thresholds);
}

/// Where a passage was, in the reader's own words.
fn Where_In(passage: &Passage) -> String
{
    let span = passage.Span();

    return format!(
        "characters {} to {} of the text",
        span.First().Zero_Based(),
        span.Last().Zero_Based()
    );
}

/// What the answer proposes, or a refusal if it is not an answer of that shape.
///
/// # This was written as a total function and that was wrong
///
/// The first version returned `Vec::new()` for an answer that was not a sequence, on the premise
/// that *a schema is a constraint, not a suggestion* had already refused every other shape
/// upstream. **Measured: that premise does not hold for a replaying strategy.** A schema
/// constrains a strategy that *produces* an answer; `ReplayInference` returns what was recorded,
/// so a recording carrying a non-conforming answer replays as a perfectly valid one.
///
/// The consequence was the exact failure this seam exists to prevent. No proposals meant an
/// empty reading, an empty reading is a success, and a successful reading that proposed nothing
/// is `Barren` — **evidence of absence**. A model that answered nonsense would have been
/// recorded as a source that asserts nothing.
///
/// So the check is here, where it cannot be delegated to a mechanism that only sometimes
/// performs it. An answer of the wrong shape is a reader that did not answer usably, which is
/// `Unmet`.
///
/// # Why one malformed proposition refuses the whole reading
///
/// The same rule the seam already applies to passages. Dropping the bad one and keeping the rest
/// would report a passage that yielded two propositions where the reader offered three, and
/// nothing downstream could tell that from a passage that only had two.
///
/// # Errors
///
/// [`ExtractionRefused::ReaderFailed`] when the answer is not a sequence of records each
/// carrying a textual `concept` and `claim`.
fn Proposed_From(answer: &AnswerValue) -> Result<Vec<Extraction>, ExtractionRefused>
{
    let malformed = |what: &str| {
        return ExtractionRefused::ReaderFailed {
            cause: format!("the answer is not what was asked for: {what}"),
        };
    };

    let Some(propositions) = answer.As_Sequence()
    else
    {
        return Err(malformed("a list of propositions was expected"));
    };

    let mut proposed = Vec::new();
    for proposition in propositions
    {
        let concept = proposition
            .Field("concept")
            .and_then(AnswerValue::As_Text)
            .ok_or_else(|| return malformed("a proposition carries no textual concept"))?;
        let claim = proposition
            .Field("claim")
            .and_then(AnswerValue::As_Text)
            .ok_or_else(|| return malformed("a proposition carries no textual claim"))?;

        proposed.push(Extraction::New(concept.to_owned(), claim.to_owned()));
    }

    return Ok(proposed);
}
