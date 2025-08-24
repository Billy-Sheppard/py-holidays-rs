use py_holidays_rs::{CountryCode, get_holidays_by_country};
fn main() {
    let hols = get_holidays_by_country(CountryCode::AU);

    dbg!(hols.unwrap());
}
