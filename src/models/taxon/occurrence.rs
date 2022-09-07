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

impl TaxonOccurrence {
    pub fn key(&self) -> &str {
        &self.taxon_occurrence_key
    }

    pub fn comment(&self) -> Option<&String> {
        self.comment.as_ref()
    }

    pub fn zero_abundance(&self) -> bool {
        self.zero_abundance
    }

    pub fn confidential(&self) -> bool {
        self.confidential
    }

    pub fn verified(&self) -> i16 {
        self.verified
    }

    pub fn checked(&self) -> bool {
        self.checked
    }

    pub fn checked_by(&self) -> Option<&String> {
        self.checked_by.as_ref()
    }

    pub fn checked_date(&self) -> Option<DateTime<Utc>> {
        self.checked_date
    }

    pub fn surveyors_ref(&self) -> Option<&String> {
        self.surveyors_ref.as_ref()
    }

    pub fn provenance(&self) -> Option<&String> {
        self.provenance.as_ref()
    }

    pub fn sample_key(&self) -> &str {
        self.sample_key.as_ref()
    }

    pub fn substrate_key(&self) -> Option<&String> {
        self.substrate_key.as_ref()
    }

    pub fn record_type_key(&self) -> Option<&String> {
        self.record_type_key.as_ref()
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

    pub fn custodian(&self) -> Option<&String> {
        self.custodian.as_ref()
    }
}
