use std::fs;
use std::collections::HashMap;
use std::io::Error;
use std::path::Path;

use crate::widgets::Widget;
use crate::widgets::WidgetError;

/// A struct that holds a Map of all paths that we want to watch over
pub struct Disk {
    pub paths_to_watch: HashMap<String, String>
}

impl<'a> Disk {

    fn calulcate_available_disk_storage(&self, path: &Path) -> Result<u64, Error> {
        let mut directory_size = 0;

        if path.is_dir() {
            for directory_entry in fs::read_dir(path)? {
                let directory_entry_path = directory_entry?.path();
                directory_size += directory_entry_path.metadata()?.len();
                if directory_entry_path.is_dir() {
                    directory_size += self.calulcate_available_disk_storage(&directory_entry_path)?;
                }
            }
        } else {
            directory_size = path.metadata()?.len();
        }

        Ok(directory_size)
    }

}

impl<'a> Widget for Disk {

    fn name(&self) -> &str {
        "disk"
    }

    fn display_text(&self) -> Result<String, WidgetError> {
        let mut output: String = String::new();
        // We create a iterator from the vector and consume it directly in the for loop
        for (name, path) in &self.paths_to_watch {

            // Add comma to differentiate between multiple paths
            if output != "" {
                output.push_str(", ");
            }

            match self.calulcate_available_disk_storage(&Path::new(path)) {
                Ok(calculated_storage) => {
                    output.push_str(name);
                    output.push_str(": ");
                    output.push_str(&calculated_storage.to_string());
                    output.push_str(" GiB");
                },
                Err(msg) => return Err(WidgetError { error_message: msg.to_string() } )
            }
        }
        Ok(output)
    }
}
