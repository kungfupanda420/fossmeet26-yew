use serde::{Deserialize, Serialize};

/// Social media link for speakers
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocialLink {
    pub platform: String,
    pub url: String,
}

/// Speaker information
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Speaker {
    pub id: String,
    pub name: String,
    pub talk_title: String,
    pub bio: String,
    pub image_url: String,
    pub socials: Vec<SocialLink>,
}

/// Workshop information
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workshop {
    pub id: String,
    pub title: String,
    pub description: String,
    pub logo_url: String,
    pub date: String,
    pub duration: String,
    pub speaker_name: String,
    pub speaker_image_url: String,
    pub registration_link: Option<String>,
    pub prerequisites: Vec<String>,
}

/// Event type for schedule
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EventType {
    Talk,
    Workshop,
    Break,
    Keynote,
    Panel,
    Networking,
}

impl EventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EventType::Talk => "talk",
            EventType::Workshop => "workshop",
            EventType::Break => "break",
            EventType::Keynote => "keynote",
            EventType::Panel => "panel",
            EventType::Networking => "networking",
        }
    }
}

/// Schedule event
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleEvent {
    pub id: String,
    pub title: String,
    pub time: String,
    pub duration: String,
    pub event_type: EventType,
    pub speaker: Option<Speaker>,
    pub location: Option<String>,
}

/// Schedule day
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleDay {
    pub date: String,
    pub day_label: String,
    pub events: Vec<ScheduleEvent>,
}

/// Sponsor tier
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SponsorTier {
    Platinum,
    Gold,
    Silver,
    Community,
}

/// Sponsor information
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sponsor {
    pub name: String,
    pub logo_url: String,
    pub website: String,
    pub tier: SponsorTier,
}

/// Navigation item
#[derive(Clone, Debug, PartialEq)]
pub struct NavItem {
    pub label: &'static str,
    pub route: &'static str,
    pub external: bool,
}

/// Route enum for the application
#[derive(Clone, PartialEq, Debug)]
pub enum Route {
    Home,
    Speakers,
    Workshops,
    Schedule,
    About,
    NotFound,
}
