extern crate reqwest;
extern crate mockito;
#[macro_use] extern crate serde_derive;

#[cfg(test)]
const CARD_SET_REQUEST_URL: &'static str = mockito::SERVER_URL;

#[cfg(not(test))]
const CARD_SET_REQUEST_URL: &str =  "https://playartifact.com/cardset/";

pub const BASE_SET_ID: &str = "00";
pub const CALL_TO_ARMS_SET_ID: &str = "01";

#[derive(Debug)]
pub enum CardSetRequestError {
	InvalidSetID { kind: reqwest::UrlError },
	ReqwestError { kind: reqwest::Error },
}

impl CardSetRequestError {
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

impl PartialEq for CardSetRequest {
    fn eq(&self, other: &CardSetRequest) -> bool {
        self.cdn_root == other.cdn_root &&
        self.url == other.url &&
        self.expire_time == other.expire_time
    }	
}

pub fn get_card_set_request(set_id: &str) -> Result<CardSetRequest, CardSetRequestError> {
	parse_url(set_id)
		.and_then(|url| reqwest::get(url).map_err(|e| CardSetRequestError::ReqwestError{kind: e}))
		.and_then(|mut response| response.json().map_err(|e| CardSetRequestError::ReqwestError{kind: e}))
}

fn parse_url(set_id: &str) -> Result<reqwest::Url, CardSetRequestError> {
	reqwest::Url::parse(CARD_SET_REQUEST_URL).map_err(|e| CardSetRequestError::InvalidSetID{kind: e})
		.and_then(|base_url| base_url.join(set_id).map_err(|e| CardSetRequestError::InvalidSetID{kind: e}))
}	

#[cfg(test)]
mod tests {
	extern crate mockito;
	extern crate serde_json;
	use tests::mockito::mock;
	use {CardSetRequest, parse_url, get_card_set_request};


	#[test]
	fn get_card_set_request_success() {
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
		let card_set_request = get_card_set_request("01").unwrap();
		assert_eq!(card_set_request, expected_body);
	}

    #[test]
    fn parse_url_parses_url() {
    	let url = parse_url("00").unwrap();
        assert_eq!(url.as_str(), "http://127.0.0.1:1234/00");
    }

    #[test]
    fn parse_url_returns_proper_err() {
    	let err = parse_url("//").unwrap_err();
        assert_eq!(err.to_string(), "empty host");
    }    
}
