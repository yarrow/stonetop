//! Spoken forms of print glyphs: slot diamonds become "N-slot", a resource's `{resource}`
//! placeholder becomes its count or its list of states, and a gizmo reference becomes the
//! gizmo's printed line, diamonds and all, for [`speak_slots`] to finish.

use std::sync::LazyLock;

use regex_lite::{Captures, Regex};

use crate::fixed::{CanBe, GizmoFixed, RESOURCE_PLACEHOLDER, Resource, gizmo_references};

/// A run of slot diamonds with the article beside it, if any: "an ◇◇", "◇ a ", or a bare
/// run. The alternatives are tried in that order, so an article before the run wins.
static SLOT_RUN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b([aA]n?) (◇+)|(◇+) ([aA]n?) |(◇+)").expect("valid regex"));

/// Speaks every run of slot diamonds in `text` as "N-slot", keeping the article in front
/// of the gizmo's name: "an ◇◇ iron hauberk" is "a 2-slot iron hauberk", "◇◇ A shield" is
/// "A 2-slot shield", "◇ a notebook" is "a 1-slot notebook".
pub fn speak_slots(text: &str) -> String {
    SLOT_RUN
        .replace_all(text, |caps: &Captures| {
            let run = [2, 3, 5].into_iter().find_map(|group| caps.get(group));
            let count = run.expect("one alternative matched").as_str().chars().count();
            let mut spoken = String::new();
            if let Some(article) = caps.get(1).or_else(|| caps.get(4)) {
                // "an" and "An" become "a" and "A", case kept.
                spoken.push_str(&article.as_str()[..1]);
                spoken.push(' ');
            }
            spoken.push_str(&format!("{count}-slot"));
            if caps.get(4).is_some() {
                // The space after a following article was part of the match.
                spoken.push(' ');
            }
            spoken
        })
        .into_owned()
}

/// A resource's capacity as words: a count for `CanBe::Max`, or the states best first for
/// `CanBe::Labels` ("plenty left, low ammo, or all out").
///
/// # Panics
/// If a `CanBe::Labels` resource has no labels.
pub fn resource_value(resource: &Resource) -> String {
    match resource.can_be {
        CanBe::Max(n) => n.to_string(),
        // The json5 lists states worst first; spoken, the best state comes first.
        CanBe::Labels(labels) => {
            assert!(!labels.is_empty(), "resource {:?} has no labels", resource.hold);
            let best_first: Vec<&str> = labels.iter().copied().rev().collect();
            join_with("or", &best_first)
        }
    }
}

/// `items` as a spoken list: "a", "a or b", "a, b, or c", with `conjunction` before the last.
pub fn join_with(conjunction: &str, items: &[impl AsRef<str>]) -> String {
    let items: Vec<&str> = items.iter().map(AsRef::as_ref).collect();
    match items.split_last() {
        None => String::new(),
        Some((only, [])) => (*only).to_string(),
        Some((last, [first])) => format!("{first} {conjunction} {last}"),
        Some((last, rest)) => format!("{}, {conjunction} {last}", rest.join(", ")),
    }
}

/// `description` with its one `{resource}` placeholder replaced by the resource's value.
///
/// # Panics
/// If the description has no placeholder or more than one.
pub fn expand_resource(description: &str, resource: &Resource) -> String {
    match description.matches(RESOURCE_PLACEHOLDER).count() {
        1 => description.replacen(RESOURCE_PLACEHOLDER, &resource_value(resource), 1),
        0 => panic!("no {RESOURCE_PLACEHOLDER} placeholder in {description:?}"),
        n => panic!("{n} {RESOURCE_PLACEHOLDER} placeholders in {description:?}"),
    }
}

/// The body line under a Move's heading naming its resource: "Keep fighting: 5".
pub fn move_resource_line(resource: &Resource) -> String {
    format!("{}: {}", resource.hold, resource_value(resource))
}

/// A gizmo's printed head, spoken for `phrase`: its diamonds, then the phrase. "◇ a lantern",
/// "◇◇ firkins", "Sword". The diamonds are left for [`speak_slots`], which also moves an
/// article in the phrase to the front.
pub fn gizmo_head(phrase: &str, gizmo: &GizmoFixed) -> String {
    let diamonds = gizmo.slots.diamonds();
    if diamonds.is_empty() { phrase.to_string() } else { format!("{diamonds} {phrase}") }
}

/// `description` with its resource, if it has one, spoken in place of the `{resource}`
/// placeholder; a description with no resource has no placeholder and is unchanged.
pub fn speak_resource(description: &str, resource: Option<&Resource>) -> String {
    match resource {
        Some(resource) => expand_resource(description, resource),
        None => description.to_string(),
    }
}

/// A gizmo's description with its resource, if any, spoken: ", iron (<em>close</em>, +1
/// damage)", "({resource} hours, …)" with the hours counted.
pub fn gizmo_body(gizmo: &GizmoFixed) -> String {
    speak_resource(gizmo.description, gizmo.resource.as_ref())
}

/// `head` and `body` as one line: a body that continues the head with a comma or a colon
/// follows it directly, any other body after a space, and an empty body leaves the head alone.
fn join_head_and_body(head: &str, body: &str) -> String {
    if body.is_empty() {
        head.to_string()
    } else if body.starts_with([',', ':']) {
        format!("{head}{body}")
    } else {
        format!("{head} {body}")
    }
}

/// `text` with each gizmo reference replaced by its gizmo's line, head and body, so that
/// "{a lantern|Lantern}" reads as the print's "◇ a lantern (5 hours, <em>close, area</em>)"
/// once [`speak_slots`] has spoken the diamonds. `gizmos` are the referenced gizmos in order
/// of appearance, as a possession's kit lists them.
///
/// # Panics
/// If `text` has a different number of references from `gizmos`.
pub fn expand_gizmos(text: &str, gizmos: &[&GizmoFixed]) -> String {
    let references = gizmo_references(text);
    assert_eq!(
        references.len(),
        gizmos.len(),
        "{text:?} has {} gizmo references but {} gizmos were supplied",
        references.len(),
        gizmos.len()
    );
    let mut spoken = String::with_capacity(text.len());
    let mut from = 0;
    for (reference, gizmo) in references.iter().zip(gizmos) {
        spoken.push_str(&text[from..reference.span.start]);
        spoken.push_str(&join_head_and_body(
            &gizmo_head(reference.spoken(), gizmo),
            &gizmo_body(gizmo),
        ));
        from = reference.span.end;
    }
    spoken.push_str(&text[from..]);
    spoken
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixed::{EmptyFull, SlotCount};
    use crate::keys::GizmoKey;

    #[test]
    fn a_run_of_one_or_two_diamonds_is_spoken_as_n_slot() {
        assert_eq!(speak_slots("◇ Sword, iron"), "1-slot Sword, iron");
        assert_eq!(speak_slots("malt, ◇◇ firkins, stills"), "malt, 2-slot firkins, stills");
    }

    #[test]
    fn an_article_after_the_run_moves_in_front_and_an_becomes_a() {
        assert_eq!(speak_slots("◇ a notebook"), "a 1-slot notebook");
        assert_eq!(speak_slots("◇◇ A shield, bearing"), "A 2-slot shield, bearing");
        assert_eq!(speak_slots("quills, ◇ an oboe, etc."), "quills, a 1-slot oboe, etc.");
        assert_eq!(speak_slots("◇◇ An anvil"), "A 2-slot anvil");
    }

    #[test]
    fn an_article_before_the_run_becomes_a() {
        assert_eq!(speak_slots("add an ◇◇ iron hauberk"), "add a 2-slot iron hauberk");
        assert_eq!(speak_slots("An ◇ anvil"), "A 1-slot anvil");
        assert_eq!(
            speak_slots("gloves, ◇ a boiled leather cuirass"),
            "gloves, a 1-slot boiled leather cuirass"
        );
    }

    #[test]
    fn text_with_no_diamonds_is_unchanged() {
        let text = "mark only 1 inventory slot (instead of 2). Also, an ☐ and a ▁▁▁▁";
        assert_eq!(speak_slots(text), text);
    }

    /// Lines from the json5 paired with their `markdown/word-slots/` reading.
    #[test]
    fn word_slots_lines_read_as_the_golden_files_say() {
        let pairs = [
            (
                "If you take this move at the start of play, add an ◇◇ iron hauberk, ◇◇ bronze cuirass, or ◇◇ scale coat to your inventory",
                "If you take this move at the start of play, add a 2-slot iron hauberk, 2-slot bronze cuirass, or 2-slot scale coat to your inventory",
            ),
            (
                "picks, files, snippers, wire, ◇ prybars, ◇ hacksaws, ◇ a lantern (○○○○○ hours, <em>close, area</em>), ◇ a grappling hook, etc.",
                "picks, files, snippers, wire, 1-slot prybars, 1-slot hacksaws, a 1-slot lantern (○○○○○ hours, <em>close, area</em>), a 1-slot grappling hook, etc.",
            ),
            ("◇◇ A shield, bearing ▁▁▁▁▁▁▁▁'s crest", "A 2-slot shield, bearing ▁▁▁▁▁▁▁▁'s crest"),
            ("◇ Crossbow (<em>far</em>, +1 damage)", "1-slot Crossbow (<em>far</em>, +1 damage)"),
        ];
        for (json5, golden) in pairs {
            assert_eq!(speak_slots(json5), golden);
        }
    }

    #[test]
    fn words_that_merely_start_with_an_article_are_not_articles() {
        assert_eq!(speak_slots("◇ and-iron"), "1-slot and-iron");
        assert_eq!(speak_slots("Aran ◇ spear"), "Aran 1-slot spear");
    }

    fn resource(hold: &'static str, can_be: CanBe) -> Resource {
        Resource { hold, can_be, start: EmptyFull::Empty }
    }

    #[test]
    fn a_max_resource_is_spoken_as_its_count() {
        assert_eq!(resource_value(&resource("Uses", CanBe::Max(3))), "3");
    }

    #[test]
    fn labelled_states_are_spoken_best_first_with_or_before_the_last() {
        let arrows = resource("Arrows", CanBe::Labels(&["all out", "low ammo", "plenty left"]));
        assert_eq!(resource_value(&arrows), "plenty left, low ammo, or all out");
        let two = resource("Arrows", CanBe::Labels(&["all out", "plenty left"]));
        assert_eq!(resource_value(&two), "plenty left or all out");
        let one = resource("Arrows", CanBe::Labels(&["some"]));
        assert_eq!(resource_value(&one), "some");
    }

    #[test]
    fn the_placeholder_expands_to_the_value_and_the_prose_keeps_its_label() {
        let uses = resource("Uses", CanBe::Max(3));
        assert_eq!(
            expand_resource("({resource} uses): each use produces valuables", &uses),
            "(3 uses): each use produces valuables"
        );
        let arrows = resource("Arrows", CanBe::Labels(&["all out", "low ammo", "plenty left"]));
        assert_eq!(
            expand_resource("(<em>far</em>, +1 damage; Arrows: {resource})", &arrows),
            "(<em>far</em>, +1 damage; Arrows: plenty left, low ammo, or all out)"
        );
    }

    #[test]
    #[should_panic(expected = "no {resource} placeholder")]
    fn a_description_without_a_placeholder_is_a_bug() {
        expand_resource("see back page. Stock: ○○○", &resource("Stock", CanBe::Max(3)));
    }

    #[test]
    #[should_panic(expected = "2 {resource} placeholders")]
    fn a_description_with_two_placeholders_is_a_bug() {
        expand_resource("{resource} and {resource}", &resource("Stock", CanBe::Max(3)));
    }

    #[test]
    fn a_moves_resource_is_a_hold_line() {
        assert_eq!(
            move_resource_line(&resource("Keep fighting", CanBe::Max(5))),
            "Keep fighting: 5"
        );
    }

    fn gizmo(
        name: &'static str,
        slots: SlotCount,
        description: &'static str,
        resource: Option<Resource>,
    ) -> GizmoFixed {
        GizmoFixed {
            key: GizmoKey::Lantern,
            name,
            material: None,
            piercing: None,
            slots,
            description,
            resource,
        }
    }

    #[test]
    fn a_reference_becomes_the_printed_line_with_its_diamonds_and_resource() {
        let lantern = gizmo(
            "Lantern",
            SlotCount::One,
            "({resource} hours, <em>close, area</em>)",
            Some(resource("Hours", CanBe::Max(5))),
        );
        let prybars = gizmo("Prybars", SlotCount::One, "", None);
        let picks = gizmo("Picks", SlotCount::Zero, "", None);
        let expanded = expand_gizmos(
            "{picks}, {prybars}, {a lantern|Lantern}, etc.",
            &[&picks, &prybars, &lantern],
        );
        assert_eq!(expanded, "picks, ◇ prybars, ◇ a lantern (5 hours, <em>close, area</em>), etc.");
        assert_eq!(
            speak_slots(&expanded),
            "picks, 1-slot prybars, a 1-slot lantern (5 hours, <em>close, area</em>), etc."
        );
    }

    #[test]
    fn a_body_that_starts_with_a_comma_or_colon_continues_the_head() {
        let sword = gizmo("Sword", SlotCount::One, ", iron (<em>close</em>, +1 damage)", None);
        assert_eq!(
            expand_gizmos("{Sword}", &[&sword]),
            "◇ Sword, iron (<em>close</em>, +1 damage)"
        );
        let pouch = gizmo("Sacred pouch", SlotCount::Zero, ": see back page", None);
        assert_eq!(expand_gizmos("{Sacred pouch}", &[&pouch]), "Sacred pouch: see back page");
    }

    #[test]
    fn a_resource_placeholder_is_not_a_reference_and_is_left_for_the_possession() {
        let whisky = gizmo("Whisky", SlotCount::Zero, "", None);
        assert_eq!(
            expand_gizmos("({resource} uses) of {whisky}", &[&whisky]),
            "({resource} uses) of whisky"
        );
    }

    #[test]
    #[should_panic(expected = "has 2 gizmo references but 1 gizmos were supplied")]
    fn a_kit_that_does_not_match_the_references_is_a_bug() {
        let awl = gizmo("Awl", SlotCount::Zero, "", None);
        expand_gizmos("{awl} and {awl}", &[&awl]);
    }
}
