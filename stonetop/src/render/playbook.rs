//! The walk of a `PlaybookFixed` into one Markdown document in the printed booklet's order,
//! with Special possessions ahead of Moves: H1 the playbook name, H2 each section, H3 each
//! background and each Move. Every piece of content
//! passes through the spoken forms in [`super::spoken`] and the converter in
//! [`super::markdown`]. The fixed prose here (section lead-ins, the Stats block, the blank
//! lines to fill in) is the Heavy's golden file's wording, shared by every playbook.

use crate::fixed::{
    BackgroundChecklist, BackgroundChunk, BackgroundFixed, BackstoryFixed, BackstoryItem,
    MoveChecklist, MoveFixed, Naming, Origin, PlaybookFixed, Requirement, SpecialPossessionFixed,
    TaggedRow,
};
use crate::keys::{MoveKey, PlaybookKey};

use super::markdown::html_to_markdown;
use super::spoken::{expand_resource, join_with, move_resource_line, speak_slots};

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

/// The document under construction: blocks separated by blank lines. A heading, a
/// paragraph, and a whole list are each one block.
#[derive(Default)]
struct Document {
    blocks: Vec<String>,
}

impl Document {
    fn block(&mut self, block: impl Into<String>) {
        self.blocks.push(block.into());
    }

    /// One block of lines, such as a list.
    fn lines(&mut self, lines: impl IntoIterator<Item = String>) {
        self.block(lines.into_iter().collect::<Vec<_>>().join("\n"));
    }

    fn finish(self) -> String {
        let mut text = self.blocks.join("\n\n");
        text.push('\n');
        text
    }
}

/// Content text as spoken Markdown: resource placeholders are the caller's business, since
/// only a possession has one; here slot diamonds become words and the HTML becomes Markdown.
fn speak(text: &str) -> String {
    html_to_markdown(&speak_slots(text))
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
                BackgroundChunk::Move(anonymous) => doc.block(speak(anonymous.description)),
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
        lines.push(format!("- {} {}", checkbox(preselected), possession_text(possession)));
        for option in possession.pick {
            lines.push(format!("  - {UNCHECKED} {}", speak(option)));
        }
    }
    lines.push(format!("- {UNCHECKED} {BLANK} (discuss with GM)"));
    doc.lines(lines);
}

/// "**Name:** description", except that a description beginning with a parenthesis or a
/// comma continues the name: "**Smithy** (or access to it): iron goods…".
fn possession_text(possession: &SpecialPossessionFixed) -> String {
    let name = speak(possession.name);
    let description = match &possession.resource {
        Some(resource) => expand_resource(possession.description, resource),
        None => possession.description.to_string(),
    };
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
    doc.block(speak(playbook.starting_moves_note));
    for a_move in playbook.moves {
        move_section(doc, a_move, playbook.grants_outright(a_move.name));
    }
    if let Some(footnote) = playbook.moves_footnote {
        doc.block(speak(footnote));
    }
}

fn move_section(doc: &mut Document, a_move: &MoveFixed, granted: bool) {
    doc.block(format!("### {} {}", checkbox(granted), speak(a_move.name)));
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn a_possession_whose_description_starts_with_a_parenthesis_has_no_colon() {
        let smithy = PlaybookKey::TheHeavy
            .fixed_part()
            .special_possessions
            .options
            .iter()
            .find(|possession| possession.name == "Smithy")
            .expect("the Heavy has a Smithy");
        assert!(possession_text(smithy).starts_with("**Smithy** (or access to it): iron goods"));
    }
}
