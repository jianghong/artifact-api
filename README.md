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