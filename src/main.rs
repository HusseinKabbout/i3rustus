mod utils;
mod widgets;

use serde_json::{Value, json};
use utils::json::jsonify;
use utils::logger::Logger;
use widgets::time::Time;
use widgets::{Widget};
use log::{LevelFilter};

static LOGGER: Logger = Logger;

struct I3Config {
    // This is currently not used in the code,
    // but i3 needs it to be in the json
    #[allow(dead_code)]
    version: u8,
    widgets: Vec<Box<dyn Widget>>
}

impl I3Config {

    fn to_json(&self) -> Value {
        let mut json: Value = json!({});

        for widget in self.widgets.iter() {
            match jsonify::<String>(&widget.to_json()) {
                Ok(display_text) => json[widget.name()] = display_text,
                Err(error) => {
                    LOGGER.warning(
                        &format!("Invalid config for {}: \n\t{}", widget.name(), error),
                        &file!());
                    continue
                }
            }

        }

        return json;
    }
}

fn main() {

    // Set logger
    match log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Error)) {
        Err(error) => println!("Enable to set logger: {}", error),
        _ => {}
    }

    let final_config = I3Config{
        version: 1,
        widgets: vec![
            Box::new(Time {})
        ]
    };

    let config_json = final_config.to_json();

    if config_json != json!({}) {
        LOGGER.info(
            &format!("Active config: {}", config_json),
            &file!());
    } else {
        LOGGER.error(
            "Error trying to load config!",
            &file!());
    }

}
