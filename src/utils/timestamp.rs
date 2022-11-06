use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Timestamp {
    pub utc: DateTime<Utc>,
}
impl Timestamp {
    pub fn new(utc: DateTime<Utc>) -> Self {
        Self { utc }
    }
    pub fn get_smh(&self, dt: &DateTime<Utc>) -> (i64, i64, i64) {
        let diff = dt.timestamp() - self.utc.timestamp();
        let s = diff % 60;
        let m = diff / 60 % 60;
        let h = diff / 3600;
        (s, m, h)
    }
    pub fn to_string_with_dt(&self, dt: &DateTime<Utc>) -> String {
        match self.get_smh(dt) {
            (s, m, h) if h > 0 => format!("{}h {}m {}s ago", h, m, s),
            (s, m, 0) if m > 0 => format!("{}m {}s ago", m, s),
            (s, 0, 0) if s > 5 => format!("{}s ago", s),
            _ => "Just now".to_string(),
        }
    }
}
impl fmt::Display for Timestamp {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(&self.to_string_with_dt(&Utc::now()))?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{DateTime, Timestamp, Utc};
    use chrono::NaiveDateTime;
    use regex::Regex;

    #[test]
    fn get_smh() {
        const SECS: i64 = 3;
        const MINS: i64 = 2;
        const HOURS: i64 = 1;
        let ts = Timestamp::new(Utc::now());
        let offset = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(
                ts.utc.timestamp() + (SECS + MINS * 60 + HOURS * 60 * 60),
                0,
            ),
            Utc,
        );
        assert_eq!(ts.get_smh(&offset), (SECS, MINS, HOURS));
    }

    #[test]
    fn to_string_with_dt() {
        let ts = Timestamp::new(Utc::now());
        assert_eq!(ts.to_string_with_dt(&ts.utc), "Just now");

        let offset_7_secs = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(ts.utc.timestamp() + 7, 0),
            Utc,
        );
        assert_eq!(ts.to_string_with_dt(&offset_7_secs), "7s ago");

        let offset_5_mins = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(ts.utc.timestamp() + 5 * 60, 0),
            Utc,
        );
        assert_eq!(ts.to_string_with_dt(&offset_5_mins), "5m 0s ago");

        let offset_3_hours_4_mins_1_sec = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(ts.utc.timestamp() + (1 + 4 * 60 + 3 * 60 * 60), 0),
            Utc,
        );
        assert_eq!(
            ts.to_string_with_dt(&offset_3_hours_4_mins_1_sec),
            "3h 4m 1s ago"
        );
    }

    #[test]
    fn timestamp_fmt() {
        let ts = Timestamp::new(Utc::now());
        let re = Regex::new(r"^(((\d+h )?\d+m )?\d+s ago|Just now)$").unwrap();
        assert!(re.is_match(&ts.to_string()));
    }
}
