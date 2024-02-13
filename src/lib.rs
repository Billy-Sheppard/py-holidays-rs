use std::{
    collections::BTreeMap,
    io::{Cursor, Read},
};
mod types;

pub use types::*;

type CountryHolidayMap =
    BTreeMap<types::CountryCode, BTreeMap<String, BTreeMap<chrono::NaiveDate, String>>>;

include_flate::flate!(static HOLIDAYS: [u8] from "holidays.zip");

pub fn initialise() -> Result<CountryHolidayMap, String> {
    let mut zip =
        zip::ZipArchive::new(Cursor::new(HOLIDAYS.as_slice())).map_err(|e| format!("{e:?}"))?;
    let mut out = Vec::new();
    zip.by_index(0)
        .map_err(|e| format!("{e:?}"))?
        .read_to_end(&mut out)
        .map_err(|e| format!("{e:?}"))?;

    ron::de::from_bytes(&out).map_err(|e| format!("{e:?}"))
}

#[cfg(all(feature = "years", not(target_arch = "wasm_32")))]
pub use years::*;

#[cfg(all(feature = "years", not(target_arch = "wasm_32")))]
mod years {
    use std::{io::Write, process::Stdio};

    use crate::CountryHolidayMap;

    const SCRIPT: &str = include_str!("../gen_objects.py");

    pub fn generate_with_years(years: Vec<usize>) -> Result<CountryHolidayMap, String> {
        let script = SCRIPT.replace("[2001, 2002, 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011, 2012, 2013, 2014, 2015, 2016, 2017, 2018, 2019, 2020, 2021, 2022, 2023, 2024, 2025, 2026, 2027, 2028, 2029, 2030]", &format!("{years:?}"));

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

        std::fs::write("test.ron", String::from_utf8_lossy(&out).to_string())
            .map_err(|e| format!("{e:?}"))?;

        ron::de::from_bytes(&out).map_err(|e| format!("{e:?}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialise() {
        assert!(initialise().is_ok());
    }

    #[test]
    #[cfg(all(feature = "years", not(target_arch = "wasm_32")))]
    fn test_generate_with_years() {
        assert!(generate_with_years(Vec::from([2023])).is_ok());
    }

    // todo: more tests
}