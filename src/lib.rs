mod types;
pub use types::*;

use std::collections::BTreeMap;

type CountryHolidayMap =
    BTreeMap<types::CountryCode, BTreeMap<types::SubDivision, BTreeMap<chrono::NaiveDate, String>>>;

const HOLIDAYS: &[u8] = include_bytes!("holidays.ron.gz");

pub fn initialise() -> Result<CountryHolidayMap, String> {
    let d = flate2::read::GzDecoder::new(HOLIDAYS);
    ron::de::from_reader(d).map_err(|e| format!("{e:?}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialise() {
        assert!(initialise().is_ok());
    }
}
