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


### Abstracts away initial request

The nature of the Card Set API involves making an initial request in order to fetch the CDN URL to request the actual card set data. This library abstracts away that detail.