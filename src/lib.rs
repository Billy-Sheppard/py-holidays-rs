mod types;
pub use types::*;

#[cfg(not(docsrs))]
use std::collections::BTreeMap;

#[cfg(not(docsrs))]
type CountryHolidayMap =
    BTreeMap<types::CountryCode, BTreeMap<types::SubDivision, BTreeMap<chrono::NaiveDate, String>>>;

#[cfg(not(docsrs))]
const HOLIDAYS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/holidays"));

#[cfg(not(docsrs))]
pub fn initialise() -> Result<CountryHolidayMap, String> {
    serde_json::from_reader(flate2::read::DeflateDecoder::new(HOLIDAYS))
        .map_err(|e| format!("{e:?}"))
}

#[cfg(all(feature = "years", not(target_arch = "wasm32")))]
pub use years::*;

#[cfg(all(feature = "years", not(target_arch = "wasm32")))]
mod years {
    use std::{io::Write, process::Stdio};

    use crate::CountryHolidayMap;

    const SCRIPT: &str = include_str!("../gen_objects.py");

    pub fn generate_with_years(years: Vec<usize>) -> Result<CountryHolidayMap, String> {
        let script = SCRIPT.replace("[2000,2001,2002,2003,2004,2005,2006,2007,2008,2009,2010,2011,2012,2013,2014,2015,2016,2017,2018,2019,2020,2021,2022,2023,2024,2025,2026,2027,2028,2029,2030,2031,2032,2033,2034,2035,2036,2037,2038,2039,2040,2041,2042,2043,2044,2045,2046,2047,2048,2049,2050]", &format!("{years:?}"));

        let mut process = std::process::Command::new("python")
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| format!("{e:?}"))?;

        write!(
            process
                .stdin
                .as_mut()
                .ok_or("Unable to get stdin as mut!")?,
            "{}",
            script
        )
        .map_err(|e| format!("{e:?}"))?;

        let out = process
            .wait_with_output()
            .map_err(|e| format!("{e:?}"))?
            .stdout;

        ron::de::from_bytes(&out).map_err(|e| format!("{e:?}"))
    }
}

#[cfg(test)]
#[cfg(not(docsrs))]
mod tests {
    use super::*;

    #[test]
    fn test_initialise() {
        assert!(initialise().is_ok());
    }

    #[test]
    #[cfg(all(feature = "years", not(target_arch = "wasm32")))]
    fn test_generate_with_years() {
        assert!(generate_with_years(Vec::from([2023])).is_ok());
    }

    // todo: more tests
}
