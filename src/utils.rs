pub fn is_version_number(version: &str) -> bool {
    let version_split = version.split('.').collect::<Vec<&str>>();

    let version_split_clone = version_split.clone();
    if version_split_clone.len() != 3 {
        return false;
    }

    let major = version_split[0];

    for letter in major.chars() {
        if !letter.is_numeric() {
            return false;
        }
    }

    let minor = version_split[1];

    for letter in minor.chars() {
        if !letter.is_numeric() {
            return false;
        }
    }

    let patch = version_split[2];

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

pub fn parse_version(version: &str) -> Result<Version, Rc<str>> {
    let version_split = version.split('.').collect::<Vec<&str>>();

    let major = version_split[0]
        .parse::<i32>()
        .map_err(map_error!("Major value could not be parsed as an int.({})"))?;

    let minor = version_split[1]
        .parse::<i32>()
        .map_err(map_error!("Minor value could not be parsed as an int.({})"))?;

    let patch = version_split[2].parse::<i32>().map_err(map_error!(
        "Patch value could not be parsed as an int. ({})"
    ))?;

    Ok(Version {
        major,
        minor,
        patch,
    })
}

use std::rc::Rc;

use crate::macros::map_error;
