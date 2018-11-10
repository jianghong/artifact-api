extern crate reqwest;

const CARD_SET_REQUEST_URL: &str =  "https://playartifact.com/cardset/";

pub const BASE_SET_ID: &str = "00";
pub const CALL_TO_ARMS_SET_ID: &str = "01";

#[derive(Debug)]
pub enum CardSetRequestError {
	InvalidSetID { err: reqwest::UrlError },
	ReqwestError { err: reqwest::Error },
}

pub fn card_set_request(set_id: &str) -> Result<String, CardSetRequestError> {
	parse_url(set_id)
		.and_then(|url| reqwest::get(url).map_err(|e| CardSetRequestError::ReqwestError{err: e}))
		.and_then(|mut response| response.text().map_err(|e| CardSetRequestError::ReqwestError{err: e}))
		.and_then(|body| Ok(body))
}

fn parse_url(set_id: &str) -> Result<reqwest::Url, CardSetRequestError> {
	reqwest::Url::parse(CARD_SET_REQUEST_URL).map_err(|e| CardSetRequestError::InvalidSetID{err: e})
		.and_then(|base_url| base_url.join(set_id).map_err(|e| CardSetRequestError::InvalidSetID{err: e}))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
