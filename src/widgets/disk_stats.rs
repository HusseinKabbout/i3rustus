use std::fs;
use std::collections::HashMap;
use std::io::Error;
use std::path::Path;

use crate::widgets::Widget;

/// A struct that holds a Map of all paths that we want to watch over
pub struct Disk<'a> {
    pub paths_to_watch: HashMap<&'a str, &'a str>
}

impl<'a> Disk<'a> {

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

impl<'a> Widget for Disk<'a> {

    fn name(&self) -> &str {
        "disk"
    }

    fn display_text(&self) -> String {
        let mut output: String = String::new();
        // We create a iterator from the vector and consume it directly in the for loop
        for (name, path) in self.paths_to_watch.iter() {

            // Add comma to differentiate between multiple paths
            if output != "" {
                output.push_str(", ");
            }

            // Check if path exists and if it is also a directory
            if match fs::metadata(path) {
                Ok(metadata) => metadata.is_dir(),
                _ => false
            } {
                output.push_str(name);
                output.push_str(": ");
                output.push_str(&(self.calulcate_available_disk_storage(path).to_string()));
                output.push_str(" GiB");
            }

        }

        output
    }

}
