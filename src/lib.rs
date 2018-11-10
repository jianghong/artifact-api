extern crate reqwest;
extern crate mockito;
#[macro_use] extern crate serde_derive;

#[cfg(test)]
const CARD_SET_REQUEST_URL: &'static str = mockito::SERVER_URL;

#[cfg(not(test))]
const CARD_SET_REQUEST_URL: &str =  "https://playartifact.com/cardset/";

pub const BASE_SET_ID: &str = "00";
pub const CALL_TO_ARMS_SET_ID: &str = "01";

pub struct CardSetApi {}

impl CardSetApi {
	pub fn new() -> Self {
		CardSetApi {}
	}

	pub fn get_set(self, set_id: &str) -> Result<CardSetRequest, CardSetRequestError> {
		self.get_set_request(set_id)
	}

	fn get_set_request(self, set_id: &str) -> Result<CardSetRequest, CardSetRequestError> {
		self.parse_url(set_id)
			.and_then(|url| reqwest::get(url).map_err(|e| CardSetRequestError::ReqwestError{kind: e}))
			.and_then(|mut response| response.json().map_err(|e| CardSetRequestError::ReqwestError{kind: e}))
	}

	fn parse_url(self, set_id: &str) -> Result<reqwest::Url, CardSetRequestError> {
		reqwest::Url::parse(CARD_SET_REQUEST_URL).map_err(|e| CardSetRequestError::InvalidSetID{kind: e})
			.and_then(|base_url| base_url.join(set_id).map_err(|e| CardSetRequestError::InvalidSetID{kind: e}))
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CardSetRequest {
	cdn_root: String,
	url: String,
	expire_time: i32,
}

impl CardSetRequest {
	pub fn url(self) -> String {
		format!("{}{}", self.cdn_root, self.url)
	}
}

impl PartialEq for CardSetRequest {
    fn eq(&self, other: &CardSetRequest) -> bool {
        self.cdn_root == other.cdn_root &&
        self.url == other.url &&
        self.expire_time == other.expire_time
    }	
}

#[cfg(test)]
mod tests {
	extern crate mockito;
	extern crate serde_json;
	use tests::mockito::mock;
	use {CardSetRequest, CardSetApi};

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
	    let card_set_request = CardSetApi::new().get_set("01").unwrap();
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
	    let err = CardSetApi::new().get_set("01").unwrap_err();
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

		assert_eq!(card_set_request.url(), "https://cdnroot.com/path/to/set");
	}  
}
