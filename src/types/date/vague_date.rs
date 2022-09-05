use super::{date_type::VagueDateType, recorder_date::RecorderDate};
use serde::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VagueDate {
    start: RecorderDate,
    end: Option<RecorderDate>,
    date_type: VagueDateType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_de_serialize() {
        let json = r#"{"start":1,"end":null,"date_type":"DD"}"#;

        // Deserialize, then re-serialize
        let date: VagueDate = serde_json::from_str(json).unwrap();
        assert_eq!(json, serde_json::to_string(&date).unwrap());
    }
}
