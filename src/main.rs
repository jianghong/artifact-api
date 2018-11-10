extern crate artifact_api;
use artifact_api::{CardSetApi, BASE_SET_ID};


fn main() {
	let card_set_api = CardSetApi::new();
	match card_set_api.get_set(BASE_SET_ID) {
		Ok(response) => println!("{:?}", response),
		Err(e) => println!("{:?}", e)
	}
}