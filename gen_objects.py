import sys
import holidays
import json
from concurrent.futures import ProcessPoolExecutor

# Define the range of years to include in holiday data
years = list(range(2000, 2051))  # Inclusive: 2000 to 2050

# List of countries: (Country Name, ISO Code, Subdivisions)
countries = [
    ("Albania", "AL", []),
    ("Algeria", "DZ", []),
    ("American Samoa", "AS", []),
    ("Andorra", "AD", ["02", "03", "04", "05", "06", "07", "08"]),
    ("Angola", "AO", []),
    ("Argentina", "AR", []),
    ("Armenia", "AM", []),
    ("Aruba", "AW", []),
    ("Australia", "AU", ["ACT", "NSW", "NT", "QLD", "SA", "TAS", "VIC", "WA"]),
    ("Austria", "AT", ["1", "2", "3", "4", "5", "6", "7", "8", "9"]),
    ("Azerbaijan", "AZ", []),
    ("Bahamas", "BS", []),
    ("Bahrain", "BH", []),
    ("Bangladesh", "BD", []),
    ("Barbados", "BB", []),
    ("Belarus", "BY", []),
    ("Belgium", "BE", []),
    ("Belize", "BZ", []),
    ("Bolivia", "BO", ["B", "C", "H", "L", "N", "O", "P", "S", "T"]),
    ("Bosnia and Herzegovina", "BA", ["BIH", "BRC", "SRP"]),
    ("Botswana", "BW", []),
    ("Brazil", "BR", ["AC", "AL", "AM", "AP", "BA", "CE", "DF", "ES", "GO", "MA", "MG", "MS", "MT", "PA", "PB", "PE", "PI", "PR", "RJ", "RN", "RO", "RR", "RS", "SC", "SE", "SP", "TO"]),
    ("Brunei", "BN", []),
    ("Bulgaria", "BG", []),
    ("Burkina Faso", "BF", []),
    ("Burundi", "BI", []),
    ("Cambodia", "KH", []),
    ("Cameroon", "CM", []),
    ("Canada", "CA", ["AB", "BC", "MB", "NB", "NL", "NS", "NT", "NU", "ON", "PE", "QC", "SK", "YT"]),
    ("Chad", "TD", []),
    ("Chile", "CL", ["AI", "AN", "AP", "AR", "AT", "BI", "CO", "LI", "LL", "LR", "MA", "ML", "NB", "RM", "TA", "VS"]),
    ("China", "CN", []),
    ("Colombia", "CO", []),
    ("Costa Rica", "CR", []),
    ("Croatia", "HR", []),
    ("Cuba", "CU", []),
    ("Curacao", "CW", []),
    ("Cyprus", "CY", []),
    ("Czechia", "CZ", []),
    ("Denmark", "DK", []),
    ("Djibouti", "DJ", []),
    ("Dominican Republic", "DO", []),
    ("Ecuador", "EC", []),
    ("Egypt", "EG", []),
    ("El Salvador", "SV", ["AH", "CA", "CH", "CU", "LI", "MO", "PA", "SA", "SM", "SO", "SS", "SV", "UN", "US"]),
    ("Estonia", "EE", []),
    ("Eswatini", "SZ", []),
    ("Ethiopia", "ET", []),
    ("Finland", "FI", []),
    ("France", "FR", ["BL", "GES", "GP", "GY", "MF", "MQ", "NC", "PF", "RE", "WF", "YT"]),
    ("Gabon", "GA", []),
    ("Georgia", "GE", []),
    ("Germany", "DE", ["BB", "BE", "BW", "BY", "BYP", "HB", "HE", "HH", "MV", "NI", "NW", "RP", "SH", "SL", "SN", "ST", "TH"]),
    ("Greece", "GR", []),
    ("Guam", "GU", []),
    ("Guatemala", "GT", []),
    ("Honduras", "HN", []),
    ("Hong Kong", "HK", []),
    ("Hungary", "HU", []),
    ("Iceland", "IS", []),
    ("India", "IN", ["AN", "AP", "AR", "AS", "BR", "CG", "CH", "DD", "DH", "DL", "GA", "GJ", "HP", "HR", "JH", "JK", "KA", "KL", "LA", "LD", "MH", "ML", "MN", "MP", "MZ", "NL", "OR", "PB", "PY", "RJ", "SK", "TN", "TR", "TS", "UK", "UP", "WB"]),
    ("Indonesia", "ID", []),
    ("Iran", "IR", []),
    ("Ireland", "IE", []),
    ("Isle of Man", "IM", []),
    ("Israel", "IL", []),
    ("Italy", "IT", ["AG", "AL", "AN", "AO", "AP", "AQ", "AR", "AT", "AV", "BA", "BG", "BI", "BL", "BN", "BO", "BR", "BS", "BT", "BZ", "CA", "CB", "CE", "CH", "CL", "CN", "CO", "CR", "CS", "CT", "CZ", "EN", "FC", "FE", "FG", "FI", "FM", "FR", "GE", "GO", "GR", "IM", "IS", "KR", "LC", "LE", "LI", "LO", "LT", "LU", "MB", "MC", "ME", "MI", "MN", "MO", "MS", "MT", "NA", "NO", "NU", "OR", "PA", "PC", "PD", "PE", "PG", "PI", "PN", "PO", "PR", "PT", "PU", "PV", "PZ", "RA", "RC", "RE", "RG", "RI", "RM", "RN", "RO", "SA", "SI", "SO", "SP", "SR", "SS", "SU", "SV", "TA", "TE", "TN", "TO", "TP", "TR", "TS", "TV", "UD", "VA", "VB", "VC", "VE", "VI", "VR", "VT", "VV", "Andria", "Barletta", "Cesena", "Forli", "Pesaro", "Trani", "Urbino"]),
    ("Jamaica", "JM", []),
    ("Japan", "JP", []),
    ("Kazakhstan", "KZ", []),
    ("Kenya", "KE", []),
    ("Kyrgyzstan", "KG", []),
    ("Laos", "LA", []),
    ("Latvia", "LV", []),
    ("Lesotho", "LS", []),
    ("Liechtenstein", "LI", []),
    ("Lithuania", "LT", []),
    ("Luxembourg", "LU", []),
    ("Madagascar", "MG", []),
    ("Malawi", "MW", []),
    ("Malaysia", "MY", ["JHR", "KDH", "KTN", "KUL", "LBN", "MLK", "NSN", "PHG", "PJY", "PLS", "PNG", "PRK", "SBH", "SGR", "SWK", "TRG"]),
    ("Maldives", "MV", []),
    ("Malta", "MT", []),
    ("Marshall Islands", "MH", []),
    ("Mexico", "MX", []),
    ("Moldova", "MD", []),
    ("Monaco", "MC", []),
    ("Montenegro", "ME", []),
    ("Morocco", "MA", []),
    ("Mozambique", "MZ", []),
    ("Namibia", "NA", []),
    ("Netherlands", "NL", []),
    ("New Zealand", "NZ", ["AUK", "BOP", "CAN", "CIT", "GIS", "HKB", "MBH", "MWT", "NSN", "NTL", "OTA", "STL", "TAS", "TKI", "WGN", "WKO", "WTC"]),
    ("Nicaragua", "NI", ["AN", "AS", "BO", "CA", "CI", "CO", "ES", "GR", "JI", "LE", "MD", "MN", "MS", "MT", "NS", "RI", "SJ"]),
    ("Nigeria", "NG", []),
    ("Northern Mariana Islands", "MP", []),
    ("North Macedonia", "MK", []),
    ("Norway", "NO", []),
    ("Pakistan", "PK", []),
    ("Panama", "PA", []),
    ("Papua New Guinea", "PG", []),
    ("Paraguay", "PY", []),
    ("Peru", "PE", []),
    ("Philippines", "PH", []),
    ("Poland", "PL", []),
    ("Portugal", "PT", ["01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12", "13", "14", "15", "16", "17", "18", "20", "30"]),
    ("Puerto Rico", "PR", []),
    ("Romania", "RO", []),
    ("Russia", "RU", []),
    ("San Marino", "SM", []),
    ("Saudi Arabia", "SA", []),
    ("Serbia", "RS", []),
    ("Singapore", "SG", []),
    ("Slovakia", "SK", []),
    ("Slovenia", "SI", []),
    ("South Africa", "ZA", []),
    ("South Korea", "KR", []),
    ("Spain", "ES", ["AN", "AR", "AS", "CB", "CE", "CL", "CM", "CN", "CT", "EX", "GA", "IB", "MC", "MD", "ML", "NC", "PV", "RI", "VC"]),
    ("Sweden", "SE", []),
    ("Switzerland", "CH", ["AG", "AR", "AI", "BL", "BS", "BE", "FR", "GE", "GL", "GR", "JU", "LU", "NE", "NW", "OW", "SG", "SH", "SZ", "SO", "TG", "TI", "UR", "VD", "VS", "ZG", "ZH"]),
    ("Taiwan", "TW", []),
    ("Tanzania", "TZ", []),
    ("Thailand", "TH", []),
    ("Timor Leste", "TL", []),
    ("Tonga", "TO", []),
    ("Tunisia", "TN", []),
    ("Turkey", "TR", []),
    ("Ukraine", "UA", []),
    ("United Arab Emirates", "AE", []),
    ("United Kingdom", "GB", ["ENG", "NIR", "SCT", "WLS"]),
    ("United States Minor Outlying Islands", "UM", []),
    ("United States of America", "US", ["AK", "AL", "AR", "AS", "AZ", "CA", "CO", "CT", "DC", "DE", "FL", "GA", "GU", "HI", "IA", "ID", "IL", "IN", "KS", "KY", "LA", "MA", "MD", "ME", "MI", "MN", "MO", "MP", "MS", "MT", "NC", "ND", "NE", "NH", "NJ", "NM", "NV", "NY", "OH", "OK", "OR", "PA", "PR", "RI", "SC", "SD", "TN", "TX", "UM", "UT", "VA", "VI", "VT", "WA", "WI", "WV", "WY"]),
    ("Uruguay", "UY", []),
    ("Uzbekistan", "UZ", []),
    ("Vanuatu", "VU", []),
    ("Vatican City", "VA", []),
    ("Venezuela", "VE", []),
    ("Vietnam", "VN", []),
    ("Virgin Islands (U.S.)", "VI", []),
    ("Zambia", "ZM", []),
    ("Zimbabwe", "ZW", [])
]

# Helper to fetch holiday data for a country or subdivision
def get_holidays(country_code, subdiv=None):
    return {
        str(date): name
        for date, name in holidays.country_holidays(
            country_code, subdiv=subdiv, language="en_US", years=years
        ).items()
    }

# Worker function to process one country
def process_country(args):
    country_name, country_code, subdivisions = args
    country_holidays = {}

    try:
        # National holidays
        country_holidays["National"] = get_holidays(country_code)

        # Subdivision holidays
        for subdiv in subdivisions:
            key = f"_{subdiv}" if subdiv[0].isdigit() else subdiv
            if key == "National":
                return (country_code, {
                    "error": f"Conflict: subdivision named 'National' in country {country_code}"
                })

            subdiv_holidays = get_holidays(country_code, subdiv=subdiv)
            if subdiv_holidays:
                country_holidays[key] = subdiv_holidays

    except Exception as e:
        return (country_code, {"error": str(e)})

    return (country_code, country_holidays)

# Run in parallel
def main():
    all_data = {}

    with ProcessPoolExecutor() as executor:
        results = executor.map(process_country, countries)
        for country_code, data in results:
            if "error" in data:
                print(f"Error processing {country_code}: {data['error']}", file=sys.stderr)
                sys.exit(1)
            all_data[country_code] = data

    # Output JSON
    json.dump(all_data, sys.stdout, ensure_ascii=False)

if __name__ == "__main__":
    main()