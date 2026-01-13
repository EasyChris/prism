use axum::http::HeaderMap;

/// 将 Axum 的 HeaderMap 转换为 Reqwest 的 HeaderMap
pub(super) fn convert_headers(headers: &HeaderMap) -> reqwest::header::HeaderMap {
    let mut new_headers = reqwest::header::HeaderMap::new();

    for (key, value) in headers.iter() {
        if let Ok(value) = value.to_str() {
            if let Ok(header_value) = reqwest::header::HeaderValue::from_str(value) {
                new_headers.insert(key.clone(), header_value);
            }
        }
    }

    new_headers
}
