pub mod header;
pub mod palette;
pub mod sidebar;

pub fn format_age(secs: f32) -> String {
    if secs < 60.0 {
        format!("{:.0}s ago", secs)
    } else if secs < 3600.0 {
        format!("{:.0}m ago", secs / 60.0)
    } else {
        format!("{:.1}h ago", secs / 3600.0)
    }
}

/// HH:MM:SS rendering of a wall-clock-style time. The lab does not have a
/// real wall clock; callers feed it `t - secs` derived from the egui app
/// timer, which is honest enough for the synthetic mockup.
pub fn format_wall(t: f64) -> String {
    let secs = t.max(0.0) as u64;
    format!(
        "{:02}:{:02}:{:02}",
        (secs / 3600) % 24,
        (secs / 60) % 60,
        secs % 60
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_age_buckets() {
        assert_eq!(format_age(0.0), "0s ago");
        assert_eq!(format_age(42.0), "42s ago");
        assert_eq!(format_age(180.0), "3m ago");
        assert_eq!(format_age(7200.0), "2.0h ago");
    }
}
