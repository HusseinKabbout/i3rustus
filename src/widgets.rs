use std::error::Error;
use std::fmt;

pub mod time;
pub mod disk_stats;
pub mod memory_stats;

/// All widgets HAVE to implement this trait
pub trait Widget {
    // Name is a &str because we know at compile time how big the string will be
    // so we don't need to use String
    fn name(&self) -> &str;
    // The text that will be shown on the status bar
    fn display_text(&self) -> Result<String, WidgetError>;
    // JSON representation of the widget
    fn to_json(&self) -> String {
        // full_text is defined by i3 and is the display_text
        // Name is not defined by i3 and is only used to know which
        // config belongs to which widget
        let text = self.display_text();

        match text {
            Ok(text) => format!(
                "{{ \"full_text\": \"{}\", \"name\": \"{}\" }}",
                // unwrap should be safe to use here, because we check
                // wether text is OK or not first
                text,
                self.name()
            ),
            Err(error) => String::from(error.error_message)
        }
    }

}

// This should be used to signal that a widget is not working properly
// The idea is that display_text return this Error and then we can so some generic
// Error handling for all widgets in to_json
#[derive(Debug)]
pub struct WidgetError { error_message: String }

impl fmt::Display for WidgetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Woah, something bad happend!")
    }
}

impl Error for WidgetError {}
