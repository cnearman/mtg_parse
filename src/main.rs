use std::{str};

fn main() {
    let keyword_ability_matcher = LinearKeywordMatcher::new(vec!["deathtouch",  "defender",  "double strike",  "enchant",  "equip",  "first strike",  "flash",  "flying",  "haste",  "hexproof",  "indestructible",  "lifelink",  "menace",  "prowess",  "reach",  "trample",  "vigilance",  "absorb",  "affinity",  "amplify",  "annihilator",  "aura swap",  "awaken",  "banding",  "battle cry",  "bestow",  "bloodthirst",  "bushido",  "buyback",  "cascade",  "champion",  "changeling",  "cipher",  "conspire",  "convoke",  "cumulative upkeep",  "cycling",  "dash",  "delve",  "dethrone",  "devoid",  "devour",  "dredge",  "echo",  "entwine",  "epic",  "evoke",  "evolve",  "exalted",  "exploit",  "extort",  "fading",  "fear",  "flanking",  "flashback",  "forecast",  "fortify",  "frenzy",  "fuse",  "graft",  "gravestorm",  "haunt",  "hidden agenda",  "hideaway",  "horsemanship",  "infect",  "ingest",  "intimidate",  "kicker",  "landhome",  "landwalk",  "level up",  "living weapon",  "madness",  "megamorph",  "miracle",  "modular",  "morph",  "myriad",  "ninjutsu",  "offering",  "outlast",  "overload",  "persist",  "phasing",  "poisonous",  "protection",  "provoke",  "prowl",  "rampage",  "rebound",  "recover",  "reinforce",  "renown",  "replicate",  "retrace",  "ripple",  "scavenge",  "skulk",  "shadow",  "shroud",  "soulbond",  "soulshift",  "splice",  "split second",  "storm",  "substance",  "sunburst",  "surge",  "suspend",  "totem armor",  "transfigure",  "transmute",  "tribute",  "undying",  "unearth",  "unleash",  "vanishing",  "wither"]);
    //let args: Vec<String> = env::args().collect();
    let source = String::from("First strike (This creature deals combat damage before creatures without first strike.)\nWhen Ancestor's Chosen enters the battlefield, you gain 1 life for each card in your graveyard.").to_lowercase();
    let tokenizer = Tokenizer::new(vec![&keyword_ability_matcher], String::from("ancestor's chosen"), source);
    for token in tokenizer {
        println!("{:?} - {}", token.token_type, token.token_string);
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub length: usize,
    pub token_string: String
}

#[derive(Debug)]
pub enum TokenType {
    LeftBrace, RightBrace, LeftParen, RightParen, Colon, WhiteSpace, Identifier, Digit, Comma, Period, NewLine, PlusSign, MinusSign,

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


    AbilityKeyword,

    /*AbilityKeywordDeathtouch, AbilityKeywordDefender, AbilityKeywordDoubleStrike, AbilityKeywordEnchant, AbilityKeywordEquip, AbilityKeywordFirstStrike,
    AbilityKeywordFlash, AbilityKeywordFlying, AbilityKeywordHaste, AbilityKeywordHexproof, AbilityKeywordIndestructible, AbilityKeywordIntimidate, 
    //AbilityKeywordLandwalk
    AbilityKeywordLifelink, 
    //AbilityKeywordProtection,
    AbilityKeywordReach, AbilityKeywordShroud, AbilityKeywordTrample, AbilityKeywordVigilance,
    //AbilityKeywordBanding,
    AbilityKeywordRampage, AbilityKeywordCumulativeUpkeep, AbilityKeywordFlanking, AbilityKeywordPhasing, AbilityKeywordBuyBack, AbilityKeywordShadow, 
    AbilityKeywordCycling, AbilityKeywordEcho, AbilityKeywordHorsemanship, AbilityKeywordFading, AbilityKeywordKicker, AbilityKeywordFlashback, 
    AbilityKeywordMadness, AbilityKeywordFear, AbilityKeywordMorph, AbilityKeywordAmplify, AbilityKeywordProvoke, AbilityKeywordStorm, AbilityKeywordAffinity,
    AbilityKeywordEntwine, AbilityKeywordModular, AbilityKeywordSunburst, AbilityKeywordBushido, AbilityKeywordSoulshift, 
    //AbilityKeywordSplice,
    AbilityKeywordOffering, AbilityKeywordNinjutsu, AbilityKeywordEpic, AbilityKeywordConvoke, AbilityKeywordDredge, AbilityKeywordTransmute,
    AbilityKeywordBloodthirst, AbilityKeywordHaunt, AbilityKeywordForecast, AbilityKeywordReplicate, AbilityKeywordGraft, AbilityKeywordRecover, AbilityKeywordRipple,
    AbilityKeywordSplitSecond, AbilityKeywordSuspend, AbilityKeywordVanishing, AbilityKeywordAbsorb, AbilityKeywordAuraSwap, AbilityKeywordDelve, AbilityKeywordFortify,
    AbilityKeywordFrenzy, AbilityKeywordGravestorm, AbilityKeywordPoisonous, AbilityKeywordTransfigure, AbilityKeywordChampion, AbilityKeywordChangeling,
    AbilityKeywordEvoke, AbilityKeywordHideaway, AbilityKeywordProwl, AbilityKeywordReinforce, AbilityKeywordConspire, AbilityKeywordPersist, AbilityKeywordWither,
    AbilityKeywordRetrace, AbilityKeywordDevour, AbilityKeywordExalted, AbilityKeywordUnearth, AbilityKeywordCascade, AbilityKeywordAnnihilator, AbilityKeywordLevelUp,
    AbilityKeywordRebound, AbilityKeywordTotemArmor, AbilityKeywordInfect, AbilityKeywordBattleCry, AbilityKeywordLivingWeapon, AbilityKeywordUndying, AbilityKeywordMiracle,
    AbilityKeywordSoulbond, AbilityKeywordOverload, AbilityKeywordScavenge, AbilityKeywordUnleash, AbilityKeywordCipher, AbilityKeywordEvolve, AbilityKeywordExtort,
    AbilityKeywordFuse, AbilityKeywordBestow, AbilityKeywordTribute, AbilityKeywordDethrone, AbilityKeywordHiddenAgenda, AbilityKeywordOutlast, AbilityKeywordProwess,
    AbilityKeywordDash, AbilityKeywordExploit, AbilityKeywordMenace, AbilityKeywordRenown, AbilityKeywordAwaken, AbilityKeywordDevoid, AbilityKeywordIngest, AbilityKeywordMyriad,
    AbilityKeywordSurge, AbilityKeywordSulk, AbilityKeywordEmerge, AbilityKeywordEscalate, AbilityKeywordMelee, AbilityKeywordCrew, AbilityKeywordFabricate, 
    //AbilityKeywordPartner, 
    AbilityKeywordUndaunted, AbilityKeywordImprovise, AbilityKeywordAftermath, AbilityKeywordEmbalm, AbilityKeywordEternalize, AbilityKeywordAfflict, AbilityKeywordAscend,
    AbilityKeywordAssist, AbilityKeywordJumpStart, AbilityKeywordMentor, AbilityKeywordAfterlife, AbilityKeywordRiot, AbilityKeywordSpectacle, AbilityKeywordEscape,
    AbilityKeywordCompanion, AbilityKeywordMutate, AbilityKeywordEncore, AbilityKeywordBoast, AbilityKeywordForetell, AbilityKeywordDemonstrate, 
    //AbilityKeywordDaybound
    AbilityKeywordDisturb, AbilityKeywordDecayed, AbilityKeywordCleave, AbilityKeywordTraining, AbilityKeywordCompleated, AbilityKeywordReconfigure, AbilityKeywordBlitz,
    AbilityKeywordCasualty, AbilityKeywordEnlist, AbilityKeywordReadAhead, AbilityKeywordRavenous, AbilityKeywordSquad, 
    //AbilityKeywordSpaceSculptor,
    AbilityKeywordVisit, AbilityKeywordPrototype, AbilityKeywordLivingMetal, AbilityKeywordMoreThanMeetsTheEye, AbilityKeywordForMirrodin, AbilityKeywordToxic, 
    AbilityKeywordBackup, AbilityKeywordBargain, AbilityKeywordCraft, AbilityKeywordDisguise, AbilityKeywordSolved, AbilityKeywordPlot, AbilityKeywordSaddle, 
    AbilityKeywordSpree,*/

    //Game Objects
    GameObjectPlayer, GameObjectOpponent, GameObjectYou, 

    ColorWhite, ColorBlue, ColorBlack, ColorRed, ColorGreen, ColorColorless,

    //Land Types
    LandTypeCave, LandTypeDesert, LandTypeForest, LandTypeGate, LandTypeIsland, LandTypeLair, LandTypeLocus, LandTypeMine, LandTypeMountain, LandTypePlains,
    LandTypePowerPlant, LandTypeSphere, LandTypeSwamp, LandTypeTower, LandTypeUrzas,

    //CreatureTypes
    CreatureTypeAdvisor, CreatureTypeAetherborn, CreatureTypeAlien, CreatureTypeAlly, CreatureTypeAngel, CreatureTypeAntelope, CreatureTypeApe, CreatureTypeArcher,
    CreatureTypeArchon, CreatureTypeArmadillo, CreatureTypeArmy, CreatureTypeArtificer, CreatureTypeAssassin, CreatureTypeAssemblyWorker, CreatureTypeAstartes,
    CreatureTypeAtog, CreatureTypeAurochs, CreatureTypeAvatar, CreatureTypeAzra, CreatureTypeBadger, CreatureTypeBalloon, CreatureTypeBarbarian, CreatureTypeBard,
    CreatureTypeBasilisk, CreatureTypeBat, CreatureTypeBear, CreatureTypeBeast, CreatureTypeBeaver, CreatureTypeBeeble, CreatureTypeBeholder, CreatureTypeBerserker,
    CreatureTypeBird, CreatureTypeBlinkmoth, CreatureTypeBoar, CreatureTypeBringer, CreatureTypeBrushwagg, CreatureTypeCamarid, CreatureTypeCamel,
    CreatureTypeCapybara, CreatureTypeCaribou, CreatureTypeCarrier, CreatureTypeCat, CreatureTypeCentaur, CreatureTypeCephalid, CreatureTypeChild, CreatureTypeChimera,
    CreatureTypeCitizen, CreatureTypeCleric, CreatureTypeClown, CreatureTypeCockatrice, CreatureTypeConstruct, CreatureTypeCoward, CreatureTypeCoyote,
    CreatureTypeCrab, CreatureTypeCrocodile, CreatureTypeCtan, CreatureTypeCustodes, CreatureTypeCyberman, CreatureTypeCyclops, CreatureTypeDalek,
    CreatureTypeDauthi, CreatureTypeDemigod, CreatureTypeDemon, CreatureTypeDeserter, CreatureTypeDetective, CreatureTypeDevil, CreatureTypeDinosaur, CreatureTypeDjinn,
    CreatureTypeDoctor, CreatureTypeDog, CreatureTypeDragon, CreatureTypeDrake, CreatureTypeDreadnought, CreatureTypeDrone, CreatureTypeDruid, CreatureTypeDryad,
    CreatureTypeDwarf, CreatureTypeEfreet, CreatureTypeEgg, CreatureTypeElder, CreatureTypeEldrazi, CreatureTypeElemental, CreatureTypeElephant, CreatureTypeElf,
    CreatureTypeElk, CreatureTypeEmployee, CreatureTypeEye, CreatureTypeFaerie, CreatureTypeFerret, CreatureTypeFish, CreatureTypeFlagbearer, CreatureTypeFox,
    CreatureTypeFractal, CreatureTypeFrog, CreatureTypeFungus, CreatureTypeGamer, CreatureTypeGargoyle, CreatureTypeGerm, CreatureTypeGiant, CreatureTypeGith,
    CreatureTypeGnoll, CreatureTypeGnome, CreatureTypeGoat, CreatureTypeGoblin, CreatureTypeGod, CreatureTypeGolem, CreatureTypeGorgon, CreatureTypeGraveborn,
    CreatureTypeGremlin, CreatureTypeGriffin, CreatureTypeGuest, CreatureTypeHag, CreatureTypeHalfling, CreatureTypeHamster, CreatureTypeHarpy,
    CreatureTypeHellion, CreatureTypeHippo, CreatureTypeHippogriff, CreatureTypeHomarid, CreatureTypeHomunculus, CreatureTypeHorror, CreatureTypeHorse,
    CreatureTypeHuman, CreatureTypeHydra, CreatureTypeHyena, CreatureTypeIllusion, CreatureTypeImp, CreatureTypeIncarnation, CreatureTypeInkling, 
    CreatureTypeInquisitor, CreatureTypeInsect, CreatureTypeJackal, CreatureTypeJellyfish, CreatureTypeJuggernaut, CreatureTypeKavu, CreatureTypeKirin, 
    CreatureTypeKithkin, CreatureTypeKnight, CreatureTypeKobold, CreatureTypeKor, CreatureTypeKraken, CreatureTypeLlama, CreatureTypeLamia, CreatureTypeLammasu, 
    CreatureTypeLeech, CreatureTypeLeviathan, CreatureTypeLhurgoyf, CreatureTypeLicid, CreatureTypeLizard, CreatureTypeManticore, CreatureTypeMasticore, 
    CreatureTypeMercenary, CreatureTypeMerfolk, CreatureTypeMetathran, CreatureTypeMinion, CreatureTypeMinotaur, CreatureTypeMite, CreatureTypeMole, 
    CreatureTypeMonger, CreatureTypeMongoose, CreatureTypeMonk, CreatureTypeMonkey, CreatureTypeMoonfolk, CreatureTypeMount, CreatureTypeMouse, CreatureTypeMutant, 
    CreatureTypeMyr, CreatureTypeMystic, CreatureTypeNaga, CreatureTypeNautilus, CreatureTypeNecron, CreatureTypeNephilim, CreatureTypeNightmare, 
    CreatureTypeNightstalker, CreatureTypeNinja, CreatureTypeNoble, CreatureTypeNoggle, CreatureTypeNomad, CreatureTypeNymph, CreatureTypeOctopus, CreatureTypeOgre, 
    CreatureTypeOoze, CreatureTypeOrb, CreatureTypeOrc, CreatureTypeOrgg, CreatureTypeOtter, CreatureTypeOuphe, CreatureTypeOx, CreatureTypeOyster, 
    CreatureTypePangolin, CreatureTypePeasant, CreatureTypePegasus, CreatureTypePentavite, CreatureTypePerformer, CreatureTypePest, CreatureTypePhelddagrif, 
    CreatureTypePhoenix, CreatureTypePhyrexian, CreatureTypePilot, CreatureTypePincher, CreatureTypePirate, CreatureTypePlant, CreatureTypePorcupine, 
    CreatureTypePossum, CreatureTypePraetor, CreatureTypePrimarch, CreatureTypePrism, CreatureTypeProcessor, CreatureTypeRabbit, CreatureTypeRaccoon, 
    CreatureTypeRanger, CreatureTypeRat, CreatureTypeRebel, CreatureTypeReflection, CreatureTypeRhino, CreatureTypeRigger, CreatureTypeRobot, CreatureTypeRogue, 
    CreatureTypeSable, CreatureTypeSalamander, CreatureTypeSamurai, CreatureTypeSand, CreatureTypeSaproling, CreatureTypeSatyr, CreatureTypeScarecrow, 
    CreatureTypeScientist, CreatureTypeScion, CreatureTypeScorpion, CreatureTypeScout, CreatureTypeSculpture, CreatureTypeSerf, CreatureTypeSerpent, 
    CreatureTypeServo, CreatureTypeShade, CreatureTypeShaman, CreatureTypeShapeshifter, CreatureTypeShark, CreatureTypeSheep, CreatureTypeSiren, CreatureTypeSkeleton, 
    CreatureTypeSlith, CreatureTypeSliver, CreatureTypeSloth, CreatureTypeSlug, CreatureTypeSnail, CreatureTypeSnake, CreatureTypeSoldier, CreatureTypeSoltari, 
    CreatureTypeSpawn, CreatureTypeSpecter, CreatureTypeSpellshaper, CreatureTypeSphinx, CreatureTypeSpider, CreatureTypeSpike, CreatureTypeSpirit, 
    CreatureTypeSplinter, CreatureTypeSponge, CreatureTypeSquid, CreatureTypeSquirrel, CreatureTypeStarfish, CreatureTypeSurrakar, CreatureTypeSurvivor, 
    CreatureTypeSynth, CreatureTypeTentacle, CreatureTypeTetravite, CreatureTypeThalakos, CreatureTypeThopter, CreatureTypeThrull, CreatureTypeTiefling, 
    CreatureTypeTimeLord, CreatureTypeTreefolk, CreatureTypeTrilobite, CreatureTypeTriskelavite, CreatureTypeTroll, CreatureTypeTurtle, CreatureTypeTyranid, 
    CreatureTypeUnicorn, CreatureTypeVampire, CreatureTypeVarmint, CreatureTypeVedalken, CreatureTypeViashino, CreatureTypeVolver, CreatureTypeWall, 
    CreatureTypeWalrus, CreatureTypeWarlock, CreatureTypeWarrior, CreatureTypeWeird, CreatureTypeWerewolf, CreatureTypeWhale, CreatureTypeWizard, CreatureTypeWolf, 
    CreatureTypeWolverine, CreatureTypeWombat, CreatureTypeWorm, CreatureTypeWraith, CreatureTypeWurm, CreatureTypeYeti, CreatureTypeZombie, CreatureTypeZubera,

    Unknown
}

pub struct Tokenizer<'a> {
    keyword_matchers: Vec<&'a dyn KeywordMatcher>,
    card_name: String,
    source: String,
    current_index: usize,
}

impl Tokenizer<'_> {
    
    fn new(keyword_matchers: Vec<&dyn KeywordMatcher>, card_name: String, source: String) -> Tokenizer<'_> {
        Tokenizer {
            keyword_matchers: keyword_matchers,
            card_name,
            source,
            current_index: 0
        }
    }

    fn get_current_token(&mut self) -> Token {
        let remaining_text = &self.source[self.current_index..];
        let token_type: TokenType;
        let length: usize;
        for matcher in &self.keyword_matchers {
            let matched_keyword = matcher.find_match(remaining_text);
            if let KeywordMatch::Found(keyword) = matched_keyword {
                length = keyword.len();
                token_type = TokenType::AbilityKeyword;
                return Token {
                    token_type,
                    length,
                    token_string: remaining_text[..length].to_string()
                }
            } 
        }
        if remaining_text.starts_with(&self.card_name) {
            token_type = TokenType::CardName;
            length = self.card_name.len();
        } else if remaining_text.starts_with('\n') {
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


trait KeywordMatcher {
    fn find_match(&self, input_string: &str) -> KeywordMatch;
}

#[derive(PartialEq, Eq)]
enum KeywordMatch {
    NotFound,
    Found(String)
}

pub struct LinearKeywordMatcher<'a> {
    keywords: Vec<&'a str>
}

impl LinearKeywordMatcher<'_>  {
    fn new(keywords: Vec<&'_ str>) -> LinearKeywordMatcher<'_>  {
        LinearKeywordMatcher {
            keywords
        }
    }
}

impl KeywordMatcher for LinearKeywordMatcher<'_>  {
    fn find_match(&self, input_string: &str) -> KeywordMatch {
        for keyword in &self.keywords {
            if input_string.starts_with(keyword) {
                return KeywordMatch::Found(keyword.to_string());
            }
        }

        KeywordMatch::NotFound
    }
}