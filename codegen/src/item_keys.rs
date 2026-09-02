//! Lookup keys, used on the server to look up fixed values baked into the binary, and on both
//! client and server to look up persistent state. This file is the source of truth: every
//! playbook and item name in `json5/` must resolve to a variant here, and every variant must be
//! produced by some name (`tests/keys_consistent.rs` checks both). A shipped key is permanent, so
//! you can add variants freely but you can't rename or remove one.
//!
//! Playbooks share items: every playbook has the same Improved Stat move, for instance. A shared
//! item gets one variant, listed under the first playbook to use it, and later playbooks show a
//! comment where that variant would have been.
//!
//! The client crate gets a copy of this file with the strum derives stripped; see
//! `bin/copy-item-keys.rs`.

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Display,
    EnumString,
    EnumIter,
)]
pub enum PlaybookKey {
    TheBlessed,
    TheFox,
    TheHeavy,
    TheJudge,
    TheLightbearer,
    TheMarshal,
    TheRanger,
    TheSeeker,
    TheWouldBeHero,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Display,
    EnumString,
    EnumIter,
)]
pub enum ItemKey {
    // Backgrounds for the Blessed
    Initiate,
    RaisedByWolves,
    Vessel,

    // Backgrounds for the Fox
    TheNatural,
    ALifeOfCrime,
    TheProdigalReturned,

    // Backgrounds for the Heavy
    Sheriff,
    BloodSoakedPast,
    StormMarked,

    // Backgrounds for the Judge
    Legacy,
    Missionary,
    Prophet,

    // Backgrounds for the Lightbearer
    AuspiciousBirth,
    ItinerantMystic,
    SoulOnFire,

    // Backgrounds for the Marshal
    Scion,
    Penitent,
    Luminary,

    // Backgrounds for the Ranger
    MightyHunter,
    WideWanderer,
    BeastBonded,

    // Backgrounds for the Seeker
    Patriot,
    Antiquarian,
    WitchHunter,

    // Backgrounds for the Would-be Hero
    ImpetuousYouth,
    Driven,
    Destined,

    // Special Possessions for the Blessed
    SacredPouch,
    Apiary,
    CollectedOfferings,
    GoatHerd,
    HerbGarden,
    Mastiffs,

    // Special Possessions for the Fox
    BurglarsKit,
    CarpentersTools,
    Distillery,
    HiddenStash,
    MummersKit,
    ScribesTools,
    Tannery,
    TradeContacts,

    // Special Possessions for the Heavy
    // Distillery
    ChirurgeonsTools,
    HusbandryTools,
    Smithy,
    StoneworkersTools,
    PhWeaponsOfWar,

    // Special Possessions for the Judge
    YourSymbolOfAuthority,
    // ScribesTools
    Aviary,
    // CarpentersTools
    EngineersTools,
    // Smithy

    // Special Possessions for the Lightbearer
    // Apiary
    BooksScrolls,
    Chandlery,
    // Distillery
    Glassworks,
    HolyRelics,
    LuthiersTools,

    // Special Possessions for the Marshal
    // ChirurgeonsTools
    // Distillery
    // EngineersTools
    PersonalSymbol,
    // ScribesTools
    PmWeaponsOfWar,

    // Special Possessions for the Ranger
    CompositeBow,
    // Distillery
    Hideouts,
    // HusbandryTools
    Hounds,
    LayOfTheLand,
    TrappingGear,

    // Special Possessions for the Seeker
    // ScribesTools
    // BooksScrolls
    // Distillery
    // EngineersTools
    Laboratory,
    Paraphernalia,
    // TradeContacts

    // Special Possessions for the Would-be Hero
    AHeapOfExpectations,
    AGoodDog,
    // HusbandryTools
    // Smithy
    // StoneworkersTools
    PersonalTokenFraughtWithMeaning,
    // Tannery

    // Moves for the Blessed
    FromRaisedByWolves,
    FromVessel,
    AmuletsTalismans,
    Barkskin,
    BigMagic,
    DanusGrasp,
    HealersArts,
    HeedMyWords,
    ImprovedStat,
    IntoTheLionsDen,
    LightningRod,
    RitesOfTheLand,
    SpiritTongue,
    BorrowPower,
    CallTheSpirits,
    TracklessStep,
    Veil,
    WardsBindings,
    WildSoul,
    NaturesWrath,
    PotentWorkings,
    SharedSouls,
    SuckThePoisonOut,
    SuperiorStat,
    VoiceOfTheEarthMother,

    // Moves for the Fox
    FromTheNatural,
    FromTheProdigalReturned,
    AllInTheWrist,
    Ambush,
    Burgle,
    Catlike,
    Dabbler,
    DangerSense,
    FreeRunning,
    // ImprovedStat
    Irresistible,
    LaughAtDanger,
    LightFingers,
    Perceptive,
    RapierWit,
    SkillAtArms,
    ParryRiposte,
    SilverTongued,
    UnderYourSkin,
    BattleDancer,
    CheapShot,
    EyeOnTheDoor,
    PantsOnFire,
    SecondIntent,
    Slippery,
    // SuperiorStat

    // Moves for the Heavy
    FromSheriff,
    FromBloodSoakedPast,
    Armored,
    BattleJoy,
    Berserker,
    CarvedOutOfWood,
    Dangerous,
    Formidable,
    Frosty,
    Guardian,
    // ImprovedStat
    Intimidating,
    HardToKill,
    Unstoppable,
    Musclebound,
    Payback,
    Relentless,
    SeasonedWarrior,
    SituationalAwareness,
    UncannyReflexes,
    Unfettered,
    TerrorOnTheField,
    BringerOfRuin,
    CutFromGranite,
    MightyThews,
    Nemesis,
    SteadfastGuardian,
    StoneCold,
    // SuperiorStat

    // Moves for the Judge
    FromLegacy,
    FromMissionary,
    FromProphet,
    AegisOfFaith,
    // Armored
    BearWitness,
    BreakBread,
    Bulwark,
    Censure,
    Castigate,
    ChroniclerOfStonetop,
    ForTheGreaterGood,
    HoundOfAratis,
    LikeADogWithABone,
    // ImprovedStat
    KnowledgeIsPower,
    ManyHandsMakeLightWork,
    ABundleOfSticksUnbroken,
    TheHammerAndTheBook,
    TruthOrConsequences,
    BindingArbitration,
    VisionUnclouded,
    WellRead,
    AMightyRampart,
    Armistice,
    Condemn,
    Proclamation,
    Mirrorshield,
    // SuperiorStat
    TheTowerEternal,

    // Moves for the Lightbearer
    FromAuspiciousBirth,
    FromItinerantMystic,
    FromSoulOnFire,
    ACandleAgainstTheDark,
    LuminousShield,
    AllIsIlluminated,
    AndBeholdAPaleHorse,
    ConsecratedFlame,
    FireWithin,
    GuidingLight,
    HeliorsUnblinkingEye,
    // ImprovedStat
    InvokeTheSunGod,
    KeepTheHomeFiresBurning,
    Lamplighter,
    Piety,
    PurifyingFlames,
    RadiantCountenance,
    RiseLikeTheSun,
    SpringsFirstThaw,
    BurnTwiceAsBright,
    EmpoweredInvocations,
    GloriousServant,
    HungryFlames,
    LightMoreLight,
    // SuperiorStat
    WielderOfTheWhiteFlame,

    // Moves for the Marshal
    FromPenitent,
    // Armored
    ArtsOfWar,
    Crew,
    VeteranCrew,
    FrontLineLeader,
    // ImprovedStat
    Logistics,
    ReadTheLand,
    PrepareAWelcome,
    SetUpStrike,
    ShakeItOff,
    ShieldWall,
    SirPermissionToDieSir,
    SpeakSoftly,
    Stentorian,
    TakeTheMeasure,
    WeHappyFew,
    BattlefieldGrace,
    HeroesToTheLast,
    FocusFire,
    LikeAnOpenBook,
    NobleMien,
    PeaceThroughStrength,
    // SuperiorStat

    // Moves for the Ranger
    FromWideWanderer,
    FromBeastBonded,
    ASafePlace,
    AnimalCompanion,
    MagnificentSpecimen,
    BigGameHunter,
    BlotOutTheSun,
    CallTheShot,
    ExpertTracker,
    HomeOnTheRange,
    // ImprovedStat
    MentalMap,
    Naturalist,
    OnTheHoof,
    PackHorse,
    Pathfinder,
    Predator,
    SniffOutCorruption,
    Stalker,
    Survivalist,
    WardenOfTheWild,
    WildSpeech,
    Worldly,
    Alpha,
    BeastOfLegend,
    ConstantVigilance,
    GiantSlayer,
    // SuperiorStat
    Trailblazer,
    WalkItOff,

    // Moves for the Seeker
    Attuned,
    ConduitOfPower,
    Countermeasures,
    EverythingBleeds,
    EverythingBurns,
    // ImprovedStat
    InitiateOfTheSecretArts,
    LetsMakeADeal,
    Logbook,
    Magpie,
    NeverAtALoss,
    Polyglot,
    Cryptologist,
    QuickStudy,
    SafetyFirst,
    SageAdvice,
    WellVersed,
    WorkWithWhatYouveGot,
    ArcaneAdept,
    DeepInsight,
    Improvise,
    // SuperiorStat
    MindOverMagic,
    Overchannel,
    ProofAgainstDetection,

    // Moves for the Would-be Hero
    FromImpetuousYouth,
    FromDriven,
    FromDestined,
    AngerIsAGift,
    SpeakTruthToPower,
    BetterPartOfValor,
    IGetKnockedDown,
    ButIGetUpAgain,
    // ImprovedStat
    InOverYourHead,
    IronWill,
    InquiringMinds,
    NeverGonnaKeepMeDown,
    PotentialForGreatness,
    Resourceful,
    SomethingToRememberMeBy,
    ToughLove,
    Underestimated,
    UpWithPeople,
    Versatile,
    AForceToBeReckonedWith,
    BigDamnHero,
    PwSuperiorStat,
    Undaunted,
    VoiceOfExperience,

    // Backstories for the Blessed
    YourSacredPouch,
    TheEarthMother,

    // Backstories for the Fox
    TallTales,

    // Backstories for the Heavy
    AHistoryOfViolence,

    // Backstories for the Judge
    TheChronicle,
    TheLawkeeper,

    // Backstories for the Lightbearer
    PraiseTheDay,

    // Backstories for the Marshal
    WarStories,

    // Backstories for the Ranger
    SomethingWickedThisWayComes,

    // Backstories for the Seeker
    Collection,
    MajorArcana,
    MinorArcana,

    // Backstories for the Would-be Hero
    FearAnger,
}
