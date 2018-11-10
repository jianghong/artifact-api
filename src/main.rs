extern crate artifact_api;
use artifact_api::{CardSetApi, BASE_SET_ID, CALL_TO_ARMS_SET_ID};


fn main() {
	let card_set_api = CardSetApi::new();
	let card_set = card_set_api.get_set(CALL_TO_ARMS_SET_ID).unwrap();
	println!("{:?}", card_set);
}