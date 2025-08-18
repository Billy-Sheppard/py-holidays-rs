mod types;
pub use types::*;

use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

type CountryHolidayMap =
    BTreeMap<types::CountryCode, BTreeMap<types::SubDivision, BTreeMap<chrono::NaiveDate, String>>>;

static ALL_HOLIDAYS: std::sync::LazyLock<Result<Arc<CountryHolidayMap>, String>> =
    std::sync::LazyLock::new(|| {
        let mut body = ureq::get("https://raw.githubusercontent.com/Billy-Sheppard/py-holidays-rs/refs/heads/main/all_holidays.json.gz") 
            .call()
            .map_err(|e| e.to_string())?
            .into_body();
        let deflated = flate2::read::GzDecoder::new(body.as_reader());

        Ok(Arc::new(
            serde_json::from_reader(deflated).map_err(|e| e.to_string())?,
        ))
    });

pub fn initialise_all() -> Result<Arc<CountryHolidayMap>, String> {
    ALL_HOLIDAYS.clone()
}

static PER_COUNTRY_HOLIDAYS: std::sync::LazyLock<Mutex<CountryHolidayMap>> =
    std::sync::LazyLock::new(|| Mutex::new(Default::default()));

pub fn initialise_country(
    country: CountryCode,
) -> Result<BTreeMap<types::SubDivision, BTreeMap<chrono::NaiveDate, String>>, String> {
    let mut map = PER_COUNTRY_HOLIDAYS.lock().map_err(|e| e.to_string())?;

    match map.contains_key(&country) {
        true => Ok(map.get(&country).unwrap().clone()),
        false => {
            let mut body = ureq::get(format!("https://raw.githubusercontent.com/Billy-Sheppard/py-holidays-rs/refs/heads/main/files/{:?}.json.gz", country)) 
                .call()
                .map_err(|e| e.to_string())?
                .into_body();
            let deflated = flate2::read::GzDecoder::new(body.as_reader());

            let holidays = serde_json::from_reader::<_, BTreeMap<_, _>>(deflated)
                .map_err(|e| e.to_string())?;
            map.insert(country, holidays.clone());

            Ok(holidays)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialise() {
        let i = initialise_all()
            // .inspect_err(|e| {
            //     dbg!(e);
            // })
            ;
        assert!(i.is_ok());
    }

    #[test]
    fn test_au() {
        let i = initialise_country(CountryCode::AU)
            // .inspect_err(|e| {
            //     dbg!(e);
            // })
            ;
        assert!(i.is_ok());
    }

    #[test]
    fn test_gb() {
        let i = initialise_country(CountryCode::GB)
            // .inspect_err(|e| {
            //     dbg!(e);
            // })
            ;
        assert!(i.is_ok());
    }

    #[test]
    fn test_us() {
        let i = initialise_country(CountryCode::US)
            // .inspect_err(|e| {
            //     dbg!(e);
            // })
            ;
        assert!(i.is_ok());
    }
}
