use crate::fixed::{BackgroundFixed, GrantMove, PlaybookFixed};

use crate::keys::MoveKey;

pub fn note(playbook: &PlaybookFixed, background: Option<&BackgroundFixed>) -> String {
    fn name(key: &MoveKey) -> &'static str {
        key.fixed_part().name
    }
    let background_grants: &[GrantMove] = background.map_or(&[], |b| b.grants_moves);
    let grants = playbook.grants_moves.iter().chain(background_grants);

    // If we don't know exact Moves granted, and we do know that at least one Background
    // might grant a move, set `from_background` to `Some(x)` where x is a String that
    // says what we do know.
    let from_background = if background.is_none() {
        let mut grant_count = Vec::new();
        for bkg in playbook.backgrounds {
            grant_count.push(bkg.grants_moves.len());
        }
        grant_count.sort();
        grant_count.dedup();
        match grant_count.len() {
            0 => None,
            1 if grant_count[0] == 0 => None,
            1 => Some(format!("{} from your Background", grant_count[0])),
            _ => Some("any moves from your Background".to_string()),
        }
    } else {
        None // we already know more exact information
    };

    // Soon we'll list Moves granted outright, then the choices; so separate them now
    let mut outright_grants = Vec::new();
    let mut pairs = Vec::new();
    for grant in grants {
        match grant {
            GrantMove::Simply(a_move) => outright_grants.push(a_move),
            GrantMove::ChooseOne(m1, m2) => pairs.push((m1, m2)),
        }
    }

    // Initialize `snippets` with the names of Moves granted outright
    let mut snippets: Vec<_> = outright_grants.iter().map(|g| name(g).to_string()).collect();

    // Add choices between pairs. Set `separator` to comma if there is at most one, semicolon
    // otherwise
    let mut separator = ", ";
    match pairs.len() {
        0 => {}
        1 => snippets.push(format!("either {} OR {}", name(pairs[0].0), name(pairs[0].1))),
        _ => {
            separator = "; ";
            snippets
                .extend(pairs.iter().map(|pair| format!("{} OR {}", name(pair.0), name(pair.1))));
        }
    }

    // If some or all of the backgrounds grant Moves, but we don't know which will be chosen, give
    // as much information as possible.
    if let Some(any) = from_background {
        snippets.push(any);
    }

    // If there is a further choice of Moves, describe it.
    match playbook.starting_move_choices {
        0 => {}
        n => snippets.push(format!("{n} of your choice")),
    };

    // Set `starting` to a properly punctuated list of Moves and descriptions.
    let starting = match snippets.len() {
        0 => "NO MOVES".to_string(), // Could happen if a future version allows user-defined Moves
        1 => snippets[0].clone(),
        _ => {
            let last = snippets.last_mut().unwrap();
            *last = ["and ", last].concat();
            snippets.join(separator)
        }
    };
    format!("You start with {starting}.")
}

#[cfg(test)]
mod test {
    use super::*;
    use BackgroundKey::*;
    use PlaybookKey::*;
    use stonetop::keys::{BackgroundKey, PlaybookKey};
    use test_case::test_case;

    #[test_case(TheBlessed, None,
    "You start with Spirit Tongue, Call the Spirits, 1 from your Background, and 1 of your choice.";
    "TheBlessed")]
    #[test_case(TheBlessed, Some(Initiate),
    "You start with Spirit Tongue, Call the Spirits, Rites of the Land, and 1 of your choice.";
    "TheBlessedInitiate")]
    #[test_case(TheFox, None,
    "You start with Ambush OR Skill at Arms; Danger Sense OR Perceptive; any moves from your Background; and 1 of your choice.";
    "TheFox")]
    #[test_case(TheFox, Some(TheNatural),
    "You start with Ambush OR Skill at Arms; Danger Sense OR Perceptive; and 1 of your choice.";
    "TheFoxNatural")]
    #[test_case(TheFox, Some(ALifeOfCrime),
    "You start with Ambush OR Skill at Arms; Danger Sense OR Perceptive; Burgle OR Light Fingers; and 1 of your choice.";
    "TheFoxLifeOfCrime")]
    #[test_case(TheHeavy, None,
    "You start with Dangerous, Hard to Kill, and either Armored OR Uncanny Reflexes.";
    "TheHeavy")]
    #[test_case(TheHeavy, Some(Sheriff),
    "You start with Dangerous, Hard to Kill, and either Armored OR Uncanny Reflexes.";
    "TheHeavySheriff")]
    #[test_case(TheJudge, None,
    "You start with Censure, Chronicler of Stonetop, and 2 of your choice.";
    "TheJudge")]
    #[test_case(TheJudge, Some(Prophet),
        "You start with Censure, Chronicler of Stonetop, and 2 of your choice.";
    "TheJudgeProphet")]
    #[test_case(TheLightbearer, None,
    "You start with Consecrated Flame, Invoke the Sun God, and 1 of your choice.";
    "TheLightbearer")]
    #[test_case(TheLightbearer, Some(AuspiciousBirth),
    "You start with Consecrated Flame, Invoke the Sun God, and 1 of your choice.";
    "TheLightbearerAuspicious")]
    #[test_case(TheMarshal, None,
    "You start with Crew, Logistics, any moves from your Background, and 1 of your choice.";
    "TheMarshal")]
    #[test_case(TheMarshal, Some(Scion),
    "You start with Crew, Logistics, Veteran Crew, and 1 of your choice.";
    "TheMarshalScion")]
    #[test_case(TheMarshal, Some(Penitent),
    "You start with Crew, Logistics, and 1 of your choice.";
    "TheMarshalPenitent")]
    #[test_case(TheMarshal, Some(Luminary),
    "You start with Crew, Logistics, We Happy Few, and 1 of your choice.";
    "TheMarshalLuminary")]
    #[test_case(TheRanger, None,
    "You start with Home on the Range, any moves from your Background, and 1 of your choice.";
    "TheRanger")]
    #[test_case(TheRanger, Some(MightyHunter),
    "You start with Home on the Range, Expert Tracker, Stalker, and 1 of your choice.";
    "TheRangerMightyHunter")]
    #[test_case(TheRanger, Some(WideWanderer),
    "You start with Home on the Range, Mental Map, and 1 of your choice.";
    "TheRangerWideWanderer")]
    #[test_case(TheRanger, Some(BeastBonded),
    "You start with Home on the Range, Animal Companion, and 1 of your choice.";
    "TheRangerBeastBonded")]
    #[test_case(TheSeeker, None,
    "You start with Well Versed, Work With What You've Got, and 1 from your Background.";
    "TheSeeker")]
    #[test_case(TheSeeker, Some(Antiquarian),
    "You start with Well Versed, Work With What You've Got, and Polyglot.";
    "TheSeekerAntiquarian")]
    #[test_case(TheWouldBeHero, None,
    "You start with Anger is a Gift, Potential for Greatness, and 2 of your choice.";
    "TheWouldBeHero")]
    #[test_case(TheWouldBeHero, Some(Driven),
    "You start with Anger is a Gift, Potential for Greatness, and 2 of your choice.";
    "TheWouldBeHeroDriven")]
    fn move_note(playbook: PlaybookKey, background: Option<BackgroundKey>, expected: &str) {
        assert_eq!(note(playbook.fixed_part(), background.map(|b| b.fixed_part())), expected)
    }
}
