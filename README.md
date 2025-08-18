<p align="center">
  <img width="200" src="https://raw.githubusercontent.com/Billy-Sheppard/py-holidays-rs/main/logo.png">
</p>

# python Holidays package in Rust
[![crates.io](https://img.shields.io/crates/v/py-holidays-rs.svg)](https://crates.io/crates/py-holidays-rs)
[![docs.rs](https://docs.rs/py-holidays-rs/badge.svg)](https://docs.rs/py-holidays-rs)

## ***In beta, feel free to PR.***

## Usage
NB: This package requires an internet connection at runtime, unless you use the feature `offline`. 
Turning that feature on will embed holidays into your compiled artifact and may have size implications.

### x86_64 `(default-features = true)`
```rust
// Result<Arc<CountryHolidayMap>, String>
let all_holidays = py_holidays_rs::get_all_holidays(CountryCode::AU); 
```
```rust
// Result<BTreeMap<SubDivision, BTreeMap<chrono::NaiveDate, String>>, String>
let australian_holidays = py_holidays_rs::get_holidays_by_country(CountryCode::AU); 
```
### x86_64 `(features = ["offline"])`
```rust
// Result<Arc<CountryHolidayMap>, String>
let all_holidays = py_holidays_rs::get_all_holidays(CountryCode::AU); 
```
```rust
// Result<BTreeMap<SubDivision, BTreeMap<chrono::NaiveDate, String>>, String>
let australian_holidays = py_holidays_rs::get_holidays_by_country(CountryCode::AU); 
```
### wasm32 `(default-features = true)`
```rust
py_holidays_rs::wasm32::initialise_all_holidays(CountryCode::AU).await; // you must call this somewhere before you call the get_* function, you only need to call it once

// Result<Arc<CountryHolidayMap>, String>
let all_holidays = py_holidays_rs::get_all_holidays(CountryCode::AU); 
```
```rust
py_holidays_rs::wasm32::initialise_holidays_by_country(CountryCode::AU).await; // you must call this somewhere before you call the get_* function, you only need to call it once per country code

// Result<BTreeMap<SubDivision, BTreeMap<chrono::NaiveDate, String>>, String>
let australian_holidays = py_holidays_rs::get_holidays_by_country(CountryCode::AU); 
```
### wasm32 `(features = ["offline"])`
```rust
// Result<Arc<CountryHolidayMap>, String>
let all_holidays = py_holidays_rs::get_all_holidays(CountryCode::AU); 
```
```rust
// Result<BTreeMap<SubDivision, BTreeMap<chrono::NaiveDate, String>>, String>
let australian_holidays = py_holidays_rs::get_holidays_by_country(CountryCode::AU); 
```