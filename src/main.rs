pub mod tokenizer;

use tokenizer::{LinearKeywordMatcher, LinearTupleKeywordMatcher, TokenType, Tokenizer, ABILITY_KEYWORDS, ARTIFACT_SUBTYPES, CARD_SUPERTYPES, CARD_TYPES, COLORS, CREATURE_TYPES, GAME_ACTIONS, GAME_OBJECTS, GAME_ZONES, LAND_TYPES, PARTS_OF_SPEECH};

fn main() {
    //let args: Vec<String> = env::args().collect();
    let keyword_ability_matcher = LinearKeywordMatcher::new(ABILITY_KEYWORDS.to_vec(), TokenType::AbilityKeyword);
    let creature_type_matcher = LinearKeywordMatcher::new(CREATURE_TYPES.to_vec(), TokenType::CreatureType);
    let color_matcher = LinearKeywordMatcher::new(COLORS.to_vec(), TokenType::Color);
    let card_type_matcher = LinearKeywordMatcher::new(CARD_TYPES.to_vec(), TokenType::CardType);
    let card_supertype_matcher = LinearKeywordMatcher::new(CARD_SUPERTYPES.to_vec(), TokenType::CardSupertype);
    let land_type_matcher = LinearKeywordMatcher::new(LAND_TYPES.to_vec(), TokenType::LandType);
    let artifact_subtype_matcher = LinearKeywordMatcher::new(ARTIFACT_SUBTYPES.to_vec(), TokenType::ArtifactSubtype);
    let game_zone_matcher = LinearKeywordMatcher::new(GAME_ZONES.to_vec(), TokenType::GameZone);
    let game_object_matcher = LinearTupleKeywordMatcher::new(GAME_OBJECTS.to_vec());
    let game_action_matcher = LinearTupleKeywordMatcher::new(GAME_ACTIONS.to_vec());
    let parts_of_speech_matcher = LinearTupleKeywordMatcher::new(PARTS_OF_SPEECH.to_vec());
    let source = String::from("First strike (This creature deals combat damage before creatures without first strike.)\nWhen Ancestor's Chosen enters the battlefield, you gain 1 life for each card in your graveyard.").to_lowercase();
    let tokenizer = Tokenizer::new(
        vec![
        &game_object_matcher,
        &game_zone_matcher,
        &keyword_ability_matcher,
        &color_matcher,
        &card_type_matcher,
        &card_supertype_matcher,
        &land_type_matcher,
        &artifact_subtype_matcher,
        &creature_type_matcher,
        &game_action_matcher,
        &parts_of_speech_matcher
        ],   
        String::from("Ancestor's Chosen").to_lowercase(), 
        source
    );
    for token in tokenizer {
        if token.token_type != TokenType::WhiteSpace {
            println!("{:?} - {}", token.token_type, token.token_string);
        }
    }
}