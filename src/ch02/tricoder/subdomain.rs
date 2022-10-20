use reqwest::blocking::Client;

pub fn enumerate(http_client: &Client, target: &str) -> bool {
    let resp = http_client.get(target).send();

    match resp {
        Ok(resp) => {
            if resp.status().is_success() {
                true
            } else {
                false
            }
        }
        Err(_e) => false,
    }
}
