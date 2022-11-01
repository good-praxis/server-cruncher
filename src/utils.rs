use chrono::{DateTime, Utc};
use hcloud::models::server::Server;

const SECOND: i64 = 1000;
const MINUTE: i64 = SECOND * 60;
const HOUR: i64 = MINUTE * 60;

#[derive(Debug, Clone)]
pub enum Data {
    Servers(Vec<Server>),
}

#[derive(Debug, Clone)]
pub struct RemoteData {
    pub data: Data,
    pub updated_at: DateTime<Utc>,
    seconds_since: Option<i64>,
    minutes_since: Option<i64>,
    hours_since: Option<i64>,
    pub label: String,
}

impl RemoteData {
    pub fn new(data: Data) -> Self {
        Self {
            data,
            updated_at: Utc::now(),
            seconds_since: None,
            minutes_since: None,
            hours_since: None,
            label: "Just now".to_string(),
        }
    }

    fn update_inner_timing(&mut self) -> &mut Self {
        let mut diff = Utc::now().timestamp_millis() - self.updated_at.timestamp_millis();

        self.hours_since = Some(diff / HOUR);
        diff %= HOUR;

        self.minutes_since = Some(diff / MINUTE);
        diff %= MINUTE;

        self.seconds_since = Some(diff / SECOND);

        self
    }

    pub fn generate_update_label(&mut self) -> &mut Self {
        self.update_inner_timing();

        self.label = match self {
            Self {
                hours_since: Some(h),
                minutes_since: Some(m),
                seconds_since: Some(s),
                ..
            } if h > &mut 0 => format!("{}h {}m {}s ago", h, m, s),
            Self {
                hours_since: Some(0),
                minutes_since: Some(m),
                seconds_since: Some(s),
                ..
            } if m > &mut 0 => format!("{}m {}s ago", m, s),
            Self {
                hours_since: Some(0),
                minutes_since: Some(0),
                seconds_since: Some(s),
                ..
            } if s > &mut 5 => format!("{}s ago", s),
            _ => "Just now".to_string(),
        };

        self
    }
}
