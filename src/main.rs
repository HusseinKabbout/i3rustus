mod utils;
mod widgets;

use std::{thread, time};
use serde_json::{ Value, json };
use utils::json::jsonify;
use utils::logger::Logger;
use widgets::time::Time;
use widgets::{ Widget };
use log::{ LevelFilter };

static LOGGER: Logger = Logger;

struct I3Config {
    version: u8,
    widgets: Vec<Box<dyn Widget>>
}

impl I3Config {

    fn version(&self) -> u8 {
        return self.version;
    }

    fn widgets_config(&self) -> Value {
        let mut config: Value = json!([]);

        for widget in self.widgets.iter() {
            let widget_config: Value;
            // The brackets for widget.to_json were added
            // to indicate that we are getting a ref from the
            // result of `to_json` and not the widget itself
            match jsonify::<String>(&(widget.to_json())) {
                Ok(conf) => widget_config = conf,
                Err(error) => {
                    LOGGER.warning(
                        &format!("Invalid config for {}: \n\t{}", widget.name(), error),
                        &file!());
                    continue
                }
            }

            // Is it OK to do an unwrap here?
            config.as_array_mut().unwrap().push(widget_config);
        }

        return config;
    }

    fn init(&self) {
        // This is the output that is read by i3
        println!("{}", json!({"version": self.version()}));
        // Begin endless array
        println!("[");
        // REMOVE ME: This is used to make the output simpler, but is ugly
        // Arrays have to be separated by comma in output
        println!("[]");
        loop {
            // The actual config for the status bar
            println!(",{}", self.widgets_config());
            // Wait 1 secs before printing update
            thread::sleep(time::Duration::from_secs(1));
        }
    }
}

fn main() {

    // Set logger
    match log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Error)) {
        Err(error) => println!("Enable to set logger: {}", error),
        _ => {}
    }

    let final_config = I3Config {
        version: 1,
        widgets: vec![
            Box::new(Time {})
        ]
    };

    final_config.init();
}
