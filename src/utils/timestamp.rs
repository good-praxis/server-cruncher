use chrono::{DateTime, Utc};
use std::fmt;

const SECOND: i64 = 1000;
const MINUTE: i64 = SECOND * 60;
const HOUR: i64 = MINUTE * 60;

#[derive(Debug, Clone)]
pub struct Timestamp {
    pub utc: DateTime<Utc>,
}
impl Timestamp {
    pub fn new(utc: DateTime<Utc>) -> Self {
        Self { utc }
    }
    pub fn get_smh(&self) -> (i64, i64, i64) {
        let diff = Utc::now().timestamp_millis() - self.utc.timestamp_millis();
        let s = diff / SECOND % 60;
        let m = diff / MINUTE % 60;
        let h = diff / HOUR;
        (s, m, h)
    }
}
impl fmt::Display for Timestamp {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self.get_smh() {
            (s, m, h) if h > 0 => format!("{}h {}m {}s ago", h, m, s),
            (s, m, 0) if m > 0 => format!("{}m {}s ago", m, s),
            (s, 0, 0) if s > 5 => format!("{}s ago", s),
            _ => "Just now".to_string(),
        };
        fmt.write_str(&str)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{DateTime, Timestamp, Utc};
    use chrono::NaiveDateTime;

    #[test]
    fn timestamp_display() {
        let now = Timestamp::new(Utc::now());
        assert_eq!(&now.to_string(), "Just now");

        let seven_seconds_ago = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(Utc::now().timestamp() - 7, 0),
            Utc,
        );
        let seconds_ago = Timestamp::new(seven_seconds_ago);
        assert_eq!(&seconds_ago.to_string(), "7s ago");

        let five_minutes_ago = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(Utc::now().timestamp() - 5 * 60, 0),
            Utc,
        );
        let minutes_ago = Timestamp::new(five_minutes_ago);
        assert_eq!(&minutes_ago.to_string(), "5m 0s ago");

        let three_hours_four_minutes_and_one_second_ago = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(Utc::now().timestamp() - (1 + 4 * 60 + 3 * 60 * 60), 0),
            Utc,
        );
        let some_time_ago = Timestamp::new(three_hours_four_minutes_and_one_second_ago);
        assert_eq!(&some_time_ago.to_string(), "3h 4m 1s ago");
    }
}
