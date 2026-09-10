//! Spoken forms of print glyphs: slot diamonds become "N-slot", and a resource's
//! `{resource}` placeholder becomes its count or its list of states.

use std::sync::LazyLock;

use regex_lite::{Captures, Regex};

use crate::fixed::{CanBe, Resource};

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

/// The marker in a possession's description where its resource's value is spoken.
const RESOURCE_PLACEHOLDER: &str = "{resource}";

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixed::EmptyFull;

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
}
