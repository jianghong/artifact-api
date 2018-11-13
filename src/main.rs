extern crate artifact_api;
use artifact_api::{CardSetApi, BASE_SET_ID, CALL_TO_ARMS_SET_ID};

fn main() {
	// let mut card_set_api = CardSetApi::new();
	// card_set_api.get_set(BASE_SET_ID).unwrap();
	// card_set_api.get_set(BASE_SET_ID).unwrap();
	// card_set_api.get_set(CALL_TO_ARMS_SET_ID).unwrap();
	// card_set_api.get_set(CALL_TO_ARMS_SET_ID).unwrap();
	// let base_set = card_set_api.get_set(BASE_SET_ID).unwrap();
	// let call_to_arms_set = card_set_api.get_set(CALL_TO_ARMS_SET_ID).unwrap();
	// println!("Size of base set: {}", base_set.card_set.card_list.len());
	// println!("Size of call to arms set: {}", call_to_arms_set.card_set.card_list.len());

	let mut card_set_api = CardSetApi::new();
	let card_list = card_set_api.get_sets().unwrap();
	println!("Size of entire set: {}", card_list.len());
}