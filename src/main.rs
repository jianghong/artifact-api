extern crate artifact_api;


fn main() {
	match artifact_api::get_card_set_request(artifact_api::BASE_SET_ID) {
		Ok(response) => println!("{:?}", response),
		Err(e) => println!("{:?}", e)
	}
}