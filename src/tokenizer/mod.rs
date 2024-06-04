pub const CARD_TYPES: &[&str] = &["artifact", "battle", "conspiracy", "creature", "dungeon", "enchantment", "instant", "land", "phenomenon", "plane", "planeswalker", "scheme", "sorcery", "tribal", "vanguard"];
pub const CARD_SUPERTYPES: &[&str] = &["basic", "legendary", "world", "snow"];
pub const ABILITY_KEYWORDS: &[&str] = &["deathtouch",  "defender",  "double strike",  "enchant",  "equip",  "first strike",  "flash",  "flying",  "haste",  "hexproof",  "indestructible",  "lifelink",  "menace",  "prowess",  "reach",  "trample",  "vigilance",  "absorb",  "affinity",  "amplify",  "annihilator",  "aura swap",  "awaken",  "banding",  "battle cry",  "bestow",  "bloodthirst",  "bushido",  "buyback",  "cascade",  "champion",  "changeling",  "cipher",  "conspire",  "convoke",  "cumulative upkeep",  "cycling",  "dash",  "delve",  "dethrone",  "devoid",  "devour",  "dredge",  "echo",  "entwine",  "epic",  "evoke",  "evolve",  "exalted",  "exploit",  "extort",  "fading",  "fear",  "flanking",  "flashback",  "forecast",  "fortify",  "frenzy",  "fuse",  "graft",  "gravestorm",  "haunt",  "hidden agenda",  "hideaway",  "horsemanship",  "infect",  "ingest",  "intimidate",  "kicker",  "landhome",  "landwalk",  "level up",  "living weapon",  "madness",  "megamorph",  "miracle",  "modular",  "morph",  "myriad",  "ninjutsu",  "offering",  "outlast",  "overload",  "persist",  "phasing",  "poisonous",  "protection",  "provoke",  "prowl",  "rampage",  "rebound",  "recover",  "reinforce",  "renown",  "replicate",  "retrace",  "ripple",  "scavenge",  "skulk",  "shadow",  "shroud",  "soulbond",  "soulshift",  "splice",  "split second",  "storm",  "substance",  "sunburst",  "surge",  "suspend",  "totem armor",  "transfigure",  "transmute",  "tribute",  "undying",  "unearth",  "unleash",  "vanishing",  "wither"];
pub const CREATURE_TYPES: &[&str] = &["advisor",  "aetherborn",  "alien",  "ally",  "angel",  "antelope",  "ape",  "archer",  "archon",  "armadillo",  "army",  "artificer",  "assassin",  "assembly-worker",  "astartes",  "atog",  "aurochs",  "avatar",  "azra",  "badger",  "balloon",  "barbarian",  "bard",  "basilisk",  "bat",  "bear",  "beast",  "beaver",  "beeble",  "beholder",  "berserker",  "bird",  "blinkmoth",  "boar",  "bringer",  "brushwagg",  "camarid",  "camel",  "capybara",  "caribou",  "carrier",  "cat",  "centaur",  "cephalid",  "child",  "chimera",  "citizen",  "cleric",  "clown",  "cockatrice",  "construct",  "coward",  "coyote",  "crab",  "crocodile",  "c’tan",  "custodes",  "cyberman",  "cyclops",  "dalek",  "dauthi",  "demigod",  "demon",  "deserter",  "detective",  "devil",  "dinosaur",  "djinn",  "doctor",  "dog",  "dragon",  "drake",  "dreadnought",  "drone",  "druid",  "dryad",  "dwarf",  "efreet",  "egg",  "elder",  "eldrazi",  "elemental",  "elephant",  "elf",  "elk",  "employee",  "eye",  "faerie",  "ferret",  "fish",  "flagbearer",  "fox",  "fractal",  "frog",  "fungus",  "gamer",  "gargoyle",  "germ",  "giant",  "gith",  "gnoll",  "gnome",  "goat",  "goblin",  "god",  "golem",  "gorgon",  "graveborn",  "gremlin",  "griffin",  "guest",  "hag",  "halfling",  "hamster",  "harpy",  "hellion",  "hippo",  "hippogriff",  "homarid",  "homunculus",  "horror",  "horse",  "human",  "hydra",  "hyena",  "illusion",  "imp",  "incarnation",  "inkling",  "inquisitor",  "insect",  "jackal",  "jellyfish",  "juggernaut",  "kavu",  "kirin",  "kithkin",  "knight",  "kobold",  "kor",  "kraken",  "llama",  "lamia",  "lammasu",  "leech",  "leviathan",  "lhurgoyf",  "licid",  "lizard",  "manticore",  "masticore",  "mercenary",  "merfolk",  "metathran",  "minion",  "minotaur",  "mite",  "mole",  "monger",  "mongoose",  "monk",  "monkey",  "moonfolk",  "mount",  "mouse",  "mutant",  "myr",  "mystic",  "naga",  "nautilus",  "necron",  "nephilim",  "nightmare",  "nightstalker",  "ninja",  "noble",  "noggle",  "nomad",  "nymph",  "octopus",  "ogre",  "ooze",  "orb",  "orc",  "orgg",  "otter",  "ouphe",  "ox",  "oyster",  "pangolin",  "peasant",  "pegasus",  "pentavite",  "performer",  "pest",  "phelddagrif",  "phoenix",  "phyrexian",  "pilot",  "pincher",  "pirate",  "plant",  "porcupine",  "possum",  "praetor",  "primarch",  "prism",  "processor",  "rabbit",  "raccoon",  "ranger",  "rat",  "rebel",  "reflection",  "rhino",  "rigger",  "robot",  "rogue",  "sable",  "salamander",  "samurai",  "sand",  "saproling",  "satyr",  "scarecrow",  "scientist",  "scion",  "scorpion",  "scout",  "sculpture",  "serf",  "serpent",  "servo",  "shade",  "shaman",  "shapeshifter",  "shark",  "sheep",  "siren",  "skeleton",  "slith",  "sliver",  "sloth",  "slug",  "snail",  "snake",  "soldier",  "soltari",  "spawn",  "specter",  "spellshaper",  "sphinx",  "spider",  "spike",  "spirit",  "splinter",  "sponge",  "squid",  "squirrel",  "starfish",  "surrakar",  "survivor",  "synth",  "tentacle",  "tetravite",  "thalakos",  "thopter",  "thrull",  "tiefling",  "time lord",  "treefolk",  "trilobite",  "triskelavite",  "troll",  "turtle",  "tyranid",  "unicorn",  "vampire",  "varmint",  "vedalken",  "viashino",  "volver",  "wall",  "walrus",  "warlock",  "warrior",  "weird",  "werewolf",  "whale",  "wizard",  "wolf",  "wolverine",  "wombat",  "worm",  "wraith",  "wurm",  "yeti",  "zombie",  "zubera"];
pub const COLORS: &[&str] = &["white", "blue", "black", "red", "green", "colorless"];
pub const LAND_TYPES: &[&str] = &["cave", "desert", "forest", "gate", "island", "lair", "locus", "mine", "mountain", "plains", "power-plant", "sphere", "swamp", "tower", "urza’s",];
pub const ARTIFACT_SUBTYPES: &[&str] = &["attraction", "blood", "bobblehead", "clue", "contraption", "equipment", "food", "fortification", "gold", "incubator", "junk", "map", "powerstone", "treasure", "vehicle"];
pub const GAME_ZONES: &[&str] = &["hand", "battlefield", "graveyard", "library"];
pub const GAME_OBJECTS: &[(&str, TokenType)] = &[("player", TokenType::GameObjectPlayer), ("your", TokenType::GameObjectYouPossessive), ("you", TokenType::GameObjectYou), ("when", TokenType::TriggeredAbilityKeywordWhen), ("card", TokenType::GameObjectCard), ("enters the battlefield", TokenType::PermanentActionEnterTheBattlefield), ("life", TokenType::GameObjectLife)];
pub const PARTS_OF_SPEECH: &[(&str, TokenType)] = &[("the", TokenType::DefiniteArticle), ("a", TokenType::IndefiniteArticle), ("for", TokenType::Preposition), ("in", TokenType::Preposition), ("each", TokenType::Quantifier)];
pub const GAME_ACTIONS: &[(&str, TokenType)] = &[("gain", TokenType::GameActionGain), ("lose", TokenType::GameActionLose)];

pub struct Token {
    pub token_type: TokenType,
    pub length: usize,
    pub token_string: String
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    LeftBrace, RightBrace, LeftParen, RightParen, Colon, WhiteSpace, Identifier, Digit, Comma, Period, NewLine, PlusSign, MinusSign, LongDash,

    CardName,
    GameKeyword, 
    // GameKeywords
    GameKeywordActivate, GameKeywordAttach, GameKeywordCast, GameKeywordCounter, GameKeywordDestroy, GameKeywordDiscard, GameKeywordExchange, 
    GameKeywordExile, GameKeywordFight, GameKeywordPlay, GameKeywordRegenerate, GameKeywordReveal, GameKeywordSacrifice, GameKeywordSearch, GameKeywordShuffle,
    GameKeywordTap, GameKeywordUntap, GameKeywordScry, GameKeywordFateseal, GameKeywordClash, GameKeywordPlaneswalk, GameKeywordSetInMotion, GameKeywordAbandon,
    GameKeywordProliferate, GameKeywordTransform, GameKeywordDetain, GameKeywordPopulate, GameKeywordMonstrosity, GameKeywordVote, GameKeywordBolster, GameKeywordManifest,
    GameKeywordSupport, GameKeywordInvestigate, GameKeywordMeld, GameKeywordGoad, GameKeywordExert, GameKeywordExplore, GameKeywordAssemble, GameKeywordSurveil,
    GameKeywordAdapt, GameKeywordAmass, GameKeywordLearn, GameKeywordVentureIntoTheDungeon, GameKeywordConnive, GameKeywordOpenAnAttraction, GameKeywordRollToVisitYourAttractions,
    GameKeywordConvert, GameKeywordIncubate, GameKeywordTheRingTemptsYou, GameKeywordFaceAVillainousChoice, GameKeywordTimeTravel, GameKeywordDiscover, GameKeywordCloak,
    GameKeywordCollectEvidence, GameKeywordSuspect, 

    PermanentActionAttack, PermanentActionEnterTheBattlefield, 

    GameActionGain, GameActionLose,

    //Game Objects
    GameObjectPlayer, GameObjectOpponent, GameObjectYou, GameObjectYouPossessive, GameObjectCard, GameObjectLife,
    TriggeredAbilityKeywordWhen, 

    Preposition, DefiniteArticle, IndefiniteArticle, Quantifier,

    GameZone,
    Color,
    LandType,
    CreatureType,
    CardType,
    CardSupertype,
    ArtifactSubtype,
    AbilityKeyword,
    Unknown
}

pub struct Tokenizer<'a> {
    keyword_matchers: Vec<&'a dyn KeywordMatcher>,
    card_name: String,
    source: String,
    current_index: usize,
}

impl Tokenizer<'_> {
    
    pub fn new(keyword_matchers: Vec<&dyn KeywordMatcher>, card_name: String, source: String) -> Tokenizer<'_> {
        Tokenizer {
            keyword_matchers: keyword_matchers,
            card_name,
            source,
            current_index: 0
        }
    }

    pub fn get_current_token(&mut self) -> Token {
        let remaining_text = &self.source[self.current_index..];
        let token_type: TokenType;
        let length: usize;
        //let card_name_no_epithet = &self.card_name.split(",").collect()[1];
        if remaining_text.starts_with(&self.card_name) {
            token_type = TokenType::CardName;
            length = self.card_name.len();
            return Token {
                token_type,
                length,
                token_string: remaining_text[..length].to_string()
            }
        }

        for matcher in &self.keyword_matchers {
            let matched_keyword = matcher.find_match(remaining_text);
            if let KeywordMatch::Found(keyword_match) = matched_keyword {
                length = keyword_match.0.len();
                token_type = keyword_match.1;
                return Token {
                    token_type,
                    length,
                    token_string: remaining_text[..length].to_string()
                }
            } 
        }

        if remaining_text.starts_with('\n') {
            length = remaining_text.find(is_not_whitespace).unwrap_or(remaining_text.len());
            token_type = TokenType::NewLine;
        }
        else if remaining_text.starts_with(is_whitespace) {
            length = remaining_text.find(is_not_whitespace).unwrap_or(remaining_text.len());
            token_type = TokenType::WhiteSpace;
        } else if remaining_text.starts_with(is_identifier) {
            length = remaining_text.find(is_not_identifier).unwrap_or(remaining_text.len());
            token_type = TokenType::Identifier;
        } else if remaining_text.starts_with(is_digit) {
            length = remaining_text.find(is_not_digit).unwrap_or(remaining_text.len());
            token_type = TokenType::Digit;
        } else {
            let character = remaining_text.chars().next().unwrap();
            length = character.len_utf8();
            token_type =  match character {
                '(' => TokenType::LeftParen,
                ')' => TokenType::RightParen,
                ',' => TokenType::Comma,
                '.' => TokenType::Period,
                '+' => TokenType::PlusSign,
                '-' => TokenType::MinusSign,
                '—' => TokenType::LongDash,
                ':' => TokenType::Colon,
                _ => TokenType::Unknown,
            }
        }
        Token {
            token_type,
            length,
            token_string: remaining_text[..length].to_string()
        }
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.source.len() <= self.current_index {
            return None
        }
        
        let token = self.get_current_token();
        self.current_index += token.length;
        Some(token)
    } 
}

fn is_identifier(character: char) -> bool {
    'A' <= character && character <= 'z' || character == '\''
}

fn is_not_identifier(character: char) -> bool {
    !is_identifier(character)
}

fn is_whitespace(character: char) -> bool {
    character.is_whitespace()
}

fn is_not_whitespace(character: char) -> bool {
    !is_whitespace(character)
}

fn is_digit(character: char) -> bool {
    character.is_ascii_digit()
}

fn is_not_digit(character: char) -> bool {
    !is_digit(character)
}


pub trait KeywordMatcher {
    fn find_match(&self, input_string: &str) -> KeywordMatch;
}

#[derive(PartialEq, Eq)]
pub enum KeywordMatch {
    NotFound,
    Found(KeywordMatchResult)
}

type KeywordMatchResult = (String, TokenType);

pub struct LinearKeywordMatcher<'a> {
    keywords: Vec<&'a str>,
    token_type: TokenType
}

impl LinearKeywordMatcher<'_>  {
    pub fn new(keywords: Vec<&'_ str>, token_type: TokenType) -> LinearKeywordMatcher<'_>  {
        LinearKeywordMatcher {
            keywords,
            token_type
        }
    }
}

impl KeywordMatcher for LinearKeywordMatcher<'_>  {
    fn find_match(&self, input_string: &str) -> KeywordMatch {
        for keyword in &self.keywords {
            if input_string.starts_with(keyword) {
                return KeywordMatch::Found((keyword.to_string(), self.token_type));
            }
        }

        KeywordMatch::NotFound
    }
}

pub struct LinearTupleKeywordMatcher<'a> {
    keywords: Vec<(&'a str, TokenType)>
}

impl LinearTupleKeywordMatcher<'_>  {
    pub fn new(keywords: Vec<(&'_ str, TokenType)>) -> LinearTupleKeywordMatcher<'_>  {
        LinearTupleKeywordMatcher {
            keywords
        }
    }
}

impl KeywordMatcher for LinearTupleKeywordMatcher<'_>  {
    fn find_match(&self, input_string: &str) -> KeywordMatch {
        for keyword in &self.keywords {
            if input_string.starts_with(keyword.0) {
                return KeywordMatch::Found((keyword.0.to_string(), keyword.1));
            }
        }

        KeywordMatch::NotFound
    }
}