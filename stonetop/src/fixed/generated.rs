//! Baked Fixed content: written by `cargo xtask bake` from `codegen/json5/`, checked by
//! `codegen/tests/generated_fresh.rs`. Do not edit; change the json5 and run the command.
//!
//! The statics are grouped by playbook, a shared item appearing once under the first playbook
//! that uses it, as in `keys.rs`. Each playbook's static references its items' statics. Each
//! kind's `fixed_part()` matches every key to its static.

// The Blessed: Moves

static MOVE_FROM_RAISED_BY_WOLVES: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::FromRaisedByWolves,
    name: "From Raised by Wolves",
    description: "<p>When you <strong><em>Forage</em></strong>, you have advantage.</p><p>Once per session, when <strong><em>your wild ways offend or alienate you from someone</em></strong>, mark XP.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_FROM_VESSEL: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::FromVessel,
    name: "From Vessel",
    description: "<p>Danu's power flows through you, but at great cost. When you <strong><em>would spend 1 Stock from your sacred pouch</em></strong>, you may choose to lose 2d4 HP instead.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_AMULETS_TALISMANS: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::AmuletsTalismans,
    name: "Amulets & Talismans",
    description: "<p>When you <strong><em>craft a protective charm for someone</em></strong>, spend 1 Stock and name a source of harm (fire, stabbing, etc.). When they <strong><em>would suffer such harm while bearing your charm</em></strong>, roll +INT: <strong>on a 10+</strong>, they ignore the harm entirely; <strong>on a 7-9</strong>, they suffer only half the damage or effect; <strong>on a 6-</strong>, they suffer the harm normally. One can benefit from only 1 charm at a time, and it loses its potency after 1 use.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_BARKSKIN: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Barkskin,
    name: "Barkskin",
    description: "<p>When you <strong><em>are touching the earth</em></strong>, you have 2 armor. When you <strong><em>mark another with 1 Stock</em></strong>, they gain this benefit so long as the mark remains.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_BIG_MAGIC: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::BigMagic,
    name: "Big Magic",
    description: "<p>Each time you take this move, choose an additional remarkable trait for your sacred pouch and increase your max Stock by 2.</p>",
    requires: &[],
    max_picks: 2u8,
    resource: &[],
    checklist: None,
};

static MOVE_DANUS_GRASP: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::DanusGrasp,
    name: "Danu's Grasp",
    description: "<p>When you <strong><em>call on the world itself to bind a spirit or a perversion of nature</em></strong>, spend 1 Stock and roll +WIS: <strong>on a 7+</strong>, roots, vines, and earth pull at them, and they pick 1; <strong>on a 10+</strong>, as a 7-9, but both apply.</p><ul><li>They're restrained, unable to act freely until your focus slips or they tear their way free</li><li>They take 2d4 damage (ignores armor)</li></ul><p>If this brings them to 0 HP, they are pulled into the earth and bound in rune-etched stone.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_HEALERS_ARTS: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::HealersArts,
    name: "Healer's Arts",
    description: "<p>When <strong><em>someone Recovers under your care</em></strong>, they recover (extra) HP equal to your WIS. If you <strong><em>also spend 1 Stock</em></strong>, they heal an extra 5 HP and their wounds/injuries are stabilized.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_HEED_MY_WORDS: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::HeedMyWords,
    name: "Heed My Words",
    description: "<p>When you <strong><em>Persuade by talking sense or warning against foolishness</em></strong>, you have advantage.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_IMPROVED_STAT: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::ImprovedStat,
    name: "Improved Stat",
    description: "<p>Each time you take this move, increase one of your stats by 1 (to a max of +2).</p>",
    requires: &[],
    max_picks: 3u8,
    resource: &[],
    checklist: None,
};

static MOVE_INTO_THE_LIONS_DEN: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::IntoTheLionsDen,
    name: "Into the Lion's Den",
    description: "<p>When you <strong><em>approach a beast calmly and show no fear</em></strong>, it will not harm you (though it may threaten you and test your nerve). When you <strong><em>lay your hand gently upon a beast</em></strong>, it calms to your touch.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_LIGHTNING_ROD: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::LightningRod,
    name: "Lightning Rod",
    description: "<p>When you <strong><em>Defend while touching the earth</em></strong>, you can spend 1 Readiness to intercept a nearby magical attack and redirect it harmlessly into the ground.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_RITES_OF_THE_LAND: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::RitesOfTheLand,
    name: "Rites of the Land",
    description: "<p>Once per season, when you <strong><em>oversee the sacred rites</em></strong>, hold 1 Favor. If you also sacrifice 1 Surplus, hold 4 Favor instead. Spend Favor in lieu of Stock, 1-for-1.</p><p>When you <strong><em>publicly sacrifice something or someone much-loved</em></strong>, either clear a steading debility or gain advantage when the steading next rolls +Fortunes.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[stonetop::fixed::Resource {
        hold: "Favor",
        can_be: stonetop::fixed::CanBe::Max(4u8),
        start: stonetop::fixed::EmptyFull::Empty,
    }],
    checklist: None,
};

static MOVE_SPIRIT_TONGUE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::SpiritTongue,
    name: "Spirit Tongue",
    description: "<p>You can speak with beasts and spirits. You can always ask the GM, \"What spirits are active here?\" and get an honest answer.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_BORROW_POWER: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::BorrowPower,
    name: "Borrow Power",
    description: "<p>When <strong><em>a spirit or beast loans you power</em></strong>, ask the GM for one of its tags or moves. Store it in your pouch in place of 1 Stock. When you <strong><em>use the borrowed tag or move</em></strong>, roll +WIS: <strong>on a 10+</strong>, you do it and can use the power again; <strong>on a 7-9</strong>, you do it, but lose the power.</p>",
    requires: &[stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::SpiritTongue)],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_CALL_THE_SPIRITS: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::CallTheSpirits,
    name: "Call the Spirits",
    description: "<p>When you <strong><em>spend 1 Stock and perform a short rite</em></strong>, the spirit(s) of a place or object manifest and hear you out. What they do next is up to them.</p>",
    requires: &[stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::SpiritTongue)],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_TRACKLESS_STEP: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::TracklessStep,
    name: "Trackless Step",
    description: "<p>When you <strong><em>move through nature with care and patience</em></strong>, you make no sound, leave no trace and can ignore any hindering or treacherous terrain (briars, mire, scree, etc.). When you <strong><em>spend 1 Stock and mark others</em></strong>, they each gain this benefit so long as the mark remains. 1 Stock can mark a number of individuals up to your level +INT.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_VEIL: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Veil,
    name: "Veil",
    description: "<p>When you <strong><em>wrap yourself or another in a subtle veil</em></strong>, spend 1 Stock and choose 1:</p><ul><li>A type of being you name (including \"people\") will tend to ignore your presence</li><li>People will perceive you as someone else, though you must wear something of an individual's in order to impersonate them</li></ul><p>When <strong><em>your deception comes under scrutiny</em></strong>, roll +INT: <strong>on a 10+</strong>, the veil holds, and no one is the wiser; <strong>on a 7-9</strong>, the veil holds, but there is further scrutiny or a complication (GM's choice).</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_WARDS_BINDINGS: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::WardsBindings,
    name: "Wards & Bindings",
    description: "<p>When you <strong><em>mark a boundary with sacred signs</em></strong>, spend 1 Stock and describe who or what they affect (using no more words than your level). Also, choose whether the affected beings are repelled or trapped by the signs.</p><p>When <strong><em>your wards or bindings are first tested</em></strong>, roll +INT: <strong>on 10+</strong>, they will hold as long as the signs remain unmarred (and the affected creature can do nothing to affect them directly); <strong>on a 7-9</strong>, they hold for now but may be overcome through might or will.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_WILD_SOUL: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::WildSoul,
    name: "Wild Soul",
    description: "<p>Each time you take this move, gain a Ranger move of your choice for which you qualify. You can't pick Improved Stat or Superior Stat.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(2u8),
        stonetop::fixed::Requirement::NeedsPlaybook(stonetop::keys::PlaybookKey::TheBlessed),
    ],
    max_picks: 2u8,
    resource: &[],
    checklist: None,
};

static MOVE_NATURES_WRATH: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::NaturesWrath,
    name: "Nature's Wrath",
    description: "<p>Danu's Grasp gains the <em>area</em> tag and can affect any creature. A mortal reduced to 0 HP is subdued or killed (your choice) rather than bound in stone.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::DanusGrasp),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_POTENT_WORKINGS: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::PotentWorkings,
    name: "Potent Workings",
    description: "<p>When you <strong><em>craft a protective charm</em></strong>, you may spend 1 additional Stock to choose 1:</p><ul><li>Name an additional type of harm</li><li>On a 10+, the charm retains its potency</li></ul>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::AmuletsTalismans),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_SHARED_SOULS: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::SharedSouls,
    name: "Shared Souls",
    description: "<p>When you <strong><em>mark a beast with 1 Stock</em></strong>, you can direct its actions and perceive through its senses no matter the distance between you. Treat it as a follower with 3 Loyalty; when you spend its last Loyalty, the effect ends.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::IntoTheLionsDen),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_SUCK_THE_POISON_OUT: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::SuckThePoisonOut,
    name: "Suck the Poison Out",
    description: "<p>When you <strong><em>draw a malady from a patient's body, mind, or soul</em></strong>, spend 1 Stock and roll +WIS: <strong>on a 10+</strong>, you remove the malady and can safely discard it or store it in your sacred pouch (taking the space of 1 Stock) to study or inflict on another; <strong>on a 7-9</strong>, you remove it, but choose 1:</p><ul><li>Your patient suffers lingering harm or trauma</li><li>You suffer some of the malady's effects</li><li>It will be harmful and dangerous to discard</li></ul>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::HealersArts),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_SUPERIOR_STAT: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::SuperiorStat,
    name: "Superior Stat",
    description: "<p>Increase one of your stats by +1 (to a max of +3).</p>",
    requires: &[stonetop::fixed::Requirement::Level(6u8)],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_VOICE_OF_THE_EARTH_MOTHER: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::VoiceOfTheEarthMother,
    name: "Voice of the Earth Mother",
    description: "<p>When you <strong><em>speak on behalf of Danu</em></strong>, natural beasts and spirits of the wild respect your authority. All but the most headstrong will do as you command, even against their instincts.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::SpiritTongue),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

// The Fox: Moves

static MOVE_FROM_THE_NATURAL: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::FromTheNatural,
    name: "From The Natural",
    description: "<p>When you <strong><em>Seek Insight</em></strong>, you may roll +INT instead of +WIS and add \"What opportunity does no one else see?\" to the list of possible questions.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_FROM_THE_PRODIGAL_RETURNED: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::FromTheProdigalReturned,
    name: "From The Prodigal Returned",
    description: "<p>When you <strong><em>declare that you know someone outside of Stonetop</em></strong>, someone who can help, name them and roll +CHA: <strong>on a 10+</strong>, yeah, they can help (tell us why they're willing); <strong>on a 7-9</strong>, they can help but pick 1 from the list below; <strong>on a 6-</strong>, the GM chooses 1 and then some.</p><ul><li>They still hold a grudge</li><li>They're going to need something from you first</li><li>They swore off this sort of thing long ago</li><li>You can't exactly, y'know, trust them</li></ul>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_ALL_IN_THE_WRIST: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::AllInTheWrist,
    name: "All in the Wrist",
    description: "<p>Any knife or dagger gets the <em>thrown</em> tag in your hands. Also, you keep a few iron throwing blades (<em>near</em>) on you; they don't take up space in your inventory. Reset your ammo whenever you Outfit.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[stonetop::fixed::Resource {
        hold: "Throwing blades",
        can_be: stonetop::fixed::CanBe::Labels(&["out", "a few left", "plenty left"]),
        start: stonetop::fixed::EmptyFull::Full,
    }],
    checklist: None,
};

static MOVE_AMBUSH: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Ambush,
    name: "Ambush",
    description: "<p>When you <strong><em>get the drop on a nearby foe</em></strong>, you can deal your damage or opt to roll +DEX: <strong>on a 10+</strong>, deal your damage and pick 2; <strong>on a 7-9</strong>, deal damage and pick 1:</p><ul><li>Deal +1d4 damage</li><li>Stop them from making noise/raising an alarm</li><li>Slip away before they can react</li><li>Create an opportunity; you or an ally gains advantage on the next move to act on it</li></ul>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_BURGLE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Burgle,
    name: "Burgle",
    description: "<p>When you <strong><em>sneak off on your own into a dangerous place</em></strong>, roll +INT: <strong>on a 7+</strong>, you make it back, and the GM says where you got to and what you learned. Then, <strong>on a 10+</strong>, also pick 2; <strong>on a 7-9</strong>, also pick 1:</p><ul><li>You got away clean, rousing no suspicion</li><li>You swiped something valuable (GM's choice)</li><li>You set something up to exploit on your return</li><li>Ask a Seek Insight question about what you saw</li></ul><p><strong>On a 6-</strong>, you either make it back but with trouble in tow, or you're missing in action (your call).</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_CATLIKE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Catlike,
    name: "Catlike",
    description: "<p>When you <strong><em>carry a light load and act with care</em></strong>, you move silently. When you <strong><em>hide in shadows or darkness</em></strong>, you remain unseen until you draw attention to yourself, move positions, or attack.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_DABBLER: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Dabbler,
    name: "Dabbler",
    description: "<p>Each time you take this move, choose a move from the Heavy, Marshal, Ranger, or Seeker playbooks for which you otherwise qualify. (You can't take Improved Stat or Superior Stat.)</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(2u8),
        stonetop::fixed::Requirement::NeedsPlaybook(stonetop::keys::PlaybookKey::TheFox),
    ],
    max_picks: 3u8,
    resource: &[],
    checklist: None,
};

static MOVE_DANGER_SENSE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::DangerSense,
    name: "Danger Sense",
    description: "<p>You can always ask the GM, \"Is there an ambush or trap here?\" If they say \"yes,\" roll +INT: <strong>on a 10+</strong>, ask the GM both of the questions below; <strong>on a 7-9</strong>, ask 1; <strong>either way</strong>, gain advantage on your next roll to act on the answer(s).</p><ul><li>What will trigger the ambush or trap?</li><li>What will happen once it's triggered?</li></ul><p><strong>On a 6-</strong>, don't mark XP; you know there's a trap or ambush, but nothing bad happens just yet.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_FREE_RUNNING: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::FreeRunning,
    name: "Free Running",
    description: "<p>When you <strong><em>carry a light load and move with speed and grace</em></strong>, gain advantage on any move to surmount or bypass a physical obstacle.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_IRRESISTIBLE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Irresistible,
    name: "Irresistible",
    description: "<p>When you <strong><em>interact with someone</em></strong>, you can ask their player if they find you attractive and get an honest answer (usually \"yes\").</p><p>When you <strong><em>Persuade by using your considerable charms as leverage</em></strong>, you have advantage.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_LAUGH_AT_DANGER: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::LaughAtDanger,
    name: "Laugh at Danger",
    description: "<p>When you <strong><em>are about to roll +CON and you make a joke about the adversity you face</em></strong>, you can roll +CHA instead.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_LIGHT_FINGERS: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::LightFingers,
    name: "Light Fingers",
    description: "<p>When you <strong><em>perform sleight of hand on an unwary mark</em></strong>, you succeed and no one's the wiser. If you're being watched, roll +DEX: <strong>on a 10+</strong>, you succeed and no one's the wiser; <strong>on a 7-9</strong>, you succeed OR no one's the wiser (your choice).</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_PERCEPTIVE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Perceptive,
    name: "Perceptive",
    description: "<p>When you <strong><em>Seek Insight</em></strong>, you may ask 1 additional question. Even on a 6-, you can ask 1 question (though you might not like how you learn the answer).</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_RAPIER_WIT: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::RapierWit,
    name: "Rapier Wit",
    description: "<p>When you <strong><em>pierce an NPC's pride with a well-placed quip</em></strong>, they must do 1 (their choice):</p><ul><li>Attack, doing +1d4 damage if they hit but giving you advantage on your next roll against them</li><li>Stoop to your level and respond in kind</li><li>Spend a few moments fuming, sputtering, or controlling their temper</li></ul>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_SKILL_AT_ARMS: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::SkillAtArms,
    name: "Skill at Arms",
    description: "<p>When you <strong><em>wield a weapon with speed and grace</em></strong>, roll +DEX to Clash (instead of +STR).</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_PARRY_RIPOSTE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::ParryRiposte,
    name: "Parry & Riposte",
    description: "<p>When you <strong><em>Defend with a weapon that you can wield quickly</em></strong>, you can spend 1 Readiness to both halve an attack's effects/damage and strike back at the attacker (deal your damage with disadvantage), instead of spending 1 Readiness for each.</p>",
    requires: &[stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::SkillAtArms)],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_SILVER_TONGUED: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::SilverTongued,
    name: "Silver Tongued",
    description: "<p>When you <strong><em>use words to avoid suspicion or trouble</em></strong>, roll +CHA: <strong>on a 10+</strong>, hold 3 Nerve; <strong>on a 7-9</strong>, hold 1 Nerve. You may spend Nerve, 1-for-1, to:</p><ul><li>Move about or maneuver unchallenged</li><li>Withstand direct scrutiny or questioning</li><li>Direct suspicion or attention elsewhere</li></ul>",
    requires: &[],
    max_picks: 1u8,
    resource: &[stonetop::fixed::Resource {
        hold: "Nerve",
        can_be: stonetop::fixed::CanBe::Max(3u8),
        start: stonetop::fixed::EmptyFull::Empty,
    }],
    checklist: None,
};

static MOVE_UNDER_YOUR_SKIN: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::UnderYourSkin,
    name: "Under Your Skin",
    description: "<p>When you <strong><em>engage an NPC in conversation</em></strong>, you can ask the GM 1 of these and get an honest answer:</p><ul><li>What are they expecting me to do?</li><li>What, in general, are they trying to hide?</li><li>What do they want to happen?</li></ul>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_BATTLE_DANCER: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::BattleDancer,
    name: "Battle Dancer",
    description: "<p>When you <strong><em>roll +DEX to Clash</em></strong>, <strong>on a 12+</strong> you deal your damage, avoid your enemy's attack, and impress/embarrass/overawe your foes.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::SkillAtArms),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_CHEAP_SHOT: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::CheapShot,
    name: "Cheap Shot",
    description: "<p>When you <strong><em>Ambush with a</em></strong> <strong>hand</strong> <strong><em>weapon</em></strong>, you have advantage on your damage roll.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::Ambush),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_EYE_ON_THE_DOOR: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::EyeOnTheDoor,
    name: "Eye on the Door",
    description: "<p>When <strong><em>you and your allies need to get out of here</em></strong>, name your escape route and roll +INT: <strong>on a 10+</strong>, you're gone; <strong>on a 7-9</strong>, you can stay or go, but if you go, it costs you—the GM will tell you what (or who) you leave behind or take with you.</p>",
    requires: &[stonetop::fixed::Requirement::Level(6u8)],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_PANTS_ON_FIRE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::PantsOnFire,
    name: "Pants on Fire",
    description: "<p>When you <strong><em>Defy Danger, Persuade, or Interfere by being deceitful</em></strong>, you have advantage.</p><p>When another move (like Seek Insight) allows a player to ask you a question, you can opt not to answer.</p>",
    requires: &[stonetop::fixed::Requirement::Level(6u8)],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_SECOND_INTENT: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::SecondIntent,
    name: "Second Intent",
    description: "<p>When you <strong><em>Defend and spend 1 Readiness to Parry & Riposte</em></strong>, also pick 1 option from the Ambush list.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::ParryRiposte),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::Ambush),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_SLIPPERY: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Slippery,
    name: "Slippery",
    description: "<p>When you <strong><em>roll to escape being caught or controlled</em></strong>, treat a 6- as a 7-9. On a 12+, say how you turn the tables or use the circumstances to your advantage.</p>",
    requires: &[stonetop::fixed::Requirement::Level(6u8)],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

// The Heavy: Moves

static MOVE_FROM_SHERIFF: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::FromSheriff,
    name: "From Sheriff",
    description: "<p>When you <strong><em>bark an order or warning</em></strong>, roll +CHA: <strong>on a 7+</strong>, they must choose 1:</p><ul><li>Do what you say</li><li>Dig in/take cover/flee</li><li>Attack you</li></ul><p><strong>On a 10+</strong>, you can sense which one they're about to do and act first if you like; gain advantage if you do.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_FROM_BLOOD_SOAKED_PAST: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::FromBloodSoakedPast,
    name: "From Blood-Soaked Past",
    description: "<p>When you <strong><em>Persuade using violence or threats against someone who knows your black reputation</em></strong>, you can roll +STR instead of +CHA. Also, if you take the Formidable move, you can choose to roll +CON instead of +CHA.</p><p>When you <strong><em>fight to kill without mercy or hesitation</em></strong>, you deal +1d4 damage.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_ARMORED: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Armored,
    name: "Armored",
    description: "<p>When you <strong><em>carry a shield</em></strong>, mark only 1 inventory slot (instead of 2). Also, you can ignore the <em>cumbersome</em> tag on any armor you wear.</p><p>If you take this move at the start of play, add an ◇◇ iron hauberk, ◇◇ bronze cuirass, or ◇◇ scale coat to your inventory (all are 2 armor, <em>warm, cumbersome</em>).</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_BATTLE_JOY: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::BattleJoy,
    name: "Battle Joy",
    description: "<p>When you <strong><em>spill blood—yours or another's—and lose yourself in battle</em></strong>, you ignore fear, pain, mind-control, and the effects of debilities as long as you keep fighting.</p><p>When <strong><em>the action stops</em></strong>, roll +CON: <strong>on a 10+</strong>, that was a rush, regain 1d4 HP; <strong>on a 7-9</strong>, you're winded and out of it, but you'll be fine with a few minutes' rest; <strong>on a 6-</strong>, mark a debility but don't mark XP.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_BERSERKER: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Berserker,
    name: "Berserker",
    description: "<p>While <strong><em>in your Battle Joy</em></strong>, add the <em>area</em> tag to your melee attacks, lashing out at anyone nearby (friend and foe alike). Roll damage separately for each target.</p>",
    requires: &[stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::BattleJoy)],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_CARVED_OUT_OF_WOOD: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::CarvedOutOfWood,
    name: "Carved Out of Wood",
    description: "<p>Increase your max HP by 4.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_DANGEROUS: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Dangerous,
    name: "Dangerous",
    description: "<p>When you <strong><em>deal your damage</em></strong>, you have advantage.</p>",
    requires: &[stonetop::fixed::Requirement::NeedsPlaybook(stonetop::keys::PlaybookKey::TheHeavy)],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_FORMIDABLE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Formidable,
    name: "Formidable",
    description: "<p>When you <strong><em>wade into battle</em></strong>, you can choose to roll +CHA: <strong>on a 10+</strong>, both; <strong>on a 7-9</strong>, pick 1:</p><ul><li>Lesser foes will quail, hesitate, or flee before you.</li><li>Doughty foes will focus on you, seeing you as the greatest threat.</li></ul><p><strong>On a 6-</strong>, pick 1 but ask the GM what you've missed.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_FROSTY: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Frosty,
    name: "Frosty",
    description: "<p>When you <strong><em>Defy Danger by keeping calm and carrying on</em></strong>, <strong>on a 10+</strong> you can also ask the GM a question that you could ask when Seeking Insight. You have advantage on your next move to act on the answer.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_GUARDIAN: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Guardian,
    name: "Guardian",
    description: "<p>When you <strong><em>Defend</em></strong>, hold 1 extra Readiness. Even on a 6-, hold 1 Readiness (plus whatever the GM says).</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_INTIMIDATING: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Intimidating,
    name: "Intimidating",
    description: "<p>When you <strong><em>Persuade using violence or threats</em></strong>, you have advantage.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_HARD_TO_KILL: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::HardToKill,
    name: "Hard to Kill",
    description: "<p>When you <strong><em>are at Death's Door</em></strong>, you can roll +CON or +nothing (your choice). On a 7-9, you can mark a debility of your choice to regain 1 HP.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_UNSTOPPABLE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Unstoppable,
    name: "Unstoppable",
    description: "<p>When you <strong><em>are reduced to 0 HP in battle</em></strong>, you can keep fighting. Each time you take damage while at 0 HP, mark 1. If you would regain HP while fighting, clear one mark instead.</p><p>When you <strong><em>stop fighting</em></strong>, roll for Death's Door with a -1 penalty for each circle marked. If you survive, clear all your circles.</p>",
    requires: &[stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::HardToKill)],
    max_picks: 1u8,
    resource: &[stonetop::fixed::Resource {
        hold: "Keep fighting",
        can_be: stonetop::fixed::CanBe::Max(5u8),
        start: stonetop::fixed::EmptyFull::Empty,
    }],
    checklist: None,
};

static MOVE_MUSCLEBOUND: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Musclebound,
    name: "Musclebound",
    description: "<p>When you <strong><em>make a hand-to-hand or thrown attack</em></strong>, it's <em>forceful</em> and <em>messy</em>. If it would already be <em>forceful</em> and/or <em>messy</em>, it's even more so.</p>",
    requires: &[stonetop::fixed::Requirement::NeedsStrength],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_PAYBACK: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Payback,
    name: "Payback",
    description: "<p>When you <strong><em>deal damage to a foe that has harmed you or one of your allies</em></strong>, deal +1d4 damage.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_RELENTLESS: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Relentless,
    name: "Relentless",
    description: "<p>When you <strong><em>Clash and your foe survives</em></strong>, you gain advantage the next time you Clash with them.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_SEASONED_WARRIOR: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::SeasonedWarrior,
    name: "Seasoned Warrior",
    description: "<p>Take a move from the Fox, Marshal, Ranger, or Seeker playbooks, for which you otherwise qualify. You can pick from a different playbook each time. (You can't pick Improved Stat or Superior Stat.)</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(2u8),
        stonetop::fixed::Requirement::NeedsPlaybook(stonetop::keys::PlaybookKey::TheHeavy),
    ],
    max_picks: 3u8,
    resource: &[],
    checklist: None,
};

static MOVE_SITUATIONAL_AWARENESS: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::SituationalAwareness,
    name: "Situational Awareness",
    description: "<p>When you <strong><em>Seek Insight</em></strong>, add the following to the list of questions you can ask:</p><ul><li>Who or what here is the biggest threat?</li><li>What is my enemy's true position?</li><li>What here can I use as a weapon?</li></ul><p>When <strong><em>a fight breaks out</em></strong>, ask the GM 1 question that you could ask when Seeking Insight.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_UNCANNY_REFLEXES: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::UncannyReflexes,
    name: "Uncanny Reflexes",
    description: "<p>When you <strong><em>are unarmored and carrying a normal or light load</em></strong>, you impose disadvantage on any damage you take that you could dodge or roll with.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_UNFETTERED: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Unfettered,
    name: "Unfettered",
    description: "<p>When you <strong><em>are subject to physical or mental restraint</em></strong>, you may mark a debility to immediately break free of that restraint.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_TERROR_ON_THE_FIELD: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::TerrorOnTheField,
    name: "Terror on the Field",
    description: "<p>When you <strong><em>reduce a foe to 0 HP</em></strong>, describe how you take them out. If you fell them in a particularly brutal or impressive manner, their allies are impressed, dismayed, or frightened and respond accordingly.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_BRINGER_OF_RUIN: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::BringerOfRuin,
    name: "Bringer of Ruin",
    description: "<p>When you <strong><em>roll a 12+ to Clash and your foe survives</em></strong>, name something they possess (like their sword, their position, a limb, their dignity, etc.), but nothing that would kill them outright. Whatever you name, it is broken, shattered, lost. Tell us how.</p>",
    requires: &[stonetop::fixed::Requirement::Level(6u8)],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_CUT_FROM_GRANITE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::CutFromGranite,
    name: "Cut from Granite",
    description: "<p>Gain +1 armor (stacks with other sources) and increase your max HP by another 2 (+6 HP total).</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::CarvedOutOfWood),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_MIGHTY_THEWS: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::MightyThews,
    name: "Mighty Thews",
    description: "<p>When you <strong><em>perform a feat of extraordinary strength</em></strong> (bursting chains, smashing through a wall, heaving a boulder, etc.), you do it (OH YEAH!) but pick 1:</p><ul><li>It takes a while</li><li>You cause unwanted damage or harm</li><li>It takes a toll (mark a debility)</li></ul>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::Musclebound),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_NEMESIS: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Nemesis,
    name: "Nemesis",
    description: "<p>When you <strong><em>Clash and your foe survives</em></strong>, all of your future attacks against them do +1d6 damage.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::Relentless),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_STEADFAST_GUARDIAN: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::SteadfastGuardian,
    name: "Steadfast Guardian",
    description: "<p>While you <strong><em>hold Readiness (from Defend)</em></strong>, you can always suffer the damage/effects of an attack instead of your ward; no need to spend Readiness, you can just do it.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::Guardian),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_STONE_COLD: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::StoneCold,
    name: "Stone Cold",
    description: "<p>When you <strong><em>Defy Danger (or Struggle as One) by keeping calm and carrying on</em></strong>, treat a 6- as a 7-9.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::Frosty),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

// The Judge: Moves

static MOVE_FROM_LEGACY: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::FromLegacy,
    name: "From Legacy",
    description: "<p>When you <strong><em>Know Things about the people or history of Stonetop</em></strong>, you have advantage.</p><p>When you <strong><em>spend days, weeks, or months poring over the Chronicle</em></strong>, ask the GM a question, and the GM will tell you what you learn in that time.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_FROM_MISSIONARY: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::FromMissionary,
    name: "From Missionary",
    description: "<p>When you <strong><em>call upon the Judge of another steading for aid or information</em></strong>, they are oathbound to give it. You are likewise oathbound to support them.</p><p>You have an aviary in addition to your usual choice of special possessions. When you <strong><em>send a message via trained bird</em></strong>, as is the way of the Judges of your order, the GM will tell you if and when you receive a response, and what it says.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_FROM_PROPHET: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::FromProphet,
    name: "From Prophet",
    description: "<p>When you <strong><em>spend a few days communing with Aratis about a threat facing Stonetop or civilization as a whole</em></strong>, roll +WIS: <strong>on a 7+</strong>, Aratis reveals the course of action she would have you take; <strong>on a 10+</strong>, you also hold 2 Sanction. While <strong><em>acting on her orders</em></strong>, spend 1 Sanction to add +1 to a roll you just made.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[stonetop::fixed::Resource {
        hold: "Sanction",
        can_be: stonetop::fixed::CanBe::Max(2u8),
        start: stonetop::fixed::EmptyFull::Empty,
    }],
    checklist: None,
};

static MOVE_AEGIS_OF_FAITH: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::AegisOfFaith,
    name: "Aegis of Faith",
    description: "<p>When you <strong><em>wield a shield</em></strong>, it can turn away spells, magical effects, and insubstantial attacks as if they were physical blows.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_BEAR_WITNESS: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::BearWitness,
    name: "Bear Witness",
    description: "<p>When you <strong><em>speak the truth with conviction and candor</em></strong>, none can doubt you. They might deny what you say, but in their hearts they recognize the truth.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_BREAK_BREAD: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::BreakBread,
    name: "Break Bread",
    description: "<p>When you <strong><em>share a proper meal with someone and each of you eats their fill</em></strong>, each of you recovers 1d8 (extra) HP.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_BULWARK: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Bulwark,
    name: "Bulwark",
    description: "<p>When you <strong><em>Defend</em></strong>, you can spend 1 Readiness to stand fast, holding your position despite what befalls you.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_CENSURE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Censure,
    name: "Censure",
    description: "<p>When you <strong><em>first denounce an individual in your presence as an agent of chaos or anathema to civilization</em></strong>, they pick 1:</p><ul><li>They are ashamed, and act accordingly</li><li>They are doubtful, and hesitate, pause</li><li>They are afraid, and seek to escape</li><li>They are enraged, and lash out predictably (the next roll against them has advantage)</li></ul>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_CASTIGATE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Castigate,
    name: "Castigate",
    description: "<p>When you <strong><em>Censure someone</em></strong>, your voice deals 1d4 damage to them (<em>near, loud</em>, ignores armor).</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(2u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::Censure),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_CHRONICLER_OF_STONETOP: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::ChroniclerOfStonetop,
    name: "Chronicler of Stonetop",
    description: "<p>When you <strong><em>write up detailed session notes and share them with the other players</em></strong>, hold +1 Diligence.</p><p>You can spend 1 Diligence at any time to add +1 to a roll that you or a fellow player just made.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[stonetop::fixed::Resource {
        hold: "Diligence",
        can_be: stonetop::fixed::CanBe::Max(3u8),
        start: stonetop::fixed::EmptyFull::Empty,
    }],
    checklist: None,
};

static MOVE_FOR_THE_GREATER_GOOD: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::ForTheGreaterGood,
    name: "For the Greater Good",
    description: "<p>When you <strong><em>Persuade someone to act in defense of their community or civilization at large</em></strong>, you have advantage.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_HOUND_OF_ARATIS: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::HoundOfAratis,
    name: "Hound of Aratis",
    description: "<p>When you <strong><em>Seek Insight</em></strong>, you can always ask \"What here is tainted by chaos?\" for free, even on a 6-.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_LIKE_A_DOG_WITH_A_BONE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::LikeADogWithABone,
    name: "Like a Dog with a Bone",
    description: "<p>When you <strong><em>attack something you know to be tainted by chaos</em></strong>, deal +1d6 damage.</p>",
    requires: &[stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::HoundOfAratis)],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_KNOWLEDGE_IS_POWER: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::KnowledgeIsPower,
    name: "Knowledge is Power",
    description: "<p>When you <strong><em>roll 10+ to Know Things</em></strong>, you or an ally gain advantage on the next roll to act on what you learn.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_MANY_HANDS_MAKE_LIGHT_WORK: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::ManyHandsMakeLightWork,
    name: "Many Hands Make Light Work",
    description: "<p>When you <strong><em>jump in to help another character who just rolled</em></strong>, tell us how and ask the GM what else is required or what the consequences will be. If you accept, increase your ally's roll by +1.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_A_BUNDLE_OF_STICKS_UNBROKEN: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::ABundleOfSticksUnbroken,
    name: "A Bundle of Sticks Unbroken",
    description: "<p>When you <strong><em>Struggle as One</em></strong>, you and one ally of your choice have advantage.</p>",
    requires: &[stonetop::fixed::Requirement::Needs(
        stonetop::keys::MoveKey::ManyHandsMakeLightWork,
    )],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_THE_HAMMER_AND_THE_BOOK: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::TheHammerAndTheBook,
    name: "The Hammer and the Book",
    description: "<p>When you <strong><em>strike a thing of supernatural chaos</em></strong>, roll +WIS: <strong>on a 10+</strong>, deal your damage and choose 1 from the list below; <strong>on a 7-9</strong>, deal damage and choose 1, but you also expose yourself to harm or unwanted attention.</p><ul><li>Deal +1d6 damage</li><li>Ignore the thing's armor or other defenses</li><li>Suppress one of its unnatural powers</li><li>Force it from its host</li></ul>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_TRUTH_OR_CONSEQUENCES: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::TruthOrConsequences,
    name: "Truth or Consequences",
    description: "<p>When you <strong><em>look into someone's eyes and gaze upon their soul</em></strong>, you can ask their player, \"Are you lying or hiding something from me?\" and get an honest answer. If the answer is \"Yes,\" you have advantage on your next roll against them.</p><p>When you <strong><em>lie or otherwise deceive someone through words</em></strong>, you have disadvantage on your next roll against them.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_BINDING_ARBITRATION: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::BindingArbitration,
    name: "Binding Arbitration",
    description: "<p>When you <strong><em>bear witness to someone's promise or oath</em></strong>, henceforth you may ask their player if they have kept their word. They must answer honestly. The character need not be present. If <strong><em>they have broken their word</em></strong>, you gain advantage on all rolls against them until they admit their wrong and suffer an appropriate consequence (your call).</p>",
    requires: &[stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::TruthOrConsequences)],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_VISION_UNCLOUDED: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::VisionUnclouded,
    name: "Vision Unclouded",
    description: "<p>When you <strong><em>Seek Insight</em></strong>, you can always ask \"What here is hidden by illusion or magic?\" for free, even on a 6-.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_WELL_READ: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::WellRead,
    name: "Well-Read",
    description: "<p>When you <strong><em>name the source in which you read about the matter at hand</em></strong>, roll +WIS to Know Things instead of +INT.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_A_MIGHTY_RAMPART: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::AMightyRampart,
    name: "A Mighty Rampart",
    description: "<p>When you <strong><em>hold Readiness (from Defend)</em></strong>, you cannot be forced from your position. Also, you can spend 1 Readiness to completely ignore the effects/damage of an attack that you suffer.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Replaces(stonetop::keys::MoveKey::Bulwark),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_ARMISTICE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Armistice,
    name: "Armistice",
    description: "<p>When you <strong><em>approach an enemy to negotiate in good faith</em></strong>, they will at least hear you out. Even the most debased and savage foe will delay violence until you've had your say.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::BearWitness),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_CONDEMN: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Condemn,
    name: "Condemn",
    description: "<p>When you <strong><em>Censure someone</em></strong>, they are marked with a mystical brand that cannot be removed or hidden until you dismiss it. Any intelligent creature who sees the mark recognizes the bearer as an agent of chaos and anathema to civilization.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::Censure),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_PROCLAMATION: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Proclamation,
    name: "Proclamation",
    description: "<p>When you <strong><em>Censure</em></strong>, you may denounce a group or faction as long as you can clearly identify them. Apply the effects of Censure to every member of that group, regardless of distance.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::Condemn),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_MIRRORSHIELD: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Mirrorshield,
    name: "Mirrorshield",
    description: "<p>When you <strong><em>Defend with a shield</em></strong>, you can spend 1 Readiness to intercept a magical force and redirect it to a different target (or none).</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::AegisOfFaith),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_THE_TOWER_ETERNAL: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::TheTowerEternal,
    name: "The Tower Eternal",
    description: "<p>When you <strong><em>Defy Danger against magic</em></strong>, treat a result of 6- as a 7-9.</p>",
    requires: &[stonetop::fixed::Requirement::Level(6u8)],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

// The Lightbearer: Moves

static MOVE_FROM_AUSPICIOUS_BIRTH: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::FromAuspiciousBirth,
    name: "From Auspicious Birth",
    description: "<p>When <strong><em>one of your moves has you mark a debility</em></strong>, you may mark this background's circle instead, to no ill effect. Clear it when you Make Camp or Convalesce.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[stonetop::fixed::Resource {
        hold: "Avoid debility",
        can_be: stonetop::fixed::CanBe::Max(1u8),
        start: stonetop::fixed::EmptyFull::Empty,
    }],
    checklist: None,
};

static MOVE_FROM_ITINERANT_MYSTIC: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::FromItinerantMystic,
    name: "From Itinerant Mystic",
    description: "<p>When you <strong><em>go off a-wandering</em></strong>, hold 1 Enigma if you're gone for days, 2 if you're gone for weeks, or 3 if you're gone for months. At the very start of play, hold 3 Enigma. Spend Enigma 1-for-1 to:</p><ul><li>Return from your wandering exactly when and where you are needed, fully Outfitted</li><li>Know Things as if you rolled a 10+, drawing on what you learned while away</li><li>Have What You Need to produce an oddly specific yet mundane item of Value 1 or less</li></ul>",
    requires: &[],
    max_picks: 1u8,
    resource: &[stonetop::fixed::Resource {
        hold: "Enigma",
        can_be: stonetop::fixed::CanBe::Max(3u8),
        start: stonetop::fixed::EmptyFull::Full,
    }],
    checklist: None,
};

static MOVE_FROM_SOUL_ON_FIRE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::FromSoulOnFire,
    name: "From Soul on Fire",
    description: "<p>When you <strong><em>Persuade a group by preaching charity, mercy, and hope and roll a 7+</em></strong>, aside from the usual effect, choose 1:</p><ul><li>Your name and your message spread</li><li>Someone approaches you, now or later, eager to know more</li></ul>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_A_CANDLE_AGAINST_THE_DARK: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::ACandleAgainstTheDark,
    name: "A Candle Against the Dark",
    description: "<p>When you <strong><em>wield a holy light but go otherwise unarmed</em></strong>, you have 2 Armor.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_LUMINOUS_SHIELD: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::LuminousShield,
    name: "Luminous Shield",
    description: "<p>When you <strong><em>brandish a holy light to turn aside an attack against body, mind, or soul</em></strong>, roll +CHA: <strong>on a 10+</strong>, the attack is deflected and, if the attacker is in range of your light, they are briefly blinded; <strong>on a 7-9</strong>, the attack is deflected but your holy light flickers and dims, threatening to go out; <strong>on a 6-</strong>, your light snuffs out and the attack is unimpeded.</p>",
    requires: &[stonetop::fixed::Requirement::Needs(
        stonetop::keys::MoveKey::ACandleAgainstTheDark,
    )],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_ALL_IS_ILLUMINATED: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::AllIsIlluminated,
    name: "All is Illuminated",
    description: "<p>When you <strong><em>look closely on another and see their soul laid bare</em></strong>, roll +WIS: <strong>on a 10+</strong>, ask their player 1 question from the list below, plus \"And what would make them feel loved, beautiful, or worthy?\"; <strong>on a 7-9</strong>, ask them 1 question from the list. In any case, they must answer truthfully.</p><ul><li>Of what are they most ashamed?</li><li>What do they most desire or covet?</li><li>What hope have they abandoned?</li><li>Who or what is most precious to them?</li></ul>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_AND_BEHOLD_A_PALE_HORSE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::AndBeholdAPaleHorse,
    name: "And Behold a Pale Horse",
    description: "<p>When you <strong><em>spend the night gazing into a flame</em></strong>, ask the GM to reveal an impending doom or grim portent that will come to pass unless you intervene, and how you might yet do so.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_CONSECRATED_FLAME: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::ConsecratedFlame,
    name: "Consecrated Flame",
    description: "<p>When you <strong><em>whisper words of consecration to a flame</em></strong>, the flame casts a holy light. Holy light is uncomfortable for creatures of darkness to look upon, but does no true harm. The holy light lasts until the flame goes out or until you consecrate another flame, whichever comes first.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_FIRE_WITHIN: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::FireWithin,
    name: "Fire Within",
    description: "<p>When you <strong><em>are in darkness</em></strong>, you are able to see by the light of your inner fire. When you <strong><em>take damage from cold or fire</em></strong>, reduce that damage by 2.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_GUIDING_LIGHT: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::GuidingLight,
    name: "Guiding Light",
    description: "<p>When you <strong><em>lead one or more NPCs through danger</em></strong>, roll +CHA: <strong>on a 10+</strong>, you all make it through (Helior be praised); <strong>on a 7-9</strong>, the GM will tell you what's required to get everyone through safely.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_HELIORS_UNBLINKING_EYE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::HeliorsUnblinkingEye,
    name: "Helior's Unblinking Eye",
    description: "<p>When you <strong><em>stare into the sun long enough to lose your vision</em></strong>, name a person or place that you know and roll +WIS: <strong>on a 10+</strong>, you briefly glimpse your subject as if from a great height, and choose 2 from the list below; <strong>on a 7-9</strong>, you briefly glimpse your subject as if from a great height, and choose 1:</p><ul><li>The glimpse lasts as long as you wish</li><li>Your point of view shifts to very close range</li><li>You recover your vision quickly</li></ul>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_INVOKE_THE_SUN_GOD: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::InvokeTheSunGod,
    name: "Invoke the Sun God",
    description: "<p>When you <strong><em>imbue a holy light with Helior's power</em></strong>, choose an Invocation you know and roll +WIS: <strong>on a 10+</strong>, it works as described but you must choose 1 consequence from the list below; <strong>on a 7-9</strong>, it works as described, but you and the GM each choose 1.</p><ul><li>The Invocation has its <em>reduced</em> effect</li><li>The effort taxes you; mark a debility</li><li>The light is snuffed out when the Invocation is complete, its fuel consumed</li><li>You must bask in sunlight for an hour or so before using that Invocation again</li></ul><p>See the <strong>Invocations</strong> insert for details.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_KEEP_THE_HOME_FIRES_BURNING: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::KeepTheHomeFiresBurning,
    name: "Keep the Home-Fires Burning",
    description: "<p>When you <strong><em>build a camp fire and sprinkle it with ash from your own hearth</em></strong>, anyone who Makes Camp with you is free from nightmares or bad dreams and recovers (extra) HP equal to your CHA.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_LAMPLIGHTER: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Lamplighter,
    name: "Lamplighter",
    description: "<p>When you <strong><em>whisper to a flammable object</em></strong> (a torch, a wick, kindling, etc.), it ignites in moments.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_PIETY: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Piety,
    name: "Piety",
    description: "<p>When you <strong><em>spend at least an hour in proper worship to Helior</em></strong>, hold 1 Blessing. Other faithful PCs who partake in this worship also hold 1 Blessing. At any time, you can spend 1 Blessing to add +1 to a roll you just made in pursuit of a righteous cause.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[stonetop::fixed::Resource {
        hold: "Blessing",
        can_be: stonetop::fixed::CanBe::Max(1u8),
        start: stonetop::fixed::EmptyFull::Empty,
    }],
    checklist: None,
};

static MOVE_PURIFYING_FLAMES: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::PurifyingFlames,
    name: "Purifying Flames",
    description: "<p>When you <strong><em>wield a holy light against a creature of darkness</em></strong>, it counts as a weapon (d10 damage, <em>hand, close, area</em>, 2 piercing) and you can choose to roll +WIS to Clash.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_RADIANT_COUNTENANCE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::RadiantCountenance,
    name: "Radiant Countenance",
    description: "<p>When you <strong><em>give someone your fond attention</em></strong>, you can then Persuade them with advantage. If they are a follower, you can instead choose to Strengthen Your Bond (as if you paid their cost).</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_RISE_LIKE_THE_SUN: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::RiseLikeTheSun,
    name: "Rise Like the Sun",
    description: "<p>When you <strong><em>draw attention to yourself by word or deed</em></strong>, roll +CHA: <strong>on a 10+</strong>, everyone turns and looks, and you hold their gaze as long as you keep giving them reason to look; <strong>on a 7-9</strong>, everyone turns and looks.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_SPRINGS_FIRST_THAW: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::SpringsFirstThaw,
    name: "Spring's First Thaw",
    description: "<p>When you <strong><em>spend time (an hour at least, maybe more) seeking to stir hope, kindness, or mercy in an NPC</em></strong>, roll +CHA: <strong>on a 10+</strong>, you light a fire deep within them and effect a lasting change; <strong>on a 7-9</strong>, you kindle goodness in their heart for now, but they will eventually return to their old ways; <strong>on a 6-</strong>, their heart hardens and, whatever else the GM says, you can't use this move on them again.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_BURN_TWICE_AS_BRIGHT: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::BurnTwiceAsBright,
    name: "Burn Twice as Bright",
    description: "<p>When you <strong><em>Invoke the Sun God</em></strong>, you may mark a debility to use 2 Invocations at once. Roll once, and apply any consequences to both Invocations.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::InvokeTheSunGod),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_EMPOWERED_INVOCATIONS: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::EmpoweredInvocations,
    name: "Empowered Invocations",
    description: "<p>When you Invoke the Sun God, you can choose an extra consequence before you roll. If you do, the Invocation has its <em>empowered</em> effect.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::InvokeTheSunGod),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_GLORIOUS_SERVANT: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::GloriousServant,
    name: "Glorious Servant",
    description: "<p>When you <strong><em>Invoke the Sun God and roll a 10+</em></strong>, you need not choose a consequence; <strong>on a 7-9</strong>, you choose a consequence but the GM does not.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::InvokeTheSunGod),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_HUNGRY_FLAMES: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::HungryFlames,
    name: "Hungry Flames",
    description: "<p>When you <strong><em>deal damage with a holy light</em></strong>, you deal +1d6 damage and your target is engulfed in holy light and flames.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::PurifyingFlames),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_LIGHT_MORE_LIGHT: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::LightMoreLight,
    name: "Light, More Light",
    description: "<p>When you <strong><em>consecrate a flame</em></strong>, it burns brighter than normal. A rushlight or candle illuminates to <em>reach</em> range, an oil lamp, lantern, or torch out to <em>near</em> range, and a bullseye lantern out to <em>far</em> range.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::ConsecratedFlame),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_WIELDER_OF_THE_WHITE_FLAME: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::WielderOfTheWhiteFlame,
    name: "Wielder of the White Flame",
    description: "<p>When you <strong><em>channel Helior's essence into an object you carry</em></strong>, roll +WIS: <strong>on a 10+</strong>, it ignites with a white flame that casts a holy light (<em>reach, area</em>) and burns neither you nor the object, and you may Invoke the Sun God right now as if you rolled a 10+; <strong>on a 7-9</strong>, it ignites with a white flame that casts a holy light (<em>reach, area</em>) and burns neither you nor the object.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::InvokeTheSunGod),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

// The Marshal: Moves

static MOVE_FROM_PENITENT: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::FromPenitent,
    name: "From Penitent",
    description: "<p>When you <strong><em>draw on your bloody past to Know Things</em></strong>, you may roll +STR instead of +INT. If you do, the GM will ask you who you wronged back then or who might still hold a grudge. Answer them now.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_ARTS_OF_WAR: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::ArtsOfWar,
    name: "Arts of War",
    description: "<p>Take a move from the Fox, Heavy, Judge, Ranger, or Seeker playbooks, for which you otherwise qualify. You can pick from a different playbook each time. You can't take Improved Stat or Superior Stat.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(2u8),
        stonetop::fixed::Requirement::NeedsPlaybook(stonetop::keys::PlaybookKey::TheMarshal),
    ],
    max_picks: 2u8,
    resource: &[],
    checklist: None,
};

static MOVE_CREW: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Crew,
    name: "Crew",
    description: "<p>You've got a crew of stalwarts, six or so residents of Stonetop with some steel to them. See the Crew insert for details.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_VETERAN_CREW: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::VeteranCrew,
    name: "Veteran Crew",
    description: "<p>Each time you take this move, pick 1. You can also choose to reselect their Instinct and Cost.</p>",
    requires: &[stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::Crew)],
    max_picks: 2u8,
    resource: &[],
    checklist: Some(stonetop::fixed::MoveChecklist::Options(&[
        "Select 2 new tags for your Crew",
        "Increase their damage die from d6 to d8",
        "Increase their max HP by 2 each",
    ])),
};

static MOVE_FRONT_LINE_LEADER: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::FrontLineLeader,
    name: "Front Line Leader",
    description: "<p>When you <strong><em>lead your crew into battle</em></strong>, hold 2 Presence. Spend Presence in lieu of your crew's Loyalty or as Readiness (as if you Defended them).</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[stonetop::fixed::Resource {
        hold: "Presence",
        can_be: stonetop::fixed::CanBe::Max(2u8),
        start: stonetop::fixed::EmptyFull::Empty,
    }],
    checklist: None,
};

static MOVE_LOGISTICS: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Logistics,
    name: "Logistics",
    description: "<p>When you <strong><em>have a steading Muster or Pull Together</em></strong>, or when you <strong><em>Requisition</em></strong>, you have advantage.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_READ_THE_LAND: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::ReadTheLand,
    name: "Read the Land",
    description: "<p>When you <strong><em>first take a moment to survey the terrain</em></strong>, ask the GM one of the following; gain advantage on your next roll to act on the answer.</p><ul><li>What's the best way in, out, through, or past?</li><li>Where's the best spot for a trap or an ambush?</li><li>Where's the most defensible position?</li><li>What here is out of place?</li></ul>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_PREPARE_A_WELCOME: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::PrepareAWelcome,
    name: "Prepare a Welcome",
    description: "<p>When you <strong><em>have your allies fortify a position and lie in wait for battle</em></strong>, hold 1 Surprise if you're rushed or 2 Surprises if you can take your time.</p><p>Once battle is joined, spend 1 Surprise to reveal a ploy, defense, or dirty trick you prepared in advance and roll +INT: <strong>on a 10+</strong>, it works as well as can be expected, and you've still got a few tricks up your sleeve—regain 1 Surprise; <strong>on a 7-9</strong>, it works as well as can be expected.</p>",
    requires: &[stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::ReadTheLand)],
    max_picks: 1u8,
    resource: &[stonetop::fixed::Resource {
        hold: "Surprise",
        can_be: stonetop::fixed::CanBe::Max(2u8),
        start: stonetop::fixed::EmptyFull::Empty,
    }],
    checklist: None,
};

static MOVE_SET_UP_STRIKE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::SetUpStrike,
    name: "Set-Up Strike",
    description: "<p>When you <strong><em>Clash and get a 7+</em></strong>, you can choose to deal damage with disadvantage. If you do, you create an opening for an ally to act on, as if you provided Aid. Describe it!</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_SHAKE_IT_OFF: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::ShakeItOff,
    name: "Shake It Off",
    description: "<p>When you <strong><em>order an ally to overcome fear, pain, doubt, or delusion</em></strong>, roll +CHA: <strong>on a 10+</strong>, they do it; <strong>on a 7-9</strong>, a PC gets advantage to do it; an NPC will do it, but they'll need time, they'll resent you, or they'll feel humiliated (GM decides).</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_SHIELD_WALL: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::ShieldWall,
    name: "Shield Wall",
    description: "<p>When you <strong><em>have your crew form a shield wall</em></strong>, they Defend with advantage and <strong>on a 7+</strong> they hold +2 Readiness (instead of the usual +1 for shields). As long as they maintain formation, they can go on the offensive without losing Readiness.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_SIR_PERMISSION_TO_DIE_SIR: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::SirPermissionToDieSir,
    name: "Sir, Permission to Die, Sir",
    description: "<p>When <strong><em>one of your followers would die</em></strong>, you can spend 1 of their Loyalty to have them survive (out of the action, but alive). If you let them go, mark XP.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_SPEAK_SOFTLY: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::SpeakSoftly,
    name: "Speak Softly",
    description: "<p>When you <strong><em>offer peace but your enemy refuses</em></strong>, gain advantage on your next roll against them.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_STENTORIAN: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Stentorian,
    name: "Stentorian",
    description: "<p>When you <strong><em>raise your voice</em></strong>, it carries far and cuts through even the din of battle. When you <strong><em>go into battle</em></strong>, hold 2 Command. Spend 1 Command to shout an order or warning and pick 1:</p><ul><li>PCs get advantage on their next roll to do as you say</li><li>You have advantage to Order Followers or Deploy</li></ul>",
    requires: &[],
    max_picks: 1u8,
    resource: &[stonetop::fixed::Resource {
        hold: "Command",
        can_be: stonetop::fixed::CanBe::Max(2u8),
        start: stonetop::fixed::EmptyFull::Empty,
    }],
    checklist: None,
};

static MOVE_TAKE_THE_MEASURE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::TakeTheMeasure,
    name: "Take the Measure",
    description: "<p>When you <strong><em>size someone up</em></strong>, ask their player one of the questions below and get an honest answer. If they <strong><em>fear or respect you</em></strong> (their call), you can ask another question. You can't use this move on them again until your relationship significantly changes.</p><ul><li>Can I trust them (to ▁▁▁▁▁▁)?</li><li>What do they intend to do?</li><li>How are they most useful/dangerous?</li><li>What weakness of theirs can I exploit?</li></ul>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_WE_HAPPY_FEW: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::WeHappyFew,
    name: "We Happy Few",
    description: "<p>When you <strong><em>give an inspiring speech to your allies before facing a dire threat</em></strong>, roll +CHA: <strong>on a 10+</strong>, each ally holds 2 Inspiration; <strong>on a 7-9</strong>, each ally holds 1 Inspiration; <strong>on a 6-</strong>, each ally holds 1, but you have disadvantage on all rolls until you share your nagging doubts with someone else.</p><p>Once battle is joined, your allies can spend their Inspiration at any time, 1-for-1 to do the following:</p><ul><li>Act fearlessly in the face of terror or overwhelming odds</li><li>Keep 1 HP instead of being reduced to 0 HP</li><li>Add 1d6 to a damage roll they just made</li></ul>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_BATTLEFIELD_GRACE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::BattlefieldGrace,
    name: "Battlefield Grace",
    description: "<p>When you <strong><em>take damage while leading your allies in battle</em></strong>, the damage roll has disadvantage.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::FrontLineLeader),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_HEROES_TO_THE_LAST: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::HeroesToTheLast,
    name: "Heroes to the Last",
    description: "<p>Each time you take this move, pick 1:</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::VeteranCrew),
    ],
    max_picks: 2u8,
    resource: &[],
    checklist: Some(stonetop::fixed::MoveChecklist::Options(&[
        "They are <em>exceptional</em> (and roll +2 instead of +1)",
        "They are inured to terror & horror",
        "Increase their max HP by 4 each",
        "Increase their damage die one size (max d10)",
    ])),
};

static MOVE_FOCUS_FIRE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::FocusFire,
    name: "Focus Fire",
    description: "<p>You can spend 1 Command to order your allies to bring down a foe. If you do, each ally has advantage on their next damage roll against that foe.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::Stentorian),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_LIKE_AN_OPEN_BOOK: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::LikeAnOpenBook,
    name: "Like an Open Book",
    description: "<p>When you <strong><em>Take the Measure of someone who fears or respects you</em></strong>, your second question can be anything you want. The GM might ask how you could possibly know this; tell them or ask something else.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::TakeTheMeasure),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_NOBLE_MIEN: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::NobleMien,
    name: "Noble Mien",
    description: "<p>When you <strong><em>lead an NPC through danger and return them to safety</em></strong>, if they aren't part of your crew they will either offer to join your crew or pledge their future aid and support.</p>",
    requires: &[stonetop::fixed::Requirement::Level(6u8)],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_PEACE_THROUGH_STRENGTH: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::PeaceThroughStrength,
    name: "Peace Through Strength",
    description: "<p>When you <strong><em>stand ready to fight alongside like-minded allies</em></strong>, anything capable of fear recognizes you as a serious threat and treats you accordingly.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::SpeakSoftly),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

// The Ranger: Moves

static MOVE_FROM_WIDE_WANDERER: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::FromWideWanderer,
    name: "From Wide Wanderer",
    description: "<p>When you <strong><em>Know Things about the wider world</em></strong>, you can roll +WIS instead of +INT.</p><p>When you <strong><em>arrive somewhere you've visited before</em></strong> (your call), tell the GM when you were last here, and the GM will tell you how it's changed.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_FROM_BEAST_BONDED: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::FromBeastBonded,
    name: "From Beast-Bonded",
    description: "<p>When you <strong><em>focus on your animal companion for a few moments</em></strong>, you can use any of the actions you've marked below, no matter the distance between you. Mark 1 action at 1st level, then another at 3rd, 5th, 7th, and 9th.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: Some(stonetop::fixed::MoveChecklist::Options(&[
        "Gauge its distance and direction from you",
        "Call it back to your side",
        "Sense its emotional state",
        "Get a brief impression of what it senses",
        "Lend it your strength—lose 1d6 HP, and it regains an equal amount",
    ])),
};

static MOVE_A_SAFE_PLACE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::ASafePlace,
    name: "A Safe Place",
    description: "<p>When you <strong><em>select and prepare the party's camp site</em></strong>, hold 1 Precaution, or 2 Precaution if you are well-versed with this area and its dangers.</p><p>If trouble finds your camp site, you can spend 1 Precaution to reveal a simple defense, warning, or trick that you prepared in advance. If you do, tell us how you knew to make that specific preparation.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[stonetop::fixed::Resource {
        hold: "Precaution",
        can_be: stonetop::fixed::CanBe::Max(2u8),
        start: stonetop::fixed::EmptyFull::Empty,
    }],
    checklist: None,
};

static MOVE_ANIMAL_COMPANION: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::AnimalCompanion,
    name: "Animal Companion",
    description: "<p>You are accompanied by a beast of uncommon loyalty and cleverness. See the Animal Companion insert for details.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_MAGNIFICENT_SPECIMEN: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::MagnificentSpecimen,
    name: "Magnificent Specimen",
    description: "<p>Each time you take this move, your companion gains 2 additional options of your choice.</p>",
    requires: &[stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::AnimalCompanion)],
    max_picks: 2u8,
    resource: &[],
    checklist: None,
};

static MOVE_BIG_GAME_HUNTER: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::BigGameHunter,
    name: "Big Game Hunter",
    description: "<p>When you <strong><em>strike at the weak spot of a large or huge creature</em></strong>, you deal +2 damage.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_BLOT_OUT_THE_SUN: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::BlotOutTheSun,
    name: "Blot Out the Sun",
    description: "<p>When you <strong><em>Let Fly with a bow</em></strong>, you can deplete your ammunition (mark the next ammo status after your weapon) before you roll. If you do, choose 1:</p><ul><li>Gain advantage on your damage roll</li><li>Add the <em>area</em> tag to your attack; roll damage separately for each target</li></ul>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_CALL_THE_SHOT: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::CallTheShot,
    name: "Call the Shot",
    description: "<p>When you <strong><em>take your time and calmly line up the perfect shot</em></strong>, either deal your damage or roll +DEX: <strong>on a 10+</strong>, deal your damage and pick 2; <strong>on a 7-9</strong>, deal your damage and pick 1.</p><ul><li>Ignore armor or deal +1d4 damage (your call)</li><li>Stun, hobble, or hinder them</li><li>Make them trip or drop what they're holding</li><li>Do no harm; don't deal your damage after all</li></ul>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_EXPERT_TRACKER: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::ExpertTracker,
    name: "Expert Tracker",
    description: "<p>When you <strong><em>Seek Insight by searching for or studying the signs left by passing creatures</em></strong>, you can ask \"What happened here recently?\" for free, even on a 6-.</p><p>When you <strong><em>follow a creature's trail</em></strong>, roll +WIS: <strong>on a 7+</strong> you follow it to a significant change in terrain or activity; <strong>on a 10+</strong>, you can ask the GM a reasonable question about your quarry and get a useful answer.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_HOME_ON_THE_RANGE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::HomeOnTheRange,
    name: "Home on the Range",
    description: "<p>When <strong><em>a journey requires you to Defy Danger or Struggle as One</em></strong>, treat a 6- as a 7-9.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_MENTAL_MAP: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::MentalMap,
    name: "Mental Map",
    description: "<p>You can always retrace your steps and can accurately gauge distances and directions. You might not know the way forward but can always find your way back.</p><p>When you <strong><em>think back on a place you've been</em></strong>, you can Seek Insight retroactively, as if you were still there.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_NATURALIST: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Naturalist,
    name: "Naturalist",
    description: "<p>When you <strong><em>Know Things about beasts, natural environs, or spirits of the wild</em></strong>, you have advantage.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_ON_THE_HOOF: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::OnTheHoof,
    name: "On the Hoof",
    description: "<p>When you <strong><em>travel through the wilderness</em></strong>, you can procure 1d6 uses of ◇ provisions each day (roll with disadvantage in winter or barren terrain). Provisions can substitute for supplies when you Make Camp.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_PACK_HORSE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::PackHorse,
    name: "Pack Horse",
    description: "<p>You can carry up to 4 ◆ with a light load, 7 ◆ with a normal load, and 10 ◆ with a heavy load.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_PATHFINDER: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Pathfinder,
    name: "Pathfinder",
    description: "<p>When you <strong><em>lead your people to Pull Together or Deploy beyond sight of home</em></strong>, you have advantage.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_PREDATOR: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Predator,
    name: "Predator",
    description: "<p>When you <strong><em>Seek Insight</em></strong>, add the following to the list of questions you can ask. When <strong><em>acting on the answer to either question</em></strong>, deal an extra 1d4 damage.</p><ul><li>Who or what here is the easiest prey?</li><li>How is ▁▁▁▁▁▁▁▁ weak or vulnerable?</li></ul>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_SNIFF_OUT_CORRUPTION: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::SniffOutCorruption,
    name: "Sniff Out Corruption",
    description: "<p>When you <strong><em>Seek Insight</em></strong>, you can ask, \"What here stinks of the unnatural?\" for free, even on a 6-.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_STALKER: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Stalker,
    name: "Stalker",
    description: "<p>When you <strong><em>carry a normal or light load and move with care</em></strong>, you make no noise and leave no sign of your passing. When you <strong><em>hide yourself in a natural environment</em></strong>, you remain unseen until you draw attention to yourself, move positions, or attack.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_SURVIVALIST: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Survivalist,
    name: "Survivalist",
    description: "<p>When you <strong><em>Forage</em></strong>, pick 1 extra choice (even on a 6-, pick 1) and add \"Find or fashion some useful item or supply (GM can veto)\" to the list of options.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_WARDEN_OF_THE_WILD: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::WardenOfTheWild,
    name: "Warden of the Wild",
    description: "<p>When you <strong><em>defeat a perversion of nature</em></strong>, you can ask the GM 2 of the following and get a useful answer:</p><ul><li>Will it come back? If so, how can I stop it?</li><li>Will its taint spread? If so, how can I contain it?</li><li>What useful (but grisly) bits can I harvest?</li><li>What else can I learn about it or its ilk?</li></ul>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_WILD_SPEECH: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::WildSpeech,
    name: "Wild Speech",
    description: "<p>The grunts, barks, chirps, and calls of natural beasts are as a language to you. You can understand their intentions and communicate basic ideas. When you <strong><em>Persuade a beast</em></strong>, you can choose to roll +WIS.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_WORLDLY: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Worldly,
    name: "Worldly",
    description: "<p>Take a move from the Blessed, Fox, Heavy, Marshal, or Seeker playbooks, for which you otherwise qualify. You can pick from a different playbook each time. You can't pick Improved Stat or Superior Stat.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(2u8),
        stonetop::fixed::Requirement::NeedsPlaybook(stonetop::keys::PlaybookKey::TheRanger),
    ],
    max_picks: 2u8,
    resource: &[],
    checklist: None,
};

static MOVE_ALPHA: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Alpha,
    name: "Alpha",
    description: "<p>When you <strong><em>assert dominance over another (beast, spirit, Fae, or person)</em></strong>, roll +WIS: <strong>on a 7+</strong>, they must pick 1 from the list below; <strong>on a 10+</strong>, you also have advantage on your next roll against them.</p><ul><li>Accept your authority, at least for now</li><li>Slink away or flee, then avoid you</li><li>Fight you for dominance</li></ul>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::NeedsOneOf(
            stonetop::keys::MoveKey::WildSpeech,
            stonetop::keys::MoveKey::SpiritTongue,
        ),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_BEAST_OF_LEGEND: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::BeastOfLegend,
    name: "Beast of Legend",
    description: "<p>Each time you take this move, pick 1:</p><ul><li>They are <em>exceptional</em> (and roll +2 instead of +1)</li><li>They get +4 HP and +1 armor</li><li>They develop some unique ability or trait</li></ul>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::MagnificentSpecimen),
    ],
    max_picks: 2u8,
    resource: &[],
    checklist: None,
};

static MOVE_CONSTANT_VIGILANCE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::ConstantVigilance,
    name: "Constant Vigilance",
    description: "<p>Unless you're <em>dazed</em>, you're never caught off guard—not even when asleep or if you roll a 6-. When you <strong><em>intercept a sudden threat</em></strong> (to yourself or an ally), you have advantage on whatever move you make.</p>",
    requires: &[stonetop::fixed::Requirement::Level(6u8)],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_GIANT_SLAYER: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::GiantSlayer,
    name: "Giant Slayer",
    description: "<p>When you <strong><em>strike at a weak spot of a large or huge creature</em></strong>, you deal another +2 damage (+4 total).</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::BigGameHunter),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_TRAILBLAZER: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Trailblazer,
    name: "Trailblazer",
    description: "<p>When <strong><em>a journey causes you to Defy Danger or Struggle as One</em></strong>, on a 10+ you also learn or discover something interesting and useful—ask the GM what.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::HomeOnTheRange),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_WALK_IT_OFF: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::WalkItOff,
    name: "Walk It Off",
    description: "<p>When you'd <strong><em>mark a debility</em></strong>, you can mark this move instead to no ill effect. Clear it as you would a debility.</p>",
    requires: &[stonetop::fixed::Requirement::Level(6u8)],
    max_picks: 1u8,
    resource: &[stonetop::fixed::Resource {
        hold: "Avoid debility",
        can_be: stonetop::fixed::CanBe::Max(1u8),
        start: stonetop::fixed::EmptyFull::Empty,
    }],
    checklist: None,
};

// The Seeker: Moves

static MOVE_ATTUNED: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Attuned,
    name: "Attuned",
    description: "<p>When you <strong><em>Seek Insight</em></strong>, you can always ask, \"What here is infused with magic?\" for free, even on a 6-.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_CONDUIT_OF_POWER: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::ConduitOfPower,
    name: "Conduit of Power",
    description: "<p>When you <strong><em>would mark a Consequence from a major arcanum</em></strong>, you can mark 1 use here instead, with no negative effect. (There is no reset)</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[stonetop::fixed::Resource {
        hold: "Evade consequence",
        can_be: stonetop::fixed::CanBe::Max(3u8),
        start: stonetop::fixed::EmptyFull::Empty,
    }],
    checklist: None,
};

static MOVE_COUNTERMEASURES: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Countermeasures,
    name: "Countermeasures",
    description: "<p>When you <strong><em>witness a magical effect</em></strong>, you may ask the GM, \"How can this be countered or interrupted?\" and get an honest answer. You or an ally gain advantage on your next roll to act on the answer.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_EVERYTHING_BLEEDS: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::EverythingBleeds,
    name: "Everything Bleeds",
    description: "<p>When you <strong><em>exploit an unnatural foe's specific weakness or vulnerability</em></strong>, deal +1d6 damage.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_EVERYTHING_BURNS: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::EverythingBurns,
    name: "Everything Burns",
    description: "<p>When you <strong><em>inspect a work of artifice or magic for a fatal flaw</em></strong>, roll +INT: <strong>on a 7+</strong>, the GM will reveal the best way to destroy/sabotage it; <strong>on a 10+</strong>, you or an ally also gain advantage to act on the info.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_INITIATE_OF_THE_SECRET_ARTS: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::InitiateOfTheSecretArts,
    name: "Initiate of the Secret Arts",
    description: "<p>You have a \"Sacred Pouch\" (3 Stock, <em>magical</em>), as per the Blessed, but with no remarkable traits. Each time you take this move, choose a Blessed move for which you otherwise qualify. (You can't take Improved Stat or Superior Stat.)</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(2u8),
        stonetop::fixed::Requirement::NeedsPlaybook(stonetop::keys::PlaybookKey::TheSeeker),
    ],
    max_picks: 3u8,
    resource: &[],
    checklist: None,
};

static MOVE_LETS_MAKE_A_DEAL: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::LetsMakeADeal,
    name: "Let's Make a Deal",
    description: "<p>When you <strong><em>Seek Insight</em></strong>, add \"What do they really want or need?\" to the list of questions. When you <strong><em>Persuade by offering them something that you know they want or need</em></strong>, treat a 7-9 as a 10+.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_LOGBOOK: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Logbook,
    name: "Logbook",
    description: "<p>You have a logbook (2 uses, <em>slow</em>) that doesn't take up space in your inventory. When you (and only you) <strong><em>consult your logbook and expend a use</em></strong>, you can ignore a Know Things roll you just made and treat the result as a 10+. When <strong><em>the Seasons Change</em></strong>, reset your logbook to 2 uses.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[stonetop::fixed::Resource {
        hold: "Uses",
        can_be: stonetop::fixed::CanBe::Max(2u8),
        start: stonetop::fixed::EmptyFull::Empty,
    }],
    checklist: None,
};

static MOVE_MAGPIE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Magpie,
    name: "Magpie",
    description: "<p>When you <strong><em>Have What You Need</em></strong>, you can produce something strange, specific, maybe even valuable or a little bit magical, but if you do, tell us where you got it and 2 of the following:</p><ul><li>How it's not quite right, but maybe it'll do?</li><li>The trouble you caused back home by getting it</li><li>Why using it will draw unwanted attention</li><li>That it's the only thing like this that you've got, and why it'll only work the one time</li></ul>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_NEVER_AT_A_LOSS: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::NeverAtALoss,
    name: "Never at a Loss",
    description: "<p>When you <strong><em>Know Things and roll a 6-</em></strong>, you may choose to not mark XP. If you don't mark XP, the worst that happens is that the GM tells you nothing interesting or useful about the subject, but instead tells you how you could learn more.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_POLYGLOT: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Polyglot,
    name: "Polyglot",
    description: "<p>When you <strong><em>first encounter a living language in play</em></strong>, describe your proficiency with it (if any) and how you came to acquire it.</p><p>When you <strong><em>Know Things about any script, text, runes or symbols that you encounter</em></strong>, you have advantage.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_CRYPTOLOGIST: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Cryptologist,
    name: "Cryptologist",
    description: "<p>When you <strong><em>study encoded, forgotten, or arcane marks or writing</em></strong>, roll +INT: <strong>on a 10+</strong>, you can fully decipher them in just a few minutes; <strong>on a 7-9</strong>, you get the gist in a few minutes, but fully deciphering them will take you an hour or so.</p>",
    requires: &[stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::Polyglot)],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_QUICK_STUDY: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::QuickStudy,
    name: "Quick Study",
    description: "<p>When you <strong><em>study something magical that should take months to understand</em></strong>, it instead takes mere weeks. If it should take weeks, it takes days. If it should take days, it takes only a few hours.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_SAFETY_FIRST: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::SafetyFirst,
    name: "Safety First",
    description: "<p>When you <strong><em>spend an hour or so preparing your mystical defenses</em></strong>, hold 2 Protection. When you <strong><em>are affected by harmful magic</em></strong>, spend 1 Protection either to gain advantage on any roll to resist it or to halve its damage/effects.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[stonetop::fixed::Resource {
        hold: "Protection",
        can_be: stonetop::fixed::CanBe::Max(2u8),
        start: stonetop::fixed::EmptyFull::Empty,
    }],
    checklist: None,
};

static MOVE_SAGE_ADVICE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::SageAdvice,
    name: "Sage Advice",
    description: "<p>When <strong><em>another PC asks you for guidance</em></strong>, they get advantage on their next roll to follow your advice.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_WELL_VERSED: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::WellVersed,
    name: "Well Versed",
    description: "<p>Mark 1 topic, in addition to the one noted in your Background. Each additional time you take this move, mark 2 more topics.</p><p>When you <strong><em>Know Things about one of your topics</em></strong>, you can ask the GM a follow-up question of your choice (even on a 6-).</p>",
    requires: &[],
    max_picks: 3u8,
    resource: &[],
    checklist: Some(stonetop::fixed::MoveChecklist::Options(&[
        "The Last Door, death, and the undead",
        "The civilizations of humanity",
        "The Fae and their strange ways",
        "The Makers and their arts",
        "The primordial powers",
        "The Things Below",
        "The wild world and its spirits",
    ])),
};

static MOVE_WORK_WITH_WHAT_YOUVE_GOT: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::WorkWithWhatYouveGot,
    name: "Work With What You've Got",
    description: "<p>When you <strong><em>cleverly use your environment to harm or impede your foe(s)</em></strong>, roll +INT: <strong>on a 10+</strong>, pick 2; <strong>on a 7-9</strong>, pick 1:</p><ul><li>Interrupt or thwart their action(s)</li><li>Create an opportunity that grants you or an ally advantage on the next roll to exploit it</li><li>Deal damage appropriate to the source (d4 for bruises/scrapes, d6 for bloodshed, d8 if it'd break bones, d10 if it'd kill a common person)</li></ul>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_ARCANE_ADEPT: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::ArcaneAdept,
    name: "Arcane Adept",
    description: "<p>When you <strong><em>wish to invent a spell or magical effect</em></strong>, detail its workings with the GM and Make a Plan to invent it. If you like, pick one requirement and ask the GM to provide an alternative (for example \"first you must ▁▁▁▁▁▁▁▁\" could become \"first you must ▁▁▁▁▁▁▁▁, or it will take months\").</p>",
    requires: &[stonetop::fixed::Requirement::Level(6u8)],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_DEEP_INSIGHT: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::DeepInsight,
    name: "Deep Insight",
    description: "<p>When you <strong><em>Seek Insight about something magical</em></strong>, you may ask one additional question, not limited to the list. Even on a 6-, you get to ask this question.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::Attuned),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_IMPROVISE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Improvise,
    name: "Improvise",
    description: "<p>When you <strong><em>wish to use an arcanum's move or option without having unlocked it</em></strong>, ask the GM what fool risk(s) it requires and/or what consequence(s) you'll incur. If you go for it, roll +INT: <strong>on a 7+</strong>, you get it to work this once—trigger the move or use the option as if you'd unlocked it; and <strong>on a 10+</strong>, also mark one step towards unlocking the arcanum's mysteries.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::QuickStudy),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_MIND_OVER_MAGIC: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::MindOverMagic,
    name: "Mind Over Magic",
    description: "<p>When you <strong><em>roll to study or use an arcanum</em></strong>, you can roll +INT instead of the stat you'd normally roll.</p>",
    requires: &[stonetop::fixed::Requirement::Level(6u8)],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_OVERCHANNEL: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Overchannel,
    name: "Overchannel",
    description: "<p>When you <strong><em>would mark a Consequence from a major arcanum</em></strong>, you may mark a debility instead.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::ConduitOfPower),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_PROOF_AGAINST_DETECTION: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::ProofAgainstDetection,
    name: "Proof Against Detection",
    description: "<p>When you <strong><em>hold Protection</em></strong>, you can't be scried upon or sensed by magical means, and have advantage to Defy Danger by being stealthy.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::SafetyFirst),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

// The Would-be Hero: Moves

static MOVE_FROM_IMPETUOUS_YOUTH: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::FromImpetuousYouth,
    name: "From Impetuous Youth",
    description: "<p>When you <strong><em>make a move and come up short</em></strong>, you can give it your all and turn a 6- into a 7-9, a 7-9 into a 10+, and (if it matters), a 10-11 into a 12+. But if you do, pick 1 (the GM will fill in the details):</p><ul><li>You get hurt (2d4 damage and an actual injury)</li><li>You cause collateral damage, endanger others, or otherwise escalate the situation</li><li>Something on your person is lost or breaks</li></ul>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_FROM_DRIVEN: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::FromDriven,
    name: "From Driven",
    description: "<p>You always have the option to Burn Brightly; you can spend 2 XP after you roll to add +1, even if you don't have enough XP to level.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_FROM_DESTINED: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::FromDestined,
    name: "From Destined",
    description: "<p>At <strong><em>the start of a session</em></strong>, roll +Omens: <strong>on a 7+</strong>, lose all Omens and the GM will describe a vision or portent that points toward your fate and/or clarifies your current situation; also, <strong>on a 10+</strong>, ask the GM a follow-up question and get a clear, helpful answer; <strong>on a 6-</strong>, don't mark XP, hold +1 Omen, and tell us of your recent nightmares or a troubling vision, and how your fears play into them.</p><p><strong><em>Until your destiny is fulfilled</em></strong>, treat a 6- on Death's Door as a 7-9, and a 7-9 as a 10+.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[stonetop::fixed::Resource {
        hold: "Omens",
        can_be: stonetop::fixed::CanBe::Max(3u8),
        start: stonetop::fixed::EmptyFull::Empty,
    }],
    checklist: None,
};

static MOVE_ANGER_IS_A_GIFT: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::AngerIsAGift,
    name: "Anger is a Gift",
    description: "<p>When you <strong><em>burn with righteous anger</em></strong> (see Fear & Anger on back of playbook), hold 2 Resolve. You can spend your Resolve 1-for-1 to:</p><ul><li>Set aside fear and doubt to do what must be done</li><li>Act suddenly, catching them off-guard</li><li>Inspire allies or bystanders to follow your lead</li><li>Strike hard (+1d4 damage, <em>forceful</em>)</li><li>Keep your footing, position, and/or your course despite what befalls you</li></ul>",
    requires: &[],
    max_picks: 1u8,
    resource: &[stonetop::fixed::Resource {
        hold: "Resolve",
        can_be: stonetop::fixed::CanBe::Max(2u8),
        start: stonetop::fixed::EmptyFull::Empty,
    }],
    checklist: None,
};

static MOVE_SPEAK_TRUTH_TO_POWER: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::SpeakTruthToPower,
    name: "Speak Truth to Power",
    description: "<p>When you <strong><em>demand that someone does what is clearly good and right</em></strong>, you have advantage to Persuade. If they refuse, gain +1 Resolve.</p>",
    requires: &[stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::AngerIsAGift)],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_BETTER_PART_OF_VALOR: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::BetterPartOfValor,
    name: "Better Part of Valor",
    description: "<p>When you <strong><em>are outnumbered or facing a foe bigger than you</em></strong>, you have advantage to hide from, escape from, or sneak past them.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_I_GET_KNOCKED_DOWN: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::IGetKnockedDown,
    name: "I Get Knocked Down",
    description: "<p>When you <strong><em>take damage despite your best efforts to avoid it</em></strong>, you can choose to halve the damage but pick 1 of the following:</p><ul><li>You lose something (footing, grip, etc.)</li><li>Something on your person breaks</li><li>You're out of it for a moment</li></ul><p>Whatever you choose, the GM will describe the details.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_BUT_I_GET_UP_AGAIN: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::ButIGetUpAgain,
    name: "But I Get Up Again",
    description: "<p>When you <strong><em>use I Get Knocked Down</em></strong>, you have advantage on your next roll against whatever dealt the damage and your next blow against them does +1d4 damage.</p>",
    requires: &[stonetop::fixed::Requirement::Needs(stonetop::keys::MoveKey::IGetKnockedDown)],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_IN_OVER_YOUR_HEAD: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::InOverYourHead,
    name: "In Over Your Head",
    description: "<p>When <strong><em>another PC rescues you from danger</em></strong>, mark XP.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_IRON_WILL: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::IronWill,
    name: "Iron Will",
    description: "<p>When you <strong><em>are subject to mind control or magic that affects your feelings</em></strong>, you can take 1d4 damage (ignoring armor) to disregard its influence.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_INQUIRING_MINDS: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::InquiringMinds,
    name: "Inquiring Minds",
    description: "<p>When you <strong><em>seek out and receive honest advice</em></strong>, gain advantage on your next roll to follow that advice.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_NEVER_GONNA_KEEP_ME_DOWN: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::NeverGonnaKeepMeDown,
    name: "Never Gonna Keep Me Down",
    description: "<p>When you <strong><em>have 5 or fewer current HP</em></strong>, you impose disadvantage on any damage you take.</p><p>Once per session, when you <strong><em>are at Death's Door</em></strong>, don't roll. You get a 10+.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[stonetop::fixed::Resource {
        hold: "Skip Death's Door",
        can_be: stonetop::fixed::CanBe::Max(1u8),
        start: stonetop::fixed::EmptyFull::Empty,
    }],
    checklist: None,
};

static MOVE_POTENTIAL_FOR_GREATNESS: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::PotentialForGreatness,
    name: "Potential for Greatness",
    description: "<p>Once per level, <strong><em>when you roll a stat and get a 10+</em></strong>, mark one of the following (note the level during which you marked it). You don't have to mark them in order.</p>",
    requires: &[stonetop::fixed::Requirement::NeedsPlaybook(
        stonetop::keys::PlaybookKey::TheWouldBeHero,
    )],
    max_picks: 1u8,
    resource: &[],
    checklist: Some(stonetop::fixed::MoveChecklist::OptionsWithLevel(&[
        "Increase the stat you rolled by 1, to a max of +2",
        "Increase the stat you rolled by 1, to a max of +2",
        "Increase the stat you rolled by 1, to a max of +2",
        "Increase the stat you rolled by 1, to a max of +2",
        "Increase your max HP by 4",
        "Increase your damage die to a d8",
    ])),
};

static MOVE_RESOURCEFUL: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Resourceful,
    name: "Resourceful",
    description: "<p>When you <strong><em>Defy Danger and roll a 6-</em></strong>, ask the GM a question from Seek Insight after they describe what happens. Gain advantage on your next roll to act on the answer.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_SOMETHING_TO_REMEMBER_ME_BY: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::SomethingToRememberMeBy,
    name: "Something to Remember Me By",
    description: "<p>When you <strong><em>spend Readiness (from Defend) to strike back at an attacker</em></strong>, you deal +1d4 damage and scar, mark, or diminish them in some way (the GM will say how, or ask you to).</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_TOUGH_LOVE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::ToughLove,
    name: "Tough Love",
    description: "<p>When you <strong><em>honestly think another PC is in the wrong and call them on it</em></strong>, they have disadvantage on any rolls against you until you two work it out.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_UNDERESTIMATED: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Underestimated,
    name: "Underestimated",
    description: "<p>As long as you <strong><em>avoid overt hostility</em></strong>, no enemy will consider you a threat.</p><p>When you <strong><em>first make your move against an enemy who underestimates you</em></strong>, you have advantage.</p>",
    requires: &[],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_UP_WITH_PEOPLE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::UpWithPeople,
    name: "Up With People",
    description: "<p>When you <strong><em>converse with someone</em></strong> (PC or NPC) you can hold 2 Rapport with them. If you do, they hold 1 Rapport with you. During the conversation, either of you can spend 1 Rapport to ask the other player one of the following and get an honest answer.</p><ul><li>What weighs you down or holds you back?</li><li>What drives you forward?</li><li>What lesson would you have me learn?</li><li>What do you think of me, truly?</li></ul>",
    requires: &[],
    max_picks: 1u8,
    resource: &[
        stonetop::fixed::Resource {
            hold: "Your Rapport",
            can_be: stonetop::fixed::CanBe::Max(2u8),
            start: stonetop::fixed::EmptyFull::Empty,
        },
        stonetop::fixed::Resource {
            hold: "Their Rapport",
            can_be: stonetop::fixed::CanBe::Max(1u8),
            start: stonetop::fixed::EmptyFull::Empty,
        },
    ],
    checklist: None,
};

static MOVE_VERSATILE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Versatile,
    name: "Versatile",
    description: "<p>Choose a move from any other playbook, as long as you meet its requirements. You can pick from a different playbook each time. You can't take Improved Stat or Superior Stat.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(2u8),
        stonetop::fixed::Requirement::NeedsPlaybook(stonetop::keys::PlaybookKey::TheWouldBeHero),
    ],
    max_picks: 4u8,
    resource: &[],
    checklist: None,
};

static MOVE_A_FORCE_TO_BE_RECKONED_WITH: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::AForceToBeReckonedWith,
    name: "A Force to Be Reckoned With",
    description: "<p>Any intelligent creature who looks you in the eye or hears the steel in your voice instinctively knows that you are a force to be reckoned with, and treats you appropriately.</p><p>When you <strong><em>Defy Danger against something trying to harm or constrain you</em></strong>, on a 12+ you turn the tables on them (the GM will say how, or ask you to).</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Replaces(stonetop::keys::MoveKey::Underestimated),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_BIG_DAMN_HERO: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::BigDamnHero,
    name: "Big Damn Hero",
    description: "<p>When you <strong><em>first leap into danger to protect someone</em></strong>, don't roll to Defend. Instead, treat it as though you rolled a 10+.</p><p>When you <strong><em>Defend</em></strong>, you can spend 1 Readiness to lock eyes with an attacker; they have disadvantage on damage rolls against you and your ward for the rest of the fight.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Replaces(stonetop::keys::MoveKey::InOverYourHead),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_PW_SUPERIOR_STAT: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::PwSuperiorStat,
    name: "Superior Stat",
    description: "<p>Increase one of your stats by +1 (to a max of +3).</p>",
    requires: &[stonetop::fixed::Requirement::NeedsSixInPotentialFG],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_UNDAUNTED: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::Undaunted,
    name: "Undaunted",
    description: "<p>When you <strong><em>are outnumbered or facing a foe bigger than you</em></strong>, you get +1 armor and deal +1d6 damage.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Replaces(stonetop::keys::MoveKey::BetterPartOfValor),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

static MOVE_VOICE_OF_EXPERIENCE: stonetop::fixed::MoveFixed = stonetop::fixed::MoveFixed {
    key: stonetop::keys::MoveKey::VoiceOfExperience,
    name: "Voice of Experience",
    description: "<p>When <strong><em>another PC comes to you for advice and you tell them what you think is best</em></strong>, they have advantage on their first roll to follow your advice.</p><p>When you <strong><em>Seek Insight</em></strong>, you can always ask, \"What is about to happen?\" for free, even on a 6-.</p>",
    requires: &[
        stonetop::fixed::Requirement::Level(6u8),
        stonetop::fixed::Requirement::Replaces(stonetop::keys::MoveKey::InquiringMinds),
    ],
    max_picks: 1u8,
    resource: &[],
    checklist: None,
};

// The Blessed: Backgrounds

static BACKGROUND_INITIATE: stonetop::fixed::BackgroundFixed = stonetop::fixed::BackgroundFixed {
    key: stonetop::keys::BackgroundKey::Initiate,
    name: "Initiate",
    description: &[
        stonetop::fixed::BackgroundChunk::Flavor(
            "<p>Stonetop has long been home to a sacred order, keepers of the old ways and speakers for Danu. You are one such initiate, the most gifted in generations. You gain the Rites of the Land move.</p>",
        ),
        stonetop::fixed::BackgroundChunk::Flavor(
            "<p>There are other initiates in Stonetop, serving the goddess and the village. They aid you as followers—see the Initiates of Danu insert. Who are they? Choose 2 or 3:</p>",
        ),
        stonetop::fixed::BackgroundChunk::Checklist(stonetop::fixed::BackgroundChecklist::Options(
            &[
                "<strong>Enfys</strong>, your acolyte, beloved by birds",
                "<strong>Afon</strong>, strange and Fae-touched",
                "<strong>Gwendyl</strong>, your mentor, a talented healer",
                "<strong>Olwin</strong>, your anointed lover, seer of fates",
                "<strong>Seren the Eldest</strong>, wise and hard as winter",
            ],
        )),
    ],
    grants_moves: &[stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::RitesOfTheLand)],
    grants_possession: None,
    grants_topic: None,
};

static BACKGROUND_RAISED_BY_WOLVES: stonetop::fixed::BackgroundFixed =
    stonetop::fixed::BackgroundFixed {
        key: stonetop::keys::BackgroundKey::RaisedByWolves,
        name: "Raised by Wolves",
        description: &[
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>Maybe not by <em>wolves</em>, but you grew up in the wild. Beasts of land and air were your siblings. The sighing wind taught you language. The trees and rocks were your home. Were you one of the Forest Folk? Abandoned or orphaned? Lured into the Wood? Regardless, you get the Trackless Step move.</p>",
            ),
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>For some reason, you've made yourself known to Stonetop and perhaps you even call it home. But the ways of humans are still strange to you.</p>",
            ),
            stonetop::fixed::BackgroundChunk::Move(&MOVE_FROM_RAISED_BY_WOLVES),
        ],
        grants_moves: &[stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::TracklessStep)],
        grants_possession: None,
        grants_topic: None,
    };

static BACKGROUND_VESSEL: stonetop::fixed::BackgroundFixed = stonetop::fixed::BackgroundFixed {
    key: stonetop::keys::BackgroundKey::Vessel,
    name: "Vessel",
    description: &[
        stonetop::fixed::BackgroundChunk::Flavor(
            "<p>A seed of Danu's power has taken root in your soul. Perhaps it has always been there and only recently sprouted. Or maybe it was planted in you during some portentous event.</p>",
        ),
        stonetop::fixed::BackgroundChunk::Flavor(
            "<p>Regardless, your dreams have been haunted by strange markings and symbols. You feel the mystic power in plants, stones, and soil. And you've felt the growing wrath of the Earth Mother as foul things begin to move about. Take the Danu's Grasp move.</p>",
        ),
        stonetop::fixed::BackgroundChunk::Move(&MOVE_FROM_VESSEL),
    ],
    grants_moves: &[stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::DanusGrasp)],
    grants_possession: None,
    grants_topic: None,
};

// The Fox: Backgrounds

static BACKGROUND_THE_NATURAL: stonetop::fixed::BackgroundFixed =
    stonetop::fixed::BackgroundFixed {
        key: stonetop::keys::BackgroundKey::TheNatural,
        name: "The Natural",
        description: &[
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>You grew up around here, and always picked things up quickly. Reading and numbers, sure, but more. Hide and seek. Throwing stones. Climbing. Fighting. Whatever you tried, you were good at it. As good as anyone else, if not better.</p>",
            ),
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>Sure, you've got a reputation for bending the rules. Playing dirty. But why play if you don't play to win, right? And who do they come to when there's a problem needs solving? You, that's who.</p>",
            ),
            stonetop::fixed::BackgroundChunk::Move(&MOVE_FROM_THE_NATURAL),
        ],
        grants_moves: &[],
        grants_possession: None,
        grants_topic: None,
    };

static BACKGROUND_A_LIFE_OF_CRIME: stonetop::fixed::BackgroundFixed =
    stonetop::fixed::BackgroundFixed {
        key: stonetop::keys::BackgroundKey::ALifeOfCrime,
        name: "A Life of Crime",
        description: &[
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>You're new to Stonetop, having left behind a... <em>colorful</em> past. How did you get into that life? Why and how did you get out? Who and what did you leave behind?</p>",
            ),
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>Regardless, these people have taken you in. Time to lead an honest life, right?</p>",
            ),
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>You start with either Burgle or Light Fingers (your choice) as an extra move, and either burglar tools or a hidden stash (your choice) as an additional special possession.</p>",
            ),
        ],
        grants_moves: &[stonetop::fixed::GrantMove::ChooseOne(
            stonetop::keys::MoveKey::Burgle,
            stonetop::keys::MoveKey::LightFingers,
        )],
        grants_possession: Some(stonetop::fixed::GrantPossession::ChooseOne(
            stonetop::keys::SpecialPossessionKey::BurglarsKit,
            stonetop::keys::SpecialPossessionKey::HiddenStash,
        )),
        grants_topic: None,
    };

static BACKGROUND_THE_PRODIGAL_RETURNED: stonetop::fixed::BackgroundFixed =
    stonetop::fixed::BackgroundFixed {
        key: stonetop::keys::BackgroundKey::TheProdigalReturned,
        name: "The Prodigal Returned",
        description: &[
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>You left long ago, travelling far and living by your wits. Why did you leave? What deeds do you boast of, and which do you regret?</p>",
            ),
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>You always longed to return to Stonetop, and return you have. You're a bit of a celebrity now, and you've got friends (or close enough) strewn about the known world.</p>",
            ),
            stonetop::fixed::BackgroundChunk::Move(&MOVE_FROM_THE_PRODIGAL_RETURNED),
        ],
        grants_moves: &[],
        grants_possession: None,
        grants_topic: None,
    };

// The Heavy: Backgrounds

static BACKGROUND_SHERIFF: stonetop::fixed::BackgroundFixed = stonetop::fixed::BackgroundFixed {
    key: stonetop::keys::BackgroundKey::Sheriff,
    name: "Sheriff",
    description: &[
        stonetop::fixed::BackgroundChunk::Flavor(
            "<p>You keep order in Stonetop and protect it from outside threats. It might not be anything official, but everyone knows you've got a cool head and the weight to back up your words.</p>",
        ),
        stonetop::fixed::BackgroundChunk::Move(&MOVE_FROM_SHERIFF),
    ],
    grants_moves: &[],
    grants_possession: None,
    grants_topic: None,
};

static BACKGROUND_BLOOD_SOAKED_PAST: stonetop::fixed::BackgroundFixed =
    stonetop::fixed::BackgroundFixed {
        key: stonetop::keys::BackgroundKey::BloodSoakedPast,
        name: "Blood-Soaked Past",
        description: &[
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>You left behind a life of violence and a name mothers used to scare their children. For whatever reason, the people of Stonetop took you (back?) in and treat you like one of their own.</p>",
            ),
            stonetop::fixed::BackgroundChunk::Move(&MOVE_FROM_BLOOD_SOAKED_PAST),
        ],
        grants_moves: &[],
        grants_possession: None,
        grants_topic: None,
    };

static BACKGROUND_STORM_MARKED: stonetop::fixed::BackgroundFixed =
    stonetop::fixed::BackgroundFixed {
        key: stonetop::keys::BackgroundKey::StormMarked,
        name: "Storm-Marked",
        description: &[
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>You've been touched by Tor (Rain-maker, Thunderhead, Slayer-of-Beasts!) and bear runic markings similar to those etched into the Stone. When did the marks manifest? Are they a symbol of your strength, speed, and courage? Or their source?</p>",
            ),
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>You start with the Storm Markings major arcanum. Mark one of the boxes on the front of the Storm Markings sheet, and describe here the time you were struck by lightning and walked away unharmed:</p>",
            ),
            stonetop::fixed::BackgroundChunk::Flavor("<p>▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁</p>"),
        ],
        grants_moves: &[],
        grants_possession: None,
        grants_topic: None,
    };

// The Judge: Backgrounds

static BACKGROUND_LEGACY: stonetop::fixed::BackgroundFixed = stonetop::fixed::BackgroundFixed {
    key: stonetop::keys::BackgroundKey::Legacy,
    name: "Legacy",
    description: &[
        stonetop::fixed::BackgroundChunk::Flavor(
            "<p>You are the latest in a long line of Judges—born here, apprenticed to the prior Judge, and charged with the passing of the mantle. The Chronicle is a rich repository of lore, but there's no index, so good luck finding anything.</p>",
        ),
        stonetop::fixed::BackgroundChunk::Move(&MOVE_FROM_LEGACY),
    ],
    grants_moves: &[],
    grants_possession: None,
    grants_topic: None,
};

static BACKGROUND_MISSIONARY: stonetop::fixed::BackgroundFixed = stonetop::fixed::BackgroundFixed {
    key: stonetop::keys::BackgroundKey::Missionary,
    name: "Missionary",
    description: &[
        stonetop::fixed::BackgroundChunk::Flavor(
            "<p>You are part of a larger order of Judges, sent here to protect the flickering flame of civilization. The Chronicle is relatively new; your position in town is far from certain. Add these Judges to the Neighbors section of the steading playbook (pick 2 more):</p>",
        ),
        stonetop::fixed::BackgroundChunk::Checklist(
            stonetop::fixed::BackgroundChecklist::OptionsPrecheckable(&[
                stonetop::fixed::PrecheckableOption {
                    prechecked: true,
                    text: "Devin (from Marshedge)",
                },
                stonetop::fixed::PrecheckableOption {
                    prechecked: true,
                    text: "Haeris (from Gordin's Delve)",
                },
                stonetop::fixed::PrecheckableOption {
                    prechecked: false,
                    text: "Isalde (from the Manmarch)",
                },
                stonetop::fixed::PrecheckableOption {
                    prechecked: false,
                    text: "Rahat (from Lygos)",
                },
                stonetop::fixed::PrecheckableOption {
                    prechecked: false,
                    text: "Tejisha (from Barrier Pass)",
                },
                stonetop::fixed::PrecheckableOption {
                    prechecked: false,
                    text: "Unz (from the Hillfolk)",
                },
            ]),
        ),
        stonetop::fixed::BackgroundChunk::Move(&MOVE_FROM_MISSIONARY),
    ],
    grants_moves: &[],
    grants_possession: Some(stonetop::fixed::GrantPossession::Simply(
        stonetop::keys::SpecialPossessionKey::Aviary,
    )),
    grants_topic: None,
};

static BACKGROUND_PROPHET: stonetop::fixed::BackgroundFixed = stonetop::fixed::BackgroundFixed {
    key: stonetop::keys::BackgroundKey::Prophet,
    name: "Prophet",
    description: &[
        stonetop::fixed::BackgroundChunk::Flavor(
            "<p>The line of Judges was broken long ago, the Chronicle lost or fallen into ruin. Aratis has called you personally to her service through dreams, omens, and visions. Some in town resent the authority you've assumed.</p>",
        ),
        stonetop::fixed::BackgroundChunk::Move(&MOVE_FROM_PROPHET),
    ],
    grants_moves: &[],
    grants_possession: None,
    grants_topic: None,
};

// The Lightbearer: Backgrounds

static BACKGROUND_AUSPICIOUS_BIRTH: stonetop::fixed::BackgroundFixed =
    stonetop::fixed::BackgroundFixed {
        key: stonetop::keys::BackgroundKey::AuspiciousBirth,
        name: "Auspicious Birth",
        description: &[
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>You were born in Stonetop, and that birth was marked by the God of Light. You were born during an eclipse, perhaps, or under the light of a bright new star? Maybe you bear a sun-shaped birthmark?</p>",
            ),
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>Whatever the sign, your connection to Helior was clear early on. You've a place of honor in Stonetop, though it'd be a lie to say you don't make some uneasy.</p>",
            ),
            stonetop::fixed::BackgroundChunk::Move(&MOVE_FROM_AUSPICIOUS_BIRTH),
        ],
        grants_moves: &[],
        grants_possession: None,
        grants_topic: None,
    };

static BACKGROUND_ITINERANT_MYSTIC: stonetop::fixed::BackgroundFixed =
    stonetop::fixed::BackgroundFixed {
        key: stonetop::keys::BackgroundKey::ItinerantMystic,
        name: "Itinerant Mystic",
        description: &[
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>They think of you as a self-important kook who comes through now and again, speaking in riddles and playing tricks with the light. Sure, they know there's something holy about you, but it's not like you're a priest or anything. Priests talk sense.</p>",
            ),
            stonetop::fixed::BackgroundChunk::Move(&MOVE_FROM_ITINERANT_MYSTIC),
        ],
        grants_moves: &[],
        grants_possession: None,
        grants_topic: None,
    };

static BACKGROUND_SOUL_ON_FIRE: stonetop::fixed::BackgroundFixed =
    stonetop::fixed::BackgroundFixed {
        key: stonetop::keys::BackgroundKey::SoulOnFire,
        name: "Soul on Fire",
        description: &[
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>You once led a worldly life, full of fear and doubt, base pleasures and petty grudges. But something happened. Injury, illness, a brush with death. Or just a moment of such profound misery and self-loathing that you thought you could fall no further.</p>",
            ),
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>There, in the dark, Helior's light shined upon you, igniting in your soul, lifting you and filling you with a profound sense of purpose.</p>",
            ),
            stonetop::fixed::BackgroundChunk::Move(&MOVE_FROM_SOUL_ON_FIRE),
        ],
        grants_moves: &[],
        grants_possession: None,
        grants_topic: None,
    };

// The Marshal: Backgrounds

static BACKGROUND_SCION: stonetop::fixed::BackgroundFixed = stonetop::fixed::BackgroundFixed {
    key: stonetop::keys::BackgroundKey::Scion,
    name: "Scion",
    description: &[
        stonetop::fixed::BackgroundChunk::Flavor(
            "<p>You grew up here, descended from a long line. Some of the biggest names in Stonetop's past are perched in your family tree. Everyone in the village takes your authority as a given, and your crew is a well-established institution in town.</p>",
        ),
        stonetop::fixed::BackgroundChunk::Flavor(
            "<p>You start with the Veteran Crew move, in addition to your usual moves.</p>",
        ),
        stonetop::fixed::BackgroundChunk::Flavor(
            "<p>When you <strong><em>create your Crew</em></strong>, they automatically have the <em>respected</em> tag (in addition to your usual picks, and any you get from Veteran Crew).</p>",
        ),
    ],
    grants_moves: &[stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::VeteranCrew)],
    grants_possession: None,
    grants_topic: None,
};

static BACKGROUND_PENITENT: stonetop::fixed::BackgroundFixed = stonetop::fixed::BackgroundFixed {
    key: stonetop::keys::BackgroundKey::Penitent,
    name: "Penitent",
    description: &[
        stonetop::fixed::BackgroundChunk::Flavor(
            "<p>Before you came here, you led a band of ne'er-do-wells: bandits, raiders, or bloody-handed mercenaries. But something changed. A moment of truth led you and your followers—some of them at least—to leave that life behind. And for whatever reason, the people of Stonetop took you in.</p>",
        ),
        stonetop::fixed::BackgroundChunk::Move(&MOVE_FROM_PENITENT),
        stonetop::fixed::BackgroundChunk::Flavor(
            "<p>When you <strong><em>create your Crew</em></strong>, they automatically have the <em>warriors</em> tag (in addition to your usual picks).</p>",
        ),
    ],
    grants_moves: &[],
    grants_possession: None,
    grants_topic: None,
};

static BACKGROUND_LUMINARY: stonetop::fixed::BackgroundFixed = stonetop::fixed::BackgroundFixed {
    key: stonetop::keys::BackgroundKey::Luminary,
    name: "Luminary",
    description: &[
        stonetop::fixed::BackgroundChunk::Flavor(
            "<p>You're a natural leader—your words inspire, your plans win the day, your deeds are recounted far and wide. Are you touched by the gods? Does ancient blood flow in your veins? Or are you simply the champion that Stonetop needs in these trying times?</p>",
        ),
        stonetop::fixed::BackgroundChunk::Flavor(
            "<p>You start with the We Happy Few move, in addition to your usual moves.</p>",
        ),
        stonetop::fixed::BackgroundChunk::Flavor(
            "<p>When you <strong><em>create your Crew</em></strong>, they automatically have the <em>devoted</em> tag (in addition to your usual picks).</p>",
        ),
    ],
    grants_moves: &[stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::WeHappyFew)],
    grants_possession: None,
    grants_topic: None,
};

// The Ranger: Backgrounds

static BACKGROUND_MIGHTY_HUNTER: stonetop::fixed::BackgroundFixed =
    stonetop::fixed::BackgroundFixed {
        key: stonetop::keys::BackgroundKey::MightyHunter,
        name: "Mighty Hunter",
        description: &[
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>You are a hunter of the Great Wood, the best the town has seen in generations. You know every part of the Wood within a two-day march.</p>",
            ),
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>You start with both the Expert Tracker move and the Stalker move.</p>",
            ),
        ],
        grants_moves: &[
            stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::ExpertTracker),
            stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::Stalker),
        ],
        grants_possession: None,
        grants_topic: None,
    };

static BACKGROUND_WIDE_WANDERER: stonetop::fixed::BackgroundFixed =
    stonetop::fixed::BackgroundFixed {
        key: stonetop::keys::BackgroundKey::WideWanderer,
        name: "Wide Wanderer",
        description: &[
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>You have travelled much of the known world and perhaps parts beyond. Add each of the following to the Neighbors list in the Stonetop playbook, choosing 1 trait for each:</p>",
            ),
            stonetop::fixed::BackgroundChunk::Flavor(
                "<ul><li><strong>Ennis</strong> (from Marshedge)</li><li><strong>Shahar</strong> (from Gordin's Delve)</li><li><strong>Yannic</strong> (from the Hillfolk)</li><li><strong>Tovia</strong> (from Lygos)</li><li><strong>Sasca</strong> (from the northern Manmarch)</li></ul>",
            ),
            stonetop::fixed::BackgroundChunk::Flavor("<p>You start with the Mental Map move.</p>"),
            stonetop::fixed::BackgroundChunk::Move(&MOVE_FROM_WIDE_WANDERER),
        ],
        grants_moves: &[stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::MentalMap)],
        grants_possession: None,
        grants_topic: None,
    };

static BACKGROUND_BEAST_BONDED: stonetop::fixed::BackgroundFixed =
    stonetop::fixed::BackgroundFixed {
        key: stonetop::keys::BackgroundKey::BeastBonded,
        name: "Beast-Bonded",
        description: &[
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>You grew up civilized, but your soul is bound to a beast of the wild. You're closer to it than to any man or woman. How did this bond come about? How long ago? Regardless, you start with the Animal Companion move.</p>",
            ),
            stonetop::fixed::BackgroundChunk::Move(&MOVE_FROM_BEAST_BONDED),
        ],
        grants_moves: &[stonetop::fixed::GrantMove::Simply(
            stonetop::keys::MoveKey::AnimalCompanion,
        )],
        grants_possession: None,
        grants_topic: None,
    };

// The Seeker: Backgrounds

static BACKGROUND_PATRIOT: stonetop::fixed::BackgroundFixed = stonetop::fixed::BackgroundFixed {
    key: stonetop::keys::BackgroundKey::Patriot,
    name: "Patriot",
    description: &[
        stonetop::fixed::BackgroundChunk::Flavor(
            "<p>These people are family. Chaos grows all around, but you'll be damned if you'll let your family come to harm. Damned indeed.</p>",
        ),
        stonetop::fixed::BackgroundChunk::Flavor(
            "<p>You have sought out and embraced dark power to protect that which you hold dear. Or perhaps that power fell upon you, and you took it up for the greater good. Either way, you seek more.</p>",
        ),
        stonetop::fixed::BackgroundChunk::Flavor(
            "<p>You start with the Let's Make a Deal move and are Well Versed in the Things Below. You've also acquired 1 major arcanum:</p>",
        ),
        stonetop::fixed::BackgroundChunk::Checklist(stonetop::fixed::BackgroundChecklist::Options(
            &["◇ The Hec'tumel Codex", "◇ The Red Scepter", "◇ The Staff of the Lidless Orb"],
        )),
    ],
    grants_moves: &[stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::LetsMakeADeal)],
    grants_possession: None,
    grants_topic: Some(stonetop::fixed::GrantTopic::Simply("The Things Below")),
};

static BACKGROUND_ANTIQUARIAN: stonetop::fixed::BackgroundFixed =
    stonetop::fixed::BackgroundFixed {
        key: stonetop::keys::BackgroundKey::Antiquarian,
        name: "Antiquarian",
        description: &[
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>The past has buried many secrets, and you are determined to dig them up. Years of study across the land have led you here, and you are convinced that this town holds the key to your greatest discoveries. What is it you hope to find? What is it that keeps you here?</p>",
            ),
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>In any case, your travels and studies mean that you start with the Polyglot move and that you are Well Versed in the Makers and their arts. You've also acquired 1 major arcanum:</p>",
            ),
            stonetop::fixed::BackgroundChunk::Checklist(
                stonetop::fixed::BackgroundChecklist::Options(&[
                    "◇ Noruba's Ice Sphere",
                    "◇ The Azure Hand",
                    "◇◇ The Mindgem",
                ]),
            ),
        ],
        grants_moves: &[stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::Polyglot)],
        grants_possession: None,
        grants_topic: Some(stonetop::fixed::GrantTopic::Simply("The Makers and their arts")),
    };

static BACKGROUND_WITCH_HUNTER: stonetop::fixed::BackgroundFixed =
    stonetop::fixed::BackgroundFixed {
        key: stonetop::keys::BackgroundKey::WitchHunter,
        name: "Witch Hunter",
        description: &[
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>You've dedicated your life to rooting out and destroying horrors and their servants. What set you down this path? What did you sacrifice to walk it? What led you to call Stonetop home?</p>",
            ),
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>Regardless, you start with the Everything Bleeds move and are Well Versed in (pick 1) the Fae, the Things Below, or the Last Door and what lies beyond. You've also acquired 1 major arcanum:</p>",
            ),
            stonetop::fixed::BackgroundChunk::Checklist(
                stonetop::fixed::BackgroundChecklist::Options(&[
                    "◇ The Demonhide Cloak",
                    "The Redwood Effigy",
                    "◇◇ The Twisted Spear",
                ]),
            ),
        ],
        grants_moves: &[stonetop::fixed::GrantMove::Simply(
            stonetop::keys::MoveKey::EverythingBleeds,
        )],
        grants_possession: None,
        grants_topic: Some(stonetop::fixed::GrantTopic::ChooseOne(&[
            "the Fae",
            "the Things Below",
            "the Last Door",
        ])),
    };

// The Would-be Hero: Backgrounds

static BACKGROUND_IMPETUOUS_YOUTH: stonetop::fixed::BackgroundFixed =
    stonetop::fixed::BackgroundFixed {
        key: stonetop::keys::BackgroundKey::ImpetuousYouth,
        name: "Impetuous Youth",
        description: &[
            stonetop::fixed::BackgroundChunk::Flavor(
                "<p>Stonetop has always been home, but you chafe at the demands of mundane life and have always longed for more. Excitement! Danger!</p>",
            ),
            stonetop::fixed::BackgroundChunk::Move(&MOVE_FROM_IMPETUOUS_YOUTH),
        ],
        grants_moves: &[],
        grants_possession: None,
        grants_topic: None,
    };

static BACKGROUND_DRIVEN: stonetop::fixed::BackgroundFixed = stonetop::fixed::BackgroundFixed {
    key: stonetop::keys::BackgroundKey::Driven,
    name: "Driven",
    description: &[
        stonetop::fixed::BackgroundChunk::Flavor(
            "<p>You once led a simple life, but something happened. Something changed you, burdened you with terrible purpose. What was it? Choose 1:</p>",
        ),
        stonetop::fixed::BackgroundChunk::Checklist(stonetop::fixed::BackgroundChecklist::Options(
            &[
                "A loved one was killed or abducted",
                "Someone gave their life to save you",
                "Your idol sacrificed themselves to save many",
                "You stumbled upon a dark mystery",
                "You must make amends for a terrible mistake",
            ],
        )),
        stonetop::fixed::BackgroundChunk::Move(&MOVE_FROM_DRIVEN),
    ],
    grants_moves: &[],
    grants_possession: None,
    grants_topic: None,
};

static BACKGROUND_DESTINED: stonetop::fixed::BackgroundFixed = stonetop::fixed::BackgroundFixed {
    key: stonetop::keys::BackgroundKey::Destined,
    name: "Destined",
    description: &[
        stonetop::fixed::BackgroundChunk::Flavor(
            "<p>Fate has laid her hand upon you. Choose 3-4 of the items below to describe your destiny:</p>",
        ),
        stonetop::fixed::BackgroundChunk::Checklist(stonetop::fixed::BackgroundChecklist::Rows(&[
            stonetop::fixed::TaggedRow {
                tag: "Portents",
                items: &["anointed", "marked at birth", "your coming foretold"],
            },
            stonetop::fixed::TaggedRow {
                tag: "Actions",
                items: &["destroy", "discover", "free", "protect", "restore", "unify"],
            },
            stonetop::fixed::TaggedRow {
                tag: "Elements",
                items: &["earth & stone", "darkness", "fire", "ice", "light", "water"],
            },
            stonetop::fixed::TaggedRow {
                tag: "History",
                items: &["blood", "civilization", "life", "storms", "war"],
            },
            stonetop::fixed::TaggedRow {
                tag: "Mysteries",
                items: &["the Fae", "the gods", "the Makers", "the Stone", "the Things Below"],
            },
        ])),
        stonetop::fixed::BackgroundChunk::Move(&MOVE_FROM_DESTINED),
    ],
    grants_moves: &[],
    grants_possession: None,
    grants_topic: None,
};

// The Blessed: Special Possessions

static SPECIAL_POSSESSION_SACRED_POUCH: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::SacredPouch,
        name: "Sacred pouch (<em>magical</em>)",
        description: "see back page. Stock: {resource}",
        resource: Some(stonetop::fixed::Resource {
            hold: "Stock",
            can_be: stonetop::fixed::CanBe::Max(3u8),
            start: stonetop::fixed::EmptyFull::Full,
        }),
        pick: &[],
    };

static SPECIAL_POSSESSION_APIARY: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::Apiary,
        name: "Apiary",
        description: "beeswax, candles (<em>close, area</em>, lasts ~1 hr), honey, ◇ bee smokers, ◇ hats & veils, etc.",
        resource: None,
        pick: &[],
    };

static SPECIAL_POSSESSION_COLLECTED_OFFERINGS: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::CollectedOfferings,
        name: "Collected offerings",
        description: "({resource} uses): Expend a use to produce something valuable to a spirit of the wild. Restore 1 use each season.",
        resource: Some(stonetop::fixed::Resource {
            hold: "Uses",
            can_be: stonetop::fixed::CanBe::Max(3u8),
            start: stonetop::fixed::EmptyFull::Full,
        }),
        pick: &[],
    };

static SPECIAL_POSSESSION_GOAT_HERD: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::GoatHerd,
        name: "Goat herd",
        description: "milk, cheese, pelts, meat, blood, horn, wool, etc. Each season, 1 in 4 chance of having a bezoar (swallow it to cure poison).",
        resource: None,
        pick: &[],
    };

static SPECIAL_POSSESSION_HERB_GARDEN: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::HerbGarden,
        name: "Herb garden",
        description: "shears, mortars & pestles, herbs, seeds, remedies, mild poisons, ◇ spades, etc. Each spring, d4 uses of bendis root (<em>reach, area</em>, burns ~1 hr, fumes repel perversions of nature).",
        resource: None,
        pick: &[],
    };

static SPECIAL_POSSESSION_MASTIFFS: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::Mastiffs,
        name: "Mastiffs",
        description: ", 2-3 followers (<em>alert, keen-nosed, fierce, overprotective</em>); HP 6; Damage d6 (<em>hand, grabby</em>); Instinct: to bark &amp; threaten; Cost: affection.",
        resource: None,
        pick: &[],
    };

// The Fox: Special Possessions

static SPECIAL_POSSESSION_BURGLARS_KIT: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::BurglarsKit,
        name: "Burglar's kit",
        description: "picks, files, snippers, wire, ◇ prybars, ◇ hacksaws, ◇ a lantern (○○○○○ hours, <em>close, area</em>), ◇ a grappling hook, etc.",
        resource: None,
        pick: &[],
    };

static SPECIAL_POSSESSION_CARPENTERS_TOOLS: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::CarpentersTools,
        name: "Carpenter's tools",
        description: "chisels, files, nails, pitch, ◇ prybars, ◇ saws, ◇◇ firkins, barrels, etc.",
        resource: None,
        pick: &[],
    };

static SPECIAL_POSSESSION_DISTILLERY: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::Distillery,
        name: "Distillery",
        description: "skins of fine whisky (○○ uses, grants advantage to Persuade), copper tubes, malt, ◇◇ firkins, stills, barrels, etc.",
        resource: None,
        pick: &[],
    };

static SPECIAL_POSSESSION_HIDDEN_STASH: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::HiddenStash,
        name: "Hidden stash",
        description: "({resource} uses): each use produces valuables worth a purse of silvers (Value 2)",
        resource: Some(stonetop::fixed::Resource {
            hold: "Uses",
            can_be: stonetop::fixed::CanBe::Max(3u8),
            start: stonetop::fixed::EmptyFull::Full,
        }),
        pick: &[],
    };

static SPECIAL_POSSESSION_MUMMERS_KIT: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::MummersKit,
        name: "Mummer's kit",
        description: "juggling balls, whirlybird seeds, motley, ribbons, bells, ◇ puppets, ◇ a fiddle, etc.",
        resource: None,
        pick: &[],
    };

static SPECIAL_POSSESSION_SCRIBES_TOOLS: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::ScribesTools,
        name: "Scribe's tools",
        description: "parchment, ink, pigments, vials, quills, ◇ a notebook, etc.",
        resource: None,
        pick: &[],
    };

static SPECIAL_POSSESSION_TANNERY: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::Tannery,
        name: "Tannery",
        description: "(or access to it): lime, acid, salts, thick gloves, ◇ a boiled leather cuirass (1 armor), etc.",
        resource: None,
        pick: &[],
    };

static SPECIAL_POSSESSION_TRADE_CONTACTS: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::TradeContacts,
        name: "Trade contacts",
        description: "small amounts of salt, glass, silk, spice, medicinal herbs, pigments, ivory, etc.",
        resource: None,
        pick: &[],
    };

// The Heavy: Special Possessions

static SPECIAL_POSSESSION_CHIRURGEONS_TOOLS: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::ChirurgeonsTools,
        name: "Chirurgeon's tools",
        description: "catgut, straps, bandages, tubes, poultices, willow bark, ◇ bonesaws, etc.",
        resource: None,
        pick: &[],
    };

static SPECIAL_POSSESSION_HUSBANDRY_TOOLS: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::HusbandryTools,
        name: "Husbandry tools",
        description: "brushes, muzzles, collars, feed, ◇ whips, ◇ bridles, etc. Gain advantage to Persuade domestic beasts (livestock, dogs, etc.).",
        resource: None,
        pick: &[],
    };

static SPECIAL_POSSESSION_SMITHY: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::Smithy,
        name: "Smithy",
        description: "(or access to it): iron goods, ingots, thick gloves, ◇ tongs, ◇ bellows, an anvil, etc.",
        resource: None,
        pick: &[],
    };

static SPECIAL_POSSESSION_STONEWORKERS_TOOLS: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::StoneworkersTools,
        name: "Stoneworker's tools",
        description: "chisels, drills, ◇ prybars, ◇ spikes, ◇ block & tackles, wheelbarrow, etc.",
        resource: None,
        pick: &[],
    };

static SPECIAL_POSSESSION_PH_WEAPONS_OF_WAR: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::PhWeaponsOfWar,
        name: "Weapons of war",
        description: "choose up to 3 (now or later):",
        resource: None,
        pick: &[
            "◇ Sword, iron (<em>close</em>, +1 damage)",
            "◇ Battleaxe, iron (<em>close, messy</em>)",
            "◇ Warhammer, iron (<em>close</em>, 2 piercing)",
            "◇ Mace or flail, iron (<em>close, forceful</em>)",
            "◇ Crossbow (<em>far</em>, +1 damage, <em>reload</em>, x piercing, ○ low ammo, ○ all out)",
        ],
    };

// The Judge: Special Possessions

static SPECIAL_POSSESSION_YOUR_SYMBOL_OF_AUTHORITY: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::YourSymbolOfAuthority,
        name: "Your symbol of authority",
        description: "(pick 1):",
        resource: None,
        pick: &[
            "◇◇ Black iron maul, utterly immune to all magic (<em>close, forceful, awkward</em>, +1 damage)",
            "◇◇ Makerglass shield, etched with Aratis's symbol (<em>indestructible</em>, +1 armor, +1 Readiness on a Defend 7+)",
            "◇ Helm set with a dark ice \"jewel.\" Grants advantage to resist mind-affecting magic.",
        ],
    };

static SPECIAL_POSSESSION_AVIARY: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::Aviary,
        name: "Aviary",
        description: "thick gloves, bird hoods, tethers, seed, ◇ messenger birds, ◇ birdcages, etc.",
        resource: None,
        pick: &[],
    };

static SPECIAL_POSSESSION_ENGINEERS_TOOLS: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::EngineersTools,
        name: "Engineer's tools",
        description: "rulers, tapes, rods, plumb-bobs, ◇ tripods, ◇ block & tackles, wheelbarrow, etc.",
        resource: None,
        pick: &[],
    };

// The Lightbearer: Special Possessions

static SPECIAL_POSSESSION_BOOKS_SCROLLS: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::BooksScrolls,
        name: "Books & scrolls",
        description: "({resource} uses): expend a use to consult your collection and turn a Know Things roll you just made into a 10+.",
        resource: Some(stonetop::fixed::Resource {
            hold: "Uses",
            can_be: stonetop::fixed::CanBe::Max(5u8),
            start: stonetop::fixed::EmptyFull::Empty,
        }),
        pick: &[],
    };

static SPECIAL_POSSESSION_CHANDLERY: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::Chandlery,
        name: "Chandlery",
        description: "beeswax, candles (<em>close, area</em>, lasts ~1 hr), wicks, scented herbs, soap, lye, ash, etc.",
        resource: None,
        pick: &[],
    };

static SPECIAL_POSSESSION_GLASSWORKS: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::Glassworks,
        name: "Glassworks",
        description: "vials, charms, lenses, sand, marbles, bellows, crucible, ◇ lanterns (○○○○○ hours, <em>close, area</em>), etc.",
        resource: None,
        pick: &[],
    };

static SPECIAL_POSSESSION_HOLY_RELICS: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::HolyRelics,
        name: "Holy relics",
        description: "({resource} uses): if you have one in inventory when you Invoke the Sun God, you can mark a use in lieu of choosing a consequence.",
        resource: Some(stonetop::fixed::Resource {
            hold: "Uses",
            can_be: stonetop::fixed::CanBe::Max(3u8),
            start: stonetop::fixed::EmptyFull::Empty,
        }),
        pick: &[],
    };

static SPECIAL_POSSESSION_LUTHIERS_TOOLS: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::LuthiersTools,
        name: "Luthier's tools",
        description: "chisels, files, catgut, various woods, stains, ◇ a lute, ◇ a fiddle, etc.",
        resource: None,
        pick: &[],
    };

// The Marshal: Special Possessions

static SPECIAL_POSSESSION_PERSONAL_SYMBOL: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::PersonalSymbol,
        name: "Personal symbol",
        description: "(a flag, crest, marking, etc.): when you <strong><em>display or reveal it in a dramatic fashion</em></strong>, your crew holds +1 Loyalty (max 3).",
        resource: None,
        pick: &[],
    };

static SPECIAL_POSSESSION_PM_WEAPONS_OF_WAR: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::PmWeaponsOfWar,
        name: "Weapons of war",
        description: "choose up to 3 (now or later):",
        resource: None,
        pick: &[
            "◇ Sword, iron (<em>close</em>, +1 damage)",
            "◇◇ Long spear, fine steel (<em>reach</em>, 2 piercing)",
            "◇ Battleaxe, iron (<em>close, messy</em>)",
            "◇ Composite bow (<em>far</em>, +1 damage, x piercing; ○ low ammo, ○ all out)",
        ],
    };

// The Ranger: Special Possessions

static SPECIAL_POSSESSION_COMPOSITE_BOW: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::CompositeBow,
        name: "◇ Composite bow",
        description: "(<em>far</em>, +1 damage, x piercing; Arrows: {resource})",
        resource: Some(stonetop::fixed::Resource {
            hold: "Arrows",
            can_be: stonetop::fixed::CanBe::Labels(&["all out", "low ammo", "plenty left"]),
            start: stonetop::fixed::EmptyFull::Full,
        }),
        pick: &[],
    };

static SPECIAL_POSSESSION_HIDEOUTS: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::Hideouts,
        name: "Hideouts",
        description: "({resource} uses): expend a use to have a well-stocked, safe shelter nearby; GM can veto.",
        resource: Some(stonetop::fixed::Resource {
            hold: "Uses",
            can_be: stonetop::fixed::CanBe::Max(3u8),
            start: stonetop::fixed::EmptyFull::Empty,
        }),
        pick: &[],
    };

static SPECIAL_POSSESSION_HOUNDS: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::Hounds,
        name: "Hounds",
        description: ", 2-3 followers (<em>trackers, keen-nosed, fast</em>); HP 6; Damage d6 (<em>hand, grabby</em>); Instinct: to give chase; Cost: training.",
        resource: None,
        pick: &[],
    };

static SPECIAL_POSSESSION_LAY_OF_THE_LAND: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::LayOfTheLand,
        name: "Lay of the land",
        description: "({resource} uses): expend a use to know where to find ▁▁▁▁▁▁▁▁, without having to Know Things; GM can veto.",
        resource: Some(stonetop::fixed::Resource {
            hold: "Uses",
            can_be: stonetop::fixed::CanBe::Max(3u8),
            start: stonetop::fixed::EmptyFull::Empty,
        }),
        pick: &[],
    };

static SPECIAL_POSSESSION_TRAPPING_GEAR: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::TrappingGear,
        name: "Trapping gear",
        description: "snares, pelts, musk, bait, etc. When you <strong><em>Forage</em></strong>, get +1 use of provisions.",
        resource: None,
        pick: &[],
    };

// The Seeker: Special Possessions

static SPECIAL_POSSESSION_LABORATORY: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::Laboratory,
        name: "Laboratory",
        description: "chemics, reagents, vials, measures, ◇ scales, ◇ decanters, etc. Every season, produce d4-1 uses of ◇ naphtha (<em>thrown, area, dangerous</em>, ignores armor).",
        resource: None,
        pick: &[],
    };

static SPECIAL_POSSESSION_PARAPHERNALIA: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::Paraphernalia,
        name: "Paraphernalia",
        description: "crystals, incense, talismans, blood, bone, horn, eye of newt, ◇ braziers, ◇◇ a cauldron, etc.",
        resource: None,
        pick: &[],
    };

// The Would-be Hero: Special Possessions

static SPECIAL_POSSESSION_A_HEAP_OF_EXPECTATIONS: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::AHeapOfExpectations,
        name: "A heap of expectations",
        description: "of little use",
        resource: None,
        pick: &[],
    };

static SPECIAL_POSSESSION_A_GOOD_DOG: stonetop::fixed::SpecialPossessionFixed =
    stonetop::fixed::SpecialPossessionFixed {
        key: stonetop::keys::SpecialPossessionKey::AGoodDog,
        name: "A good dog",
        description: ", follower (☐ <em>retriever</em> or ☐ <em>herder</em>, <em>keen-nosed, clever</em>); HP 6; Damage d6 (<em>hand, grabby</em>); Instinct to play; Cost: affection.",
        resource: None,
        pick: &["retriever", "herder"],
    };

static SPECIAL_POSSESSION_PERSONAL_TOKEN_FRAUGHT_WITH_MEANING:
    stonetop::fixed::SpecialPossessionFixed = stonetop::fixed::SpecialPossessionFixed {
    key: stonetop::keys::SpecialPossessionKey::PersonalTokenFraughtWithMeaning,
    name: "Personal token, fraught with meaning",
    description: ": (pick 1)",
    resource: None,
    pick: &[
        "◇◇ A shield, bearing ▁▁▁▁▁▁▁▁'s crest",
        "◇ A wool cloak, woven just for you by ▁▁▁▁▁▁▁▁",
        "A letter, spattered with tears & blood",
        "A flute, a gift from someone you loved",
        "A fine locket, holding a strand of hair",
        "A tinderbox, lovingly engraved",
    ],
};

// The Blessed: Backstories

static BACKSTORY_YOUR_SACRED_POUCH: stonetop::fixed::BackstoryFixed =
    stonetop::fixed::BackstoryFixed {
        key: stonetop::keys::BackstoryKey::YourSacredPouch,
        name: "Your sacred pouch",
        list: &[
            stonetop::fixed::BackstoryItem::Text(
                "<p>Your sacred pouch (<em>magical</em>) doesn't take up space in your inventory. It can hold up to 3 Stock (sacred herbs, powders, stones, pigments, chalks, clay, and so forth). Each time you gain an even-numbered level, your pouch can hold +1 Stock. When <strong><em>anyone but you looks inside your sacred pouch and touches the materials therein</em></strong>, the Stock is ruined.</p>",
            ),
            stonetop::fixed::BackstoryItem::Text(
                "<p>When you <strong><em>have a few days of downtime in familiar terrain</em></strong>, you may replenish your Stock.</p>",
            ),
            stonetop::fixed::BackstoryItem::Text(
                "<p>When you <strong><em>Forage</em></strong>, you can produce Stock instead of provisions.</p>",
            ),
            stonetop::fixed::BackstoryItem::Text(
                "<p>Your sacred pouch is... (choose 1 on each line)</p>",
            ),
            stonetop::fixed::BackstoryItem::ChoiceRow(stonetop::fixed::TaggedRow {
                tag: "Provenance",
                items: &["an heirloom", "made just for you", "your own work"],
            }),
            stonetop::fixed::BackstoryItem::ChoiceRow(stonetop::fixed::TaggedRow {
                tag: "Materials",
                items: &["fur", "drakescale", "leather", "woven", "demonflesh"],
            }),
            stonetop::fixed::BackstoryItem::ChoiceRow(stonetop::fixed::TaggedRow {
                tag: "Trimmings",
                items: &["unadorned", "beadwork", "rich dyes", "runes"],
            }),
            stonetop::fixed::BackstoryItem::Text(
                "<p>What remarkable trait does it possess? (choose 1)</p>",
            ),
            stonetop::fixed::BackstoryItem::Choices(&[
                "It cannot be cut, torn, or burned by any natural means.",
                "Unless someone is specifically searching for your pouch, they will ignore its presence.",
                "So long as the pouch is sealed, nothing within can be detected or found by magic, nor can anything within escape or affect the outside world.",
                "Unnatural and unclean creatures cannot bear to touch it.",
            ]),
        ],
    };

static BACKSTORY_THE_EARTH_MOTHER: stonetop::fixed::BackstoryFixed =
    stonetop::fixed::BackstoryFixed {
        key: stonetop::keys::BackstoryKey::TheEarthMother,
        name: "The Earth Mother",
        list: &[
            stonetop::fixed::BackstoryItem::Text(
                "<p>Danu has long been revered by all peoples, though not always worshipped or served by priests. In Stonetop's Pavilion of the Gods, Danu's shrine is... (choose 1)</p>",
            ),
            stonetop::fixed::BackstoryItem::Choices(&[
                "... loved, well-used, dripping with offerings and petitions.",
                "... little more than a token of respect, for her holy places are anywhere but here.",
                "... given wide berth by most, and approached only with care and propitiation.",
                "... neglected and all but forgotten, except by a few.",
            ]),
            stonetop::fixed::BackstoryItem::Text(
                "<p>What do the folk of Stonetop leave as offerings? (choose 2-3)</p>",
            ),
            stonetop::fixed::BackstoryItem::Choices(&[
                "fruits of harvest",
                "whisky/spirits",
                "pure rain water",
                "blood/burnt flesh",
                "figurines/effigies",
                "salt/crystals",
                "metal nails/tools",
                "incense/sage bark",
            ]),
        ],
    };

// The Fox: Backstories

static BACKSTORY_TALL_TALES: stonetop::fixed::BackstoryFixed = stonetop::fixed::BackstoryFixed {
    key: stonetop::keys::BackstoryKey::TallTales,
    name: "Tall tales",
    list: &[
        stonetop::fixed::BackstoryItem::Text(
            "<p>Someone like you gets into all sorts of trouble, whether you mean to or not. Mix and match the following to come up with a couple of your more memorable adventures, and write them down in the space at the bottom of this column.</p>",
        ),
        stonetop::fixed::BackstoryItem::Text(
            "<p>There was that time that you… (choose 1 per tale)</p>",
        ),
        stonetop::fixed::BackstoryItem::Text(
            "<ul><li>got lost in (choose 1) the Great Wood, or the Flats, or the Steplands, or Ferrier's Fen, or the Foothills, or the Huffel Peaks</li><li>were on watch when the crinwin raided</li><li>dared each other to explore the Ruined Tower</li><li>managed to rile up a small band of Hillfolk</li><li>braved the Labyrinth, just a little</li><li>stole that crazy old man's book</li><li>went poking about the old Barrow Mounds</li></ul>",
        ),
        stonetop::fixed::BackstoryItem::Text("<p>And you ended up… (choose 1 or 2 per tale)</p>"),
        stonetop::fixed::BackstoryItem::Text(
            "<ul><li>running for your life from ▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁</li><li>landing a well-placed blow</li><li>interrupting a strange, creepy gathering</li><li>stumbling on a beast, bigger'n anything</li><li>with a sack full of treasure</li><li>getting ▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁ to fight them for you</li><li>face to face with a ghost/Fae/demon</li><li>finding those strange old runes</li><li>getting to know that fine-looking fellow/lady/person/couple</li></ul>",
        ),
        stonetop::fixed::BackstoryItem::Text("<p>But all you've got left to show for it is…</p>"),
        stonetop::fixed::BackstoryItem::Text(
            "<ul><li>a story no one believes.</li><li>a nasty scar; wanna see?</li><li>the occasional nightmare.</li><li>this map with runes no one can read.</li><li>this key that opens who-knows-what.</li></ul>",
        ),
    ],
};

// The Heavy: Backstories

static BACKSTORY_A_HISTORY_OF_VIOLENCE: stonetop::fixed::BackstoryFixed =
    stonetop::fixed::BackstoryFixed {
        key: stonetop::keys::BackstoryKey::AHistoryOfViolence,
        name: "A history of violence",
        list: &[
            stonetop::fixed::BackstoryItem::Text(
                "<p>Just about everyone here talks about the time you… (pick 1 or 2)</p>",
            ),
            stonetop::fixed::BackstoryItem::Choices(&[
                "… drove off a thunder drake that got too close to town.",
                "… killed that hagr in the Foothills.",
                "… slew a dozen crinwin in one battle.",
                "… tossed those adventurers out of town.",
                "… bested Ivan, the scariest bandit in Brennan's gang, the Claws.",
                "… dragged yourself (and another?) into town, bleeding from a dozen wounds.",
                "… were struck by lightning and woke up covered in these marks.",
            ]),
            stonetop::fixed::BackstoryItem::Text(
                "<p>But folks are less keen to discuss… (pick 1 or 2)</p>",
            ),
            stonetop::fixed::BackstoryItem::Choices(&[
                "… the look in your eye when you spilled all that blood.",
                "… those hard cases who showed up looking for you.",
                "… the shouting matches between you and your love.",
                "… the time you spent as one of Brennan's Claws.",
                "… what happened to Urbgen, even if he did have it coming.",
                "… your uncontrollable seizures, where you claw those weird marks in the dirt.",
            ]),
            stonetop::fixed::BackstoryItem::Text(
                "<p>What keeps you up at night? (pick 1 or 2)</p>",
            ),
            stonetop::fixed::BackstoryItem::Choices(&[
                "That thrice-damned temper of yours.",
                "The worry that someone's coming after you.",
                "The feeling that the crinwin are getting bolder.",
                "Wondering what Brennan's up to, now that he's the marshal of Marshedge.",
                "Dark visions of things moving in the earth, restless, whispering, and hungry.",
                "The question of who'll look after your family when you get yourself killed.",
                "The worry that they'll all learn the truth about you, sooner or later.",
            ]),
        ],
    };

// The Judge: Backstories

static BACKSTORY_THE_CHRONICLE: stonetop::fixed::BackstoryFixed = stonetop::fixed::BackstoryFixed {
    key: stonetop::keys::BackstoryKey::TheChronicle,
    name: "The Chronicle",
    list: &[
        stonetop::fixed::BackstoryItem::Text(
            "<p>The Judge of Aratis is charged with maintaining the Chronicle, a history of the community, its people, their knowledge, and their traditions. The nature of the lore contained in the Chronicle depends on your Background, but it is more than a mere book; it is a physical place. Decide on its physical structure.</p>",
        ),
        stonetop::fixed::BackstoryItem::Text("<p>On the plus side, it… (choose 3)</p>"),
        stonetop::fixed::BackstoryItem::Choices(&[
            "… is a sturdy vault from the time of the Makers.",
            "… has plenty of room to grow.",
            "… is hidden underground.",
            "… has but one entrance, magically sealed.",
            "… bears minor magics to preserve its contents.",
            "… is warded against spirits and magic.",
            "… includes your living quarters & office.",
        ]),
        stonetop::fixed::BackstoryItem::Text("<p>But alas, it… (choose 2)</p>"),
        stonetop::fixed::BackstoryItem::Choices(&[
            "… sits on the outskirts, near the Old Wall.",
            "… is cramped, chaotic, and overflowing.",
            "… is little more than a crude cellar.",
            "… seems to be haunted.",
            "… contains a few dangerous artifacts.",
        ]),
        stonetop::fixed::BackstoryItem::Text(
            "<p>Mark the location of the Chronicle on the Stonetop Playbook map.</p>",
        ),
    ],
};

static BACKSTORY_THE_LAWKEEPER: stonetop::fixed::BackstoryFixed = stonetop::fixed::BackstoryFixed {
    key: stonetop::keys::BackstoryKey::TheLawkeeper,
    name: "The Lawkeeper",
    list: &[
        stonetop::fixed::BackstoryItem::Text(
            "<p>Her Judges say that Aratis has been with humanity since they first stacked one stone upon another and called it home.</p>",
        ),
        stonetop::fixed::BackstoryItem::Text(
            "<p>In Stonetop's Pavilion of the Gods, Aratis's shrine is… (pick 1)</p>",
        ),
        stonetop::fixed::BackstoryItem::Choices(&[
            "… a hub of the community, a place of frequent rites, petitions, and celebrations",
            "… used only on high holidays, for each home keeps its own shrine above the hearth",
            "… neglected by most, tended only by you and a handful of believers",
            "… a grim place of judgement and punishment, shunned by all but her chosen",
            "… newly established, cramped and spare",
        ]),
        stonetop::fixed::BackstoryItem::Text(
            "<p>Of her true disciples, Aratis demands… (choose 3)</p>",
        ),
        stonetop::fixed::BackstoryItem::Choices(&[
            "… truth, honesty, and forthrightness",
            "… hospitality, freely given to all who ask for it",
            "… the punishment of thieves & oathbreakers",
            "… adherence to strict rules of diet and dress",
            "… respect for authority, property, and rank",
        ]),
    ],
};

// The Lightbearer: Backstories

static BACKSTORY_PRAISE_THE_DAY: stonetop::fixed::BackstoryFixed =
    stonetop::fixed::BackstoryFixed {
        key: stonetop::keys::BackstoryKey::PraiseTheDay,
        name: "Praise the day",
        list: &[
            stonetop::fixed::BackstoryItem::Text(
                "<p>You are the appointed servant of Helior the Day-bringer, god of the sun and light, beacon of hope and mercy.</p>",
            ),
            stonetop::fixed::BackstoryItem::Text("<p>The worship of Helior is… (choose 1)</p>"),
            stonetop::fixed::BackstoryItem::Choices(&[
                "… ancient, widespread, and well-known",
                "… most common in Lygos and the south",
                "… a new thing, still unheard of by many",
                "… an old thing, forgotten by most",
                "… widely persecuted",
            ]),
            stonetop::fixed::BackstoryItem::Text(
                "<p>He is worshipped through… (choose 1 or 2)</p>",
            ),
            stonetop::fixed::BackstoryItem::Choices(&[
                "… solemn hymns",
                "… serene meditation",
                "… joyful song",
                "… ascetic denial",
                "… fervent dancing",
                "… formal ceremonies",
                "… drugs & intoxicants",
                "… pain & sacrifice",
            ]),
            stonetop::fixed::BackstoryItem::Text(
                "<p>In Stonetop's Pavilion of the Gods, Helior's shrine has… (choose 1)</p>",
            ),
            stonetop::fixed::BackstoryItem::Choices(&[
                "… the place of highest honor, even if Tor is more popular",
                "… been well-tended and given due respect",
                "… recently been restored/established, perhaps by you",
                "… seen better days, for certain",
            ]),
            stonetop::fixed::BackstoryItem::Text(
                "<p>Your predecessor, the previous Lightbearer… (choose 2 or 3)</p>",
            ),
            stonetop::fixed::BackstoryItem::Choices(&[
                "… lived long ago, a figure of legend",
                "… was martyred for their faith",
                "… died facing a mighty sorcerer or demon",
                "… wrote many works of sublime beauty",
                "… faced one of the Things Below",
                "… died in their bed, peacefully",
                "… ascended bodily into the heavens",
                "… was reincarnated—as you",
            ]),
            stonetop::fixed::BackstoryItem::Text("<p>You came into your powers… (choose 1)</p>"),
            stonetop::fixed::BackstoryItem::Choices(&[
                "… through years of study and devotion",
                "… when your predecessor passed them on",
                "… suddenly, at a moment of great need",
                "… after a visitation from Helior or one of his servants",
                "… when you first laid eyes upon the ▁▁▁▁▁▁▁▁",
            ]),
        ],
    };

// The Marshal: Backstories

static BACKSTORY_WAR_STORIES: stonetop::fixed::BackstoryFixed = stonetop::fixed::BackstoryFixed {
    key: stonetop::keys::BackstoryKey::WarStories,
    name: "War stories",
    list: &[
        stonetop::fixed::BackstoryItem::Text(
            "<p>The last time the militia saw serious action, it was... (pick 1)</p>",
        ),
        stonetop::fixed::BackstoryItem::Choices(&[
            "...to repel a nighttime raid by crinwin from the Great Wood.",
            "...to drive off bandits who'd taken up near the Ruined Tower.",
            "...to fend off Hillfolk pursuing a blood feud.",
            "...against Brennan and his Claws, before they settled in Marshedge.",
            "...to face a brutish hagr, come down from the Foothills to wreak havoc.",
            "...to hunt down beasts (wolves, drakes, or bears maybe?) who'd been preying on the village.",
        ]),
        stonetop::fixed::BackstoryItem::Text(
            "<p>Answer at least 3 of the following questions about that action:</p>",
        ),
        stonetop::fixed::BackstoryItem::Choices(&[
            "When exactly did it happen?",
            "Who lost their life, and who mourns them?",
            "Who from Stonetop was maimed, and how?",
            "Who saved the day, and how?",
            "How did the enemy get away, and whom do you still blame for it?",
            "Who comported themselves with honor?",
            "What's been bugging you about it ever since?",
            "What's got you even more worried now?",
        ]),
    ],
};

// The Ranger: Backstories

static BACKSTORY_SOMETHING_WICKED_THIS_WAY_COMES: stonetop::fixed::BackstoryFixed =
    stonetop::fixed::BackstoryFixed {
        key: stonetop::keys::BackstoryKey::SomethingWickedThisWayComes,
        name: "Something wicked this way comes",
        list: &[
            stonetop::fixed::BackstoryItem::Text(
                "<p>You know firsthand that trouble is out there, and like it or not, one of these days the folk of Stonetop are going to have to face it. What is it that you're so worried about? (choose 1)</p>",
            ),
            stonetop::fixed::BackstoryItem::Choices(&[
                "A dark, unwholesome presence lurking in the Great Wood",
                "A strange, furtive figure seen near the Ruined Tower",
                "Something big & savage stalking the northern foothills",
                "Whatever's made the lizard-like suarachan of Ferrier's Fen so bold",
                "That of which the Hillfolk refuse to speak",
            ]),
            stonetop::fixed::BackstoryItem::Text(
                "<p>Then, answer at least 3 of the following questions about this threat:</p>",
            ),
            stonetop::fixed::BackstoryItem::Choices(&[
                "What, exactly, do you think it is?",
                "What did you see, and how close did you have to get to see it?",
                "Whom or what have you lost to it?",
                "What did it leave behind?",
                "What do you think it wants?",
                "Who refuses to believe you?",
                "Who can tell you more, if you can only convince them?",
            ]),
        ],
    };

// The Seeker: Backstories

static BACKSTORY_COLLECTION: stonetop::fixed::BackstoryFixed = stonetop::fixed::BackstoryFixed {
    key: stonetop::keys::BackstoryKey::Collection,
    name: "Collection",
    list: &[stonetop::fixed::BackstoryItem::Text(
        "<p>In your travels and investigations you have acquired arcana—artifacts of power and mystery.</p>",
    )],
};

static BACKSTORY_MAJOR_ARCANA: stonetop::fixed::BackstoryFixed = stonetop::fixed::BackstoryFixed {
    key: stonetop::keys::BackstoryKey::MajorArcana,
    name: "Major Arcana",
    list: &[
        stonetop::fixed::BackstoryItem::Text(
            "<p>Your Background grants you 1 major arcanum. Answer at least 2 questions about it:</p>",
        ),
        stonetop::fixed::BackstoryItem::Choices(&[
            "Where did you acquire it?",
            "From whose grasp did you wrest it?",
            "Who else wants it?",
            "What did it cost you?",
        ]),
        stonetop::fixed::BackstoryItem::Text(
            "<p>You've begun to unlock the mysteries of your major arcanum; mark 1 ☐ or ○ on the front of its insert.</p>",
        ),
        stonetop::fixed::BackstoryItem::Text("<p>When and how did that happen?</p>"),
        stonetop::fixed::BackstoryItem::Text("<p>▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁</p>"),
    ],
};

static BACKSTORY_MINOR_ARCANA: stonetop::fixed::BackstoryFixed = stonetop::fixed::BackstoryFixed {
    key: stonetop::keys::BackstoryKey::MinorArcana,
    name: "Minor Arcana",
    list: &[
        stonetop::fixed::BackstoryItem::Text(
            "<p>Ask the GM for the minor arcana cards. Draw 3 at random and review both sides.</p>",
        ),
        stonetop::fixed::BackstoryItem::Text(
            "<p>Choose one whose secrets you have unlocked. If it's portable, you either keep it on your person or hidden away somewhere safe. Where is it now? How did you come to master it?</p>",
        ),
        stonetop::fixed::BackstoryItem::Text("<p>▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁</p>"),
        stonetop::fixed::BackstoryItem::Text(
            "<p>Choose another, which you have not yet mastered. It is either in your possession or in a secret place known only to you. Where is it? How did you find it?</p>",
        ),
        stonetop::fixed::BackstoryItem::Text("<p>▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁</p>"),
        stonetop::fixed::BackstoryItem::Text(
            "<p>The third you have not yet found, but you have a lead on it. Give the card back to the GM, but make note of it below. During play, ask the GM what you know about it.</p>",
        ),
        stonetop::fixed::BackstoryItem::Text("<p>▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁</p>"),
    ],
};

// The Would-be Hero: Backstories

static BACKSTORY_FEAR_ANGER: stonetop::fixed::BackstoryFixed = stonetop::fixed::BackstoryFixed {
    key: stonetop::keys::BackstoryKey::FearAnger,
    name: "Fear & anger",
    list: &[
        stonetop::fixed::BackstoryItem::Text("<p>What do you fear most? (choose 1, maybe 2)</p>"),
        stonetop::fixed::BackstoryItem::Choices(&[
            "Fire, burning, the smell of charred flesh",
            "That they won't take you seriously",
            "That you really aren't cut out for this",
            "The death of your family or loved ones",
            "Being alone and helpless",
            "Violence, bloodshed, and pain",
            "Monsters",
            "What you're capable of",
            "What you must do",
        ]),
        stonetop::fixed::BackstoryItem::Text(
            "<p>What makes you burn with righteous anger? (choose 2, maybe 3)</p>",
        ),
        stonetop::fixed::BackstoryItem::Choices(&[
            "Bullying, slavery, and oppression",
            "Wanton cruelty and unnecessary suffering",
            "Injustice and inequality",
            "Cowardice, treachery, and selfishness",
            "The despoiling of beauty and innocence",
            "Threats to your loved ones",
            "Violence to children, animals, the innocent",
            "Perversions of nature",
        ]),
        stonetop::fixed::BackstoryItem::Text(
            "<p>When did your fear or anger last cause you trouble?</p>",
        ),
        stonetop::fixed::BackstoryItem::Text("<p>▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁</p>"),
        stonetop::fixed::BackstoryItem::Text("<p>What did you do?</p>"),
        stonetop::fixed::BackstoryItem::Text("<p>▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁</p>"),
        stonetop::fixed::BackstoryItem::Text("<p>How did it turn out?</p>"),
        stonetop::fixed::BackstoryItem::Text("<p>▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁</p>"),
    ],
};

// The Blessed

static PLAYBOOK_THE_BLESSED: stonetop::fixed::PlaybookFixed = stonetop::fixed::PlaybookFixed {
    key: stonetop::keys::PlaybookKey::TheBlessed,
    name: "The Blessed",
    description: "Danu, the Great Mother, provides. We need only learn her secrets: the names by which the trees call each other; the mark made with redberry juice to ward off impure spirits; the language of the wolves. A thousand such secrets Danu keeps, to share with only her true children. Her Blessed.",
    backgrounds: [&BACKGROUND_INITIATE, &BACKGROUND_RAISED_BY_WOLVES, &BACKGROUND_VESSEL],
    instinct: [
        stonetop::fixed::Instinct {
            title: "Delight",
            description: "To find beauty, in even the ugliest things.",
        },
        stonetop::fixed::Instinct {
            title: "Detachment",
            description: "To remain unmoved, to be cold as winter.",
        },
        stonetop::fixed::Instinct {
            title: "Nurture",
            description: "To help others grow, learn, or improve.",
        },
        stonetop::fixed::Instinct {
            title: "Preservation",
            description: "To protect the natural world.",
        },
        stonetop::fixed::Instinct {
            title: "Reverence",
            description: "To honor the spirits and give them their due.",
        },
    ],
    appearance: [
        stonetop::fixed::TaggedRow {
            tag: "Age",
            items: &["fresh-faced", "hale & hearty", "gray & wizened"],
        },
        stonetop::fixed::TaggedRow {
            tag: "Voice",
            items: &["imperious voice", "raspy voice", "soothing voice"],
        },
        stonetop::fixed::TaggedRow {
            tag: "Build",
            items: &["curvy", "strapping", "rail-thin", "solid", "willowy"],
        },
        stonetop::fixed::TaggedRow {
            tag: "Clothing",
            items: &["ceremonial robes", "furs, leather", "work clothes"],
        },
    ],
    origin_choices: &[
        stonetop::fixed::Origin {
            location: "Stonetop",
            naming: stonetop::fixed::Naming::Names(&[
                "Arwel", "Blodwen", "Brynmor", "Celyn", "Fflur", "Gwynn", "Tegwen", "Winned",
            ]),
        },
        stonetop::fixed::Origin {
            location: "Barrier Pass",
            naming: stonetop::fixed::Naming::Names(&[
                "Alagh", "Bora", "Chambui", "Enebish", "Jalakai", "Kamala", "Sechen", "Todogen",
            ]),
        },
        stonetop::fixed::Origin {
            location: "The Steplands (Hillfolk)",
            naming: stonetop::fixed::Naming::Names(&[
                "Bejn", "Decla", "Franza", "Irv", "Ivet", "Jak", "Sibl", "Yez",
            ]),
        },
        stonetop::fixed::Origin {
            location: "The Wild",
            naming: stonetop::fixed::Naming::MixAndMatch(stonetop::fixed::NameParts {
                intro: "mix and match 1-3 of these:",
                name_parts: &[
                    stonetop::fixed::TaggedRow {
                        tag: "A B C",
                        items: &[
                            "Autumn", "Badger", "Big", "Black", "Bloody", "Brave", "Crow", "Cub",
                        ],
                    },
                    stonetop::fixed::TaggedRow {
                        tag: "D F G",
                        items: &[
                            "Dark", "Doe", "Fang", "Fierce", "Flower", "Gentle", "Green", "Grim",
                        ],
                    },
                    stonetop::fixed::TaggedRow {
                        tag: "H L O P",
                        items: &["Hart", "Leaf", "Little", "Lonely", "Old", "Owl", "Pale", "Pup"],
                    },
                    stonetop::fixed::TaggedRow {
                        tag: "Q R S",
                        items: &[
                            "Quick", "Quiet", "Rain", "Red", "Sharp", "Snake", "Snow", "Spring",
                            "Summer",
                        ],
                    },
                    stonetop::fixed::TaggedRow {
                        tag: "T Y W",
                        items: &[
                            "Tall", "Tree", "Yellow", "White", "Wind", "Winter", "Wolf", "Whisper",
                        ],
                    },
                ],
            }),
        },
    ],
    stats_to_assign: [2i8, 1i8, 1i8, 0i8, 0i8, -1i8],
    damage: stonetop::Die::D6,
    hp: 18u8,
    special_possessions: stonetop::fixed::SpecialPossessions {
        pick_note: "Pick 2, in addition to your sacred pouch",
        pick_count: 2u8,
        preselected: 1u8,
        options: &[
            &SPECIAL_POSSESSION_SACRED_POUCH,
            &SPECIAL_POSSESSION_APIARY,
            &SPECIAL_POSSESSION_COLLECTED_OFFERINGS,
            &SPECIAL_POSSESSION_GOAT_HERD,
            &SPECIAL_POSSESSION_HERB_GARDEN,
            &SPECIAL_POSSESSION_MASTIFFS,
        ],
    },
    starting_move_choices: 1u8,
    grants_moves: &[
        stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::SpiritTongue),
        stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::CallTheSpirits),
    ],
    moves: &[
        &MOVE_AMULETS_TALISMANS,
        &MOVE_BARKSKIN,
        &MOVE_BIG_MAGIC,
        &MOVE_DANUS_GRASP,
        &MOVE_HEALERS_ARTS,
        &MOVE_HEED_MY_WORDS,
        &MOVE_IMPROVED_STAT,
        &MOVE_INTO_THE_LIONS_DEN,
        &MOVE_LIGHTNING_ROD,
        &MOVE_RITES_OF_THE_LAND,
        &MOVE_SPIRIT_TONGUE,
        &MOVE_BORROW_POWER,
        &MOVE_CALL_THE_SPIRITS,
        &MOVE_TRACKLESS_STEP,
        &MOVE_VEIL,
        &MOVE_WARDS_BINDINGS,
        &MOVE_WILD_SOUL,
        &MOVE_NATURES_WRATH,
        &MOVE_POTENT_WORKINGS,
        &MOVE_SHARED_SOULS,
        &MOVE_SUCK_THE_POISON_OUT,
        &MOVE_SUPERIOR_STAT,
        &MOVE_VOICE_OF_THE_EARTH_MOTHER,
    ],
    moves_footnote: None,
    intro: stonetop::fixed::Intro {
        title: "Introductions",
        text: "<p>Wait here for everyone else. When everyone's ready, take turns introducing your characters. When <strong><em>someone reveals something and you want to know more</em></strong>, ask them about it. When <strong><em>someone asks you a question</em></strong>, answer it truthfully.</p><ol><li>On your first turn, <strong>introduce yourself</strong> by name, pronouns, background, origin, and appearance.</li><li>On your second turn, <strong>describe your special possessions</strong> and how you contribute to the village (beyond working the fields).</li><li>On your third turn, <strong>describe your sacred pouch</strong> and its remarkable trait. Then, <strong>tell us about Danu's shrine</strong> in Stonetop and how she is worshipped.</li><li>On your next turn, <strong>answer one of the following</strong>, naming one or more NPCs who live in Stonetop.<ul><li>☐ Who is your closest kin?</li><li>☐ Whose heart & soul is entwined with yours?</li><li>☐ Who taught you the secret ways?</li><li>☐ Who is beloved by the goddess, your charge to nurture/guide/protect/heal?</li></ul></li><li>Go around again. Answer another question from 4, or pass. When everyone has passed, go on.</li><li>On your next turn, <strong>ask your fellow PCs one of these</strong>. When others ask you, answer as you like.<ul><li>☐ Which one of you do the spirits whisper of?</li><li>☐ Which one of you has joined me in a sacred rite?</li><li>☐ Which of you has made a blood-oath with me?</li><li>☐ Which one of you doubts the power of Danu?</li></ul></li><li>Go around again. Ask another question from 6, or pass. When everyone has passed, go on.</li><li>Add your home to the steading playbook. When everyone is done, let spring break forth!</li></ol>",
    },
    backstory: &[&BACKSTORY_YOUR_SACRED_POUCH, &BACKSTORY_THE_EARTH_MOTHER],
};

// The Fox

static PLAYBOOK_THE_FOX: stonetop::fixed::PlaybookFixed = stonetop::fixed::PlaybookFixed {
    key: stonetop::keys::PlaybookKey::TheFox,
    name: "The Fox",
    description: "The elders tell a story about Fox, who knows lots of tricks, and Hedgehog, who knows one: how to curl up into a ball when there's danger. Fox can't eat Hedgehog when he's all curled up, so in the story Fox goes hungry. But you're not that Fox, and this is no story. You want that Hedgehog? Go get a knife.",
    backgrounds: [
        &BACKGROUND_THE_NATURAL,
        &BACKGROUND_A_LIFE_OF_CRIME,
        &BACKGROUND_THE_PRODIGAL_RETURNED,
    ],
    instinct: [
        stonetop::fixed::Instinct {
            title: "Conscience",
            description: "To feel guilty, to try to do right.",
        },
        stonetop::fixed::Instinct {
            title: "Freedom",
            description: "To chafe against rules, expectations, obligations.",
        },
        stonetop::fixed::Instinct {
            title: "Comfort",
            description: "To enjoy yourself and avoid hardship.",
        },
        stonetop::fixed::Instinct {
            title: "Prestige",
            description: "To impress others, to build a name for yourself.",
        },
        stonetop::fixed::Instinct {
            title: "Trickery",
            description: "To deceive, misdirect, outthink.",
        },
    ],
    appearance: [
        stonetop::fixed::TaggedRow {
            tag: "Age",
            items: &["young pup", "\"responsible\" adult", "cagey old-timer"],
        },
        stonetop::fixed::TaggedRow {
            tag: "Voice",
            items: &["a pleasant voice", "sharp & nasally", "well-spoken"],
        },
        stonetop::fixed::TaggedRow {
            tag: "Build",
            items: &["lithe", "heavyset", "gangly", "like a whippin' stick"],
        },
        stonetop::fixed::TaggedRow {
            tag: "Pace",
            items: &["a light step", "a brisk stride", "more like a strut"],
        },
    ],
    origin_choices: &[
        stonetop::fixed::Origin {
            location: "Stonetop",
            naming: stonetop::fixed::Naming::Names(&[
                "Bran", "Carwyn", "Delyth", "Elin", "Fion", "Geral", "Mair", "Rannon", "Vaughn",
                "Wynn",
            ]),
        },
        stonetop::fixed::Origin {
            location: "Barrier Pass",
            naming: stonetop::fixed::Naming::Names(&[
                "Anarba",
                "Batu",
                "Bugadai",
                "Hujaghur",
                "Jigur",
                "Kete",
                "Sarantuya",
                "Tebengri",
            ]),
        },
        stonetop::fixed::Origin {
            location: "Gordin's Delve",
            naming: stonetop::fixed::Naming::Instructions("Pick a name from any list."),
        },
        stonetop::fixed::Origin {
            location: "Marshedge",
            naming: stonetop::fixed::Naming::Names(&[
                "Comyna", "Crevan", "Fitz", "Greagir", "Maired", "Nainsi", "Naiclas", "Saraid",
            ]),
        },
        stonetop::fixed::Origin {
            location: "Lygos or some other point south",
            naming: stonetop::fixed::Naming::Names(&[
                "Amit", "Baz", "Dafna", "Mahsa", "Parviz", "Sanaz", "Tzofiya", "Yaniv",
            ]),
        },
    ],
    stats_to_assign: [2i8, 1i8, 1i8, 0i8, 0i8, -1i8],
    damage: stonetop::Die::D8,
    hp: 16u8,
    special_possessions: stonetop::fixed::SpecialPossessions {
        pick_note: "Pick 2",
        pick_count: 2u8,
        preselected: 0u8,
        options: &[
            &SPECIAL_POSSESSION_BURGLARS_KIT,
            &SPECIAL_POSSESSION_CARPENTERS_TOOLS,
            &SPECIAL_POSSESSION_DISTILLERY,
            &SPECIAL_POSSESSION_HIDDEN_STASH,
            &SPECIAL_POSSESSION_MUMMERS_KIT,
            &SPECIAL_POSSESSION_SCRIBES_TOOLS,
            &SPECIAL_POSSESSION_TANNERY,
            &SPECIAL_POSSESSION_TRADE_CONTACTS,
        ],
    },
    starting_move_choices: 1u8,
    grants_moves: &[
        stonetop::fixed::GrantMove::ChooseOne(
            stonetop::keys::MoveKey::Ambush,
            stonetop::keys::MoveKey::SkillAtArms,
        ),
        stonetop::fixed::GrantMove::ChooseOne(
            stonetop::keys::MoveKey::DangerSense,
            stonetop::keys::MoveKey::Perceptive,
        ),
    ],
    moves: &[
        &MOVE_ALL_IN_THE_WRIST,
        &MOVE_AMBUSH,
        &MOVE_BURGLE,
        &MOVE_CATLIKE,
        &MOVE_DABBLER,
        &MOVE_DANGER_SENSE,
        &MOVE_FREE_RUNNING,
        &MOVE_IMPROVED_STAT,
        &MOVE_IRRESISTIBLE,
        &MOVE_LAUGH_AT_DANGER,
        &MOVE_LIGHT_FINGERS,
        &MOVE_PERCEPTIVE,
        &MOVE_RAPIER_WIT,
        &MOVE_SKILL_AT_ARMS,
        &MOVE_PARRY_RIPOSTE,
        &MOVE_SILVER_TONGUED,
        &MOVE_UNDER_YOUR_SKIN,
        &MOVE_BATTLE_DANCER,
        &MOVE_CHEAP_SHOT,
        &MOVE_EYE_ON_THE_DOOR,
        &MOVE_PANTS_ON_FIRE,
        &MOVE_SECOND_INTENT,
        &MOVE_SLIPPERY,
        &MOVE_SUPERIOR_STAT,
    ],
    moves_footnote: None,
    intro: stonetop::fixed::Intro {
        title: "Introductions",
        text: "<p>Wait here for everyone else. When everyone's ready, take turns introducing your characters. When <strong><em>someone reveals something and you want to know more</em></strong>, ask them about it. When <strong><em>someone asks you a question</em></strong>, answer it truthfully.</p><ol><li>On your first turn, <strong>introduce yourself</strong> by name, pronouns, background, origin, and appearance.</li><li>On your second turn, <strong>describe your special possessions</strong> and how you contribute to the village (beyond working the fields).</li><li>On your third turn, <strong>tell us your tall tales</strong>. Feel free to embellish and exaggerate to the other players, but always answer the GM truthfully.</li><li>On your next turn, <strong>answer one of the following</strong>, naming one or more NPCs who live in Stonetop.<ul><li>☐ Who is your closest kin?</li><li>☐ Who holds the reins to your heart?</li><li>☐ Whose respect means the world to you?</li><li>☐ To whom do you owe a debt that cannot be repaid?</li></ul></li><li>Go around again. Answer another question from 4, or pass. When everyone has passed, go on.</li><li>On your next turn, <strong>ask your fellow PCs one of these</strong>. When others ask you, answer as you like.<ul><li>☐ Which one of you joined me in my latest hijinx?</li><li>☐ Which one of you brings your problems to me?</li><li>☐ Which one of you saved my bacon, mor'n once?</li><li>☐ Which one of you trusts me not one bit?</li></ul></li><li>Go around again. Ask another question from 6, or pass. When everyone has passed, go on.</li><li>Add your home to the steading playbook. When everyone is done, let spring break forth!</li></ol>",
    },
    backstory: &[&BACKSTORY_TALL_TALES],
};

// The Heavy

static PLAYBOOK_THE_HEAVY: stonetop::fixed::PlaybookFixed = stonetop::fixed::PlaybookFixed {
    key: stonetop::keys::PlaybookKey::TheHeavy,
    name: "The Heavy",
    description: "These are good people. Hard-working, honest. They look out for each other. But sometimes, looking out for each other ain't enough. Sometimes, good people need someone to stick up for them. Someone who's not afraid to get a little bloody. To get heavy. Yeah, someone like you.",
    backgrounds: [&BACKGROUND_SHERIFF, &BACKGROUND_BLOOD_SOAKED_PAST, &BACKGROUND_STORM_MARKED],
    instinct: [
        stonetop::fixed::Instinct {
            title: "Peace",
            description: "To avoid (further) bloodshed or violence.",
        },
        stonetop::fixed::Instinct {
            title: "Pride",
            description: "To maintain your dignity, to demand respect.",
        },
        stonetop::fixed::Instinct {
            title: "Recklessness",
            description: "To act without thought to the consequences.",
        },
        stonetop::fixed::Instinct {
            title: "Trouble",
            description: "To stick your nose in where it's unwelcome.",
        },
        stonetop::fixed::Instinct { title: "Violence", description: "To solve problems by force." },
    ],
    appearance: [
        stonetop::fixed::TaggedRow {
            tag: "Age",
            items: &["young & brash", "in my prime", "old & leathery"],
        },
        stonetop::fixed::TaggedRow {
            tag: "Voice",
            items: &["gravelly voice", "hearty voice", "soft-spoken"],
        },
        stonetop::fixed::TaggedRow {
            tag: "Build",
            items: &["giant frame", "just ripped", "stocky", "wiry"],
        },
        stonetop::fixed::TaggedRow {
            tag: "Marks",
            items: &["distinctive scars", "oft-broken nose", "missing bits"],
        },
    ],
    origin_choices: &[
        stonetop::fixed::Origin {
            location: "Stonetop",
            naming: stonetop::fixed::Naming::Names(&[
                "Aerona", "Arthfael", "Cadmor", "Esyllt", "Pedr", "Rhonwen", "Terrwen", "Trystan",
            ]),
        },
        stonetop::fixed::Origin {
            location: "Gordin's Delve",
            naming: stonetop::fixed::Naming::Instructions("Pick a name from any list"),
        },
        stonetop::fixed::Origin {
            location: "Marshedge",
            naming: stonetop::fixed::Naming::Names(&[
                "Aengus", "Bairbre", "Bronach", "Flann", "Laughn", "Muirdoc", "Quinn", "Treasa",
            ]),
        },
        stonetop::fixed::Origin {
            location: "The Steplands (Hillfolk)",
            naming: stonetop::fixed::Naming::Names(&[
                "Andr", "Gabrl", "Kaetl", "Mael", "Maela", "Par", "Ral", "Umbert",
            ]),
        },
        stonetop::fixed::Origin {
            location: "The Manmarch",
            naming: stonetop::fixed::Naming::Names(&[
                "Bathhilde",
                "Clothar",
                "Ganter",
                "Hiltrude",
                "Ludig",
                "Luise",
                "Modd",
                "Wiland",
            ]),
        },
        stonetop::fixed::Origin {
            location: "Lygos or some other point south",
            naming: stonetop::fixed::Naming::Names(&[
                "Arihl", "Akios", "Bhadur", "Seble", "Shahnaz", "Shay", "Tisi", "Zubin",
            ]),
        },
    ],
    stats_to_assign: [2i8, 1i8, 1i8, 0i8, 0i8, -1i8],
    damage: stonetop::Die::D10,
    hp: 20u8,
    special_possessions: stonetop::fixed::SpecialPossessions {
        pick_note: "Pick 2",
        pick_count: 2u8,
        preselected: 0u8,
        options: &[
            &SPECIAL_POSSESSION_DISTILLERY,
            &SPECIAL_POSSESSION_CHIRURGEONS_TOOLS,
            &SPECIAL_POSSESSION_HUSBANDRY_TOOLS,
            &SPECIAL_POSSESSION_SMITHY,
            &SPECIAL_POSSESSION_STONEWORKERS_TOOLS,
            &SPECIAL_POSSESSION_PH_WEAPONS_OF_WAR,
        ],
    },
    starting_move_choices: 0u8,
    grants_moves: &[
        stonetop::fixed::GrantMove::ChooseOne(
            stonetop::keys::MoveKey::Armored,
            stonetop::keys::MoveKey::UncannyReflexes,
        ),
        stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::Dangerous),
        stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::HardToKill),
    ],
    moves: &[
        &MOVE_ARMORED,
        &MOVE_BATTLE_JOY,
        &MOVE_BERSERKER,
        &MOVE_CARVED_OUT_OF_WOOD,
        &MOVE_DANGEROUS,
        &MOVE_FORMIDABLE,
        &MOVE_FROSTY,
        &MOVE_GUARDIAN,
        &MOVE_IMPROVED_STAT,
        &MOVE_INTIMIDATING,
        &MOVE_HARD_TO_KILL,
        &MOVE_UNSTOPPABLE,
        &MOVE_MUSCLEBOUND,
        &MOVE_PAYBACK,
        &MOVE_RELENTLESS,
        &MOVE_SEASONED_WARRIOR,
        &MOVE_SITUATIONAL_AWARENESS,
        &MOVE_UNCANNY_REFLEXES,
        &MOVE_UNFETTERED,
        &MOVE_TERROR_ON_THE_FIELD,
        &MOVE_BRINGER_OF_RUIN,
        &MOVE_CUT_FROM_GRANITE,
        &MOVE_MIGHTY_THEWS,
        &MOVE_NEMESIS,
        &MOVE_STEADFAST_GUARDIAN,
        &MOVE_STONE_COLD,
        &MOVE_SUPERIOR_STAT,
    ],
    moves_footnote: None,
    intro: stonetop::fixed::Intro {
        title: "Introductions",
        text: "<p>Wait here for everyone else. When everyone's ready, take turns introducing your characters. When <strong><em>someone reveals something and you want to know more</em></strong>, ask them about it. When <strong><em>someone asks you a question</em></strong>, answer it truthfully.</p><ol><li>On your first turn, <strong>introduce yourself</strong> by name, pronouns, background, origin, and appearance.</li><li>On your second turn, <strong>describe your special possessions</strong> and how you contribute to the village (beyond working the fields).</li><li>On your third turn, <strong>tell us about your history of violence</strong>, and what keeps you up at night.</li><li>On your next turn, <strong>answer one of the following</strong>, naming one or more NPCs who live in Stonetop.<ul><li>☐ Who is your closest kin?</li><li>☐ Who is your lover/spouse/betrothed?</li><li>☐ Who most needs/deserves your protection?</li><li>☐ Whose forgiveness do you strive to earn?</li></ul></li><li>Go around again. Answer another question from 4, or pass. When everyone has passed, go on.</li><li>On your next turn, <strong>ask your fellow PCs one of these</strong>. When others ask you, answer as you like.<ul><li>☐ Which one of you once dragged me home, bleeding and unconscious?</li><li>☐ Which one of you can I trust to always have my back?</li><li>☐ Which one of you has stayed my hand?</li><li>☐ Which one of you has traded blows with me?</li></ul></li><li>Go around again. Ask another question from 6, or pass. When everyone has passed, go on.</li><li>Add your home to the steading playbook. When everyone is done, let spring break forth!</li></ol>",
    },
    backstory: &[&BACKSTORY_A_HISTORY_OF_VIOLENCE],
};

// The Judge

static PLAYBOOK_THE_JUDGE: stonetop::fixed::PlaybookFixed = stonetop::fixed::PlaybookFixed {
    key: stonetop::keys::PlaybookKey::TheJudge,
    name: "The Judge",
    description: "Look here at this little town, this candleflame in the darkness. Its very existence is an act of courage and faith. And Aratis has charged you to keep it: to settle its disputes; to chronicle its tales; to defend it from darkness and ruin. Take up your hammer, Judge. Your town needs you.",
    backgrounds: [&BACKGROUND_LEGACY, &BACKGROUND_MISSIONARY, &BACKGROUND_PROPHET],
    instinct: [
        stonetop::fixed::Instinct {
            title: "Ambition",
            description: "To increase your status or influence.",
        },
        stonetop::fixed::Instinct {
            title: "Dispassion",
            description: "To disregard emotion or sentiment.",
        },
        stonetop::fixed::Instinct {
            title: "Harmony",
            description: "To seek a path that makes everyone happy.",
        },
        stonetop::fixed::Instinct {
            title: "Orthodoxy",
            description: "To strictly adhere to rules and traditions.",
        },
        stonetop::fixed::Instinct {
            title: "Zeal",
            description: "To judge quickly and without doubt.",
        },
    ],
    appearance: [
        stonetop::fixed::TaggedRow {
            tag: "Age",
            items: &["eager youth", "in my prime", "showing my years"],
        },
        stonetop::fixed::TaggedRow {
            tag: "Voice",
            items: &["calm voice", "booming voice", "a voice that carries"],
        },
        stonetop::fixed::TaggedRow {
            tag: "Build",
            items: &["hard body", "powerful frame", "slim", "well-fed"],
        },
        stonetop::fixed::TaggedRow {
            tag: "Clothing",
            items: &["polished gear", "robes of office", "modest clothes"],
        },
    ],
    origin_choices: &[
        stonetop::fixed::Origin {
            location: "Stonetop",
            naming: stonetop::fixed::Naming::Names(&[
                "Arianrhod",
                "Caerwyn",
                "Einion",
                "Eleri",
                "Magda",
                "Nerys",
                "Trahaern",
                "Trefor",
            ]),
        },
        stonetop::fixed::Origin {
            location: "Barrier Pass",
            naming: stonetop::fixed::Naming::Names(&[
                "Arinasai",
                "Bortachikhan",
                "Khadagan",
                "Khojin",
                "Odval",
                "Usun",
                "Yesui",
                "Yul",
            ]),
        },
        stonetop::fixed::Origin {
            location: "Gordin's Delve",
            naming: stonetop::fixed::Naming::Instructions("Pick a name from any list"),
        },
        stonetop::fixed::Origin {
            location: "Marshedge",
            naming: stonetop::fixed::Naming::Names(&[
                "Briget", "Comhall", "Elnor", "Liadain", "Mirdach", "Onghus", "Somha", "Toal",
            ]),
        },
        stonetop::fixed::Origin {
            location: "Lygos or some other southern town",
            naming: stonetop::fixed::Naming::Names(&[
                "Abrim",
                "Cassander",
                "Despina",
                "Hypatta",
                "Morecai",
                "Nomika",
                "Sofia",
                "Yose",
            ]),
        },
    ],
    stats_to_assign: [2i8, 1i8, 1i8, 0i8, 0i8, -1i8],
    damage: stonetop::Die::D6,
    hp: 20u8,
    special_possessions: stonetop::fixed::SpecialPossessions {
        pick_note: "Pick 1, in addition to your symbol of authority and scribe's kit",
        pick_count: 1u8,
        preselected: 2u8,
        options: &[
            &SPECIAL_POSSESSION_YOUR_SYMBOL_OF_AUTHORITY,
            &SPECIAL_POSSESSION_SCRIBES_TOOLS,
            &SPECIAL_POSSESSION_AVIARY,
            &SPECIAL_POSSESSION_CARPENTERS_TOOLS,
            &SPECIAL_POSSESSION_ENGINEERS_TOOLS,
            &SPECIAL_POSSESSION_SMITHY,
        ],
    },
    starting_move_choices: 2u8,
    grants_moves: &[
        stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::Censure),
        stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::ChroniclerOfStonetop),
    ],
    moves: &[
        &MOVE_AEGIS_OF_FAITH,
        &MOVE_ARMORED,
        &MOVE_BEAR_WITNESS,
        &MOVE_BREAK_BREAD,
        &MOVE_BULWARK,
        &MOVE_CENSURE,
        &MOVE_CASTIGATE,
        &MOVE_CHRONICLER_OF_STONETOP,
        &MOVE_FOR_THE_GREATER_GOOD,
        &MOVE_HOUND_OF_ARATIS,
        &MOVE_LIKE_A_DOG_WITH_A_BONE,
        &MOVE_IMPROVED_STAT,
        &MOVE_KNOWLEDGE_IS_POWER,
        &MOVE_MANY_HANDS_MAKE_LIGHT_WORK,
        &MOVE_A_BUNDLE_OF_STICKS_UNBROKEN,
        &MOVE_THE_HAMMER_AND_THE_BOOK,
        &MOVE_TRUTH_OR_CONSEQUENCES,
        &MOVE_BINDING_ARBITRATION,
        &MOVE_VISION_UNCLOUDED,
        &MOVE_WELL_READ,
        &MOVE_A_MIGHTY_RAMPART,
        &MOVE_ARMISTICE,
        &MOVE_CONDEMN,
        &MOVE_PROCLAMATION,
        &MOVE_MIRRORSHIELD,
        &MOVE_SUPERIOR_STAT,
        &MOVE_THE_TOWER_ETERNAL,
    ],
    moves_footnote: None,
    intro: stonetop::fixed::Intro {
        title: "Introductions",
        text: "<p>Wait here for everyone else. When everyone's ready, take turns introducing your characters. When <strong><em>someone reveals something and you want to know more</em></strong>, ask them about it. When <strong><em>someone asks you a question</em></strong>, answer it truthfully.</p><ol><li>On your first turn, <strong>introduce yourself</strong> by name, pronouns, background, origin, and appearance.</li><li>On your second turn, <strong>describe your special possessions</strong> and how you contribute to the village (beyond working the fields).</li><li>On your third turn, <strong><em>describe the Chronicle</em></strong>. Then, <strong><em>tell us about Aratis and her shrine</em></strong>, and what she demands of her true disciples.</li><li>On your next turn, <strong>answer one of the following</strong>, naming one or more NPCs who live in Stonetop.<ul><li>☐ Who is your closest kin?</li><li>☐ Who is your lover/spouse/betrothed?</li><li>☐ Who is your apprentice?</li><li>☐ Who is the wisest of the town elders?</li></ul></li><li>Go around again. Answer another question from 4, or pass. When everyone has passed, go on.</li><li>On your next turn, <strong>ask your fellow PCs one of these</strong>. When others ask you, answer as you like.<ul><li>☐ Which one of you is a true disciple of Aratis?</li><li>☐ Which one of you is my closest confidant?</li><li>☐ Which one of you has stood beside me in battle against unnatural chaos?</li><li>☐ Against which of you have I passed judgement?</li></ul></li><li>Go around again. Ask another question from 6, or pass. When everyone has passed, go on.</li><li>Add your home to the steading playbook. When everyone is done, let spring break forth!</li></ol>",
    },
    backstory: &[&BACKSTORY_THE_CHRONICLE, &BACKSTORY_THE_LAWKEEPER],
};

// The Lightbearer

static PLAYBOOK_THE_LIGHTBEARER: stonetop::fixed::PlaybookFixed = stonetop::fixed::PlaybookFixed {
    key: stonetop::keys::PlaybookKey::TheLightbearer,
    name: "The Lightbearer",
    description: "Imagine yourself and your kin in a cave lit by a single torch, entranced by shadow puppet stories. Imagine realizing there is a greater truth, and stepping out of the cave into the true Light of day. Would you not bring that Light back into the darkness, to set your people free?",
    backgrounds: [
        &BACKGROUND_AUSPICIOUS_BIRTH,
        &BACKGROUND_ITINERANT_MYSTIC,
        &BACKGROUND_SOUL_ON_FIRE,
    ],
    instinct: [
        stonetop::fixed::Instinct {
            title: "Charity",
            description: "To go without so that others are better off.",
        },
        stonetop::fixed::Instinct {
            title: "Hope",
            description: "To inspire others in the face of adversity.",
        },
        stonetop::fixed::Instinct {
            title: "Mercy",
            description: "To bring relief or comfort, to give second chances.",
        },
        stonetop::fixed::Instinct {
            title: "Praise",
            description: "To spread the glory and worship of Helior.",
        },
        stonetop::fixed::Instinct {
            title: "Righteousness",
            description: "To refuse to suffer an injustice or a lesser evil.",
        },
    ],
    appearance: [
        stonetop::fixed::TaggedRow {
            tag: "Age",
            items: &["a youthful glow", "well-weathered", "old & merry"],
        },
        stonetop::fixed::TaggedRow {
            tag: "Voice",
            items: &["a lilting voice", "a melodious voice", "a soft voice"],
        },
        stonetop::fixed::TaggedRow {
            tag: "Vibe",
            items: &["beatific", "ethereal", "intense", "jovial", "serene"],
        },
        stonetop::fixed::TaggedRow {
            tag: "Clothing",
            items: &["fine robes", "threadbare cloak", "working clothes"],
        },
    ],
    origin_choices: &[
        stonetop::fixed::Origin {
            location: "Stonetop",
            naming: stonetop::fixed::Naming::Names(&[
                "Dai", "Eirian", "Eurig", "Haf", "Haul", "Hefin", "Hulwen", "Tesni",
            ]),
        },
        stonetop::fixed::Origin {
            location: "Barrier Pass",
            naming: stonetop::fixed::Naming::Names(&[
                "Alaqa", "Bat", "Dinget", "Ghoa", "Oyuun", "Sidurgu", "Temur", "Toragana",
            ]),
        },
        stonetop::fixed::Origin {
            location: "Gordin's Delve",
            naming: stonetop::fixed::Naming::Instructions("Pick a name from any list"),
        },
        stonetop::fixed::Origin {
            location: "Marshedge",
            naming: stonetop::fixed::Naming::Names(&[
                "Adfin", "Callach", "Conlad", "Eadna", "Fionntan", "Niamh", "Orlaith", "Sorsha",
            ]),
        },
        stonetop::fixed::Origin {
            location: "Lygos or some other point south",
            naming: stonetop::fixed::Naming::Names(&[
                "Arash", "Azar", "Hafiz", "Murat", "Roshan", "Shideh", "Zara", "Zohara",
            ]),
        },
    ],
    stats_to_assign: [2i8, 1i8, 1i8, 0i8, 0i8, -1i8],
    damage: stonetop::Die::D4,
    hp: 18u8,
    special_possessions: stonetop::fixed::SpecialPossessions {
        pick_note: "Pick 2",
        pick_count: 2u8,
        preselected: 0u8,
        options: &[
            &SPECIAL_POSSESSION_APIARY,
            &SPECIAL_POSSESSION_BOOKS_SCROLLS,
            &SPECIAL_POSSESSION_CHANDLERY,
            &SPECIAL_POSSESSION_DISTILLERY,
            &SPECIAL_POSSESSION_GLASSWORKS,
            &SPECIAL_POSSESSION_HOLY_RELICS,
            &SPECIAL_POSSESSION_LUTHIERS_TOOLS,
        ],
    },
    starting_move_choices: 1u8,
    grants_moves: &[
        stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::ConsecratedFlame),
        stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::InvokeTheSunGod),
    ],
    moves: &[
        &MOVE_A_CANDLE_AGAINST_THE_DARK,
        &MOVE_LUMINOUS_SHIELD,
        &MOVE_ALL_IS_ILLUMINATED,
        &MOVE_AND_BEHOLD_A_PALE_HORSE,
        &MOVE_CONSECRATED_FLAME,
        &MOVE_FIRE_WITHIN,
        &MOVE_GUIDING_LIGHT,
        &MOVE_HELIORS_UNBLINKING_EYE,
        &MOVE_IMPROVED_STAT,
        &MOVE_INVOKE_THE_SUN_GOD,
        &MOVE_KEEP_THE_HOME_FIRES_BURNING,
        &MOVE_LAMPLIGHTER,
        &MOVE_PIETY,
        &MOVE_PURIFYING_FLAMES,
        &MOVE_RADIANT_COUNTENANCE,
        &MOVE_RISE_LIKE_THE_SUN,
        &MOVE_SPRINGS_FIRST_THAW,
        &MOVE_BURN_TWICE_AS_BRIGHT,
        &MOVE_EMPOWERED_INVOCATIONS,
        &MOVE_GLORIOUS_SERVANT,
        &MOVE_HUNGRY_FLAMES,
        &MOVE_LIGHT_MORE_LIGHT,
        &MOVE_SUPERIOR_STAT,
        &MOVE_WIELDER_OF_THE_WHITE_FLAME,
    ],
    moves_footnote: None,
    intro: stonetop::fixed::Intro {
        title: "Introductions",
        text: "<p>Wait here for everyone else. When everyone's ready, take turns introducing your characters. When <strong><em>someone reveals something and you want to know more</em></strong>, ask them about it. When <strong><em>someone asks you a question</em></strong>, answer it truthfully.</p><ol><li>On your first turn, <strong>introduce yourself</strong> by name, pronouns, background, origin, and appearance.</li><li>On your second turn, <strong>describe your special possessions</strong> and how you contribute to the village (beyond working the fields).</li><li>On your third turn, <strong>praise the day! Tell us of Helior</strong>, his worship and his shrine. Tell us, too, of the prior Lightbearer and how you gained your powers.</li><li>On your next turn, <strong>answer one of the following</strong>, naming one or more NPCs who live in Stonetop.<ul><li>☐ Who is your closest kin?</li><li>☐ Who fans the flames of your heart?</li><li>☐ Whose kindness and generosity warm your soul?</li><li>☐ Who needs Helior's light, badly?</li></ul></li><li>Go around again. Answer another question from 4, or pass. When everyone has passed, go on.</li><li>On your next turn, <strong>ask your fellow PCs one of these</strong>. When others ask you, answer as you like.<ul><li>☐ Which one of you is an old and dear friend?</li><li>☐ Which one of you shares my faith?</li><li>☐ Which one of you scoffs at mercy and hope?</li><li>☐ Which one of you will need my guidance soon?</li></ul></li><li>Go around again. Ask another question from 6, or pass. When everyone has passed, go on.</li><li>Add your home to the steading playbook. When everyone is done, let spring break forth!</li></ol>",
    },
    backstory: &[&BACKSTORY_PRAISE_THE_DAY],
};

// The Marshal

static PLAYBOOK_THE_MARSHAL: stonetop::fixed::PlaybookFixed = stonetop::fixed::PlaybookFixed {
    key: stonetop::keys::PlaybookKey::TheMarshal,
    name: "The Marshal",
    description: "Hoping for peace isn't enough. Trouble always comes knocking. And that's why we need you: to run the drills, to man the towers, to take charge when things get bad. To be cold enough to send your neighbors to a sure death in order to keep Stonetop safe. That's the job, Marshal. You up for it?",
    backgrounds: [&BACKGROUND_SCION, &BACKGROUND_PENITENT, &BACKGROUND_LUMINARY],
    instinct: [
        stonetop::fixed::Instinct {
            title: "Authority",
            description: "To take charge and throw your weight around.",
        },
        stonetop::fixed::Instinct {
            title: "Caution",
            description: "To keep everyone safe, to agonize over decisions.",
        },
        stonetop::fixed::Instinct {
            title: "Drive",
            description: "To take on ever more responsibility.",
        },
        stonetop::fixed::Instinct {
            title: "Honor",
            description: "To keep your word, to follow a moral code.",
        },
        stonetop::fixed::Instinct {
            title: "Ruthlessness",
            description: "To do whatever it takes to win or survive.",
        },
    ],
    appearance: [
        stonetop::fixed::TaggedRow {
            tag: "Age",
            items: &["upstart youth", "experienced & sober", "grizzled"],
        },
        stonetop::fixed::TaggedRow {
            tag: "Voice",
            items: &["clear voice", "resonant voice", "rumbling voice"],
        },
        stonetop::fixed::TaggedRow {
            tag: "Look",
            items: &["stern frown", "grim-set jaw", "knowing smirk"],
        },
        stonetop::fixed::TaggedRow {
            tag: "Clothing",
            items: &["badge of office", "spit & polish", "timeworn gear"],
        },
    ],
    origin_choices: &[
        stonetop::fixed::Origin {
            location: "Stonetop",
            naming: stonetop::fixed::Naming::Names(&[
                "Bethan", "Cadfael", "Ffraid", "Gwythyr", "Llewelyn", "Meredith", "Rhianna",
                "Urien",
            ]),
        },
        stonetop::fixed::Origin {
            location: "Gordin's Delve",
            naming: stonetop::fixed::Naming::Instructions("Pick a name from any list"),
        },
        stonetop::fixed::Origin {
            location: "Marshedge",
            naming: stonetop::fixed::Naming::Names(&[
                "Brigh", "Cathal", "Conn", "Donal", "Fionna", "Laith", "Talulla", "Torin",
            ]),
        },
        stonetop::fixed::Origin {
            location: "The Steplands (Hillfolk)",
            naming: stonetop::fixed::Naming::Names(&[
                "Adl", "Aeln", "Clotild", "Judoc", "Katrn", "Mygl", "Pirn", "Sera",
            ]),
        },
        stonetop::fixed::Origin {
            location: "The Manmarch",
            naming: stonetop::fixed::Naming::Names(&[
                "Berkhard", "Gerhild", "Hartig", "Hilde", "Sabrinne", "Ulrike", "Urrsla", "Weillem",
            ]),
        },
        stonetop::fixed::Origin {
            location: "Lygos or some other point south",
            naming: stonetop::fixed::Naming::Names(&[
                "Ameer", "Calixta", "Hadar", "Kelila", "Sulaim", "Ursa", "Xandros",
            ]),
        },
    ],
    stats_to_assign: [2i8, 1i8, 1i8, 0i8, 0i8, -1i8],
    damage: stonetop::Die::D8,
    hp: 20u8,
    special_possessions: stonetop::fixed::SpecialPossessions {
        pick_note: "Pick 2",
        pick_count: 2u8,
        preselected: 0u8,
        options: &[
            &SPECIAL_POSSESSION_CHIRURGEONS_TOOLS,
            &SPECIAL_POSSESSION_DISTILLERY,
            &SPECIAL_POSSESSION_ENGINEERS_TOOLS,
            &SPECIAL_POSSESSION_PERSONAL_SYMBOL,
            &SPECIAL_POSSESSION_SCRIBES_TOOLS,
            &SPECIAL_POSSESSION_PM_WEAPONS_OF_WAR,
        ],
    },
    starting_move_choices: 1u8,
    grants_moves: &[
        stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::Crew),
        stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::Logistics),
    ],
    moves: &[
        &MOVE_ARMORED,
        &MOVE_ARTS_OF_WAR,
        &MOVE_CREW,
        &MOVE_VETERAN_CREW,
        &MOVE_FRONT_LINE_LEADER,
        &MOVE_IMPROVED_STAT,
        &MOVE_LOGISTICS,
        &MOVE_READ_THE_LAND,
        &MOVE_PREPARE_A_WELCOME,
        &MOVE_SET_UP_STRIKE,
        &MOVE_SHAKE_IT_OFF,
        &MOVE_SHIELD_WALL,
        &MOVE_SIR_PERMISSION_TO_DIE_SIR,
        &MOVE_SPEAK_SOFTLY,
        &MOVE_STENTORIAN,
        &MOVE_TAKE_THE_MEASURE,
        &MOVE_WE_HAPPY_FEW,
        &MOVE_BATTLEFIELD_GRACE,
        &MOVE_HEROES_TO_THE_LAST,
        &MOVE_FOCUS_FIRE,
        &MOVE_LIKE_AN_OPEN_BOOK,
        &MOVE_NOBLE_MIEN,
        &MOVE_PEACE_THROUGH_STRENGTH,
        &MOVE_SUPERIOR_STAT,
    ],
    moves_footnote: None,
    intro: stonetop::fixed::Intro {
        title: "Introductions",
        text: "<p>Wait here for everyone else. When everyone's ready, take turns introducing your characters. When <strong><em>someone reveals something and you want to know more</em></strong>, ask them about it. When <strong><em>someone asks you a question</em></strong>, answer it truthfully.</p><ol><li>On your first turn, <strong>introduce yourself</strong> by name, pronouns, background, origin, and appearance.</li><li>On your second turn, <strong>describe your special possessions</strong> and how you contribute to the village (beyond working the fields).</li><li>On your third turn, <strong>tell us the town's war stories</strong>, plus the answers to the questions you chose.</li><li>On your next turn, <strong>answer one of the following</strong>, naming one or more NPCs who live in Stonetop.<ul><li>☐ Who is your closest kin?</li><li>☐ Who is your lover/spouse/betrothed?</li><li>☐ Who is your lieutenant?</li><li>☐ Whose kin is dead because of your decisions?</li></ul></li><li>Go around again. Answer another question from 4, or pass. When everyone has passed, go on.</li><li>On your next turn, <strong>ask your fellow PCs one of these</strong>. When others ask you, answer as you like.<ul><li>☐ Which one of you is or was part of my crew?</li><li>☐ Which one of you have I promised to keep safe?</li><li>☐ Which one of you do I still have doubts about?</li><li>☐ Which one of you ignored my orders and got someone killed?</li></ul></li><li>Go around again. Ask another question from 6, or pass. When everyone has passed, go on.</li><li>Add your home to the steading playbook. When everyone is done, let spring break forth!</li></ol>",
    },
    backstory: &[&BACKSTORY_WAR_STORIES],
};

// The Ranger

static PLAYBOOK_THE_RANGER: stonetop::fixed::PlaybookFixed = stonetop::fixed::PlaybookFixed {
    key: stonetop::keys::PlaybookKey::TheRanger,
    name: "The Ranger",
    description: "Your true home is out there. Away from the Old Roads, in the wild places, where you've faced storm and beast alike. But unknown forces are at work beyond the Ringwall, and you fear for your kith and kin. These are strange times. Guide them, ranger, and keep them safe when darkness falls.",
    backgrounds: [&BACKGROUND_MIGHTY_HUNTER, &BACKGROUND_WIDE_WANDERER, &BACKGROUND_BEAST_BONDED],
    instinct: [
        stonetop::fixed::Instinct {
            title: "Adventure",
            description: "To test yourself, to experience new things.",
        },
        stonetop::fixed::Instinct {
            title: "Independence",
            description: "To refuse help and push others away.",
        },
        stonetop::fixed::Instinct {
            title: "Stewardship",
            description: "To value beasts and natural places over people.",
        },
        stonetop::fixed::Instinct { title: "Tenacity", description: "To be stubborn, to persist." },
        stonetop::fixed::Instinct {
            title: "Wonder",
            description: "To marvel at beauty, magnificence, splendor.",
        },
    ],
    appearance: [
        stonetop::fixed::TaggedRow {
            tag: "Age",
            items: &["fledgling", "prime specimen", "long in the tooth"],
        },
        stonetop::fixed::TaggedRow {
            tag: "Voice",
            items: &["barking voice", "growling voice", "sing-song voice"],
        },
        stonetop::fixed::TaggedRow {
            tag: "Build",
            items: &["compact & sturdy", "long & lean", "wolfish"],
        },
        stonetop::fixed::TaggedRow {
            tag: "Clothing",
            items: &["shaggy", "threadbare", "well-groomed"],
        },
    ],
    origin_choices: &[
        stonetop::fixed::Origin {
            location: "Stonetop",
            naming: stonetop::fixed::Naming::Names(&[
                "Aran", "Bledyn", "Branwen", "Deryn", "Ifur", "Meinir", "Rhys", "Teagan",
            ]),
        },
        stonetop::fixed::Origin {
            location: "Barrier Pass",
            naming: stonetop::fixed::Naming::Names(&[
                "Anarba",
                "Arslan",
                "Bolormaa",
                "Cirina",
                "Nergui",
                "Nomolun",
                "Saran",
                "Shigi-Qutuqu",
            ]),
        },
        stonetop::fixed::Origin {
            location: "Marshedge",
            naming: stonetop::fixed::Naming::Names(&[
                "Asling",
                "Conar",
                "Enna",
                "Flannan",
                "Macha",
                "Mave",
                "Proinsias",
                "Rowen",
            ]),
        },
        stonetop::fixed::Origin {
            location: "The Steplands (Hillfolk)",
            naming: stonetop::fixed::Naming::Names(&[
                "Bernd", "Elown", "Irn", "Kani", "Pol", "Nol", "Rozn", "Sterin",
            ]),
        },
        stonetop::fixed::Origin {
            location: "The Manmarch",
            naming: stonetop::fixed::Naming::Names(&[
                "Alfher",
                "Bertrim",
                "Dagmar",
                "Elfrida",
                "Hramn",
                "Meike",
                "Swanhilde",
                "Wulfrim",
            ]),
        },
        stonetop::fixed::Origin {
            location: "Lygos or some other point south",
            naming: stonetop::fixed::Naming::Names(&[
                "Ari", "Boriz", "Dimitra", "Gorhan", "Nitza", "Selen", "Todora", "Vasil",
            ]),
        },
    ],
    stats_to_assign: [2i8, 1i8, 1i8, 0i8, 0i8, -1i8],
    damage: stonetop::Die::D8,
    hp: 18u8,
    special_possessions: stonetop::fixed::SpecialPossessions {
        pick_note: "Pick 2, in addition to your composite bow",
        pick_count: 2u8,
        preselected: 1u8,
        options: &[
            &SPECIAL_POSSESSION_COMPOSITE_BOW,
            &SPECIAL_POSSESSION_DISTILLERY,
            &SPECIAL_POSSESSION_HIDEOUTS,
            &SPECIAL_POSSESSION_HUSBANDRY_TOOLS,
            &SPECIAL_POSSESSION_HOUNDS,
            &SPECIAL_POSSESSION_LAY_OF_THE_LAND,
            &SPECIAL_POSSESSION_TRAPPING_GEAR,
        ],
    },
    starting_move_choices: 1u8,
    grants_moves: &[stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::HomeOnTheRange)],
    moves: &[
        &MOVE_A_SAFE_PLACE,
        &MOVE_ANIMAL_COMPANION,
        &MOVE_MAGNIFICENT_SPECIMEN,
        &MOVE_BIG_GAME_HUNTER,
        &MOVE_BLOT_OUT_THE_SUN,
        &MOVE_CALL_THE_SHOT,
        &MOVE_EXPERT_TRACKER,
        &MOVE_HOME_ON_THE_RANGE,
        &MOVE_IMPROVED_STAT,
        &MOVE_MENTAL_MAP,
        &MOVE_NATURALIST,
        &MOVE_ON_THE_HOOF,
        &MOVE_PACK_HORSE,
        &MOVE_PATHFINDER,
        &MOVE_PREDATOR,
        &MOVE_SNIFF_OUT_CORRUPTION,
        &MOVE_STALKER,
        &MOVE_SURVIVALIST,
        &MOVE_WARDEN_OF_THE_WILD,
        &MOVE_WILD_SPEECH,
        &MOVE_WORLDLY,
        &MOVE_ALPHA,
        &MOVE_BEAST_OF_LEGEND,
        &MOVE_CONSTANT_VIGILANCE,
        &MOVE_GIANT_SLAYER,
        &MOVE_SUPERIOR_STAT,
        &MOVE_TRAILBLAZER,
        &MOVE_WALK_IT_OFF,
    ],
    moves_footnote: None,
    intro: stonetop::fixed::Intro {
        title: "Introductions",
        text: "<p>Wait here for everyone else. When everyone's ready, take turns introducing your characters. When <strong><em>someone reveals something and you want to know more</em></strong>, ask them about it. When <strong><em>someone asks you a question</em></strong>, answer it truthfully.</p><ol><li>On your first turn, <strong>introduce yourself</strong> by name, pronouns, background, origin, and appearance.</li><li>On your second turn, <strong>describe your special possessions</strong> and how you contribute to the village (beyond working the fields).</li><li>On your third turn, <strong>tell us what you're worried about</strong> (see \"Something wicked this way comes\").</li><li>On your next turn, <strong>answer one of the following</strong>, naming one or more NPCs who live in Stonetop.<ul><li>☐ Who is your closest kin?</li><li>☐ To whom do you always return home?</li><li>☐ Who would be lost without you?</li><li>☐ Who has much to learn from you?</li></ul></li><li>Go around again. Answer another question from 4, or pass. When everyone has passed, go on.</li><li>On your next turn, <strong>ask your fellow PCs one of these</strong>. When others ask you, answer as you like.<ul><li>☐ Which one of you fears the wider world?</li><li>☐ Which one of you has shown me great beauty?</li><li>☐ Which one of you have I caught sometimes staring out at the horizon?</li><li>☐ Which one of you lacked the stomach to put something out of its misery?</li></ul></li><li>Go around again. Ask another question from 6, or pass. When everyone has passed, go on.</li><li>Add your home to the steading playbook. When everyone is done, let spring break forth!</li></ol>",
    },
    backstory: &[&BACKSTORY_SOMETHING_WICKED_THIS_WAY_COMES],
};

// The Seeker

static PLAYBOOK_THE_SEEKER: stonetop::fixed::PlaybookFixed = stonetop::fixed::PlaybookFixed {
    key: stonetop::keys::PlaybookKey::TheSeeker,
    name: "The Seeker",
    description: "Look at us. Huddling behind our walls, hearing evil in every passing noise. Cowards, all. All, but you. You fear not the unknown. You plunge into it, searching. Grasping at what has been lost. What will you find, o Seeker? Signs of a bright new age? Or signs of our doom?",
    backgrounds: [&BACKGROUND_PATRIOT, &BACKGROUND_ANTIQUARIAN, &BACKGROUND_WITCH_HUNTER],
    instinct: [
        stonetop::fixed::Instinct {
            title: "Cunning",
            description: "To scheme, manipulate, and plot.",
        },
        stonetop::fixed::Instinct {
            title: "Curiosity",
            description: "To seek answers that maybe you oughtn't.",
        },
        stonetop::fixed::Instinct {
            title: "Hubris",
            description: "To assume you know best, that you can't fail.",
        },
        stonetop::fixed::Instinct {
            title: "Mystery",
            description: "To avoid straight answers; to keep secrets.",
        },
        stonetop::fixed::Instinct {
            title: "Vision",
            description: "To think big and pursue grandiose goals.",
        },
    ],
    appearance: [
        stonetop::fixed::TaggedRow {
            tag: "Age",
            items: &["curiously young", "world-weary", "bent with years"],
        },
        stonetop::fixed::TaggedRow {
            tag: "Voice",
            items: &["haunted voice", "rich voice", "whispery"],
        },
        stonetop::fixed::TaggedRow {
            tag: "Hands",
            items: &["ink-stained fingers", "sinewy hands", "soft hands"],
        },
        stonetop::fixed::TaggedRow {
            tag: "Build",
            items: &["bony limbed", "lean & lanky", "short", "thick-set"],
        },
    ],
    origin_choices: &[
        stonetop::fixed::Origin {
            location: "Stonetop",
            naming: stonetop::fixed::Naming::Names(&[
                "Alis", "Dylan", "Eilwen", "Gerlt", "Gwenda", "Macsen", "Mirgan", "Owena",
                "Taliesyn", "Twymor",
            ]),
        },
        stonetop::fixed::Origin {
            location: "Barrier Pass",
            naming: stonetop::fixed::Naming::Names(&[
                "Bayanaganengri",
                "Chakha",
                "Jetei",
                "Moog",
                "Narengawa",
                "Ogul",
                "Ozbeg",
                "Solongo",
            ]),
        },
        stonetop::fixed::Origin {
            location: "The Steplands (Hillfolk)",
            naming: stonetop::fixed::Naming::Names(&[
                "Anook", "Anxo", "Dors", "Jory", "Mari", "Padg", "Pons", "Silf",
            ]),
        },
        stonetop::fixed::Origin {
            location: "Gordin's Delve",
            naming: stonetop::fixed::Naming::Instructions("Pick a name from any list"),
        },
        stonetop::fixed::Origin {
            location: "Marshedge",
            naming: stonetop::fixed::Naming::Names(&[
                "Aiden",
                "Barrfind",
                "Caolan",
                "Ciara",
                "Deirbhile",
                "Moirin",
                "Tiern",
                "Reamann",
            ]),
        },
        stonetop::fixed::Origin {
            location: "Lygos or some other point south",
            naming: stonetop::fixed::Naming::Names(&[
                "Dana",
                "Eliana",
                "Erez",
                "Fikri",
                "Isra",
                "Persefoni",
                "Spiro",
                "Vahid",
            ]),
        },
    ],
    stats_to_assign: [2i8, 1i8, 1i8, 0i8, 0i8, -1i8],
    damage: stonetop::Die::D6,
    hp: 16u8,
    special_possessions: stonetop::fixed::SpecialPossessions {
        pick_note: "Pick 2, in addition to your scribe's tools",
        pick_count: 2u8,
        preselected: 1u8,
        options: &[
            &SPECIAL_POSSESSION_SCRIBES_TOOLS,
            &SPECIAL_POSSESSION_BOOKS_SCROLLS,
            &SPECIAL_POSSESSION_DISTILLERY,
            &SPECIAL_POSSESSION_ENGINEERS_TOOLS,
            &SPECIAL_POSSESSION_LABORATORY,
            &SPECIAL_POSSESSION_PARAPHERNALIA,
            &SPECIAL_POSSESSION_TRADE_CONTACTS,
        ],
    },
    starting_move_choices: 0u8,
    grants_moves: &[
        stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::WellVersed),
        stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::WorkWithWhatYouveGot),
    ],
    moves: &[
        &MOVE_ATTUNED,
        &MOVE_CONDUIT_OF_POWER,
        &MOVE_COUNTERMEASURES,
        &MOVE_EVERYTHING_BLEEDS,
        &MOVE_EVERYTHING_BURNS,
        &MOVE_IMPROVED_STAT,
        &MOVE_INITIATE_OF_THE_SECRET_ARTS,
        &MOVE_LETS_MAKE_A_DEAL,
        &MOVE_LOGBOOK,
        &MOVE_MAGPIE,
        &MOVE_NEVER_AT_A_LOSS,
        &MOVE_POLYGLOT,
        &MOVE_CRYPTOLOGIST,
        &MOVE_QUICK_STUDY,
        &MOVE_SAFETY_FIRST,
        &MOVE_SAGE_ADVICE,
        &MOVE_WELL_VERSED,
        &MOVE_WORK_WITH_WHAT_YOUVE_GOT,
        &MOVE_ARCANE_ADEPT,
        &MOVE_DEEP_INSIGHT,
        &MOVE_IMPROVISE,
        &MOVE_SUPERIOR_STAT,
        &MOVE_MIND_OVER_MAGIC,
        &MOVE_OVERCHANNEL,
        &MOVE_PROOF_AGAINST_DETECTION,
    ],
    moves_footnote: None,
    intro: stonetop::fixed::Intro {
        title: "Introductions",
        text: "<p>Wait here for everyone else. When everyone's ready, take turns introducing your characters. When <strong><em>someone reveals something and you want to know more</em></strong>, ask them about it. When <strong><em>someone asks you a question</em></strong>, answer it truthfully.</p><ol><li>On your first turn, <strong>introduce yourself</strong> by name, pronouns, background, origin, and appearance.</li><li>On your second turn, <strong>describe your special possessions</strong> and how you contribute to the village (beyond working the fields).</li><li>On your third turn, <strong>describe your major arcana</strong>. Tell us your answers to the questions you chose. Then, <strong>tell us about your minor arcana</strong>, too.</li><li>On your next turn, <strong>answer one of the following</strong>, naming one or more NPCs who live in Stonetop.<ul><li>☐ Who is your closest kin?</li><li>☐ Who is your spouse/lover/betrothed?</li><li>☐ Whom do you trust, even more than yourself?</li><li>☐ Whom do you secretly watch over, and why?</li></ul></li><li>Go around again. Answer another question from 4, or pass. When everyone has passed, go on.</li><li>On your next turn, <strong>ask your fellow PCs one of these</strong>. When others ask you, answer as you like.<ul><li>☐ Which one of you led me to a key discovery?</li><li>☐ Which one of you has been at my side the entire way?</li><li>☐ Which one of you most fears the path I tread?</li><li>☐ Which one of you is keeping secrets from me?</li></ul></li><li>Go around again. Ask another question from 6, or pass. When everyone has passed, go on.</li><li>Add your home to the steading playbook. When everyone is done, let spring break forth!</li></ol>",
    },
    backstory: &[&BACKSTORY_COLLECTION, &BACKSTORY_MAJOR_ARCANA, &BACKSTORY_MINOR_ARCANA],
};

// The Would-be Hero

static PLAYBOOK_THE_WOULD_BE_HERO: stonetop::fixed::PlaybookFixed =
    stonetop::fixed::PlaybookFixed {
        key: stonetop::keys::PlaybookKey::TheWouldBeHero,
        name: "The Would-be Hero",
        description: "Most people hope for a quiet life. They spend their days a-worrying: about a leaky roof, a sick child, their crops. But you aren't like most people—you're on a different path. A path to adventure! There's greatness in you. Let's hope you live long enough for everyone else to see it.",
        backgrounds: [&BACKGROUND_IMPETUOUS_YOUTH, &BACKGROUND_DRIVEN, &BACKGROUND_DESTINED],
        instinct: [
            stonetop::fixed::Instinct {
                title: "Defiance",
                description: "To refuse to back down, give up, give in.",
            },
            stonetop::fixed::Instinct {
                title: "Doubt",
                description: "To question yourself, your actions, your worth.",
            },
            stonetop::fixed::Instinct {
                title: "Earnestness",
                description: "To prove yourself, to yourself and others.",
            },
            stonetop::fixed::Instinct {
                title: "Optimism",
                description: "To assume the best, and that things are simple.",
            },
            stonetop::fixed::Instinct {
                title: "Sacrifice",
                description: "To put the needs/wants of others above your own.",
            },
        ],
        appearance: [
            stonetop::fixed::TaggedRow {
                tag: "Age",
                items: &["still a child", "young & beautiful", "all grown up"],
            },
            stonetop::fixed::TaggedRow {
                tag: "Voice",
                items: &["confident voice", "earnest voice", "quiet voice"],
            },
            stonetop::fixed::TaggedRow {
                tag: "Build",
                items: &["big", "scrawny", "sinewy", "slender", "thick"],
            },
            stonetop::fixed::TaggedRow {
                tag: "Vibe",
                items: &["back unbowed", "jaw firmly set", "soulful eyes"],
            },
        ],
        origin_choices: &[
            stonetop::fixed::Origin {
                location: "Stonetop",
                naming: stonetop::fixed::Naming::Names(&[
                    "Anwen", "Caradoc", "Dafyd", "Glenys", "Madoc", "Morwenna", "Siwan", "Wynfor",
                ]),
            },
            stonetop::fixed::Origin {
                location: "Barrier Pass",
                naming: stonetop::fixed::Naming::Names(&[
                    "Bala", "Cotota", "Ganzorig", "Gerelma", "Ibahka", "Jungshoi", "Mukhali",
                    "Taichu",
                ]),
            },
            stonetop::fixed::Origin {
                location: "The Steplands (Hillfolk)",
                naming: stonetop::fixed::Naming::Names(&[
                    "Annic", "Cosette", "Denl", "Hugenne", "Jag", "Marc", "Oanz", "Sandre",
                ]),
            },
            stonetop::fixed::Origin {
                location: "Gordin's Delve",
                naming: stonetop::fixed::Naming::Instructions("Pick a name from any list"),
            },
            stonetop::fixed::Origin {
                location: "Marshedge",
                naming: stonetop::fixed::Naming::Names(&[
                    "Bridin", "Clian", "Engis", "Fearghul", "Lan", "Neasa", "Nill", "Una",
                ]),
            },
            stonetop::fixed::Origin {
                location: "Lygos or some other point south",
                naming: stonetop::fixed::Naming::Names(&[
                    "Chara", "Davud", "Korina", "Omid", "Parvaneh", "Tamir", "Takish", "Yannis",
                ]),
            },
        ],
        stats_to_assign: [1i8, 0i8, 0i8, 0i8, 0i8, -1i8],
        damage: stonetop::Die::D6,
        hp: 16u8,
        special_possessions: stonetop::fixed::SpecialPossessions {
            pick_note: "Pick 2",
            pick_count: 2u8,
            preselected: 0u8,
            options: &[
                &SPECIAL_POSSESSION_A_HEAP_OF_EXPECTATIONS,
                &SPECIAL_POSSESSION_A_GOOD_DOG,
                &SPECIAL_POSSESSION_HUSBANDRY_TOOLS,
                &SPECIAL_POSSESSION_SMITHY,
                &SPECIAL_POSSESSION_STONEWORKERS_TOOLS,
                &SPECIAL_POSSESSION_PERSONAL_TOKEN_FRAUGHT_WITH_MEANING,
                &SPECIAL_POSSESSION_TANNERY,
            ],
        },
        starting_move_choices: 2u8,
        grants_moves: &[
            stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::AngerIsAGift),
            stonetop::fixed::GrantMove::Simply(stonetop::keys::MoveKey::PotentialForGreatness),
        ],
        moves: &[
            &MOVE_ANGER_IS_A_GIFT,
            &MOVE_SPEAK_TRUTH_TO_POWER,
            &MOVE_BETTER_PART_OF_VALOR,
            &MOVE_I_GET_KNOCKED_DOWN,
            &MOVE_BUT_I_GET_UP_AGAIN,
            &MOVE_IMPROVED_STAT,
            &MOVE_IN_OVER_YOUR_HEAD,
            &MOVE_IRON_WILL,
            &MOVE_INQUIRING_MINDS,
            &MOVE_NEVER_GONNA_KEEP_ME_DOWN,
            &MOVE_POTENTIAL_FOR_GREATNESS,
            &MOVE_RESOURCEFUL,
            &MOVE_SOMETHING_TO_REMEMBER_ME_BY,
            &MOVE_TOUGH_LOVE,
            &MOVE_UNDERESTIMATED,
            &MOVE_UP_WITH_PEOPLE,
            &MOVE_VERSATILE,
            &MOVE_A_FORCE_TO_BE_RECKONED_WITH,
            &MOVE_BIG_DAMN_HERO,
            &MOVE_PW_SUPERIOR_STAT,
            &MOVE_UNDAUNTED,
            &MOVE_VOICE_OF_EXPERIENCE,
        ],
        moves_footnote: Some(
            "The first time you use any move marked with an asterisk (*), cross off \"Would-be\" on the front page.",
        ),
        intro: stonetop::fixed::Intro {
            title: "Introductions",
            text: "<p>Wait here for everyone else. When everyone's ready, take turns introducing your characters. When <strong><em>someone reveals something and you want to know more</em></strong>, ask them about it. When <strong><em>someone asks you a question</em></strong>, answer it truthfully.</p><ol><li>On your first turn, <strong>introduce yourself</strong> by name, pronouns, background, origin, and appearance.</li><li>On your second turn, <strong>describe your special possessions</strong> and how you contribute to the village (beyond working the fields).</li><li>On your third turn, <strong>tell us of your fear & anger</strong>, and of the last time they caused you trouble.</li><li>On your next turn, <strong>answer one of the following</strong>, naming one or more NPCs who live in Stonetop.<ul><li>☐ Whose heart do you hope to win?</li><li>☐ Who is counting on you?</li><li>☐ Who quietly understands the path you are on?</li><li>☐ Who do you intend to prove wrong?</li></ul></li><li>Go around again. Answer another question from 4, or pass. When everyone has passed, go on.</li><li>On your next turn, <strong>ask your fellow PCs one of these</strong>. When others ask you, answer as you like.<ul><li>☐ Which one of you is my closest, truest friend?</li><li>☐ Which one of you believes in me, despite it all?</li><li>☐ Which one of you has promised to teach me?</li><li>☐ Which one of you have I hurt, through what I have done or what I've failed to do?</li></ul></li><li>Go around again. Ask another question from 6, or pass. When everyone has passed, go on.</li><li>Add your home to the steading playbook. When everyone is done, let spring break forth!</li></ol>",
        },
        backstory: &[&BACKSTORY_FEAR_ANGER],
    };

impl stonetop::keys::MoveKey {
    /// The printed content this key names.
    #[must_use]
    pub fn fixed_part(self) -> &'static stonetop::fixed::MoveFixed {
        match self {
            Self::FromRaisedByWolves => &MOVE_FROM_RAISED_BY_WOLVES,
            Self::FromVessel => &MOVE_FROM_VESSEL,
            Self::AmuletsTalismans => &MOVE_AMULETS_TALISMANS,
            Self::Barkskin => &MOVE_BARKSKIN,
            Self::BigMagic => &MOVE_BIG_MAGIC,
            Self::DanusGrasp => &MOVE_DANUS_GRASP,
            Self::HealersArts => &MOVE_HEALERS_ARTS,
            Self::HeedMyWords => &MOVE_HEED_MY_WORDS,
            Self::ImprovedStat => &MOVE_IMPROVED_STAT,
            Self::IntoTheLionsDen => &MOVE_INTO_THE_LIONS_DEN,
            Self::LightningRod => &MOVE_LIGHTNING_ROD,
            Self::RitesOfTheLand => &MOVE_RITES_OF_THE_LAND,
            Self::SpiritTongue => &MOVE_SPIRIT_TONGUE,
            Self::BorrowPower => &MOVE_BORROW_POWER,
            Self::CallTheSpirits => &MOVE_CALL_THE_SPIRITS,
            Self::TracklessStep => &MOVE_TRACKLESS_STEP,
            Self::Veil => &MOVE_VEIL,
            Self::WardsBindings => &MOVE_WARDS_BINDINGS,
            Self::WildSoul => &MOVE_WILD_SOUL,
            Self::NaturesWrath => &MOVE_NATURES_WRATH,
            Self::PotentWorkings => &MOVE_POTENT_WORKINGS,
            Self::SharedSouls => &MOVE_SHARED_SOULS,
            Self::SuckThePoisonOut => &MOVE_SUCK_THE_POISON_OUT,
            Self::SuperiorStat => &MOVE_SUPERIOR_STAT,
            Self::VoiceOfTheEarthMother => &MOVE_VOICE_OF_THE_EARTH_MOTHER,
            Self::FromTheNatural => &MOVE_FROM_THE_NATURAL,
            Self::FromTheProdigalReturned => &MOVE_FROM_THE_PRODIGAL_RETURNED,
            Self::AllInTheWrist => &MOVE_ALL_IN_THE_WRIST,
            Self::Ambush => &MOVE_AMBUSH,
            Self::Burgle => &MOVE_BURGLE,
            Self::Catlike => &MOVE_CATLIKE,
            Self::Dabbler => &MOVE_DABBLER,
            Self::DangerSense => &MOVE_DANGER_SENSE,
            Self::FreeRunning => &MOVE_FREE_RUNNING,
            Self::Irresistible => &MOVE_IRRESISTIBLE,
            Self::LaughAtDanger => &MOVE_LAUGH_AT_DANGER,
            Self::LightFingers => &MOVE_LIGHT_FINGERS,
            Self::Perceptive => &MOVE_PERCEPTIVE,
            Self::RapierWit => &MOVE_RAPIER_WIT,
            Self::SkillAtArms => &MOVE_SKILL_AT_ARMS,
            Self::ParryRiposte => &MOVE_PARRY_RIPOSTE,
            Self::SilverTongued => &MOVE_SILVER_TONGUED,
            Self::UnderYourSkin => &MOVE_UNDER_YOUR_SKIN,
            Self::BattleDancer => &MOVE_BATTLE_DANCER,
            Self::CheapShot => &MOVE_CHEAP_SHOT,
            Self::EyeOnTheDoor => &MOVE_EYE_ON_THE_DOOR,
            Self::PantsOnFire => &MOVE_PANTS_ON_FIRE,
            Self::SecondIntent => &MOVE_SECOND_INTENT,
            Self::Slippery => &MOVE_SLIPPERY,
            Self::FromSheriff => &MOVE_FROM_SHERIFF,
            Self::FromBloodSoakedPast => &MOVE_FROM_BLOOD_SOAKED_PAST,
            Self::Armored => &MOVE_ARMORED,
            Self::BattleJoy => &MOVE_BATTLE_JOY,
            Self::Berserker => &MOVE_BERSERKER,
            Self::CarvedOutOfWood => &MOVE_CARVED_OUT_OF_WOOD,
            Self::Dangerous => &MOVE_DANGEROUS,
            Self::Formidable => &MOVE_FORMIDABLE,
            Self::Frosty => &MOVE_FROSTY,
            Self::Guardian => &MOVE_GUARDIAN,
            Self::Intimidating => &MOVE_INTIMIDATING,
            Self::HardToKill => &MOVE_HARD_TO_KILL,
            Self::Unstoppable => &MOVE_UNSTOPPABLE,
            Self::Musclebound => &MOVE_MUSCLEBOUND,
            Self::Payback => &MOVE_PAYBACK,
            Self::Relentless => &MOVE_RELENTLESS,
            Self::SeasonedWarrior => &MOVE_SEASONED_WARRIOR,
            Self::SituationalAwareness => &MOVE_SITUATIONAL_AWARENESS,
            Self::UncannyReflexes => &MOVE_UNCANNY_REFLEXES,
            Self::Unfettered => &MOVE_UNFETTERED,
            Self::TerrorOnTheField => &MOVE_TERROR_ON_THE_FIELD,
            Self::BringerOfRuin => &MOVE_BRINGER_OF_RUIN,
            Self::CutFromGranite => &MOVE_CUT_FROM_GRANITE,
            Self::MightyThews => &MOVE_MIGHTY_THEWS,
            Self::Nemesis => &MOVE_NEMESIS,
            Self::SteadfastGuardian => &MOVE_STEADFAST_GUARDIAN,
            Self::StoneCold => &MOVE_STONE_COLD,
            Self::FromLegacy => &MOVE_FROM_LEGACY,
            Self::FromMissionary => &MOVE_FROM_MISSIONARY,
            Self::FromProphet => &MOVE_FROM_PROPHET,
            Self::AegisOfFaith => &MOVE_AEGIS_OF_FAITH,
            Self::BearWitness => &MOVE_BEAR_WITNESS,
            Self::BreakBread => &MOVE_BREAK_BREAD,
            Self::Bulwark => &MOVE_BULWARK,
            Self::Censure => &MOVE_CENSURE,
            Self::Castigate => &MOVE_CASTIGATE,
            Self::ChroniclerOfStonetop => &MOVE_CHRONICLER_OF_STONETOP,
            Self::ForTheGreaterGood => &MOVE_FOR_THE_GREATER_GOOD,
            Self::HoundOfAratis => &MOVE_HOUND_OF_ARATIS,
            Self::LikeADogWithABone => &MOVE_LIKE_A_DOG_WITH_A_BONE,
            Self::KnowledgeIsPower => &MOVE_KNOWLEDGE_IS_POWER,
            Self::ManyHandsMakeLightWork => &MOVE_MANY_HANDS_MAKE_LIGHT_WORK,
            Self::ABundleOfSticksUnbroken => &MOVE_A_BUNDLE_OF_STICKS_UNBROKEN,
            Self::TheHammerAndTheBook => &MOVE_THE_HAMMER_AND_THE_BOOK,
            Self::TruthOrConsequences => &MOVE_TRUTH_OR_CONSEQUENCES,
            Self::BindingArbitration => &MOVE_BINDING_ARBITRATION,
            Self::VisionUnclouded => &MOVE_VISION_UNCLOUDED,
            Self::WellRead => &MOVE_WELL_READ,
            Self::AMightyRampart => &MOVE_A_MIGHTY_RAMPART,
            Self::Armistice => &MOVE_ARMISTICE,
            Self::Condemn => &MOVE_CONDEMN,
            Self::Proclamation => &MOVE_PROCLAMATION,
            Self::Mirrorshield => &MOVE_MIRRORSHIELD,
            Self::TheTowerEternal => &MOVE_THE_TOWER_ETERNAL,
            Self::FromAuspiciousBirth => &MOVE_FROM_AUSPICIOUS_BIRTH,
            Self::FromItinerantMystic => &MOVE_FROM_ITINERANT_MYSTIC,
            Self::FromSoulOnFire => &MOVE_FROM_SOUL_ON_FIRE,
            Self::ACandleAgainstTheDark => &MOVE_A_CANDLE_AGAINST_THE_DARK,
            Self::LuminousShield => &MOVE_LUMINOUS_SHIELD,
            Self::AllIsIlluminated => &MOVE_ALL_IS_ILLUMINATED,
            Self::AndBeholdAPaleHorse => &MOVE_AND_BEHOLD_A_PALE_HORSE,
            Self::ConsecratedFlame => &MOVE_CONSECRATED_FLAME,
            Self::FireWithin => &MOVE_FIRE_WITHIN,
            Self::GuidingLight => &MOVE_GUIDING_LIGHT,
            Self::HeliorsUnblinkingEye => &MOVE_HELIORS_UNBLINKING_EYE,
            Self::InvokeTheSunGod => &MOVE_INVOKE_THE_SUN_GOD,
            Self::KeepTheHomeFiresBurning => &MOVE_KEEP_THE_HOME_FIRES_BURNING,
            Self::Lamplighter => &MOVE_LAMPLIGHTER,
            Self::Piety => &MOVE_PIETY,
            Self::PurifyingFlames => &MOVE_PURIFYING_FLAMES,
            Self::RadiantCountenance => &MOVE_RADIANT_COUNTENANCE,
            Self::RiseLikeTheSun => &MOVE_RISE_LIKE_THE_SUN,
            Self::SpringsFirstThaw => &MOVE_SPRINGS_FIRST_THAW,
            Self::BurnTwiceAsBright => &MOVE_BURN_TWICE_AS_BRIGHT,
            Self::EmpoweredInvocations => &MOVE_EMPOWERED_INVOCATIONS,
            Self::GloriousServant => &MOVE_GLORIOUS_SERVANT,
            Self::HungryFlames => &MOVE_HUNGRY_FLAMES,
            Self::LightMoreLight => &MOVE_LIGHT_MORE_LIGHT,
            Self::WielderOfTheWhiteFlame => &MOVE_WIELDER_OF_THE_WHITE_FLAME,
            Self::FromPenitent => &MOVE_FROM_PENITENT,
            Self::ArtsOfWar => &MOVE_ARTS_OF_WAR,
            Self::Crew => &MOVE_CREW,
            Self::VeteranCrew => &MOVE_VETERAN_CREW,
            Self::FrontLineLeader => &MOVE_FRONT_LINE_LEADER,
            Self::Logistics => &MOVE_LOGISTICS,
            Self::ReadTheLand => &MOVE_READ_THE_LAND,
            Self::PrepareAWelcome => &MOVE_PREPARE_A_WELCOME,
            Self::SetUpStrike => &MOVE_SET_UP_STRIKE,
            Self::ShakeItOff => &MOVE_SHAKE_IT_OFF,
            Self::ShieldWall => &MOVE_SHIELD_WALL,
            Self::SirPermissionToDieSir => &MOVE_SIR_PERMISSION_TO_DIE_SIR,
            Self::SpeakSoftly => &MOVE_SPEAK_SOFTLY,
            Self::Stentorian => &MOVE_STENTORIAN,
            Self::TakeTheMeasure => &MOVE_TAKE_THE_MEASURE,
            Self::WeHappyFew => &MOVE_WE_HAPPY_FEW,
            Self::BattlefieldGrace => &MOVE_BATTLEFIELD_GRACE,
            Self::HeroesToTheLast => &MOVE_HEROES_TO_THE_LAST,
            Self::FocusFire => &MOVE_FOCUS_FIRE,
            Self::LikeAnOpenBook => &MOVE_LIKE_AN_OPEN_BOOK,
            Self::NobleMien => &MOVE_NOBLE_MIEN,
            Self::PeaceThroughStrength => &MOVE_PEACE_THROUGH_STRENGTH,
            Self::FromWideWanderer => &MOVE_FROM_WIDE_WANDERER,
            Self::FromBeastBonded => &MOVE_FROM_BEAST_BONDED,
            Self::ASafePlace => &MOVE_A_SAFE_PLACE,
            Self::AnimalCompanion => &MOVE_ANIMAL_COMPANION,
            Self::MagnificentSpecimen => &MOVE_MAGNIFICENT_SPECIMEN,
            Self::BigGameHunter => &MOVE_BIG_GAME_HUNTER,
            Self::BlotOutTheSun => &MOVE_BLOT_OUT_THE_SUN,
            Self::CallTheShot => &MOVE_CALL_THE_SHOT,
            Self::ExpertTracker => &MOVE_EXPERT_TRACKER,
            Self::HomeOnTheRange => &MOVE_HOME_ON_THE_RANGE,
            Self::MentalMap => &MOVE_MENTAL_MAP,
            Self::Naturalist => &MOVE_NATURALIST,
            Self::OnTheHoof => &MOVE_ON_THE_HOOF,
            Self::PackHorse => &MOVE_PACK_HORSE,
            Self::Pathfinder => &MOVE_PATHFINDER,
            Self::Predator => &MOVE_PREDATOR,
            Self::SniffOutCorruption => &MOVE_SNIFF_OUT_CORRUPTION,
            Self::Stalker => &MOVE_STALKER,
            Self::Survivalist => &MOVE_SURVIVALIST,
            Self::WardenOfTheWild => &MOVE_WARDEN_OF_THE_WILD,
            Self::WildSpeech => &MOVE_WILD_SPEECH,
            Self::Worldly => &MOVE_WORLDLY,
            Self::Alpha => &MOVE_ALPHA,
            Self::BeastOfLegend => &MOVE_BEAST_OF_LEGEND,
            Self::ConstantVigilance => &MOVE_CONSTANT_VIGILANCE,
            Self::GiantSlayer => &MOVE_GIANT_SLAYER,
            Self::Trailblazer => &MOVE_TRAILBLAZER,
            Self::WalkItOff => &MOVE_WALK_IT_OFF,
            Self::Attuned => &MOVE_ATTUNED,
            Self::ConduitOfPower => &MOVE_CONDUIT_OF_POWER,
            Self::Countermeasures => &MOVE_COUNTERMEASURES,
            Self::EverythingBleeds => &MOVE_EVERYTHING_BLEEDS,
            Self::EverythingBurns => &MOVE_EVERYTHING_BURNS,
            Self::InitiateOfTheSecretArts => &MOVE_INITIATE_OF_THE_SECRET_ARTS,
            Self::LetsMakeADeal => &MOVE_LETS_MAKE_A_DEAL,
            Self::Logbook => &MOVE_LOGBOOK,
            Self::Magpie => &MOVE_MAGPIE,
            Self::NeverAtALoss => &MOVE_NEVER_AT_A_LOSS,
            Self::Polyglot => &MOVE_POLYGLOT,
            Self::Cryptologist => &MOVE_CRYPTOLOGIST,
            Self::QuickStudy => &MOVE_QUICK_STUDY,
            Self::SafetyFirst => &MOVE_SAFETY_FIRST,
            Self::SageAdvice => &MOVE_SAGE_ADVICE,
            Self::WellVersed => &MOVE_WELL_VERSED,
            Self::WorkWithWhatYouveGot => &MOVE_WORK_WITH_WHAT_YOUVE_GOT,
            Self::ArcaneAdept => &MOVE_ARCANE_ADEPT,
            Self::DeepInsight => &MOVE_DEEP_INSIGHT,
            Self::Improvise => &MOVE_IMPROVISE,
            Self::MindOverMagic => &MOVE_MIND_OVER_MAGIC,
            Self::Overchannel => &MOVE_OVERCHANNEL,
            Self::ProofAgainstDetection => &MOVE_PROOF_AGAINST_DETECTION,
            Self::FromImpetuousYouth => &MOVE_FROM_IMPETUOUS_YOUTH,
            Self::FromDriven => &MOVE_FROM_DRIVEN,
            Self::FromDestined => &MOVE_FROM_DESTINED,
            Self::AngerIsAGift => &MOVE_ANGER_IS_A_GIFT,
            Self::SpeakTruthToPower => &MOVE_SPEAK_TRUTH_TO_POWER,
            Self::BetterPartOfValor => &MOVE_BETTER_PART_OF_VALOR,
            Self::IGetKnockedDown => &MOVE_I_GET_KNOCKED_DOWN,
            Self::ButIGetUpAgain => &MOVE_BUT_I_GET_UP_AGAIN,
            Self::InOverYourHead => &MOVE_IN_OVER_YOUR_HEAD,
            Self::IronWill => &MOVE_IRON_WILL,
            Self::InquiringMinds => &MOVE_INQUIRING_MINDS,
            Self::NeverGonnaKeepMeDown => &MOVE_NEVER_GONNA_KEEP_ME_DOWN,
            Self::PotentialForGreatness => &MOVE_POTENTIAL_FOR_GREATNESS,
            Self::Resourceful => &MOVE_RESOURCEFUL,
            Self::SomethingToRememberMeBy => &MOVE_SOMETHING_TO_REMEMBER_ME_BY,
            Self::ToughLove => &MOVE_TOUGH_LOVE,
            Self::Underestimated => &MOVE_UNDERESTIMATED,
            Self::UpWithPeople => &MOVE_UP_WITH_PEOPLE,
            Self::Versatile => &MOVE_VERSATILE,
            Self::AForceToBeReckonedWith => &MOVE_A_FORCE_TO_BE_RECKONED_WITH,
            Self::BigDamnHero => &MOVE_BIG_DAMN_HERO,
            Self::PwSuperiorStat => &MOVE_PW_SUPERIOR_STAT,
            Self::Undaunted => &MOVE_UNDAUNTED,
            Self::VoiceOfExperience => &MOVE_VOICE_OF_EXPERIENCE,
        }
    }
}
impl stonetop::keys::BackgroundKey {
    /// The printed content this key names.
    #[must_use]
    pub fn fixed_part(self) -> &'static stonetop::fixed::BackgroundFixed {
        match self {
            Self::Initiate => &BACKGROUND_INITIATE,
            Self::RaisedByWolves => &BACKGROUND_RAISED_BY_WOLVES,
            Self::Vessel => &BACKGROUND_VESSEL,
            Self::TheNatural => &BACKGROUND_THE_NATURAL,
            Self::ALifeOfCrime => &BACKGROUND_A_LIFE_OF_CRIME,
            Self::TheProdigalReturned => &BACKGROUND_THE_PRODIGAL_RETURNED,
            Self::Sheriff => &BACKGROUND_SHERIFF,
            Self::BloodSoakedPast => &BACKGROUND_BLOOD_SOAKED_PAST,
            Self::StormMarked => &BACKGROUND_STORM_MARKED,
            Self::Legacy => &BACKGROUND_LEGACY,
            Self::Missionary => &BACKGROUND_MISSIONARY,
            Self::Prophet => &BACKGROUND_PROPHET,
            Self::AuspiciousBirth => &BACKGROUND_AUSPICIOUS_BIRTH,
            Self::ItinerantMystic => &BACKGROUND_ITINERANT_MYSTIC,
            Self::SoulOnFire => &BACKGROUND_SOUL_ON_FIRE,
            Self::Scion => &BACKGROUND_SCION,
            Self::Penitent => &BACKGROUND_PENITENT,
            Self::Luminary => &BACKGROUND_LUMINARY,
            Self::MightyHunter => &BACKGROUND_MIGHTY_HUNTER,
            Self::WideWanderer => &BACKGROUND_WIDE_WANDERER,
            Self::BeastBonded => &BACKGROUND_BEAST_BONDED,
            Self::Patriot => &BACKGROUND_PATRIOT,
            Self::Antiquarian => &BACKGROUND_ANTIQUARIAN,
            Self::WitchHunter => &BACKGROUND_WITCH_HUNTER,
            Self::ImpetuousYouth => &BACKGROUND_IMPETUOUS_YOUTH,
            Self::Driven => &BACKGROUND_DRIVEN,
            Self::Destined => &BACKGROUND_DESTINED,
        }
    }
}
impl stonetop::keys::SpecialPossessionKey {
    /// The printed content this key names.
    #[must_use]
    pub fn fixed_part(self) -> &'static stonetop::fixed::SpecialPossessionFixed {
        match self {
            Self::SacredPouch => &SPECIAL_POSSESSION_SACRED_POUCH,
            Self::Apiary => &SPECIAL_POSSESSION_APIARY,
            Self::CollectedOfferings => &SPECIAL_POSSESSION_COLLECTED_OFFERINGS,
            Self::GoatHerd => &SPECIAL_POSSESSION_GOAT_HERD,
            Self::HerbGarden => &SPECIAL_POSSESSION_HERB_GARDEN,
            Self::Mastiffs => &SPECIAL_POSSESSION_MASTIFFS,
            Self::BurglarsKit => &SPECIAL_POSSESSION_BURGLARS_KIT,
            Self::CarpentersTools => &SPECIAL_POSSESSION_CARPENTERS_TOOLS,
            Self::Distillery => &SPECIAL_POSSESSION_DISTILLERY,
            Self::HiddenStash => &SPECIAL_POSSESSION_HIDDEN_STASH,
            Self::MummersKit => &SPECIAL_POSSESSION_MUMMERS_KIT,
            Self::ScribesTools => &SPECIAL_POSSESSION_SCRIBES_TOOLS,
            Self::Tannery => &SPECIAL_POSSESSION_TANNERY,
            Self::TradeContacts => &SPECIAL_POSSESSION_TRADE_CONTACTS,
            Self::ChirurgeonsTools => &SPECIAL_POSSESSION_CHIRURGEONS_TOOLS,
            Self::HusbandryTools => &SPECIAL_POSSESSION_HUSBANDRY_TOOLS,
            Self::Smithy => &SPECIAL_POSSESSION_SMITHY,
            Self::StoneworkersTools => &SPECIAL_POSSESSION_STONEWORKERS_TOOLS,
            Self::PhWeaponsOfWar => &SPECIAL_POSSESSION_PH_WEAPONS_OF_WAR,
            Self::YourSymbolOfAuthority => &SPECIAL_POSSESSION_YOUR_SYMBOL_OF_AUTHORITY,
            Self::Aviary => &SPECIAL_POSSESSION_AVIARY,
            Self::EngineersTools => &SPECIAL_POSSESSION_ENGINEERS_TOOLS,
            Self::BooksScrolls => &SPECIAL_POSSESSION_BOOKS_SCROLLS,
            Self::Chandlery => &SPECIAL_POSSESSION_CHANDLERY,
            Self::Glassworks => &SPECIAL_POSSESSION_GLASSWORKS,
            Self::HolyRelics => &SPECIAL_POSSESSION_HOLY_RELICS,
            Self::LuthiersTools => &SPECIAL_POSSESSION_LUTHIERS_TOOLS,
            Self::PersonalSymbol => &SPECIAL_POSSESSION_PERSONAL_SYMBOL,
            Self::PmWeaponsOfWar => &SPECIAL_POSSESSION_PM_WEAPONS_OF_WAR,
            Self::CompositeBow => &SPECIAL_POSSESSION_COMPOSITE_BOW,
            Self::Hideouts => &SPECIAL_POSSESSION_HIDEOUTS,
            Self::Hounds => &SPECIAL_POSSESSION_HOUNDS,
            Self::LayOfTheLand => &SPECIAL_POSSESSION_LAY_OF_THE_LAND,
            Self::TrappingGear => &SPECIAL_POSSESSION_TRAPPING_GEAR,
            Self::Laboratory => &SPECIAL_POSSESSION_LABORATORY,
            Self::Paraphernalia => &SPECIAL_POSSESSION_PARAPHERNALIA,
            Self::AHeapOfExpectations => &SPECIAL_POSSESSION_A_HEAP_OF_EXPECTATIONS,
            Self::AGoodDog => &SPECIAL_POSSESSION_A_GOOD_DOG,
            Self::PersonalTokenFraughtWithMeaning => {
                &SPECIAL_POSSESSION_PERSONAL_TOKEN_FRAUGHT_WITH_MEANING
            }
        }
    }
}
impl stonetop::keys::BackstoryKey {
    /// The printed content this key names.
    #[must_use]
    pub fn fixed_part(self) -> &'static stonetop::fixed::BackstoryFixed {
        match self {
            Self::YourSacredPouch => &BACKSTORY_YOUR_SACRED_POUCH,
            Self::TheEarthMother => &BACKSTORY_THE_EARTH_MOTHER,
            Self::TallTales => &BACKSTORY_TALL_TALES,
            Self::AHistoryOfViolence => &BACKSTORY_A_HISTORY_OF_VIOLENCE,
            Self::TheChronicle => &BACKSTORY_THE_CHRONICLE,
            Self::TheLawkeeper => &BACKSTORY_THE_LAWKEEPER,
            Self::PraiseTheDay => &BACKSTORY_PRAISE_THE_DAY,
            Self::WarStories => &BACKSTORY_WAR_STORIES,
            Self::SomethingWickedThisWayComes => &BACKSTORY_SOMETHING_WICKED_THIS_WAY_COMES,
            Self::Collection => &BACKSTORY_COLLECTION,
            Self::MajorArcana => &BACKSTORY_MAJOR_ARCANA,
            Self::MinorArcana => &BACKSTORY_MINOR_ARCANA,
            Self::FearAnger => &BACKSTORY_FEAR_ANGER,
        }
    }
}
impl stonetop::keys::PlaybookKey {
    /// The printed content this key names.
    #[must_use]
    pub fn fixed_part(self) -> &'static stonetop::fixed::PlaybookFixed {
        match self {
            Self::TheBlessed => &PLAYBOOK_THE_BLESSED,
            Self::TheFox => &PLAYBOOK_THE_FOX,
            Self::TheHeavy => &PLAYBOOK_THE_HEAVY,
            Self::TheJudge => &PLAYBOOK_THE_JUDGE,
            Self::TheLightbearer => &PLAYBOOK_THE_LIGHTBEARER,
            Self::TheMarshal => &PLAYBOOK_THE_MARSHAL,
            Self::TheRanger => &PLAYBOOK_THE_RANGER,
            Self::TheSeeker => &PLAYBOOK_THE_SEEKER,
            Self::TheWouldBeHero => &PLAYBOOK_THE_WOULD_BE_HERO,
        }
    }
}
