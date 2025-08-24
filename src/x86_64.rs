use std::io::Read;
pub(crate) fn get(url: impl AsRef<str>) -> Result<Vec<u8>, String> {
    let body = reqwest::blocking::get(url.as_ref())
        .map_err(|e| e.to_string())?
        .bytes()
        .map_err(|e| e.to_string())?;

    let mut deflated = Vec::new();
    flate2::read::GzDecoder::new(body.as_ref())
        .read_to_end(&mut deflated)
        .map_err(|e| e.to_string())?;

    Ok(deflated)
}
