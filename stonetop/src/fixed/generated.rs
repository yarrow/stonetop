//! Baked Fixed content: written by `cargo xtask bake` from `codegen/json5/`, checked by
//! `codegen/tests/generated_fresh.rs`. Do not edit; change the json5 and run the command.
//!
//! The statics are grouped by playbook, a shared item appearing once under the first playbook
//! that uses it, as in `keys.rs`. Each kind's `fixed_part()` matches every key to its static.

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
    description: "<p>When you <strong><em>Ambush with a <strong>hand</strong> weapon</em></strong>, you have advantage on your damage roll.</p>",
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
    description: "<p>When you <strong><em>carry a shield</em></strong>, mark only ◆ (instead of ◆◆). Also, you can ignore the <em>cumbersome</em> tag on any armor you wear.</p><p>If you take this move at the start of play, add an ◇◇ iron hauberk, ◇◇ bronze cuirass, or ◇◇ scale coat to your inventory (all are 2 armor, <em>warm, cumbersome</em>).</p>",
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

impl stonetop::keys::MoveKey {
    /// The printed Move this key names.
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
