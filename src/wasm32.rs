#[cfg(target_arch = "wasm32")]
use {reqwasm::http::Request, std::sync::Arc};

pub fn get(url: impl AsRef<str>) -> Result<Vec<u8>, String> {
    let url = url.as_ref();

    if url.contains("all_holidays.json.gz") {
        Err(
            "Call `py_holidays_rs::wasm32::get_all().await` first! Or use features =[\"offline\"]"
                .into(),
        )
    } else {
        Err("Call `py_holidays_rs::get_country(country).await` first! Or use features =[\"offline\"]".into())
    }
}

pub async fn initialise_all_holidays() -> Result<(), String> {
    #[cfg(target_arch = "wasm32")]
    {
        let resp = Request::get("https://raw.githubusercontent.com/Billy-Sheppard/py-holidays-rs/refs/heads/main/all_holidays.json.gz")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .binary()
        .await
        .map_err(|e| e.to_string())?;

        let deflated = flate2::read::GzDecoder::new(resp.as_slice());

        *crate::ALL_HOLIDAYS.lock().unwrap() = serde_json::from_reader(deflated)
            .map_err(|e| e.to_string())
            .map(Arc::new);
    }

    Ok(())
}

pub async fn initialise_holidays_by_country(
    _country: crate::types::CountryCode,
) -> Result<(), String> {
    #[cfg(target_arch = "wasm32")]
    {
        let resp = Request::get(
            &format!(
                "https://raw.githubusercontent.com/Billy-Sheppard/py-holidays-rs/refs/heads/main/files/{:?}.json.gz",
                _country
            )
        )
        .send()
        .await
        .map_err(|e| e.to_string())?
        .binary()
        .await
        .map_err(|e| e.to_string())?;

        let deflated = flate2::read::GzDecoder::new(resp.as_slice());

        crate::PER_COUNTRY_HOLIDAYS
            .lock()
            .map_err(|e| e.to_string())?
            .insert(
                _country,
                serde_json::from_reader(deflated).map_err(|e| e.to_string())?,
            );
    }

    Ok(())
}
