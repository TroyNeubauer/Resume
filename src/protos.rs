use chrono::naive::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Resume {
    pub name: String,
    pub email: String,
    pub source_code: String,
    pub host_link: String,
    pub phone_number: PhoneNumber,
    pub location: String,
    pub parsed_location: Option<Location>,
    pub locations: Vec<Location>,
    pub linkedin_profile: String,
    pub github_profile: String,
    pub about_me: String,
    pub education: Vec<Education>,
    pub experience: Vec<Experience>,
    pub skills: Vec<SkillCategory>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PhoneNumber {
    pub country_code: u32,
    pub number: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Location {
    pub city: String,
    pub state: String,
    pub country: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DateRange {
    pub start: NaiveDate,
    pub end: Option<NaiveDate>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Education {
    pub institution: String,
    pub major: String,
    pub description: String,
    pub period: DateRange,
    pub degree: DegreeKind,
    pub location: String,
    pub parsed_location: Option<Location>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum DegreeKind {
    Bachelors,
    Masters,
    NonDegree,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Experience {
    #[serde(default)]
    pub archived: bool,
    pub title: String,
    pub organization: String,
    pub website: Option<String>,
    pub period: DateRange,
    pub location: String,
    pub parsed_location: Option<Location>,
    /// Global tasks for all duties
    #[serde(default)]
    pub tags: Vec<String>,
    pub duty: Vec<Duty>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Duty {
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SkillCategory {
    pub category: String,
    pub tags: Vec<String>,
}
