use chrono::offset::Utc;
use chrono::DateTime;
use serde::*;

use crate::types::date::date_type::VagueDateType;
use crate::types::date::recorder_date::RecorderDate;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TaxonDetermination {
    taxon_determination_key: String,
    taxon_list_item_key: String,
    taxon_occurrence_key: String,
    vague_date_start: Option<RecorderDate>,
    vague_date_end: Option<RecorderDate>,
    vague_date_type: Option<VagueDateType>,
    comment: Option<String>,
    preferred: bool,
    determiner: String,
    determination_type_key: String,
    determiner_role_key: String,
    entered_by: String,
    entry_date: DateTime<Utc>,
    changed_by: Option<String>,
    changed_date: Option<DateTime<Utc>>,
    source_key: Option<String>,
    custodian: Option<String>,
}
