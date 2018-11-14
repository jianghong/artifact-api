# artifact-api


## Getting started

````rust
extern crate artifact_api;
use artifact_api::{CardSetApi, BASE_SET_ID};

let card_set_api = CardSetApi::new();
match card_set_api.get_set(BASE_SET_ID) {
	Ok(response) => println!("{:?}", response.card_set),
	Err(e) => println!("{:?}", e)
}
````

## Features

### Caches responses by set

````rust
let mut card_set_api = CardSetApi::new();
card_set_api.get_set(BASE_SET_ID).unwrap(); // Fetches data for BASE_SET_ID from server
card_set_api.get_set(BASE_SET_ID).unwrap(); // Returns cached response for BASE_SET_ID
card_set_api.get_set(CALL_TO_ARMS_SET_ID).unwrap(); // Fetches data for CALL_TO_ARMS_SET_ID from server
card_set_api.get_set(CALL_TO_ARMS_SET_ID).unwrap(); // Returns cached response for CALL_TO_ARMS_SET_ID
card_set_api.get_set(BASE_SET_ID).unwrap(); // BASE_SET_ID response is still cached
````

### Find items by gold cost

````rust
let mut card_set_api = CardSetApi::new();
let search_params = FindItemsParams{
	gold_cost: 5, // gold cost of the item
	include_hold: true, // set this to true to include items that cost 1 less than the gold_cost param
};
let results = card_set_api.get_cards().unwrap().find_items(&search_params);
println!("Number of cards that cost {} or {} gold: {}", search_params.gold_cost, search_params.gold_cost - 1, results.len());

for item in results {
	item.print_item_info();
}
````

### Abstracts away initial request

The nature of the Card Set API involves making an initial request in order to fetch the CDN URL to request the actual card set data. This library abstracts away that detail.


## API

````rust
CardSetApi {
	fn new() -> CardSetApi,
	fn get_cards(&mut self) -> Result<CardList, CardSetRequestError>,
	fn get_set(&mut self, set_id: &str) -> Result<CardSetResponse, CardSetRequestError>,
}

struct FindItemsParams {
	gold_cost: i32,
	include_hold: bool,
}

CardList {
	fn find_items(self, params: &FindItemsParams) -> CardList
}
````