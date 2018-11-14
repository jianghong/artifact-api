extern crate artifact_api;
use artifact_api::{CardSetApi, BASE_SET_ID, CALL_TO_ARMS_SET_ID, CardListFilterable};

fn main() {
	let mut card_set_api = CardSetApi::new();
	let search_for = 5;
	let results = card_set_api.get_cards().unwrap().find_items_by_gold_cost(search_for, true);
	println!("Number of cards that cost {} or {} gold: {}", search_for, search_for - 1, results.len());
	for item in results {
		item.print_item_info();
	}	
}