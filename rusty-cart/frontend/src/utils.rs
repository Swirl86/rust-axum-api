use gloo_net::http::Request;

pub async fn fetch_json<T: serde::de::DeserializeOwned>(url: &str) -> Result<T, String> {
    match Request::get(url).send().await {
        Ok(resp) if resp.ok() => resp.json::<T>().await.map_err(|e| format!("JSON parse error: {:?}", e)),
        Ok(resp) => {
            let text = resp.text().await.unwrap_or_default();
            Err(format!("Server error {}: {}", resp.status(), text))
        }
        Err(e) => Err(format!("Network error: {:?}", e)),
    }
}

pub fn format_price(price: f64) -> String {
    format!("{:.2} kr", price)
}

/*pub fn format_price_value(price: f64) -> String {
    format!("{:.2}", price)
}*/