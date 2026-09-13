//! The property replay exists to have: a graph rebuilt from its own publications is the graph
//! that produced them.

use kwb_domain::{
    Assertion, Claim, Concept, KnowledgeGraph, Publication, Replay, ReplayError, Scope, Standing,
    Versioned,
};

/// Between the fields of a record. Named here rather than imported, because a test that took
/// the constant from the code under test could not notice the code changing it.
const SEPARATOR: char = '\u{1F}';

/// A small corpus, and the publications that would have built it.
fn Corpus() -> (KnowledgeGraph, Vec<String>)
{
    let entropy = Concept::Named("entropy");
    let enthalpy = Concept::Named("enthalpy");
    let claim = Claim::About(&entropy, "It is non-decreasing in an isolated system.");
    let assertion = Assertion::By("Callen 1985", &claim, Scope::Named("physical theory").expect("a named scope"));

    let publications = [
        Publication::Concept {
            concept: entropy.clone(),
            standing: Standing::Asserted,
        },
        Publication::Concept {
            concept: enthalpy.clone(),
            standing: Standing::Retired {
                because: "the concept was withdrawn by its source".to_owned(),
            },
        },
        Publication::Claim {
            claim: claim.clone(),
            standing: Standing::Asserted,
        },
        Publication::Assertion {
            assertion: assertion.clone(),
            standing: Standing::Asserted,
        },
    ];

    let graph = KnowledgeGraph::Empty()
        .With_Concept(Versioned::Asserted(entropy))
        .With_Concept(Versioned::Asserted(enthalpy).Closed(Standing::Retired {
            because: "the concept was withdrawn by its source".to_owned(),
        }))
        .With_Claim(Versioned::Asserted(claim))
        .With_Assertion(Versioned::Asserted(assertion));

    let records = publications.iter().map(Publication::Record).collect();
    return (graph, records);
}

#[test]
fn Test_A_Graph_Replayed_From_Its_Publications_Should_Hold_What_The_Original_Held()
{
    let (original, records) = Corpus();

    let replayed = Replay(&records).expect("replays");

    // Compared by what both hold, not by trusting the encoder round-tripped.
    assert_eq!(
        replayed.Current().Concepts().len(),
        original.Current().Concepts().len()
    );
    assert_eq!(
        replayed.Every_Version().Concepts().len(),
        original.Every_Version().Concepts().len()
    );
    assert_eq!(
        replayed.Current().Claims().iter().map(|claim| return claim.Identity()).collect::<Vec<_>>(),
        original.Current().Claims().iter().map(|claim| return claim.Identity()).collect::<Vec<_>>(),
        "a replayed claim is addressed differently from the one that was recorded"
    );
    assert_eq!(
        replayed
            .Current()
            .Assertions()
            .iter()
            .map(|found| return found.Identity())
            .collect::<Vec<_>>(),
        original
            .Current()
            .Assertions()
            .iter()
            .map(|found| return found.Identity())
            .collect::<Vec<_>>()
    );
}

#[test]
fn Test_A_Retired_Concept_Should_Replay_Retired()
{
    let (_, records) = Corpus();

    let replayed = Replay(&records).expect("replays");

    assert_eq!(replayed.Every_Version().Concepts().len(), 2);
    assert_eq!(
        replayed.Current().Concepts().len(),
        1,
        "a standing that did not survive replay would make every closed thing current again"
    );
}

#[test]
fn Test_A_Superseded_Standing_Should_Carry_Its_Successor_Through_A_Record()
{
    let loser = Concept::Named("C");
    let keeper = Concept::Named("C++");
    let records = vec![
        Publication::Concept {
            concept: keeper.clone(),
            standing: Standing::Asserted,
        }
        .Record(),
        Publication::Concept {
            concept: loser,
            standing: Standing::Superseded {
            by: keeper.Identity(),
            because: "the two names denote one concept".to_owned(),
        },
        }
        .Record(),
    ];

    let replayed = Replay(&records).expect("replays");

    let losers = replayed.Every_Version().Merge_Losers();
    assert_eq!(losers.len(), 1);
    assert_eq!(
        losers.first().and_then(|held| return held.Standing().Superseded_By()),
        Some(keeper.Identity()),
        "the successor did not survive the record, so the merge log would resolve to nothing"
    );
}

// ---- the framing rests on the domain normalizing, and that is asserted ----

#[test]
fn Test_A_Value_Should_Not_Be_Able_To_Forge_A_Field_Boundary()
{
    // The whole reason this format needs no escaping. If a domain type ever kept text without
    // normalizing it, this would start failing -- which is the point of testing it here rather
    // than assuming it.
    let forged = format!("hostile{SEPARATOR}asserted{SEPARATOR}{SEPARATOR}extra");
    let concept = Concept::Named(&forged);

    let record = Publication::Concept {
        concept,
        standing: Standing::Asserted,
    }
    .Record();

    assert_eq!(
        Fields(&record),
        Fields(&Recorded("entropy")),
        "a value carried a separator into the record, so a reader would split it into the \
         wrong number of fields: {record:?}"
    );
    assert!(!record.contains('\n'), "a value carried a newline into a one-record-per-line log");
}

// ---- what replay refuses ----

#[test]
fn Test_A_Claim_Whose_Concept_Was_Never_Published_Should_Be_Refused()
{
    let orphan = Concept::Named("never published");
    let claim = Claim::About(&orphan, "It says something.");
    let records = vec![
        Publication::Claim {
            claim,
            standing: Standing::Asserted,
        }
        .Record(),
    ];

    let refusal = Replay(&records).expect_err("must refuse");

    assert!(matches!(refusal, ReplayError::OutOfOrder { .. }), "{refusal}");
    assert!(
        format!("{refusal}").contains("not a claim"),
        "the refusal should say why guessing is worse: {refusal}"
    );
}

#[test]
fn Test_A_Record_Of_An_Unknown_Shape_Should_Be_Refused()
{
    for record in [
        String::new(),
        "concept".to_owned(),
        format!("concept{SEPARATOR}asserted{SEPARATOR}{SEPARATOR}a{SEPARATOR}b"),
        format!("unknown{SEPARATOR}asserted{SEPARATOR}{SEPARATOR}a"),
        // A successor where none belongs, and none where one does.
        format!("concept{SEPARATOR}asserted{SEPARATOR}abcd{SEPARATOR}a"),
        format!("concept{SEPARATOR}superseded{SEPARATOR}{SEPARATOR}a"),
    ]
    {
        assert!(
            Replay(std::slice::from_ref(&record)).is_err(),
            "a record this writer could not have produced was accepted: {record:?}"
        );
    }
}

#[test]
fn Test_An_Empty_Sequence_Should_Replay_To_An_Empty_Graph()
{
    let replayed = Replay(&[]).expect("replays");

    assert_eq!(replayed.Every_Version().Concepts().len(), 0);
}

/// A concept record, for a name.
fn Recorded(name: &str) -> String
{
    return Publication::Concept {
        concept: Concept::Named(name),
        standing: Standing::Asserted,
    }
    .Record();
}

/// How many fields a record splits into.
fn Fields(record: &str) -> usize
{
    return record.split(SEPARATOR).count();
}

// ---- KWB-50: a stated scope and an unstated one are two different records ----

#[test]
fn Test_Text_That_Names_Nothing_Should_Not_Name_A_Scope()
{
    // The door. `Scope::Named` used to return the unstated scope for this input, which is how
    // `kwb admit --scope "   "` came to write the record of a source that said nothing about
    // how far it reached. Closing it here rather than at the command line is what makes it
    // unreachable instead of guarded in one place.
    for names_nothing in ["", "   ", "\t", "\n  \n"]
    {
        assert_eq!(
            Scope::Named(names_nothing),
            None,
            "{names_nothing:?} named a scope, so blank input can reach a record again"
        );
    }

    assert!(
        Scope::Named("physical theory").is_some(),
        "a scope with a name was refused, so this test would pass on a constructor that \
         refuses everything"
    );
}

#[test]
fn Test_A_Stated_Scope_And_An_Unstated_One_Should_Not_Write_The_Same_Record()
{
    // Compared as **records** rather than as values, because the values comparing equal is
    // exactly what hid this: `Scope::Named("")` *was* `Scope::Unstated()`, so a test asserting
    // the types differ had nothing to assert. The record is what a later replay reads and what
    // a person inspecting a log sees, so it is the thing that has to differ.
    let concept = Concept::Named("entropy");
    let claim = Claim::About(&concept, "It is non-decreasing in an isolated system.");

    let recorded_with = |scope| {
        return Publication::Assertion {
            assertion: Assertion::By("Callen 1985", &claim, scope),
            standing: Standing::Asserted,
        }
        .Record();
    };

    let stated = recorded_with(Scope::Named("physical theory").expect("a named scope"));
    let unstated = recorded_with(Scope::Unstated());

    assert_ne!(
        stated, unstated,
        "a source that said how far it reached and one that did not wrote the same record, \
         so D-010's distinction is gone from the only place it survives a process"
    );
    assert_eq!(
        Fields(&stated),
        Fields(&unstated),
        "the two differ in field count rather than in the scope field, which would mean \
         replay reads one of them as a different kind of record"
    );
}

#[test]
fn Test_An_Unstated_Scope_Should_Survive_A_Replay_As_Unstated()
{
    // The other half: refusing blank input at the constructor must not make the *absent* case
    // unreadable. Logs on disk carry an empty trailing field for every assertion whose source
    // did not say how far it reached, and replay still has to read them.
    let concept = Concept::Named("entropy");
    let claim = Claim::About(&concept, "It is non-decreasing in an isolated system.");
    let records: Vec<String> = [
        Publication::Concept {
            concept: concept.clone(),
            standing: Standing::Asserted,
        },
        Publication::Claim {
            claim: claim.clone(),
            standing: Standing::Asserted,
        },
        Publication::Assertion {
            assertion: Assertion::By("Callen 1985", &claim, Scope::Unstated()),
            standing: Standing::Asserted,
        },
    ]
    .iter()
    .map(Publication::Record)
    .collect();

    let replayed = Replay(&records).expect("a run's own publications must replay");
    let assertions = replayed.Current().Assertions();

    assert_eq!(assertions.len(), 1);
    assert!(
        assertions.first().expect("one").Scope().Is_Unstated(),
        "an unstated scope came back from a replay as something else"
    );
}
