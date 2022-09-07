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

impl TaxonDetermination {
    pub fn taxon_determination_key(&self) -> &str {
        self.taxon_determination_key.as_ref()
    }

    pub fn taxon_list_item_key(&self) -> &str {
        self.taxon_list_item_key.as_ref()
    }

    pub fn taxon_occurrence_key(&self) -> &str {
        self.taxon_occurrence_key.as_ref()
    }

    pub fn vague_date_start(&self) -> Option<&RecorderDate> {
        self.vague_date_start.as_ref()
    }

    pub fn vague_date_end(&self) -> Option<&RecorderDate> {
        self.vague_date_end.as_ref()
    }

    pub fn vague_date_type(&self) -> Option<VagueDateType> {
        self.vague_date_type
    }

    pub fn comment(&self) -> Option<&String> {
        self.comment.as_ref()
    }

    pub fn preferred(&self) -> bool {
        self.preferred
    }

    pub fn determiner(&self) -> &str {
        self.determiner.as_ref()
    }

    pub fn determination_type_key(&self) -> &str {
        self.determination_type_key.as_ref()
    }

    pub fn determiner_role_key(&self) -> &str {
        self.determiner_role_key.as_ref()
    }

    pub fn entered_by(&self) -> &str {
        self.entered_by.as_ref()
    }

    pub fn entry_date(&self) -> DateTime<Utc> {
        self.entry_date
    }

    pub fn changed_by(&self) -> Option<&String> {
        self.changed_by.as_ref()
    }

    pub fn changed_date(&self) -> Option<DateTime<Utc>> {
        self.changed_date
    }

    pub fn source_key(&self) -> Option<&String> {
        self.source_key.as_ref()
    }

    pub fn custodian(&self) -> Option<&String> {
        self.custodian.as_ref()
    }
}
