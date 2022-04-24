use std::collections::HashMap;
use std::ffi::CString;
use std::io::Error;
use std::mem;
use std::path::Path;

use libc::statvfs;

use crate::LOGGER;
use crate::utils::macros::cast_to_u64;
use crate::widgets::Widget;
use crate::widgets::WidgetError;

/// A struct that holds a Map of all paths that we want to watch over
pub struct Disk {
    pub paths_to_watch: HashMap<String, String>
}

impl<'a> Disk {

    fn calulcate_available_disk_storage(&self, path: &Path) -> Result<f64, Error> {
        let mut directory_size = 0;

        unsafe {
            let mut stat: statvfs = mem::zeroed();
            let path_in_c = CString::new(path.to_string_lossy().as_bytes())?;
            // Look at the statvfs implementation to understand why it is way faster than to
            // calculate the value ourselfs
            if statvfs(path_in_c.as_ptr() as *const _, &mut stat) == 0 {
                let tmp = cast_to_u64!(stat.f_bsize) * cast_to_u64!(stat.f_bavail);
                directory_size = cast_to_u64!(tmp);
            }

            Ok(directory_size as f64/ 1024.0 / 1024.0 / 1024.0)
        }

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
                Err(msg) => {
                    LOGGER.error(msg.to_string());
                    return Err(WidgetError { error_message: msg.to_string() } )
                }
            }
        }
        Ok(output)
    }
}
