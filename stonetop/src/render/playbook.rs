//! The walk of a `PlaybookFixed` into one Markdown document in the printed booklet's order,
//! with Special possessions ahead of Moves: H1 the playbook name, H2 each section, H3 each
//! background and each Move. Every piece of content passes through the spoken forms in
//! [`super::spoken`] and the converter in [`super::markdown`], by way of
//! [`super::document::speak`]. The fixed prose here (section lead-ins, the Stats block, the
//! blank lines to fill in) is the Heavy's golden file's wording, shared by every playbook.

use crate::fixed::{
    BackgroundChecklist, BackgroundChunk, BackgroundFixed, BackstoryFixed, BackstoryItem,
    GizmoFixed, GizmoKit, MoveChecklist, MoveFixed, Naming, Origin, PlaybookFixed, Requirement,
    SpecialPossessionFixed, TaggedRow, gizmo_references,
};
use crate::keys::{MoveKey, PlaybookKey};

use super::document::{Document, speak};
use super::spoken::{
    expand_gizmos, gizmo_body, gizmo_head, join_with, move_resource_line, speak_resource,
};
use super::starting_moves;

const UNCHECKED: &str = "☐";
const CHECKED: &str = "☑︎";
/// A blank to fill in by hand: an instinct, a name, a possession.
const BLANK: &str = "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁";
/// A blank for a stat, a level, or the like.
const SHORT_BLANK: &str = "▁▁▁▁";

/// `playbook` as one Markdown document, ending in a newline.
#[must_use]
pub fn render_markdown(playbook: &PlaybookFixed) -> String {
    let mut doc = Document::default();
    doc.block(format!("# {}", speak(playbook.name)));
    backgrounds(&mut doc, playbook.backgrounds);
    instincts(&mut doc, playbook);
    appearance(&mut doc, &playbook.appearance);
    origins(&mut doc, playbook.origin_choices);
    doc.block("## I am called...");
    doc.block(BLANK);
    stats(&mut doc, playbook);
    special_possessions(&mut doc, playbook);
    moves(&mut doc, playbook);
    for backstory in playbook.backstory {
        backstory_section(&mut doc, backstory);
    }
    doc.block(format!("## {}", speak(playbook.intro.title)));
    doc.block(speak(playbook.intro.text));
    doc.finish()
}

/// Each of `texts` as spoken Markdown.
fn speak_all(texts: &[&str]) -> Vec<String> {
    texts.iter().map(|text| speak(text)).collect()
}

/// A list block with an unchecked box on every line: "- ☐ text".
fn unchecked_list(doc: &mut Document, texts: &[&str]) {
    doc.lines(speak_all(texts).into_iter().map(|text| format!("- {UNCHECKED} {text}")));
}

/// The items of a row as one line of boxes: "☐ young & brash ☐ in my prime ☐ old & leathery".
fn boxed_row(row: &TaggedRow) -> String {
    row.items
        .iter()
        .map(|item| format!("{UNCHECKED} {}", speak(item)))
        .collect::<Vec<_>>()
        .join(" ")
}

fn checkbox(checked: bool) -> &'static str {
    if checked { CHECKED } else { UNCHECKED }
}

fn backgrounds(doc: &mut Document, backgrounds: [&BackgroundFixed; 3]) {
    doc.block("## Background (Choose 1)");
    for background in backgrounds {
        doc.block(format!("### {UNCHECKED} {}", speak(background.name)));
        for chunk in background.description {
            match chunk {
                BackgroundChunk::Flavor(html) => doc.block(speak(html)),
                BackgroundChunk::Move(anonymous) => move_body(doc, anonymous),
                BackgroundChunk::Checklist(checklist) => background_checklist(doc, checklist),
            }
        }
    }
}

fn background_checklist(doc: &mut Document, checklist: &BackgroundChecklist) {
    match checklist {
        BackgroundChecklist::Options(options) => unchecked_list(doc, options),
        BackgroundChecklist::OptionsPrecheckable(options) => doc.lines(
            options
                .iter()
                .map(|option| format!("- {} {}", checkbox(option.prechecked), speak(option.text))),
        ),
        BackgroundChecklist::Rows(rows) => {
            doc.lines(rows.iter().map(|row| format!("- {}", boxed_row(row))));
        }
    }
}

fn instincts(doc: &mut Document, playbook: &PlaybookFixed) {
    doc.block("## Instinct (Choose 1)");
    let listed = playbook.instinct.iter().map(|instinct| {
        format!("- {UNCHECKED} **{}** — {}", speak(instinct.title), speak(instinct.description))
    });
    doc.lines(listed.chain([format!("- {UNCHECKED} {BLANK}")]));
}

fn appearance(doc: &mut Document, rows: &[TaggedRow]) {
    doc.block("## Appearance");
    doc.block("Choose 1 on each line, or make something up:");
    doc.lines(rows.iter().map(|row| format!("- {}", boxed_row(row))));
}

fn origins(doc: &mut Document, origins: &[Origin]) {
    doc.block("## Place of origin and name");
    doc.block(
        "Stonetop is your home, or close enough, but where are you (or your family) from \
         originally? Pick 1 and a name to match (or make up something similar).",
    );
    let mut lines = Vec::new();
    for origin in origins {
        let location = speak(origin.location);
        match &origin.naming {
            Naming::Instructions(text) => {
                lines.push(format!("- {UNCHECKED} **{location}:** {}", speak(text)));
            }
            Naming::Names(names) => {
                let names = join_with("or", &speak_all(names));
                lines.push(format!("- {UNCHECKED} **{location}:** {names}"));
            }
            Naming::MixAndMatch(parts) => {
                lines.push(format!("- {UNCHECKED} **{location}:** {}", speak(parts.intro)));
                for row in parts.name_parts {
                    lines.push(format!("  - {}", speak_all(row.items).join(", ")));
                }
            }
        }
    }
    doc.lines(lines);
}

fn stats(doc: &mut Document, playbook: &PlaybookFixed) {
    doc.block("## Stats");
    let scores: Vec<String> = playbook.stats_to_assign.iter().map(|n| format!("{n:+}")).collect();
    doc.block(format!(
        "Assign these scores: {}. When a debility is marked, you roll with disadvantage.",
        scores.join(", ")
    ));
    // The debility marks are fixed prose, not a Resource, so they keep the print glyph; the
    // views decide what a mark becomes, as they do for ☐.
    doc.lines([
        format!("- Str: {SHORT_BLANK} · Dex: {SHORT_BLANK} — ◯ weakened"),
        format!("- Int: {SHORT_BLANK} · Wis: {SHORT_BLANK} — ◯ dazed"),
        format!("- Con: {SHORT_BLANK} · Cha: {SHORT_BLANK} — ◯ miserable"),
    ]);
    doc.block(format!(
        "Damage: {} · HP (max {}): {SHORT_BLANK} · Armor: {SHORT_BLANK} · XP: {SHORT_BLANK} · \
         Level: {SHORT_BLANK}",
        playbook.damage, playbook.hp
    ));
}

fn special_possessions(doc: &mut Document, playbook: &PlaybookFixed) {
    let possessions = &playbook.special_possessions;
    doc.block(format!("## Special possessions ({})", speak(possessions.pick_note)));
    let mut lines = Vec::new();
    for (index, possession) in possessions.options.iter().enumerate() {
        let preselected = index < usize::from(possessions.preselected);
        let mut kit = KitCursor::of(possession);
        lines.push(format!(
            "- {} {}",
            checkbox(preselected),
            possession_text(possession, &mut kit)
        ));
        for option in possession.pick {
            lines.push(format!("  - {UNCHECKED} {}", speak(&kit.expand(option))));
        }
    }
    lines.push(format!("- {UNCHECKED} {BLANK} (discuss with GM)"));
    doc.lines(lines);
}

/// A cursor over a possession's kit, handing its gizmos out in order as the description and
/// then each pick entry are expanded, since the kit lists them in that order. A kit of one
/// has no references to expand, so its gizmo is never handed out.
struct KitCursor(std::vec::IntoIter<&'static GizmoFixed>);

impl KitCursor {
    fn of(possession: &SpecialPossessionFixed) -> Self {
        let gizmos: Vec<&'static GizmoFixed> =
            possession.kit.gizmos().iter().map(|key| key.fixed_part()).collect();
        Self(gizmos.into_iter())
    }

    /// `text` with its references expanded from the next gizmos of the kit.
    fn expand(&mut self, text: &str) -> String {
        let gizmos: Vec<&GizmoFixed> = self.0.by_ref().take(gizmo_references(text).len()).collect();
        expand_gizmos(text, &gizmos)
    }
}

/// "**Name:** description", except that a description beginning with a parenthesis or a
/// comma continues the name: "**Smithy** (or access to it): iron goods…". A kit of one is
/// its gizmo: the gizmo's diamonds go before the name and its description serves.
fn possession_text(possession: &SpecialPossessionFixed, kit: &mut KitCursor) -> String {
    let (name, description) = match possession.kit {
        GizmoKit::One(key) => {
            let gizmo = key.fixed_part();
            (gizmo_head(possession.name, gizmo), gizmo_body(gizmo))
        }
        GizmoKit::Referenced(_) => {
            let description = speak_resource(possession.description, possession.resource.as_ref());
            (possession.name.to_string(), kit.expand(&description))
        }
    };
    let name = speak(&name);
    let description = speak(&description);
    if description.starts_with('(') {
        format!("**{name}** {description}")
    } else if description.starts_with(',') {
        format!("**{name}**{description}")
    } else {
        format!("**{name}:** {description}")
    }
}

fn moves(doc: &mut Document, playbook: &PlaybookFixed) {
    doc.block("## Moves");
    doc.block(speak(&starting_moves::note(playbook, None)));
    for a_move in playbook.moves {
        move_section(doc, a_move, playbook.grants_outright(a_move.key));
    }
    if let Some(footnote) = playbook.moves_footnote {
        doc.block(speak(footnote));
    }
}

fn move_section(doc: &mut Document, a_move: &MoveFixed, granted: bool) {
    doc.block(format!("### {} {}", checkbox(granted), speak(a_move.name)));
    move_body(doc, a_move);
}

/// Everything under a Move's heading: the Requires, pick-count, and resource lines, the
/// description, then any checklist. A background's anonymous Move has no heading and
/// renders as this alone.
fn move_body(doc: &mut Document, a_move: &MoveFixed) {
    let mut lines = Vec::new();
    if !a_move.requires.is_empty() {
        lines.push(requires_line(a_move.requires));
    }
    if a_move.max_picks > 1 {
        lines.push(format!("Take up to {} times", a_move.max_picks));
    }
    lines.extend(a_move.resource.iter().map(move_resource_line));
    if !lines.is_empty() {
        // Hard breaks (two trailing spaces, Markdown's <br>), so a viewer keeps each on its
        // own line; only between lines, so no line ends in stray whitespace.
        doc.block(lines.join("  \n"));
    }
    doc.block(speak(a_move.description));
    match a_move.checklist {
        None => {}
        Some(MoveChecklist::Options(options)) => unchecked_list(doc, options),
        Some(MoveChecklist::OptionsWithLevel(options)) => doc.lines(
            options
                .iter()
                .map(|option| format!("- {UNCHECKED} {} (at level {SHORT_BLANK})", speak(option))),
        ),
    }
}

/// "(Requires level 2+ and the Heavy)": the requirements in order, joined with "and", with
/// any `Replaces` set off by a semicolon: "(Requires level 6+; replaces Bulwark)".
fn requires_line(requires: &[Requirement]) -> String {
    let mut needed = Vec::new();
    let mut replaced = Vec::new();
    for requirement in requires {
        match requirement {
            Requirement::Level(level) => needed.push(format!("level {level}+")),
            Requirement::NeedsPlaybook(playbook) => needed.push(playbook_reference(*playbook)),
            Requirement::Needs(key) => needed.push(move_name(*key)),
            Requirement::NeedsOneOf(a, b) => {
                needed.push(format!("{} or {}", move_name(*a), move_name(*b)));
            }
            Requirement::NeedsStrength => needed.push("Strength +2 or higher".to_string()),
            Requirement::NeedsSixInPotentialFG => {
                needed.push("all 6 marks in Potential for Greatness".to_string());
            }
            Requirement::Replaces(key) => replaced.push(format!("replaces {}", move_name(*key))),
        }
    }
    let mut line = String::from("(Requires ");
    line.push_str(&join_with("and", &needed));
    if !replaced.is_empty() {
        if !needed.is_empty() {
            line.push_str("; ");
        }
        line.push_str(&join_with("and", &replaced));
    }
    line.push(')');
    line
}

fn move_name(key: MoveKey) -> String {
    speak(key.fixed_part().name)
}

/// "the Heavy": the playbook's name with its leading "The" lowercased.
fn playbook_reference(playbook: PlaybookKey) -> String {
    let name = speak(playbook.fixed_part().name);
    match name.strip_prefix("The ") {
        Some(rest) => format!("the {rest}"),
        None => name,
    }
}

fn backstory_section(doc: &mut Document, backstory: &BackstoryFixed) {
    doc.block(format!("## {}", speak(backstory.name)));
    for item in backstory.list {
        match item {
            BackstoryItem::Text(html) => doc.block(speak(html)),
            BackstoryItem::Choices(choices) => unchecked_list(doc, choices),
            BackstoryItem::ChoiceRow(row) => doc.block(format!("- {}", boxed_row(row))),
            BackstoryItem::Heading(heading) => doc.block(format!("### {}", speak(heading))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keys::BackstoryKey;

    #[test]
    fn requirements_are_joined_with_and() {
        assert_eq!(requires_line(&[Requirement::Level(6)]), "(Requires level 6+)");
        assert_eq!(
            requires_line(&[
                Requirement::Level(2),
                Requirement::NeedsPlaybook(PlaybookKey::TheHeavy)
            ]),
            "(Requires level 2+ and the Heavy)"
        );
        assert_eq!(
            requires_line(&[
                Requirement::Level(6),
                Requirement::NeedsStrength,
                Requirement::NeedsSixInPotentialFG
            ]),
            "(Requires level 6+, Strength +2 or higher, and all 6 marks in Potential for Greatness)"
        );
    }

    #[test]
    fn a_needed_move_is_named_and_two_alternatives_are_joined_with_or() {
        assert_eq!(
            requires_line(&[Requirement::Needs(MoveKey::BattleJoy)]),
            "(Requires Battle Joy)"
        );
        assert_eq!(
            requires_line(&[Requirement::NeedsOneOf(MoveKey::BattleJoy, MoveKey::HardToKill)]),
            "(Requires Battle Joy or Hard to Kill)"
        );
    }

    #[test]
    fn a_replaced_move_is_set_off_by_a_semicolon() {
        assert_eq!(
            requires_line(&[Requirement::Level(6), Requirement::Replaces(MoveKey::Musclebound)]),
            "(Requires level 6+; replaces Musclebound)"
        );
    }

    #[test]
    fn a_backgrounds_anonymous_move_lists_its_checklist_after_its_description() {
        let ranger = PlaybookKey::TheRanger.fixed_part();
        let mut doc = Document::default();
        backgrounds(&mut doc, ranger.backgrounds);
        assert!(doc.finish().contains(
            "Mark 1 action at 1st level, then another at 3rd, 5th, 7th, and 9th.\n\n\
             - ☐ Gauge its distance and direction from you\n\
             - ☐ Call it back to your side\n"
        ));
    }

    #[test]
    fn a_backstory_subheading_is_an_h3_under_the_backstorys_h2() {
        let collection = BackstoryFixed {
            key: BackstoryKey::Collection,
            name: "Collection",
            list: &[
                BackstoryItem::Text("<p>You have acquired arcana.</p>"),
                BackstoryItem::Heading("Major Arcana"),
                BackstoryItem::Choices(&["Where did you acquire it?"]),
            ],
        };
        let mut doc = Document::default();
        backstory_section(&mut doc, &collection);
        assert_eq!(
            doc.finish(),
            "## Collection\n\nYou have acquired arcana.\n\n### Major Arcana\n\n\
             - ☐ Where did you acquire it?\n"
        );
    }

    #[test]
    fn a_possession_whose_description_starts_with_a_parenthesis_has_no_colon() {
        let smithy = PlaybookKey::TheHeavy
            .fixed_part()
            .special_possessions
            .options
            .iter()
            .find(|possession| possession.name == "Smithy")
            .expect("the Heavy has a Smithy");
        assert!(
            possession_text(smithy, &mut KitCursor::of(smithy))
                .starts_with("**Smithy** (or access to it): iron goods")
        );
    }

    fn possession(playbook: PlaybookKey, name: &str) -> &'static SpecialPossessionFixed {
        playbook
            .fixed_part()
            .special_possessions
            .options
            .iter()
            .find(|possession| possession.name == name)
            .unwrap_or_else(|| panic!("{playbook:?} has no {name}"))
    }

    #[test]
    fn a_kit_of_one_speaks_its_gizmos_slots_before_the_name_and_its_description_after() {
        let bow = possession(PlaybookKey::TheRanger, "Composite bow");
        assert_eq!(
            possession_text(bow, &mut KitCursor::of(bow)),
            "**1-slot Composite bow** (*far*, +1 damage, x piercing; Arrows: plenty left, low ammo, or all out)"
        );
        let pouch = possession(PlaybookKey::TheBlessed, "Sacred pouch (<em>magical</em>)");
        assert_eq!(
            possession_text(pouch, &mut KitCursor::of(pouch)),
            "**Sacred pouch (*magical*):** see back page. Stock: 3"
        );
    }

    #[test]
    fn a_kits_gizmos_are_spoken_with_their_slots_and_their_own_descriptions() {
        let kit = possession(PlaybookKey::TheFox, "Burglar's kit");
        assert_eq!(
            possession_text(kit, &mut KitCursor::of(kit)),
            "**Burglar's kit:** picks, files, snippers, wire, 1-slot prybars, 1-slot hacksaws, \
             a 1-slot lantern (5 hours, *close, area*), a 1-slot grappling hook, etc."
        );
    }

    #[test]
    fn pick_entries_take_their_gizmos_from_the_kit_after_the_description() {
        let token = possession(PlaybookKey::TheWouldBeHero, "Personal token, fraught with meaning");
        let mut kit = KitCursor::of(token);
        possession_text(token, &mut kit);
        assert_eq!(kit.expand(token.pick[0]), "◇◇ A shield, bearing ▁▁▁▁▁▁▁▁'s crest");
        assert_eq!(kit.expand(token.pick[1]), "◇ A wool cloak, woven just for you by ▁▁▁▁▁▁▁▁");
        assert_eq!(kit.expand(token.pick[2]), "A letter, spattered with tears & blood");
    }
}
