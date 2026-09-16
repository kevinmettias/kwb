//! Reading born-digital text, one passage at a time.

use kwb_domain::Scope;
use kwb_ingest::ClaimText;
use kwb_ingest::ConceptName;
use kwb_ingest::Extraction;
use kwb_ingest::ExtractionLineage;
use kwb_ingest::ExtractionError;
use kwb_ingest::ExtractionStrategy;
use kwb_ingest::ProposedReading;
use kwb_ingest::ReaderName;
use kwb_ingest::ReadingKind;
use kwb_ingest::ReadingProtocol;
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

/// The characters per page below which XVPE grades that page `ImageOnly`.
///
/// The lower of the two floors given to `FidelityThresholds::New`, and one rather than zero
/// because a page with nothing extracted at all is exactly what that verdict names: zero would
/// make the verdict unreachable. A text file is all text, so this is the lowest floor that can be
/// expressed and still say something, and it is why a page graded here arrives at the refusal in
/// [`Readings_Of`](ReadsText::Readings_Of) rather than as a surprise.
const IMAGE_ONLY_BELOW: u32 = 1;

/// The characters per page below which XVPE grades that page `Sparse`.
///
/// One character above [`IMAGE_ONLY_BELOW`], which is the narrowest pair `FidelityThresholds::New`
/// will accept: a page too thin to be sparse is thereby too thin to be born-digital, so an
/// inverted pair could not classify anything consistently. Thresholds decide the fidelity recorded
/// on each passage, and a text file is all text — nothing a real source holds falls between these
/// two floors, so nothing a real source holds is graded as needing a look.
const SPARSE_BELOW: u32 = 2;

/// The ceiling on tokens the model may generate for one passage's answer.
///
/// A working default rather than a measurement, for the reason [`PASSAGE_CHARACTERS`] gives:
/// nothing here has a corpus to measure against. It is generous enough that a passage of ordinary
/// prose never meets it, and it is named so that a corpus which says otherwise changes one number
/// in one place.
const ANSWER_TOKEN_BUDGET: u32 = 2_048;

/// The most propositions one passage's answer may carry.
///
/// A ceiling rather than a target, and part of the request's schema rather than a count taken
/// afterwards, so an answer offering more comes back non-conforming and is refused — which is
/// stronger than trimming it here, because an answer that overflowed says the model read a
/// different passage than the one it was sent.
const MAXIMUM_PROPOSITIONS: u32 = 32;

/// A model-backed reader of born-digital text.
///
/// # What it refuses, and why each refusal is a different fact
///
/// - A source needing a look rather than a read: [`ExtractionError::CannotRead`]. It does not
///   return an empty reading, because a page nobody could read is not a page with nothing on it.
/// - Bytes that are not text, or an answer that did not conform: `ReaderFailed`. The reader was
///   asked and did not answer usably, so **nothing was learned about the source** and admission
///   records `Unmet` rather than `Barren`.
/// - A passage read that asserts nothing: **not a refusal**. That is an empty reading and a
///   success, and it is the only outcome that is evidence of absence.
///
/// The middle one is where a model is most dangerous and the least work is needed, because
/// `InferenceStrategy`'s own contract already says *a schema is a constraint, not a suggestion*
/// and returns `Malformed_Answer` rather than a non-conforming answer. This maps that to a refusal; it
/// does not re-check what the mechanism already refused.
///
/// # Why this is public
///
/// Its caller is the provider adapter, which lives **outside this repository by design** — no
/// provider, credential or network dependency is in this workspace, so the composition root that
/// constructs a reader is not here and never will be. That is the one case where an export has a
/// caller the call graph cannot see: narrowing it would make this crate's entire product
/// unreachable, which rustc reports as the whole module going dead. `PROTOCOL` is public for the
/// same reason, and `Request_For` is not, because nothing outside needs to build a request.
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
    /// [`ExtractionError`] when the source needs a kind of reading this does not do, when its
    /// bytes are not text, or when the reader did not answer usably for **any** passage. The
    /// last is all-or-nothing by the seam's own rule: a source two thirds read that reported the
    /// shape of a source fully read is the prototype's 1,367 rows again.
    fn Read(
        &self,
        source: ContentIdentity,
        content: &[u8],
        needed: ReadingKind,
    ) -> Result<Vec<ProposedReading>, ExtractionError>
    {
        if needed != ReadingKind::Text
        {
            return Err(ExtractionError::CannotRead { needed });
        }

        let text = Text_Of(content)?;

        return self.Readings_Of(source, text);
    }

    fn Scope(&self) -> Scope
    {
        return self.scope.clone();
    }
}

/// The source's bytes as text, or the refusal that they were not text at all.
///
/// Separate from the reading itself because it is the one step where a file stops being bytes and
/// starts being passages. A source that is not text is not a source with nothing in it, which is
/// why this refuses rather than yielding an empty reading.
///
/// # Errors
///
/// [`ExtractionError::ReaderFailed`] when the bytes are not UTF-8.
fn Text_Of(content: &[u8]) -> Result<&str, ExtractionError>
{
    return core::str::from_utf8(content).map_err(|cause| {
        return ExtractionError::ReaderFailed {
            cause: format!("the source is not text: {cause}"),
        };
    });
}

impl<Reader: InferenceStrategy> ReadsText<Reader>
{
    /// Read every passage of `text`, or refuse on the first that cannot be read.
    ///
    /// All-or-nothing for the reason [`Read`](ExtractionStrategy::Read) gives, so the refusal is
    /// raised here rather than recorded per passage.
    ///
    /// # Errors
    ///
    /// [`ExtractionError::CannotRead`] when a passage is graded as needing a look rather than a
    /// read, and [`ExtractionError::ReaderFailed`] when the reader did not answer usably.
    fn Readings_Of(
        &self,
        source: ContentIdentity,
        text: &str,
    ) -> Result<Vec<ProposedReading>, ExtractionError>
    {
        let mut readings = Vec::new();
        for passage in Passages_Of(text)
        {
            // XVPE measured this page and says it needs looking at. That is `PageFidelity`
            // mapping *into* `ReadingKind` rather than replacing it, which is what `KWB-47`
            // said would happen when a reading adapter arrived: the grading stays XVPE's and
            // the capability question stays KWB's.
            if passage.Requires_Page_Images()
            {
                return Err(ExtractionError::CannotRead {
                    needed: ReadingKind::Visual,
                });
            }

            let reading = self.Reading_Of(source, &passage)?;
            readings.push(reading);
        }

        return Ok(readings);
    }

    /// Ask about one passage.
    fn Reading_Of(
        &self,
        source: ContentIdentity,
        passage: &Passage,
    ) -> Result<ProposedReading, ExtractionError>
    {
        let passage_text = passage.Text();
        let request = Request_For(&self.model, passage_text);

        let answered = self.reader.Infer(&request).map_err(|cause| {
            return ExtractionError::ReaderFailed {
                cause: format!("{cause}"),
            };
        })?;

        let where_in = Where_In(passage);
        let location = SourceLocation::Named(&where_in);
        let answer = answered.Answer();
        let proposed = Proposed_From(answer)?;
        let model_name = self.model.As_Str();
        let lineage =
            ExtractionLineage::Of(ReadingProtocol::Named(PROTOCOL), ReaderName::Named(model_name));
        let reading = ProposedReading::Of(source, location, proposed, lineage);

        return Ok(reading);
    }
}

/// The request this reader sends about one passage.
///
/// # Why this is crate-visible rather than private
///
/// A recording is keyed by a request's fingerprint, so a test that built its own request would
/// be recording against a question this reader never asks — and would go on passing after the
/// instructions or the schema changed underneath it. `KWB-57` is the item where a fixture built
/// from a reconstruction rather than the real instance passed while the real one slipped
/// through; this is the same hazard with a different shape, so the test and the reader ask
/// through one function. That test lives in this crate (see [`crate::tests`]), which is the
/// whole of why this is `pub(crate)` rather than private — the rung that reaches it and no
/// higher one.
#[must_use]
pub(crate) fn Request_For(model: &ModelIdentifier, passage: &str) -> InferenceRequest
{
    let role = ModelRole::Named(0, "extractor");
    let instructions = INSTRUCTIONS.to_owned();
    let passage_text = passage.to_owned();
    let content = ContentBlock::Of_Text(passage_text);
    let blocks = vec![content];
    let schema = Propositions_Schema();
    let request = InferenceRequest::New(
        model.clone(),
        role,
        instructions,
        blocks,
        Some(schema),
        None,
        ANSWER_TOKEN_BUDGET,
    );

    return request;
}

/// The shape an answer must have, or the strategy refuses it.
///
/// # Why the schema is the guard rather than a parser here
///
/// `InferenceStrategy`'s contract is that *a schema is a constraint, not a suggestion*: an answer
/// that does not conform comes back as `Malformed_Answer` rather than as data. So the strongest place to
/// say what an extraction *is* is in the request, before anything has been read — and the reading
/// below is then a total function over a value already known to have this shape, rather than a
/// parser deciding at admission time what to make of whatever arrived.
///
/// This is the user-stated invariant *malformed model output never becomes a graph mutation*,
/// enforced one layer below KWB by a mechanism that already had to enforce it.
#[must_use]
pub(crate) fn Propositions_Schema() -> ResponseSchema
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
            maximum_length: Some(MAXIMUM_PROPOSITIONS),
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
    let number = PageNumber::From_Zero_Based(0);
    let profile = PageProfile::New(characters, 0, 0);
    let content = text.to_owned();
    let page = PageText::New(number, profile, content);

    let Some(budget) = PassageBudget::New(PASSAGE_CHARACTERS, PASSAGE_PAGES)
    else
    {
        return Vec::new();
    };

    let Some(thresholds) = FidelityThresholds::New(IMAGE_ONLY_BELOW, SPARSE_BELOW)
    else
    {
        return Vec::new();
    };

    let passages = SectionBoundary.Chunk_Pages(&[page], budget, thresholds);

    return passages;
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
/// [`ExtractionError::ReaderFailed`] when the answer is not a sequence of records each
/// carrying a textual `concept` and `claim`.
fn Proposed_From(answer: &AnswerValue) -> Result<Vec<Extraction>, ExtractionError>
{
    let Some(propositions) = answer.As_Sequence()
    else
    {
        return Err(Malformed_Answer("a list of propositions was expected"));
    };

    let mut proposed = Vec::new();
    for proposition in propositions
    {
        let extraction = Proposed_Of(proposition)?;
        proposed.push(extraction);
    }

    return Ok(proposed);
}

/// The one proposition a record holds, or the refusal naming the field it is missing.
///
/// # Errors
///
/// [`ExtractionError::ReaderFailed`] when the record carries no textual `concept` or `claim`.
fn Proposed_Of(proposition: &AnswerValue) -> Result<Extraction, ExtractionError>
{
    let Some(concept) = proposition.Field("concept").and_then(AnswerValue::As_Text)
    else
    {
        return Err(Malformed_Answer("a proposition carries no textual concept"));
    };
    let Some(claim) = proposition.Field("claim").and_then(AnswerValue::As_Text)
    else
    {
        return Err(Malformed_Answer("a proposition carries no textual claim"));
    };

    let extraction = Extraction::New(ConceptName::Named(concept), ClaimText::Stated(claim));

    return Ok(extraction);
}

/// The one refusal a reader that did not answer usably gets.
///
/// Written once because the cause is the whole of what differs between the sites that make it,
/// and spelling the mapping out at each of them made a short check read as a five-line one. Every
/// reason `Proposed_From` can reject an answer is this same fact about the reader, so the
/// argument names which part of the answer was wrong and nothing else varies.
fn Malformed_Answer(what: &str) -> ExtractionError
{
    return ExtractionError::ReaderFailed {
        cause: format!("the answer is not what was asked for: {what}"),
    };
}

/// The assertions [`crate::tests`] cannot make, because they are about the rungs below it.
///
/// `crate::tests` tests the reader end to end and reaches `Request_For` only through
/// `Recording_Answering` — a fixture, not a test, so nothing there *names* it. `Over` and
/// `Propositions_Schema` are not named there at all. This module is the companion unit for this
/// file, which is where the rule reads for them, and it is deliberately small: the behaviour
/// worth asserting is already asserted next door.
#[cfg(test)]
mod tests
{
    use super::*;
    use kwb_platform_xvpe::inference::ReplayInference;

    #[test]
    fn Test_Over_Should_Carry_The_Reach_It_Was_Given_Into_The_Reader()
    {
        // `D-010` puts scope on the assertion, so the scope is the reader's to carry rather than
        // a default filled in downstream. If `Over` dropped it, every reading this reader made
        // would be asserted at a scope nobody chose and nothing would say so.
        let scope = Scope::Named("physical theory").expect("a named scope");
        let reader = ReadsText::Over(
            ReplayInference::From_Recordings(Vec::new()),
            Model(),
            scope.clone(),
        );

        assert_eq!(reader.Scope(), scope, "the scope `Over` was given is not the one it asserts at");
    }

    #[test]
    fn Test_Request_For_Should_Ask_About_One_Passage_Under_One_Model()
    {
        let model = Model();
        let request = Request_For(&model, "Entropy does not decrease in an isolated system.");

        assert_eq!(request.Model().As_Str(), "a-recorded-reader");
        assert_eq!(request.Content().len(), 1, "a passage travels as exactly one content block");
        assert_eq!(request.Maximum_Output_Tokens(), ANSWER_TOKEN_BUDGET);
        assert!(
            request.Schema().is_some(),
            "the request carries no schema, so nothing constrains the answer it comes back with"
        );
    }

    #[test]
    fn Test_Propositions_Schema_Should_Require_A_Concept_And_A_Claim_From_Each_Proposition()
    {
        let schema = Propositions_Schema();

        assert_eq!(schema.Name(), "propositions");
        let SchemaNode::Sequence {
            items,
            maximum_length,
            ..
        } = schema.Root()
        else
        {
            panic!("the schema's root is not a sequence of propositions");
        };
        assert_eq!(
            *maximum_length,
            Some(MAXIMUM_PROPOSITIONS),
            "the ceiling belongs in the request, so an answer that overflows is refused rather \
             than trimmed here"
        );

        let SchemaNode::Record { fields, .. } = items.as_ref()
        else
        {
            panic!("a proposition is not a record of fields");
        };
        let required: Vec<&str> = fields
            .iter()
            .filter(|field| return field.Is_Required())
            .map(|field| return field.Name())
            .collect();
        assert_eq!(
            required,
            ["concept", "claim"],
            "an answer could leave a concept or a claim out, so a proposition would arrive half-read"
        );
    }

    /// The model these tests ask through.
    ///
    /// Last because all three tests ask through it, so it belongs under none of them: a helper
    /// with one caller sits beneath that caller, and a helper with several sits here.
    fn Model() -> ModelIdentifier
    {
        return ModelIdentifier::New("a-recorded-reader".to_owned());
    }
}
