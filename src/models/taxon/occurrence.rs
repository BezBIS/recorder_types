use chrono::offset::Utc;
use chrono::DateTime;
use serde::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TaxonOccurrence {
    taxon_occurrence_key: String,
    comment: Option<String>,
    zero_abundance: bool,
    confidential: bool,
    verified: i16,
    checked: bool,
    checked_by: Option<String>,
    checked_date: Option<DateTime<Utc>>,
    surveyors_ref: Option<String>,
    provenance: Option<String>,
    sample_key: String,
    substrate_key: Option<String>,
    record_type_key: Option<String>,
    entered_by: String,
    entry_date: DateTime<Utc>,
    changed_by: Option<String>,
    changed_date: Option<DateTime<Utc>>,
    custodian: Option<String>,
}