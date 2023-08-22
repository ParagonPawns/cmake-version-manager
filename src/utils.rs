pub fn is_version_number(version: &str) -> bool {
    let mut version_split = version.split('.');

    let version_split_clone = version_split.clone();
    if version_split_clone.count() != 3 {
        return false;
    }

    let major = match version_split.next() {
        Some(item) => item,
        None => return false,
    };

    for letter in major.chars() {
        if !letter.is_numeric() {
            return false;
        }
    }

    let minor = match version_split.next() {
        Some(item) => item,
        None => return false,
    };

    for letter in minor.chars() {
        if !letter.is_numeric() {
            return false;
        }
    }

    let patch = match version_split.next() {
        Some(item) => item,
        None => return false,
    };

    for letter in patch.chars() {
        if !letter.is_numeric() {
            return false;
        }
    }

    true
}

#[allow(dead_code)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
}

pub fn parse_version(version: &str) -> Version {
    let mut version_split = version.split('.');

    let major = match version_split.next() {
        Some(item) => match item.parse::<i32>() {
            Ok(value) => value,
            Err(error) => {
                log_error(&format!(
                    "Major value could not be parsed as an int.({})",
                    error
                ));
                0
            }
        },
        None => {
            log_error("Failed to get major version number. Setting to 0.");
            0
        }
    };

    let minor = match version_split.next() {
        Some(item) => match item.parse::<i32>() {
            Ok(value) => value,
            Err(error) => {
                log_error(&format!(
                    "Minor value could not be parsed as an int.({})",
                    error
                ));
                0
            }
        },
        None => {
            log_error("Failed to get minor version number. Setting to 0.");
            0
        }
    };

    let patch = match version_split.next() {
        Some(item) => match item.parse::<i32>() {
            Ok(value) => value,
            Err(error) => {
                log_error(&format!(
                    "Patch value could not be parsed as an int.({})",
                    error
                ));
                0
            }
        },
        None => {
            log_error("Failed to get patch version number. Setting to 0.");
            0
        }
    };

    Version {
        major,
        minor,
        patch,
    }
}

use crate::log::log_error;

