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
        let script = SCRIPT
            .lines()
            .map(|line| {
                if line.trim_start().starts_with("years = ") {
                    format!("years = {years:?}")
                } else {
                    line.into()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        let mut process =
            std::process::Command::new(format!("{}/python-env/bin/python3", env!("OUT_DIR")))
                // .arg("--require-venv")
                .arg("-")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .env("VIRTUAL_ENV", format!("{}/python-env", env!("OUT_DIR")))
                .spawn()
                .map_err(|e| format!("{e:?}"))?;

        write!(
            process
                .stdin
                .as_mut()
                .ok_or("Unable to get stdin as mut!")?,
            "{script}",
        )
        .map_err(|e| format!("{e:?}"))?;

        let out = process
            .wait_with_output()
            .map_err(|e| format!("{e:?}"))?
            .stdout;

        serde_json::from_slice(&out).map_err(|e| format!("{e:?}"))
    }
}

#[cfg(test)]
#[cfg(not(docsrs))]
mod tests {
    use super::*;

    #[test]
    fn test_initialise() {
        let i = initialise()
        // .inspect_err(|e| {
        //     dbg!(e);
        // })
        ;
        assert!(i.is_ok());
    }

    #[test]
    #[cfg(all(feature = "years", not(target_arch = "wasm32")))]
    fn test_generate_with_years() {
        let i = generate_with_years(Vec::from([2023])).inspect_err(|e| {
            dbg!(e);
        });
        assert!(i.is_ok());
    }

    // todo: more tests
}
