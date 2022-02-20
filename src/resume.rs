use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Resume {
    // message fields
    pub name: String,
    pub email: String,
    pub source_code: String,
    pub host_link: String,
    pub phone_number: PhoneNumber,
    pub location: Location,
    pub linkedin_profile: String,
    pub github_profile: String,
    pub about_me: String,
    pub education: Vec<Education>,
    pub experience: Vec<Experience>,
    pub skills: Vec<SkillCategory>,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct PhoneNumber {
    pub country_code: u32,
    pub number: u64,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Location {
    pub city: String,
    pub state: String,
    pub country: String,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Education {
    pub institution: String,
    pub major: String,
    pub description: String,
    pub period: DateRange,
    pub degree: EducationDegree,
    pub location: Location,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum EducationDegree {
    Bachelors,
    Masters,
    NonDegree,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub title: String,
    pub organization: String,
    pub website: Option<String>,
    pub period: DateRange,
    pub location: Location,
    pub designation: Designation,
    pub chapter: Chapter,
    pub duty: Vec<Duty>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum Designation {
    Club,
    Work,
    Project,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum Chapter {
    HighSchool,
    College,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Duty {
    pub description: String,
    pub tags: Option<Vec<String>>,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SkillCategory {
    pub category: String,
    pub tags: Vec<String>,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start: Option<Date>,
    pub end: Option<Date>,
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
pub struct Date(pub chrono::NaiveDate);

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        use serde_yaml::Value;

        let value = Value::deserialize(deserializer)?; 
        let string = match value {
            Value::String(s) => s,
            _ => panic!("Expected string, got {:?}", value),
        };
        let date = Date::from_str(&string).map_err(Error::custom)?;

        Ok(date)
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = self.0.to_string();
        serializer.serialize_str(&s)
    }
}

impl FromStr for Date {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(chrono::NaiveDate::from_str(s)?))
    }
}
