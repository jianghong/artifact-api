extern crate artifact_api_rs;


fn main() {
	match artifact_api_rs::get_card_set_request(artifact_api_rs::BASE_SET_ID) {
		Ok(response) => println!("{:?}", response),
		Err(e) => println!("{:?}", e)
	}
}