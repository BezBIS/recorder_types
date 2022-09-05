use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Display, PartialEq, Eq, Clone, Copy, EnumString, Deserialize)]
#[serde(try_from = "&str")]
#[serde(into = "String")]
pub enum VagueDateType {
    #[strum(to_string = "D")]
    DayDate,
    #[strum(to_string = "DD")]
    DayDateRange,
    #[strum(to_string = "O")]
    MonthDate,
    #[strum(to_string = "OO")]
    MonthDateRange,
    #[strum(to_string = "P")]
    SeasonDate,
    #[strum(to_string = "Y")]
    Year,
    #[strum(to_string = "YY")]
    YearRange,
    #[strum(to_string = "Y-")]
    YearOpenEnded,
    #[strum(to_string = "-Y")]
    YearOpenStart,
    #[strum(to_string = "M")]
    Month,
    #[strum(to_string = "S")]
    Season,
    #[strum(to_string = "U")]
    Unknown,
    #[strum(to_string = "C")]
    Century,
    #[strum(to_string = "CC")]
    CenturyRange,
    #[strum(to_string = "C-")]
    CenturyOpenEnded,
    #[strum(to_string = "-C")]
    CenturyOpenStart,
}

impl Serialize for VagueDateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use phf::phf_map;
    use std::str::FromStr;
    use strum::ParseError;

    static VALID_TYPES: phf::Map<&'static str, VagueDateType> = phf_map! {
        "D" => VagueDateType::DayDate,
        "DD" => VagueDateType::DayDateRange,
        "O" => VagueDateType::MonthDate,
        "OO" => VagueDateType::MonthDateRange,
        "P" => VagueDateType::SeasonDate,
        "Y" => VagueDateType::Year,
        "YY" => VagueDateType::YearRange,
        "Y-" => VagueDateType::YearOpenEnded,
        "-Y" => VagueDateType::YearOpenStart,
        "M" => VagueDateType::Month,
        "S" => VagueDateType::Season,
        "U" => VagueDateType::Unknown,
        "C" => VagueDateType::Century,
        "CC" => VagueDateType::CenturyRange,
        "C-" => VagueDateType::CenturyOpenEnded,
        "-C" => VagueDateType::CenturyOpenStart
    };

    #[test]
    fn can_parse_datetype_from_string() {
        // Test all valid types.
        for (string, date_type) in &VALID_TYPES {
            assert_eq!(VagueDateType::from_str(string), Ok(*date_type));
        }

        // And something incorrect.
        assert_eq!(
            VagueDateType::from_str("SU"),
            Err(ParseError::VariantNotFound)
        );
    }

    #[test]
    fn can_serialize() {
        // Test all valid types.
        for (string, date_type) in &VALID_TYPES {
            let string = &format!("\"{}\"", string);
            // Deserialize
            assert_eq!(
                &serde_json::from_str::<VagueDateType>(string).unwrap(),
                date_type
            );

            // Serialize
            assert_eq!(string, &serde_json::to_string(&date_type).unwrap());
        }
    }
}
