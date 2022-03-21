pub mod time;
pub mod disk_stats;

/// All widgets HAVE to implement this trait
pub trait Widget {
    // Name is a &str because we know at compile time how big the string will be
    // so we don't need to use String
    fn name(&self) -> &str;
    // The text that will be shown on the status bar
    fn display_text(&self) -> String;
    // JSON representation of the widget
    fn to_json(&self) -> String {
        // full_text is defined by i3 and is the display_text
        // Name is not defined by i3 and is only used to know which
        // config belongs to which widget
        format!(
            "{{ \"full_text\": \"{}\", \"name\": \"{}\" }}",
            self.display_text(),
            self.name()
        )
    }

}
