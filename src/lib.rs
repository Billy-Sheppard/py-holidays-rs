use std::{
    collections::BTreeMap,
    io::{Cursor, Read},
};
mod types;

type CountryHolidayMap =
    BTreeMap<types::CountryCode, BTreeMap<String, BTreeMap<chrono::NaiveDate, String>>>;

include_flate::flate!(static HOLIDAYS: [u8] from "holidays.zip");

pub fn initialise() -> CountryHolidayMap {
    let mut zip = zip::ZipArchive::new(Cursor::new(HOLIDAYS.as_slice())).unwrap();
    let mut out = Vec::new();
    zip.by_index(0).unwrap().read_to_end(&mut out).unwrap();

    ron::de::from_bytes(&out).unwrap()
}

#[cfg(all(feature = "years", not(target_arch = "wasm_32")))]
pub use years::*;

#[cfg(all(feature = "years", not(target_arch = "wasm_32")))]
mod years {
    use std::{io::Write, process::Stdio};

    const SCRIPT: &str = include_str!("../gen_objects.py");

    pub fn generate_with_years(years: Vec<usize>) {
        let script = SCRIPT.replace("[2001, 2002, 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011, 2012, 2013, 2014, 2015, 2016, 2017, 2018, 2019, 2020, 2021, 2022, 2023, 2024, 2025, 2026, 2027, 2028, 2029, 2030]", &format!("{years:?}"));

        let mut process = std::process::Command::new("python")
            .arg("-")
            .stdin(Stdio::piped())
            .spawn()
            .unwrap();

        write!(process.stdin.as_mut().unwrap(), "{}", script).unwrap();

        let out = process.wait_with_output().unwrap().stdout;

        dbg!(&out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        generate_with_years(Vec::from([2023, 2024]))
    }
}
