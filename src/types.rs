use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Country {
    pub code: CountryCode,
    pub name: &'static str,
    pub subdivisions: &'static [&'static str],
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum CountryCode {
    AL,
    DZ,
    AS,
    AD,
    AO,
    AR,
    AM,
    AW,
    AU,
    AT,
    AZ,
    BS,
    BH,
    BD,
    BB,
    BY,
    BE,
    BZ,
    BO,
    BA,
    BW,
    BR,
    BN,
    BG,
    BF,
    BI,
    KH,
    CM,
    CA,
    TD,
    CL,
    CN,
    CO,
    CR,
    HR,
    CU,
    CW,
    CY,
    CZ,
    DK,
    DJ,
    DO,
    EC,
    EG,
    SV,
    EE,
    SZ,
    ET,
    FI,
    FR,
    GA,
    GE,
    DE,
    GR,
    GU,
    GT,
    HN,
    HK,
    HU,
    IS,
    IN,
    ID,
    IR,
    IE,
    IM,
    IL,
    IT,
    JM,
    JP,
    KZ,
    KE,
    KG,
    LA,
    LV,
    LS,
    LI,
    LT,
    LU,
    MG,
    MW,
    MY,
    MV,
    MT,
    MH,
    MX,
    MD,
    MC,
    ME,
    MA,
    MZ,
    NA,
    NL,
    NZ,
    NI,
    NG,
    MP,
    MK,
    NO,
    PK,
    PA,
    PG,
    PY,
    PE,
    PH,
    PL,
    PT,
    PR,
    RO,
    RU,
    SM,
    SA,
    RS,
    SG,
    SK,
    SI,
    ZA,
    KR,
    ES,
    SE,
    CH,
    TW,
    TZ,
    TH,
    TL,
    TO,
    TN,
    TR,
    UA,
    AE,
    GB,
    UM,
    US,
    UY,
    UZ,
    VU,
    VA,
    VE,
    VN,
    VI,
    ZM,
    ZW,
}

impl CountryCode {
    pub fn get_by_country_name(s: &str) -> Option<CountryCode> {
        match s {
            "Albania" => Some(CountryCode::AL),
            "Algeria" => Some(CountryCode::DZ),
            "American Samoa" => Some(CountryCode::AS),
            "Angola" => Some(CountryCode::AO),
            "Argentina" => Some(CountryCode::AR),
            "Armenia" => Some(CountryCode::AM),
            "Aruba" => Some(CountryCode::AW),
            "Azerbaijan" => Some(CountryCode::AZ),
            "Bahamas" => Some(CountryCode::BS),
            "Bahrain" => Some(CountryCode::BH),
            "Bangladesh" => Some(CountryCode::BD),
            "Barbados" => Some(CountryCode::BB),
            "Belarus" => Some(CountryCode::BY),
            "Belgium" => Some(CountryCode::BE),
            "Belize" => Some(CountryCode::BZ),
            "Bosnia and Herzegovina" => Some(CountryCode::BA),
            "Botswana" => Some(CountryCode::BW),
            "Brunei" => Some(CountryCode::BN),
            "Bulgaria" => Some(CountryCode::BG),
            "Burkina Faso" => Some(CountryCode::BF),
            "Burundi" => Some(CountryCode::BI),
            "Cambodia" => Some(CountryCode::KH),
            "Cameroon" => Some(CountryCode::CM),
            "Chad" => Some(CountryCode::TD),
            "China" => Some(CountryCode::CN),
            "Colombia" => Some(CountryCode::CO),
            "Costa Rica" => Some(CountryCode::CR),
            "Croatia" => Some(CountryCode::HR),
            "Cuba" => Some(CountryCode::CU),
            "Curacao" => Some(CountryCode::CW),
            "Cyprus" => Some(CountryCode::CY),
            "Czechia" => Some(CountryCode::CZ),
            "Denmark" => Some(CountryCode::DK),
            "Djibouti" => Some(CountryCode::DJ),
            "Dominican Republic" => Some(CountryCode::DO),
            "Ecuador" => Some(CountryCode::EC),
            "Egypt" => Some(CountryCode::EG),
            "Estonia" => Some(CountryCode::EE),
            "Eswatini" => Some(CountryCode::SZ),
            "Ethiopia" => Some(CountryCode::ET),
            "Finland" => Some(CountryCode::FI),
            "Gabon" => Some(CountryCode::GA),
            "Georgia" => Some(CountryCode::GE),
            "Greece" => Some(CountryCode::GR),
            "Guam" => Some(CountryCode::GU),
            "Guatemala" => Some(CountryCode::GT),
            "Honduras" => Some(CountryCode::HN),
            "Hong Kong" => Some(CountryCode::HK),
            "Hungary" => Some(CountryCode::HU),
            "Iceland" => Some(CountryCode::IS),
            "Indonesia" => Some(CountryCode::ID),
            "Iran" => Some(CountryCode::IR),
            "Ireland" => Some(CountryCode::IE),
            "Isle of Man" => Some(CountryCode::IM),
            "Israel" => Some(CountryCode::IL),
            "Jamaica" => Some(CountryCode::JM),
            "Japan" => Some(CountryCode::JP),
            "Kazakhstan" => Some(CountryCode::KZ),
            "Kenya" => Some(CountryCode::KE),
            "Kyrgyzstan" => Some(CountryCode::KG),
            "Laos" => Some(CountryCode::LA),
            "Latvia" => Some(CountryCode::LV),
            "Lesotho" => Some(CountryCode::LS),
            "Liechtenstein" => Some(CountryCode::LI),
            "Lithuania" => Some(CountryCode::LT),
            "Luxembourg" => Some(CountryCode::LU),
            "Madagascar" => Some(CountryCode::MG),
            "Malawi" => Some(CountryCode::MW),
            "Maldives" => Some(CountryCode::MV),
            "Malta" => Some(CountryCode::MT),
            "Marshall Islands" => Some(CountryCode::MH),
            "Mexico" => Some(CountryCode::MX),
            "Moldova" => Some(CountryCode::MD),
            "Monaco" => Some(CountryCode::MC),
            "Montenegro" => Some(CountryCode::ME),
            "Morocco" => Some(CountryCode::MA),
            "Mozambique" => Some(CountryCode::MZ),
            "Namibia" => Some(CountryCode::NA),
            "Netherlands" => Some(CountryCode::NL),
            "Nigeria" => Some(CountryCode::NG),
            "Northern Mariana Islands" => Some(CountryCode::MP),
            "North Macedonia" => Some(CountryCode::MK),
            "Norway" => Some(CountryCode::NO),
            "Pakistan" => Some(CountryCode::PK),
            "Panama" => Some(CountryCode::PA),
            "Papua New Guinea" => Some(CountryCode::PG),
            "Paraguay" => Some(CountryCode::PY),
            "Peru" => Some(CountryCode::PE),
            "Philippines" => Some(CountryCode::PH),
            "Poland" => Some(CountryCode::PL),
            "Puerto Rico" => Some(CountryCode::PR),
            "Romania" => Some(CountryCode::RO),
            "Russia" => Some(CountryCode::RU),
            "San Marino" => Some(CountryCode::SM),
            "Saudi Arabia" => Some(CountryCode::SA),
            "Serbia" => Some(CountryCode::RS),
            "Singapore" => Some(CountryCode::SG),
            "Slovakia" => Some(CountryCode::SK),
            "Slovenia" => Some(CountryCode::SI),
            "South Africa" => Some(CountryCode::ZA),
            "South Korea" => Some(CountryCode::KR),
            "Sweden" => Some(CountryCode::SE),
            "Taiwan" => Some(CountryCode::TW),
            "Tanzania" => Some(CountryCode::TZ),
            "Thailand" => Some(CountryCode::TH),
            "Timor Leste" => Some(CountryCode::TL),
            "Tonga" => Some(CountryCode::TO),
            "Tunisia" => Some(CountryCode::TN),
            "Turkey" => Some(CountryCode::TR),
            "Ukraine" => Some(CountryCode::UA),
            "United Arab Emirates" => Some(CountryCode::AE),
            "United Kingdom" => Some(CountryCode::GB),
            "United States Minor Outlying Islands" => Some(CountryCode::UM),
            "Uruguay" => Some(CountryCode::UY),
            "Uzbekistan" => Some(CountryCode::UZ),
            "Vanuatu" => Some(CountryCode::VU),
            "Vatican City" => Some(CountryCode::VA),
            "Venezuela" => Some(CountryCode::VE),
            "Vietnam" => Some(CountryCode::VN),
            "Virgin Islands (U.S.)" => Some(CountryCode::VI),
            "Zambia" => Some(CountryCode::ZM),
            "Zimbabwe" => Some(CountryCode::ZW),
            _ => None,
        }
    }

    pub fn list_subdivisions_by_country<'a>(country: CountryCode) -> &'a [&'static str] {
        Self::list_subdivisions()
            .get(&country)
            .unwrap()
            .subdivisions
    }

    pub fn list_subdivisions() -> BTreeMap<CountryCode, Country> {
        BTreeMap::from([
            (
                CountryCode::AL,
                Country {
                    code: CountryCode::AL,
                    name: "Albania",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::DZ,
                Country {
                    code: CountryCode::DZ,
                    name: "Algeria",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::AS,
                Country {
                    code: CountryCode::AS,
                    name: "American Samoa",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::AD,
                Country {
                    code: CountryCode::AD,
                    name: "Andorra",
                    subdivisions: &["02", "03", "04", "05", "06", "07", "08"],
                },
            ),
            (
                CountryCode::AO,
                Country {
                    code: CountryCode::AO,
                    name: "Angola",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::AR,
                Country {
                    code: CountryCode::AR,
                    name: "Argentina",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::AM,
                Country {
                    code: CountryCode::AM,
                    name: "Armenia",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::AW,
                Country {
                    code: CountryCode::AW,
                    name: "Aruba",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::AU,
                Country {
                    code: CountryCode::AU,
                    name: "Australia",
                    subdivisions: &["ACT", "NSW", "NT", "QLD", "SA", "TAS", "VIC", "WA"],
                },
            ),
            (
                CountryCode::AT,
                Country {
                    code: CountryCode::AT,
                    name: "Austria",
                    subdivisions: &["1", "2", "3", "4", "5", "6", "7", "8", "9"],
                },
            ),
            (
                CountryCode::AZ,
                Country {
                    code: CountryCode::AZ,
                    name: "Azerbaijan",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::BS,
                Country {
                    code: CountryCode::BS,
                    name: "Bahamas",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::BH,
                Country {
                    code: CountryCode::BH,
                    name: "Bahrain",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::BD,
                Country {
                    code: CountryCode::BD,
                    name: "Bangladesh",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::BB,
                Country {
                    code: CountryCode::BB,
                    name: "Barbados",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::BY,
                Country {
                    code: CountryCode::BY,
                    name: "Belarus",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::BE,
                Country {
                    code: CountryCode::BE,
                    name: "Belgium",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::BZ,
                Country {
                    code: CountryCode::BZ,
                    name: "Belize",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::BO,
                Country {
                    code: CountryCode::BO,
                    name: "Bolivia",
                    subdivisions: &["B", "C", "H", "L", "N", "O", "P", "S", "T"],
                },
            ),
            (
                CountryCode::BA,
                Country {
                    code: CountryCode::BA,
                    name: "Bosnia and Herzegovina",
                    subdivisions: &["BIH", "BRC", "SRP"],
                },
            ),
            (
                CountryCode::BW,
                Country {
                    code: CountryCode::BW,
                    name: "Botswana",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::BR,
                Country {
                    code: CountryCode::BR,
                    name: "Brazil",
                    subdivisions: &[
                        "AC", "AL", "AM", "AP", "BA", "CE", "DF", "ES", "GO", "MA", "MG", "MS",
                        "MT", "PA", "PB", "PE", "PI", "PR", "RJ", "RN", "RO", "RR", "RS", "SC",
                        "SE", "SP", "TO",
                    ],
                },
            ),
            (
                CountryCode::BN,
                Country {
                    code: CountryCode::BN,
                    name: "Brunei",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::BG,
                Country {
                    code: CountryCode::BG,
                    name: "Bulgaria",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::BF,
                Country {
                    code: CountryCode::BF,
                    name: "Burkina Faso",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::BI,
                Country {
                    code: CountryCode::BI,
                    name: "Burundi",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::KH,
                Country {
                    code: CountryCode::KH,
                    name: "Cambodia",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::CM,
                Country {
                    code: CountryCode::CM,
                    name: "Cameroon",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::CA,
                Country {
                    code: CountryCode::CA,
                    name: "Canada",
                    subdivisions: &[
                        "AB", "BC", "MB", "NB", "NL", "NS", "NT", "NU", "ON", "PE", "QC", "SK",
                        "YT",
                    ],
                },
            ),
            (
                CountryCode::TD,
                Country {
                    code: CountryCode::TD,
                    name: "Chad",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::CL,
                Country {
                    code: CountryCode::CL,
                    name: "Chile",
                    subdivisions: &[
                        "AI", "AN", "AP", "AR", "AT", "BI", "CO", "LI", "LL", "LR", "MA", "ML",
                        "NB", "RM", "TA", "VS",
                    ],
                },
            ),
            (
                CountryCode::CN,
                Country {
                    code: CountryCode::CN,
                    name: "China",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::CO,
                Country {
                    code: CountryCode::CO,
                    name: "Colombia",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::CR,
                Country {
                    code: CountryCode::CR,
                    name: "Costa Rica",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::HR,
                Country {
                    code: CountryCode::HR,
                    name: "Croatia",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::CU,
                Country {
                    code: CountryCode::CU,
                    name: "Cuba",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::CW,
                Country {
                    code: CountryCode::CW,
                    name: "Curacao",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::CY,
                Country {
                    code: CountryCode::CY,
                    name: "Cyprus",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::CZ,
                Country {
                    code: CountryCode::CZ,
                    name: "Czechia",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::DK,
                Country {
                    code: CountryCode::DK,
                    name: "Denmark",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::DJ,
                Country {
                    code: CountryCode::DJ,
                    name: "Djibouti",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::DO,
                Country {
                    code: CountryCode::DO,
                    name: "Dominican Republic",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::EC,
                Country {
                    code: CountryCode::EC,
                    name: "Ecuador",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::EG,
                Country {
                    code: CountryCode::EG,
                    name: "Egypt",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::SV,
                Country {
                    code: CountryCode::SV,
                    name: "El Salvador",
                    subdivisions: &[
                        "AH", "CA", "CH", "CU", "LI", "MO", "PA", "SA", "SM", "SO", "SS", "SV",
                        "UN", "US",
                    ],
                },
            ),
            (
                CountryCode::EE,
                Country {
                    code: CountryCode::EE,
                    name: "Estonia",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::SZ,
                Country {
                    code: CountryCode::SZ,
                    name: "Eswatini",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::ET,
                Country {
                    code: CountryCode::ET,
                    name: "Ethiopia",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::FI,
                Country {
                    code: CountryCode::FI,
                    name: "Finland",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::FR,
                Country {
                    code: CountryCode::FR,
                    name: "France",
                    subdivisions: &[
                        "BL", "GES", "GP", "GY", "MF", "MQ", "NC", "PF", "RE", "WF", "YT",
                    ],
                },
            ),
            (
                CountryCode::GA,
                Country {
                    code: CountryCode::GA,
                    name: "Gabon",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::GE,
                Country {
                    code: CountryCode::GE,
                    name: "Georgia",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::DE,
                Country {
                    code: CountryCode::DE,
                    name: "Germany",
                    subdivisions: &[
                        "BB", "BE", "BW", "BY", "BYP", "HB", "HE", "HH", "MV", "NI", "NW", "RP",
                        "SH", "SL", "SN", "ST", "TH",
                    ],
                },
            ),
            (
                CountryCode::GR,
                Country {
                    code: CountryCode::GR,
                    name: "Greece",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::GU,
                Country {
                    code: CountryCode::GU,
                    name: "Guam",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::GT,
                Country {
                    code: CountryCode::GT,
                    name: "Guatemala",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::HN,
                Country {
                    code: CountryCode::HN,
                    name: "Honduras",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::HK,
                Country {
                    code: CountryCode::HK,
                    name: "Hong Kong",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::HU,
                Country {
                    code: CountryCode::HU,
                    name: "Hungary",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::IS,
                Country {
                    code: CountryCode::IS,
                    name: "Iceland",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::IN,
                Country {
                    code: CountryCode::IN,
                    name: "India",
                    subdivisions: &[
                        "AN", "AP", "AR", "AS", "BR", "CG", "CH", "DD", "DH", "DL", "GA", "GJ",
                        "HP", "HR", "JH", "JK", "KA", "KL", "LA", "LD", "MH", "ML", "MN", "MP",
                        "MZ", "NL", "OR", "PB", "PY", "RJ", "SK", "TN", "TR", "TS", "UK", "UP",
                        "WB",
                    ],
                },
            ),
            (
                CountryCode::ID,
                Country {
                    code: CountryCode::ID,
                    name: "Indonesia",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::IR,
                Country {
                    code: CountryCode::IR,
                    name: "Iran",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::IE,
                Country {
                    code: CountryCode::IE,
                    name: "Ireland",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::IM,
                Country {
                    code: CountryCode::IM,
                    name: "Isle of Man",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::IL,
                Country {
                    code: CountryCode::IL,
                    name: "Israel",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::IT,
                Country {
                    code: CountryCode::IT,
                    name: "Italy",
                    subdivisions: &[
                        "AG", "AL", "AN", "AO", "AP", "AQ", "AR", "AT", "AV", "BA", "BG", "BI",
                        "BL", "BN", "BO", "BR", "BS", "BT", "BZ", "CA", "CB", "CE", "CH", "CL",
                        "CN", "CO", "CR", "CS", "CT", "CZ", "EN", "FC", "FE", "FG", "FI", "FM",
                        "FR", "GE", "GO", "GR", "IM", "IS", "KR", "LC", "LE", "LI", "LO", "LT",
                        "LU", "MB", "MC", "ME", "MI", "MN", "MO", "MS", "MT", "NA", "NO", "NU",
                        "OR", "PA", "PC", "PD", "PE", "PG", "PI", "PN", "PO", "PR", "PT", "PU",
                        "PV", "PZ", "RA", "RC", "RE", "RG", "RI", "RM", "RN", "RO", "SA", "SI",
                        "SO", "SP", "SR", "SS", "SU", "SV", "TA", "TE", "TN", "TO", "TP", "TR",
                        "TS", "TV", "UD", "VA", "VB", "VC", "VE", "VI", "VR", "VT", "VV", "Andria",
                        "Barletta", "Cesena", "Forli", "Pesaro", "Trani", "Urbino",
                    ],
                },
            ),
            (
                CountryCode::JM,
                Country {
                    code: CountryCode::JM,
                    name: "Jamaica",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::JP,
                Country {
                    code: CountryCode::JP,
                    name: "Japan",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::KZ,
                Country {
                    code: CountryCode::KZ,
                    name: "Kazakhstan",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::KE,
                Country {
                    code: CountryCode::KE,
                    name: "Kenya",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::KG,
                Country {
                    code: CountryCode::KG,
                    name: "Kyrgyzstan",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::LA,
                Country {
                    code: CountryCode::LA,
                    name: "Laos",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::LV,
                Country {
                    code: CountryCode::LV,
                    name: "Latvia",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::LS,
                Country {
                    code: CountryCode::LS,
                    name: "Lesotho",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::LI,
                Country {
                    code: CountryCode::LI,
                    name: "Liechtenstein",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::LT,
                Country {
                    code: CountryCode::LT,
                    name: "Lithuania",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::LU,
                Country {
                    code: CountryCode::LU,
                    name: "Luxembourg",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::MG,
                Country {
                    code: CountryCode::MG,
                    name: "Madagascar",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::MW,
                Country {
                    code: CountryCode::MW,
                    name: "Malawi",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::MY,
                Country {
                    code: CountryCode::MY,
                    name: "Malaysia",
                    subdivisions: &[
                        "JHR", "KDH", "KTN", "KUL", "LBN", "MLK", "NSN", "PHG", "PJY", "PLS",
                        "PNG", "PRK", "SBH", "SGR", "SWK", "TRG",
                    ],
                },
            ),
            (
                CountryCode::MV,
                Country {
                    code: CountryCode::MV,
                    name: "Maldives",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::MT,
                Country {
                    code: CountryCode::MT,
                    name: "Malta",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::MH,
                Country {
                    code: CountryCode::MH,
                    name: "Marshall Islands",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::MX,
                Country {
                    code: CountryCode::MX,
                    name: "Mexico",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::MD,
                Country {
                    code: CountryCode::MD,
                    name: "Moldova",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::MC,
                Country {
                    code: CountryCode::MC,
                    name: "Monaco",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::ME,
                Country {
                    code: CountryCode::ME,
                    name: "Montenegro",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::MA,
                Country {
                    code: CountryCode::MA,
                    name: "Morocco",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::MZ,
                Country {
                    code: CountryCode::MZ,
                    name: "Mozambique",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::NA,
                Country {
                    code: CountryCode::NA,
                    name: "Namibia",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::NL,
                Country {
                    code: CountryCode::NL,
                    name: "Netherlands",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::NZ,
                Country {
                    code: CountryCode::NZ,
                    name: "New Zealand",
                    subdivisions: &[
                        "AUK", "BOP", "CAN", "CIT", "GIS", "HKB", "MBH", "MWT", "NSN", "NTL",
                        "OTA", "STL", "TAS", "TKI", "WGN", "WKO", "WTC",
                    ],
                },
            ),
            (
                CountryCode::NI,
                Country {
                    code: CountryCode::NI,
                    name: "Nicaragua",
                    subdivisions: &[
                        "AN", "AS", "BO", "CA", "CI", "CO", "ES", "GR", "JI", "LE", "MD", "MN",
                        "MS", "MT", "NS", "RI", "SJ",
                    ],
                },
            ),
            (
                CountryCode::NG,
                Country {
                    code: CountryCode::NG,
                    name: "Nigeria",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::MP,
                Country {
                    code: CountryCode::MP,
                    name: "Northern Mariana Islands",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::MK,
                Country {
                    code: CountryCode::MK,
                    name: "North Macedonia",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::NO,
                Country {
                    code: CountryCode::NO,
                    name: "Norway",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::PK,
                Country {
                    code: CountryCode::PK,
                    name: "Pakistan",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::PA,
                Country {
                    code: CountryCode::PA,
                    name: "Panama",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::PG,
                Country {
                    code: CountryCode::PG,
                    name: "Papua New Guinea",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::PY,
                Country {
                    code: CountryCode::PY,
                    name: "Paraguay",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::PE,
                Country {
                    code: CountryCode::PE,
                    name: "Peru",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::PH,
                Country {
                    code: CountryCode::PH,
                    name: "Philippines",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::PL,
                Country {
                    code: CountryCode::PL,
                    name: "Poland",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::PT,
                Country {
                    code: CountryCode::PT,
                    name: "Portugal",
                    subdivisions: &[
                        "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12",
                        "13", "14", "15", "16", "17", "18", "20", "30",
                    ],
                },
            ),
            (
                CountryCode::PR,
                Country {
                    code: CountryCode::PR,
                    name: "Puerto Rico",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::RO,
                Country {
                    code: CountryCode::RO,
                    name: "Romania",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::RU,
                Country {
                    code: CountryCode::RU,
                    name: "Russia",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::SM,
                Country {
                    code: CountryCode::SM,
                    name: "San Marino",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::SA,
                Country {
                    code: CountryCode::SA,
                    name: "Saudi Arabia",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::RS,
                Country {
                    code: CountryCode::RS,
                    name: "Serbia",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::SG,
                Country {
                    code: CountryCode::SG,
                    name: "Singapore",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::SK,
                Country {
                    code: CountryCode::SK,
                    name: "Slovakia",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::SI,
                Country {
                    code: CountryCode::SI,
                    name: "Slovenia",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::ZA,
                Country {
                    code: CountryCode::ZA,
                    name: "South Africa",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::KR,
                Country {
                    code: CountryCode::KR,
                    name: "South Korea",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::ES,
                Country {
                    code: CountryCode::ES,
                    name: "Spain",
                    subdivisions: &[
                        "AN", "AR", "AS", "CB", "CE", "CL", "CM", "CN", "CT", "EX", "GA", "IB",
                        "MC", "MD", "ML", "NC", "PV", "RI", "VC",
                    ],
                },
            ),
            (
                CountryCode::SE,
                Country {
                    code: CountryCode::SE,
                    name: "Sweden",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::CH,
                Country {
                    code: CountryCode::CH,
                    name: "Switzerland",
                    subdivisions: &[
                        "AG", "AR", "AI", "BL", "BS", "BE", "FR", "GE", "GL", "GR", "JU", "LU",
                        "NE", "NW", "OW", "SG", "SH", "SZ", "SO", "TG", "TI", "UR", "VD", "VS",
                        "ZG", "ZH",
                    ],
                },
            ),
            (
                CountryCode::TW,
                Country {
                    code: CountryCode::TW,
                    name: "Taiwan",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::TZ,
                Country {
                    code: CountryCode::TZ,
                    name: "Tanzania",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::TH,
                Country {
                    code: CountryCode::TH,
                    name: "Thailand",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::TL,
                Country {
                    code: CountryCode::TL,
                    name: "Timor Leste",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::TO,
                Country {
                    code: CountryCode::TO,
                    name: "Tonga",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::TN,
                Country {
                    code: CountryCode::TN,
                    name: "Tunisia",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::TR,
                Country {
                    code: CountryCode::TR,
                    name: "Turkey",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::UA,
                Country {
                    code: CountryCode::UA,
                    name: "Ukraine",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::AE,
                Country {
                    code: CountryCode::AE,
                    name: "United Arab Emirates",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::GB,
                Country {
                    code: CountryCode::GB,
                    name: "United Kingdom",
                    subdivisions: &["ENG", "NIR", "SCT", "WLS"],
                },
            ),
            (
                CountryCode::UM,
                Country {
                    code: CountryCode::UM,
                    name: "United States Minor Outlying Islands",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::US,
                Country {
                    code: CountryCode::US,
                    name: "United States of America",
                    subdivisions: &[
                        "AK", "AL", "AR", "AS", "AZ", "CA", "CO", "CT", "DC", "DE", "FL", "GA",
                        "GU", "HI", "IA", "ID", "IL", "IN", "KS", "KY", "LA", "MA", "MD", "ME",
                        "MI", "MN", "MO", "MP", "MS", "MT", "NC", "ND", "NE", "NH", "NJ", "NM",
                        "NV", "NY", "OH", "OK", "OR", "PA", "PR", "RI", "SC", "SD", "TN", "TX",
                        "UM", "UT", "VA", "VI", "VT", "WA", "WI", "WV", "WY",
                    ],
                },
            ),
            (
                CountryCode::UY,
                Country {
                    code: CountryCode::UY,
                    name: "Uruguay",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::UZ,
                Country {
                    code: CountryCode::UZ,
                    name: "Uzbekistan",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::VU,
                Country {
                    code: CountryCode::VU,
                    name: "Vanuatu",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::VA,
                Country {
                    code: CountryCode::VA,
                    name: "Vatican City",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::VE,
                Country {
                    code: CountryCode::VE,
                    name: "Venezuela",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::VN,
                Country {
                    code: CountryCode::VN,
                    name: "Vietnam",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::VI,
                Country {
                    code: CountryCode::VI,
                    name: "Virgin Islands (U.S.)",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::ZM,
                Country {
                    code: CountryCode::ZM,
                    name: "Zambia",
                    subdivisions: &[],
                },
            ),
            (
                CountryCode::ZW,
                Country {
                    code: CountryCode::ZW,
                    name: "Zimbabwe",
                    subdivisions: &[],
                },
            ),
        ])
    }
}
