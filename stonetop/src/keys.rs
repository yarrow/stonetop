//! Lookup keys: `PlaybookKey`, the four item key enums, `MoveKey`, `BackgroundKey`,
//! `SpecialPossessionKey`, and `BackstoryKey`, and `GizmoKey` for gear. This file is the source
//! of truth for `XKey` types: every playbook, item, and gizmo name in `codegen/json5/` must
//! resolve to a variant here, and every variant must be produced by some name (checked by
//! `codegen/tests/keys_consistent.rs`). A shipped key is permanent, so you can add variants
//! freely but you can't rename or remove one.
//!
//! Playbooks share items: every playbook has the same Improved Stat move, for instance. A shared
//! item gets one variant, listed under the first playbook to use it, and later playbooks show a
//! comment where that variant would have been.
//!
//! Gizmos are defined once, in `gear.json5`, and listed here by that file's sections. A gizmo's
//! variant is its name, then its material unless that is iron, then `PiercingN` for the starred
//! piercing upgrade: `Battleaxe`, `BattleaxePiercing1`, `BattleaxeBronze`, `LongSpearFineSteel`.
//!
//! The strum derives (`Display`, `EnumString`, `EnumIter`) and databake's `Bake` are behind the
//! `codegen` feature: `codegen` needs them to resolve names, walk every variant, and bake Fixed
//! values, but they never reach the WASM build.

#[cfg(feature = "codegen")]
use databake::Bake;
use serde::{Deserialize, Serialize};
#[cfg(feature = "codegen")]
use strum::{Display, EnumIter, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "codegen", derive(Display, EnumString, EnumIter, Bake))]
#[cfg_attr(feature = "codegen", databake(path = stonetop::keys))]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "codegen", derive(Display, EnumString, EnumIter, Bake))]
#[cfg_attr(feature = "codegen", databake(path = stonetop::keys))]
pub enum BackgroundKey {
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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "codegen", derive(Display, EnumString, EnumIter, Bake))]
#[cfg_attr(feature = "codegen", databake(path = stonetop::keys))]
pub enum SpecialPossessionKey {
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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "codegen", derive(Display, EnumString, EnumIter, Bake))]
#[cfg_attr(feature = "codegen", databake(path = stonetop::keys))]
pub enum GizmoKey {
    // Weapons of war (gear sheet)
    MaceOrFlail,
    Battleaxe,
    BattleaxePiercing1,
    BattleaxePiercing2,
    ShortSword,
    ShortSwordPiercing1,
    ShortSwordPiercing2,
    Sword,
    SwordPiercing1,
    SwordPiercing2,
    Warhammer,
    Crossbow,
    CompositeBow,

    // Bronze weapons (gear sheet)
    MaceOrFlailBronze,
    BattleaxeBronze,
    BattleaxeBronzePiercing1,
    BattleaxeBronzePiercing2,
    ShortSwordBronze,
    ShortSwordBronzePiercing1,
    ShortSwordBronzePiercing2,
    SwordBronze,
    SwordBronzePiercing1,
    SwordBronzePiercing2,
    WarhammerBronze,

    // Armor (gear sheet)
    CuirassBoiledLeather,
    HauberkCuirassScale,
    Vest,

    // Light sources (gear sheet)
    Candle,
    Lantern,
    BullseyeLantern,

    // Tools & trades (gear sheet)
    SmallMetalTool,
    GlassVial,
    BlockTackle,
    Instrument,
    MetalTools,
    Mirror,

    // Writing implements (gear sheet)
    SlateAndChalk,
    WaxTabletAndStylus,
    Parchment,
    FineVellum,
    InkVialAndQuills,
    EmptyBookParchment,
    EmptyBookFineVellum,

    // Exotic stuff (gear sheet)
    BendisRoot,
    Bezoar,
    Naphtha,
    SilverAlloyDagger,

    // Trade goods (gear sheet)
    Salt,
    SkinOfFineWhisky,
    PurseOfCoppers,
    FirkinOfFineWhisky,
    HandfulOfSilvers,
    PurseOfSilvers,

    // Slotted items (Inventory insert)
    MessKit,
    Bedroll,
    Blanket,
    ChangeOfClothes,
    Rope,
    Shovel,
    SledgeLitterTravois,
    SnowShoes,
    Torch,
    OilLamp,
    ExtraOil,
    Firewood,
    Hatchet,
    Mallet,
    Mattock,
    Maul,
    Staff,
    Spear,
    LongSpear,
    BowIronArrows,
    ExtraArrows,
    Javelins,
    Shield,
    ThickHides,
    Cloak,

    // Small items (Inventory insert)
    KnifeOrDagger,
    Sling,
    Rushlight,
    Tinderbox,
    NeedleThread,
    HandfulOfCoppers,
    SkinOfCommonWhisky,
    Awl,
    Bowstring,
    Chalk,
    Charcoal,
    ClayJar,
    ClothRag,
    Comb,
    Cup,
    ExtraSocks,
    Gloves,
    LittleBox,
    Sack,
    Sawdust,
    Tallow,
    TwineCord,
    Waterskin,
    Whetstone,
    Whistle,

    // Kit contents: the Blessed
    SacredPouch,
    Beeswax,
    Honey,
    BeeSmokers,
    HatsVeils,
    Milk,
    Cheese,
    Pelts,
    Meat,
    Blood,
    Horn,
    Wool,
    Shears,
    MortarsPestles,
    Herbs,
    Seeds,
    Remedies,
    MildPoisons,
    Spades,

    // Kit contents: the Fox
    Picks,
    Files,
    Snippers,
    Wire,
    Prybars,
    Hacksaws,
    GrapplingHook,
    Chisels,
    Nails,
    Pitch,
    Saws,
    Firkins,
    CopperTubes,
    Malt,
    JugglingBalls,
    WhirlybirdSeeds,
    Motley,
    Ribbons,
    Bells,
    Puppets,
    Fiddle,
    Ink,
    Pigments,
    Quills,
    Notebook,
    Lime,
    Acid,
    Salts,
    ThickGloves,
    Glass,
    Silk,
    Spice,
    MedicinalHerbs,
    Ivory,

    // Kit contents: the Heavy
    Catgut,
    Straps,
    Bandages,
    Tubes,
    Poultices,
    WillowBark,
    Bonesaws,
    Brushes,
    Muzzles,
    Collars,
    Feed,
    Whips,
    Bridles,
    IronGoods,
    Ingots,
    Tongs,
    Bellows,
    Drills,
    Spikes,

    // Kit contents: the Judge
    BlackIronMaul,
    MakerglassShield,
    DarkIceHelm,
    BirdHoods,
    Tethers,
    Seed,
    MessengerBirds,
    Birdcages,
    Rulers,
    Tapes,
    Rods,
    PlumbBobs,
    Tripods,

    // Kit contents: the Lightbearer
    Wicks,
    ScentedHerbs,
    Soap,
    Lye,
    Ash,
    Charms,
    Lenses,
    Sand,
    Marbles,
    VariousWoods,
    Stains,
    Lute,

    // Kit contents: the Marshal
    LongSpearFineSteel,

    // Kit contents: the Ranger
    Snares,
    Musk,
    Bait,

    // Kit contents: the Seeker
    Chemics,
    Reagents,
    Measures,
    Scales,
    Decanters,
    Crystals,
    Incense,
    Talismans,
    Bone,
    EyeOfNewt,
    Braziers,
    Cauldron,

    // Kit contents: the Would-be Hero
    CrestedShield,
    WoolCloak,
    Letter,
    Flute,
    Locket,
    EngravedTinderbox,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "codegen", derive(Display, EnumString, EnumIter, Bake))]
#[cfg_attr(feature = "codegen", databake(path = stonetop::keys))]
pub enum MoveKey {
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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "codegen", derive(Display, EnumString, EnumIter, Bake))]
#[cfg_attr(feature = "codegen", databake(path = stonetop::keys))]
pub enum BackstoryKey {
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

    // Backstories for the Would-be Hero
    FearAnger,
}
