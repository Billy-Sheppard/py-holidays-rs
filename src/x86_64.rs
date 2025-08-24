use std::io::Read;

#[allow(dead_code)]
pub(crate) fn get(url: impl AsRef<str>) -> Result<Vec<u8>, String> {
    let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    let body = rt.block_on(async {
        let https = hyper_tls::HttpsConnector::new();
        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build(https);

        let req = hyper::Request::builder()
            .method("GET")
            .uri(url.as_ref())
            .body(String::new())
            .map_err(|e| e.to_string())?;

        let res = client.request(req).await.map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            return Err(format!("HTTP error: {}", res.status()));
        }
        let body_bytes = http_body_util::BodyExt::collect(res.into_body())
            .await
            .map_err(|e| e.to_string())?
            .to_bytes();
        Ok(body_bytes.to_vec())
    })?;

    let mut deflated = Vec::new();
    flate2::read::GzDecoder::new(body.as_slice())
        .read_to_end(&mut deflated)
        .map_err(|e| e.to_string())?;

    Ok(deflated)
}
