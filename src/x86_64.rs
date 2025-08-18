use std::io::Read;
pub(crate) fn get(url: impl AsRef<str>) -> Result<Vec<u8>, String> {
    let body = ureq::get(url.as_ref())
        .call()
        .map_err(|e| e.to_string())?
        .into_body()
        .read_to_vec()
        .map_err(|e| e.to_string())?;

    let mut deflated = Vec::new();
    flate2::read::GzDecoder::new(body.as_slice())
        .read_to_end(&mut deflated)
        .map_err(|e| e.to_string())?;

    Ok(deflated)
}
