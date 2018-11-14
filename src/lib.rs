use std::collections::HashMap;

extern crate reqwest;
extern crate mockito;
#[macro_use] extern crate serde_derive;

#[cfg(test)]
const CARD_SET_REQUEST_URL: &'static str = mockito::SERVER_URL;

#[cfg(not(test))]
const CARD_SET_REQUEST_URL: &str =  "https://playartifact.com/cardset/";

pub const BASE_SET_ID: &str = "00";
pub const CALL_TO_ARMS_SET_ID: &str = "01";
pub const SET_IDS: &'static [&'static str] = &[BASE_SET_ID, CALL_TO_ARMS_SET_ID];

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardSetRequest {
	cdn_root: String,
	url: String,
	expire_time: i32,
}

impl CardSetRequest {
	pub fn url(self) -> reqwest::Url {
		reqwest::Url::parse(&self.cdn_root).unwrap().join(&self.url).unwrap()
	}
}

impl PartialEq for CardSetRequest {
    fn eq(&self, other: &CardSetRequest) -> bool {
        self.cdn_root == other.cdn_root &&
        self.url == other.url &&
        self.expire_time == other.expire_time
    }	
}

#[derive(Debug)]
pub enum CardSetRequestError {
	InvalidSetID { kind: reqwest::UrlError },
	ReqwestError { kind: reqwest::Error },
}

impl CardSetRequestError {
	#[allow(dead_code)]
	fn to_string(self) -> String {
		match self {
			CardSetRequestError::InvalidSetID{kind} => kind.to_string(),
			CardSetRequestError::ReqwestError{kind} => kind.to_string(),
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranslationSet {
	english: Option<String>,
}

impl TranslationSet {
	fn english_val(self) -> String {
		if let Some(english) = self.english {
			english
		} else {
			"".to_string()
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageSet {
	default: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetInfo {
	set_id: i32,
	pack_item_def: i32,
	name: TranslationSet,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardReference {
	card_id: i32,
	ref_type: String,
	count: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Rarity {
	Common,
	Uncommon,
	Rare,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
	card_id: i32,
	base_card_id: i32,
	card_type: String,
	card_name: TranslationSet,
	card_text: TranslationSet,
	mini_image: ImageSet,
	large_image: ImageSet,
	ingame_image: ImageSet,
	references: Vec<CardReference>,
	attack: Option<i32>,
	hit_points: Option<i32>,
	illustrator: Option<String>,
	gold_cost: Option<i32>,
	mana_cost: Option<i32>,
	sub_type: Option<String>,
	is_green: Option<bool>,
	is_red: Option<bool>,
	is_black: Option<bool>,
	is_blue: Option<bool>,
	item_def: Option<i32>,
	rarity: Option<Rarity>,
}

impl Card {
	pub fn print_item_info(self) {
		if let Some(gold) = self.gold_cost {
			println!("Name: {} / Gold: {}", self.card_name.english_val(), gold);
		}
	}
}

type CardList = Vec<Card>;

pub struct FindItemsParams {
	pub gold_cost: i32,
	pub include_hold: bool,
}

pub trait CardListFilterable {
	fn find_items(self, &FindItemsParams) -> CardList;
}

impl CardListFilterable for CardList {
	fn find_items(self, params: &FindItemsParams) -> CardList {
		self.into_iter()
			.filter(|card| {
				if let Some(card_gold_cost) = card.gold_cost {
					card.card_type == "Item" &&
					card_gold_cost == params.gold_cost ||
					(params.include_hold && card_gold_cost == params.gold_cost -1)
				} else {
					return false
				}
			})
			.collect::<CardList>()
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardSet {
	version: i32,
	set_info: SetInfo,
	pub card_list: CardList,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardSetResponse {
	pub card_set: CardSet,
}

#[derive(Debug)]
pub struct CardSetApi {
	cached_sets: HashMap<String, CardSetResponse>,
	client: reqwest::Client,
}

impl CardSetApi {
	pub fn new() -> Self {
		CardSetApi {
			cached_sets: HashMap::new(),
			client: reqwest::Client::new(),
		}
	}

	pub fn get_set(&mut self, set_id: &str) -> Result<CardSetResponse, CardSetRequestError> {
		if let Some(cached_set) = self.cached_sets.get(set_id.into()) {
			println!("Found cached set response for set {}", cached_set.card_set.set_info.set_id);
			return Ok(cached_set.clone());
		}

		println!("Fetching set_id {} from server...", set_id);
		self.get_set_request(set_id)
			.and_then(|card_set_request| reqwest::get(card_set_request.url()).map_err(|e| CardSetRequestError::ReqwestError{kind: e}))
			.and_then(|mut response| response.json().map_err(|e| CardSetRequestError::ReqwestError{kind: e}))
			.and_then(|card_set_response: CardSetResponse| {
				self.cached_sets.insert(
					set_id.into(),
					card_set_response.clone()
				);
				Ok(card_set_response)
			})
	}

	pub fn get_cards(&mut self) -> Result<CardList, CardSetRequestError> {
		let mut card_list: CardList = vec![];

		for set_id in SET_IDS {
			match self.get_set(set_id) {
				Ok(card_set_response) => card_list.append(&mut card_set_response.card_set.card_list.clone()),
				Err(e) => return Err(e)
			}
		}
		Ok(card_list)
	}

	fn get_set_request(&mut self, set_id: &str) -> Result<CardSetRequest, CardSetRequestError> {
		self.parse_url(set_id)
			.and_then(|url| reqwest::get(url).map_err(|e| CardSetRequestError::ReqwestError{kind: e}))
			.and_then(|mut response| response.json().map_err(|e| CardSetRequestError::ReqwestError{kind: e}))
	}

	fn parse_url(&mut self, set_id: &str) -> Result<reqwest::Url, CardSetRequestError> {
		reqwest::Url::parse(CARD_SET_REQUEST_URL).map_err(|e| CardSetRequestError::InvalidSetID{kind: e})
			.and_then(|base_url| base_url.join(set_id).map_err(|e| CardSetRequestError::InvalidSetID{kind: e}))
	}
}

#[cfg(test)]
mod tests {
	extern crate mockito;
	extern crate serde_json;
	use FindItemsParams;
	use Card;
	use ImageSet;
	use CardSetResponse;
	use CardSet;
	use SetInfo;
	use TranslationSet;
	use {BASE_SET_ID, CALL_TO_ARMS_SET_ID, CardSetRequest, CardSetApi, CardList, CardListFilterable};

	use tests::mockito::mock;


	#[test]
	fn card_set_api_get_set_request_success() {
		let expected_body = CardSetRequest{
			cdn_root: "cdn/root/path".into(),
			url: "path/to/card/set".into(),
			expire_time: 54321,
		};
	    let _m = mock("GET", "/01")
	      .with_status(201)
	      .with_header("content-type", "text/plain")
	      .with_header("x-api-key", "1234")
	      .with_body(serde_json::to_string(&expected_body).unwrap())
	      .create();
	    let card_set_request = CardSetApi::new().get_set_request("01").unwrap();
		assert_eq!(card_set_request, expected_body);
	}

	#[test]
	fn card_set_api_get_set_request_fail() {
	    let _m = mock("GET", "/01")
	      .with_status(201)
	      .with_header("content-type", "text/plain")
	      .with_header("x-api-key", "1234")
	      .with_body("{}")
	      .create();
	    let err = CardSetApi::new().get_set_request("01").unwrap_err();
		assert_eq!(err.to_string(), "missing field `cdn_root` at line 1 column 2");
	}

    #[test]
    fn card_set_api_parse_url_parses_url() {
    	let url = CardSetApi::new().parse_url("00").unwrap();
        assert_eq!(url.as_str(), "http://127.0.0.1:1234/00");
    }

    #[test]
    fn card_set_api_parse_url_returns_proper_err() {
    	let err = CardSetApi::new().parse_url("//").unwrap_err();
        assert_eq!(err.to_string(), "empty host");
    }  	

	#[test]
	fn card_set_request_url() {
		let card_set_request = CardSetRequest{
			cdn_root: "https://cdnroot.com".into(),
			url: "/path/to/set".into(),
			expire_time: 123,
		};

		assert_eq!(card_set_request.url().as_str(), "https://cdnroot.com/path/to/set");
	}

	#[test]
	fn find_items() {
		let base_set_cards = CardSetResponse{
			card_set: CardSet{
				version: 1,
				set_info: SetInfo{
					set_id: 1,
					pack_item_def: 1,
					name: TranslationSet{
						english: Some("Base".into())
					}					
				},
				card_list: vec![
					Card{
						card_id: 1,
						base_card_id: 1,
						card_type: "Item".to_string(),
						card_name: TranslationSet{ english: Some("Short Sword".to_string()) },
						card_text: TranslationSet{ english: Some("+3 attack".to_string()) },
						mini_image: ImageSet{ default: None },
						large_image: ImageSet{ default: None },
						ingame_image: ImageSet{ default: None },
						references: vec![],
						attack: None,
						hit_points: None,
						illustrator: None,
						gold_cost: Some(3),
						mana_cost: None,
						sub_type: None,
						is_green: None,
						is_red: None,
						is_black: None,
						is_blue: None,
						item_def: None,
						rarity: None,
					},
					Card{
						card_id: 1,
						base_card_id: 1,
						card_type: "Item".to_string(),
						card_name: TranslationSet{ english: Some("Long Sword".to_string()) },
						card_text: TranslationSet{ english: Some("+4 attack".to_string()) },
						mini_image: ImageSet{ default: None },
						large_image: ImageSet{ default: None },
						ingame_image: ImageSet{ default: None },
						references: vec![],
						attack: None,
						hit_points: None,
						illustrator: None,
						gold_cost: Some(4),
						mana_cost: None,
						sub_type: None,
						is_green: None,
						is_red: None,
						is_black: None,
						is_blue: None,
						item_def: None,
						rarity: None,
					}					
				]
			}
		};
		let call_to_arms_cards = CardSetResponse{
			card_set: CardSet{
				version: 1,
				set_info: SetInfo{
					set_id: 1,
					pack_item_def: 1,
					name: TranslationSet{
						english: Some("Call to arms".into())
					}					
				},
				card_list: vec![
					Card{
						card_id: 1,
						base_card_id: 1,
						card_type: "Item".to_string(),
						card_name: TranslationSet{ english: Some("Broadsword".to_string()) },
						card_text: TranslationSet{ english: Some("+8 attack".to_string()) },
						mini_image: ImageSet{ default: None },
						large_image: ImageSet{ default: None },
						ingame_image: ImageSet{ default: None },
						references: vec![],
						attack: None,
						hit_points: None,
						illustrator: None,
						gold_cost: Some(10),
						mana_cost: None,
						sub_type: None,
						is_green: None,
						is_red: None,
						is_black: None,
						is_blue: None,
						item_def: None,
						rarity: None,
					}
				]
			}
		};		
		let mut api = CardSetApi::new();
		api.cached_sets.insert(
			BASE_SET_ID.into(),
			base_set_cards.clone()
		);
		api.cached_sets.insert(
			CALL_TO_ARMS_SET_ID.into(),
			call_to_arms_cards.clone()
		);
		let search_params = FindItemsParams{
			gold_cost: 3,
			include_hold: false,
		};
		let mut found_items: CardList = api.get_cards().unwrap().find_items(&search_params);
		assert_eq!(found_items.len(), 1);
		assert_eq!(found_items.pop().unwrap().gold_cost.unwrap(), 3);

		let search_params = FindItemsParams{
			gold_cost: 4,
			include_hold: true,
		};
		let mut found_items: CardList = api.get_cards().unwrap().find_items(&search_params);
		assert_eq!(found_items.len(), 2);
		assert_eq!(found_items.pop().unwrap().gold_cost.unwrap(), 4);
		assert_eq!(found_items.pop().unwrap().gold_cost.unwrap(), 3);
	}
}
