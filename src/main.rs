extern crate artifact_api;
use artifact_api::{CardSetApi, BASE_SET_ID, CALL_TO_ARMS_SET_ID};


fn main() {
	let mut card_set_api = CardSetApi::new();
	card_set_api.get_set(BASE_SET_ID).unwrap();
	card_set_api.get_set(BASE_SET_ID).unwrap();
	card_set_api.get_set(CALL_TO_ARMS_SET_ID).unwrap();
	card_set_api.get_set(CALL_TO_ARMS_SET_ID).unwrap();
	card_set_api.get_set(BASE_SET_ID).unwrap();
	card_set_api.get_set(CALL_TO_ARMS_SET_ID).unwrap();
}