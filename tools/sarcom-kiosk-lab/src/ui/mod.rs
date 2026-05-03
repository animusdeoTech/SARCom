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
