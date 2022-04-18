use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::Error;
use std::io::BufReader;
use std::num::ParseFloatError;

use crate::widgets::Widget;
use crate::widgets::WidgetError;

#[derive(Debug)]
struct MemoryInfromation {
    // Current memory that is being used by the OS
    used: f32,
    // Current memory that is not being actively used (without swapping)
    available: f32,
    // Total memory that can be used
    // Should be ~ used + available
    total_usable: f32,
    // The next fields are only needed for calculating "used" memory
    mem_free: f32,
    // Kernel buffers
    buffers: f32,
    // In memory cache for files from disk
    cache: f32,
    // Probably reclaimable cache or something like that
    reclaimable: f32
}

pub struct MemoryUsage {}

impl MemoryUsage {

    fn read_file(&self) -> Option<File> {
        let meminfo: Result<File, Error> = OpenOptions::new()
                                                        .read(true)
                                                        .open("/proc/meminfo");
        match meminfo {
            Ok(file) => Some(file),
            _ => None
        }

    }

    fn get_int_from_str(&self, str_to_parse: String) -> Result<f32, ParseFloatError> {
        // This will be a string with only numbers, so we can convert
        // it to a f32
        let mut resulting_string = String::new();

        // Could be improved by removing whitespace before iteration
        for character in str_to_parse.chars() {
            if character.is_digit(10) {
                resulting_string.push(character);
            }
        }

        resulting_string.parse::<f32>()
    }

    fn get_usage(&self) -> Result<MemoryInfromation, ParseFloatError> {
        let mut memory_information: MemoryInfromation = MemoryInfromation {
            used: 0.0,
            available: 0.0,
            total_usable: 0.0,
            mem_free: 0.0,
            buffers: 0.0,
            cache: 0.0,
            reclaimable: 0.0
        };

        if let Some(memory_file) = self.read_file() {
            let reader = BufReader::new(memory_file);
            for line in reader.lines() {
                if let Ok(line) = line {

                    if line.starts_with("MemTotal") {
                        // Convert to kb to gb
                        memory_information.total_usable = self.get_int_from_str(line)?;
                    } else if line.starts_with("MemAvailable") {
                        memory_information.available = self.get_int_from_str(line)?;
                    } else if line.starts_with("Buffers") {
                        memory_information.buffers = self.get_int_from_str(line)?;
                    } else if line.starts_with("MemFree") {
                        memory_information.mem_free = self.get_int_from_str(line)?;
                    } else if line.starts_with("Cached") {
                        memory_information.cache = self.get_int_from_str(line)?;
                    } else if line.starts_with("SReclaimable") {
                        memory_information.reclaimable = self.get_int_from_str(line)?;
                    }
                }
            }

            memory_information.used = memory_information.total_usable -
                memory_information.mem_free -
                memory_information.buffers -
                memory_information.cache -
                memory_information.reclaimable
        }

        Ok(memory_information)
    }

}

impl Widget for MemoryUsage {

    fn name(&self) -> &str {
        "memory"
    }

    fn display_text(&self) -> Result<String, WidgetError> {

        match self.get_usage() {
            Ok(usage) => Ok(
                format!(
                    "RAM (GiB): U={used:.1} A={available:.1} / {total_usable:.1}",
                    used = usage.used / 1024.0 / 1024.0,
                    available = usage.available / 1024.0 / 1024.0,
                    total_usable = usage.total_usable / 1024.0 / 1024.0
                )
            ),
            Err(msg) => return Err(WidgetError { error_message: msg.to_string() } )
        }
    }
}
