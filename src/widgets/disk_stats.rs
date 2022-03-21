use std::fs;
use std::collections::HashMap;
use libc::statvfs;
use std::mem;
use std::path::Path;

use crate::utils::macros::cast_to_u64;
use super::Widget;

/// A struct that holds a Map of all paths that we want to watch over
pub struct Disk<'a> {
    pub paths_to_watch: HashMap<&'a str, &'a str>
}

impl<'a> Widget for Disk<'a> {

    fn name(&self) -> String {
        return String::from("disk")
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
                output.push_str(&calulcate_available_disk_storage(path).to_string());
                output.push_str(" GiB");
            }

        }

        output
    }

}

/// Calculate the available disk storage for a specific path
/// Source: https://github.com/GuillaumeGomez/sysinfo/blob/master/src/linux/disk.rs#L61
fn calulcate_available_disk_storage(path: &str) -> u64{
    unsafe {
        let mut available_space: u64 = 0;
        let mut stat: statvfs = mem::zeroed();
        // convert a path to a NUL-terminated Vec<u8> suitable for use with C functions
        let path_in_c = to_cpath(Path::new(path));

        if statvfs(path_in_c.as_ptr() as *const _, &mut stat) == 0 {
            let tmp = cast_to_u64!(stat.f_bsize) * cast_to_u64!(stat.f_bavail);
            available_space = cast_to_u64!(tmp);
        }
        // Convert bytes to GiB
        available_space / 1024 / 1024 / 1024
    }
}

/// This function transforms a rust string to a Vec<u8> that is suitable to
/// use with C function
/// Source: https://github.com/GuillaumeGomez/sysinfo/blob/master/src/utils.rs#L8
fn to_cpath(path: &Path) -> Vec<u8> {
    use std::{ffi::OsStr, os::unix::ffi::OsStrExt};

    let path_os: &OsStr = path.as_ref();
    let mut cpath = path_os.as_bytes().to_vec();
    cpath.push(0);
    cpath
}
