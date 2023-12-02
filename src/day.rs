use std::{
    ffi::{OsStr, OsString},
    fs,
    path::PathBuf,
};

use anyhow::Context;

#[derive(Debug, PartialEq)]
pub enum InputType {
    Test,
    Real,
    Unknown,
}

impl From<OsString> for InputType {
    fn from(value: OsString) -> Self {
        if let Some(value) = value.to_str() {
            if value.starts_with("test") {
                Self::Test
            } else if value == "real" {
                Self::Real
            } else {
                Self::Unknown
            }
        } else {
            Self::Unknown
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Input {
    pub input_type: InputType,
    pub path: PathBuf,
    pub contents: String,
}

impl Input {
    pub fn get_expected_output(&self) -> Result<String, std::io::Error> {
        if self.input_type == InputType::Test {
            let output_path: PathBuf = self.path.iter().map(|part| {
                if part == "inputs" {
                    OsStr::new("outputs")
                } else {
                    part
                }
            }).collect();
            fs::read_to_string(output_path)
        } else {
            Ok(String::new())
        }
    }
}

#[derive(Debug)]
pub enum Day {
    Day1A,
    Day1B
}

impl TryFrom<String> for Day {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Day, anyhow::Error> {
        match value.as_str() {
            "1a" => Ok(Day::Day1A),
            "1b" => Ok(Day::Day1B),
            _ => Err(anyhow::anyhow!("Invalid day number {}", value)),
        }
    }
}

impl Into<String> for &Day {
    fn into(self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

impl Day {
    pub fn get_inputs(&self) -> Result<Vec<Input>, anyhow::Error> {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let folder_name: String = self.into();
        d.push(format!("inputs/{}", folder_name));

        let mut inputs: Vec<Input> = Vec::new();
        for entry in fs::read_dir(&d).context(format!("could not read directory {:?}", &d))? {
            let entry = entry?;
            let input_type: InputType = entry.file_name().into();
            let contents = fs::read_to_string(entry.path())?;
            inputs.push(Input {
                path: entry.path(),
                input_type,
                contents,
            })
        }

        Ok(inputs)
    }
}
