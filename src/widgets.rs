pub mod time;

pub trait Widget {
    fn name(&self) -> String;
    // The text that will be shown on the status bar
    fn display_text(&self) -> String;
    // JSON representation of the widget
    fn to_json(&self) -> String;
}
