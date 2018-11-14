extern crate artifact_api;
use artifact_api::{CardSetApi, BASE_SET_ID, CALL_TO_ARMS_SET_ID, CardListFilterable};

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
	let three_cost_items = card_set_api.get_cards().unwrap().find_items_by_gold_cost(3);
	println!("Number of cards that cost 3 gold: {}", three_cost_items.len());
	for item in three_cost_items {
		item.print_item_info();
	}
}