mod types;

pub use types::*;

pub mod wasm32;
#[cfg(target_arch = "wasm32")]
use wasm32::*;

#[cfg(not(target_arch = "wasm32"))]
mod x86_64;
#[cfg(not(target_arch = "wasm32"))]
use x86_64::*;

use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

type CountryHolidayMap =
    BTreeMap<types::CountryCode, BTreeMap<types::SubDivision, BTreeMap<chrono::NaiveDate, String>>>;

// both
static ALL_HOLIDAYS: std::sync::LazyLock<Mutex<Result<Arc<CountryHolidayMap>, String>>> =
    std::sync::LazyLock::new(get_all);

pub fn get_all_holidays() -> Result<Arc<CountryHolidayMap>, String> {
    Ok(ALL_HOLIDAYS
        .lock()
        .map_err(|e| e.to_string())?
        .as_ref()
        .cloned()?)
}

static PER_COUNTRY_HOLIDAYS: std::sync::LazyLock<Mutex<CountryHolidayMap>> =
    std::sync::LazyLock::new(|| Mutex::new(Default::default()));

pub fn get_holidays_by_country(
    country: CountryCode,
) -> Result<BTreeMap<SubDivision, BTreeMap<chrono::NaiveDate, String>>, String> {
    let mut map = PER_COUNTRY_HOLIDAYS.lock().map_err(|e| e.to_string())?;

    match map.contains_key(&country) {
        true => Ok(map.get(&country).unwrap().clone()),
        false => {
            let holidays = get_country(country)?;
            map.insert(country, holidays.clone());

            Ok(holidays)
        }
    }
}

// online
#[cfg(not(feature = "offline"))]
fn get_all() -> Mutex<Result<Arc<CountryHolidayMap>, String>> {
    let bytes = get(
        "https://raw.githubusercontent.com/Billy-Sheppard/py-holidays-rs/refs/heads/main/all_holidays.json.gz",
    );

    Mutex::new(
        bytes
            .and_then(|deflated| {
                serde_json::from_reader(deflated.as_slice()).map_err(|e| e.to_string())
            })
            .map(Arc::new),
    )
}

#[cfg(not(feature = "offline"))]
fn get_country(
    country: CountryCode,
) -> Result<BTreeMap<SubDivision, BTreeMap<chrono::NaiveDate, String>>, String> {
    let bytes = get(format!(
        "https://raw.githubusercontent.com/Billy-Sheppard/py-holidays-rs/refs/heads/main/files/{:?}.json.gz",
        country
    ))?;

    let holidays = serde_json::from_slice::<BTreeMap<_, _>>(&bytes).map_err(|e| e.to_string())?;
    Ok(holidays)
}

// offline
#[cfg(feature = "offline")]
const ALL_HOLIDAYS_BYTES: &[u8] = include_bytes!("../all_holidays.json.gz");
#[cfg(feature = "offline")]
fn get_all() -> Mutex<Result<Arc<CountryHolidayMap>, String>> {
    let deflated = flate2::read::GzDecoder::new(ALL_HOLIDAYS_BYTES);

    Mutex::new(
        serde_json::from_reader(deflated)
            .map_err(|e| e.to_string())
            .map(Arc::new),
    )
}

#[cfg(feature = "offline")]
#[derive(rust_embed::Embed)]
#[folder = "./files"]
struct PerCountryHolidays;

#[cfg(feature = "offline")]
fn get_country(
    country: CountryCode,
) -> Result<BTreeMap<SubDivision, BTreeMap<chrono::NaiveDate, String>>, String> {
    let file =
        PerCountryHolidays::get(&format!("{:?}.json.gz", country)).ok_or("File not found!")?;
    let deflated = flate2::read::GzDecoder::new(file.data.as_ref());

    let holidays =
        serde_json::from_reader::<_, BTreeMap<_, _>>(deflated).map_err(|e| e.to_string())?;
    Ok(holidays)
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialise() {
        let i = get_all_holidays()
            // .inspect_err(|e| {
            //     dbg!(e);
            // })
            ;
        assert!(i.is_ok());
    }

    #[test]
    fn test_au() {
        let i = get_holidays_by_country(CountryCode::AU)
            // .inspect_err(|e| {
            //     dbg!(e);
            // })
            ;
        assert!(i.is_ok());
    }

    #[test]
    fn test_gb() {
        let i = get_holidays_by_country(CountryCode::GB)
            // .inspect_err(|e| {
            //     dbg!(e);
            // })
            ;
        assert!(i.is_ok());
    }

    #[test]
    fn test_us() {
        let i = get_holidays_by_country(CountryCode::US)
            // .inspect_err(|e| {
            //     dbg!(e);
            // })
            ;
        assert!(i.is_ok());
    }
}
