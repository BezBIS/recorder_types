use chrono::{Duration, NaiveDate};
use serde::{Deserialize, Serialize};

const BASE_DATE: (i32, u32, u32) = (1899, 12, 30);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "i32")]
#[serde(into = "i32")]
pub struct RecorderDate(NaiveDate);

impl From<i32> for RecorderDate {
    fn from(days: i32) -> Self {
        let base_date = NaiveDate::from_ymd(BASE_DATE.0, BASE_DATE.1, BASE_DATE.2);
        let date = base_date + Duration::days(days.into());

        RecorderDate(date)
    }
}

impl Into<i32> for RecorderDate {
    fn into(self) -> i32 {
        let base_date = NaiveDate::from_ymd(BASE_DATE.0, BASE_DATE.1, BASE_DATE.2);
        (self.0 - base_date).num_days().try_into().unwrap()
    }
}

impl From<NaiveDate> for RecorderDate {
    fn from(date: NaiveDate) -> Self {
        RecorderDate(date)
    }
}

impl Into<NaiveDate> for RecorderDate {
    fn into(self) -> NaiveDate {
        self.0
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    struct TestDate {
        date: (i32, u32, u32),
        days: i32,
    }

    const SINGLE_DAY: TestDate = TestDate {
        date: (1899, 12, 31),
        days: 1,
    };
    const HASTINGS: TestDate = TestDate {
        date: (1066, 10, 14),
        days: -304324,
    };
    const BRUTUS: TestDate = TestDate {
        date: (-44, 03, 15),
        days: -709956,
    };
    const ORWELL: TestDate = TestDate {
        date: (1950, 01, 21),
        days: 18284,
    };

    #[test]
    fn can_parse_date_from_int() {
        // Add single day.
        let (year, month, day) = SINGLE_DAY.date;
        let date: NaiveDate = RecorderDate::from(SINGLE_DAY.days).into();
        assert_eq!(date, NaiveDate::from_ymd(year, month, day));

        // The French Invasion
        let (year, month, day) = HASTINGS.date;
        let date: NaiveDate = RecorderDate::from(HASTINGS.days).into();
        assert_eq!(date, NaiveDate::from_ymd(year, month, day));

        // Et tu Brute
        let (year, month, day) = BRUTUS.date;
        let date: NaiveDate = RecorderDate::from(BRUTUS.days).into();
        assert_eq!(date, NaiveDate::from_ymd(year, month, day));

        // 1984
        let (year, month, day) = ORWELL.date;
        let date: NaiveDate = RecorderDate::from(ORWELL.days).into();
        assert_eq!(date, NaiveDate::from_ymd(year, month, day));
    }

    #[test]
    fn can_build_int_from_date() {
        // Add single day.
        let (year, month, day) = SINGLE_DAY.date;
        let days: i32 = RecorderDate::from(NaiveDate::from_ymd(year, month, day)).into();
        assert_eq!(days, SINGLE_DAY.days);

        // The French Invasion
        let (year, month, day) = HASTINGS.date;
        let days: i32 = RecorderDate::from(NaiveDate::from_ymd(year, month, day)).into();
        assert_eq!(days, HASTINGS.days);

        // Et tu Brute
        let (year, month, day) = BRUTUS.date;
        let days: i32 = RecorderDate::from(NaiveDate::from_ymd(year, month, day)).into();
        assert_eq!(days, BRUTUS.days);

        // 1984
        let (year, month, day) = ORWELL.date;
        let days: i32 = RecorderDate::from(NaiveDate::from_ymd(year, month, day)).into();
        assert_eq!(days, ORWELL.days);
    }

    #[test]
    fn can_de_serialize() {
        let json = r#"1"#;

        // Deserialize, then re-serialize
        let date: RecorderDate = serde_json::from_str(json).unwrap();
        assert_eq!(json, serde_json::to_string(&date).unwrap());
    }
}
