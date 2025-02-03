use chrono::{DateTime, Utc};

pub fn time_ago(date: &DateTime<Utc>) -> askama::Result<String> {
    let now = Utc::now();
    let duration = now.signed_duration_since(*date);

    let time_ago = if duration.num_days() > 365 {
        let years = duration.num_days() / 365;
        if years == 1 {
            "1 yıl önce".to_string()
        } else {
            format!("{} yıl önce", years)
        }
    } else if duration.num_days() > 30 {
        let months = duration.num_days() / 30;
        if months == 1 {
            "1 ay önce".to_string()
        } else {
            format!("{} ay önce", months)
        }
    } else if duration.num_days() > 7 {
        let weeks = duration.num_days() / 7;
        if weeks == 1 {
            "1 hafta önce".to_string()
        } else {
            format!("{} hafta önce", weeks)
        }
    } else if duration.num_days() > 0 {
        let days = duration.num_days();
        if days == 1 {
            "dün".to_string()
        } else {
            format!("{} gün önce", days)
        }
    } else if duration.num_hours() > 0 {
        let hours = duration.num_hours();
        if hours == 1 {
            "1 saat önce".to_string()
        } else {
            format!("{} saat önce", hours)
        }
    } else if duration.num_minutes() > 0 {
        let minutes = duration.num_minutes();
        if minutes == 1 {
            "1 dakika önce".to_string()
        } else {
            format!("{} dakika önce", minutes)
        }
    } else {
        "az önce".to_string()
    };

    Ok(time_ago)
}