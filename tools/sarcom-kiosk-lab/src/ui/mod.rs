pub mod edit_panel;
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

/// Use this anywhere a relative-time string would be displayed.
/// Returns "time unavailable" when the gateway clock has not been set
/// (RTC missing at boot per ADR-011). Never fabricate "X ago" from a
/// running tick when the wall clock is invalid.
pub fn format_age_or_unavailable(secs: f32, clock_valid: bool) -> String {
    if clock_valid {
        format_age(secs)
    } else {
        "time unavailable".into()
    }
}
