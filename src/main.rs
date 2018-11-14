extern crate artifact_api;
use artifact_api::{CardSetApi, BASE_SET_ID, CALL_TO_ARMS_SET_ID, CardListFilterable, FindItemsParams};

fn main() {
	let mut card_set_api = CardSetApi::new();
	let search_params = FindItemsParams{
		gold_cost: 5,
		include_hold: true,
	};
	let results = card_set_api.get_cards().unwrap().find_items(&search_params);
	println!("Number of cards that cost {} or {} gold: {}", search_params.gold_cost, search_params.gold_cost - 1, results.len());
	for item in results {
		item.print_item_info();
	}	
}