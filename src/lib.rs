pub mod config {

    use std::fs;

    pub enum ConfigType {
        Config,
        Divider,
    }

    pub struct Config {
        pub setting: String,
        pub mode: String,
        pub file: String,
        pub divider: String,
    }

    impl Config {
        pub fn init(&self) {
            if let Err(e) = fs::write(
                &self.file,
                format!("{}\n{} = \"{}\"\n", self.divider, self.setting, self.mode),
            ) {
                panic!("Error: {e}")
            }
        }

        pub fn read_config(&self, config: &str, divider: &str) -> Option<String> {
            let contents = fs::read_to_string(&self.file).unwrap_or("".to_string());
            let mut divider_met = false;
            for line in contents.lines() {
                if line.contains(divider) && line.contains('[') && line.contains(']') {
                    divider_met = true;
                }
                if line.contains(config) && divider_met {
                    let (value, _) = line.split_once('"').unwrap().1.split_once('"').unwrap();
                    return Some(value.to_string());
                }
            }

            None
        }
        pub fn read_or(&self, config: &str, divider: &str, default: String) -> Option<String> {
            match Self::read_config(self, config, divider) {
                Some(val) => Some(val),
                None => Some(default),
            }
        }

        pub fn write_config(&self) {
            let contents = match fs::read_to_string(&self.file) {
                Ok(val) => val,
                _ => {
                    Self::init(self);
                    return;
                }
            };
            let mut final_contents =
                format!("{}\n{} = \"{}\"", self.divider, self.setting, self.mode);
            if contents.contains(&self.divider) {
                let (prefix, suffix) = match contents.split_once(&self.divider) {
                    Some((a, b)) => (a, b),
                    None => ("", ""),
                };

                final_contents = if prefix.is_empty() {
                    format!("{final_contents}\n{suffix}")
                } else {
                    format!("{prefix}\n{final_contents}\n{suffix}")
                };
            } else {
                final_contents = format!("{contents}\n{final_contents}");
            }

            let writing_content = final_contents
                .lines()
                .filter(|x| !x.is_empty())
                .fold(String::new(), |prefix, suffix| prefix + "\n" + suffix);

            if let Err(e) = fs::write(&self.file, writing_content.trim_start()) {
                panic!("Error: {e}")
            }
        }

        pub fn change_val(&self, config: &str, new_val: &str) {
            let contents = match fs::read_to_string(&self.file) {
                Ok(val) => val,
                Err(e) => panic!("Error:\n{e}\n"),
            };
            let mut final_contents = "".to_string();
            for line in contents.lines() {
                if line.contains(config) {
                    final_contents.push_str(format!("{} = \"{}\"\n", config, new_val).as_str());
                } else {
                    final_contents.push_str(format!("{line}\n").as_str());
                }
            }

            if let Err(e) = fs::write(&self.file, final_contents) {
                panic!("Error: {e}")
            }
        }

        pub fn config_exists(&self, config: &str) -> Option<ConfigType> {
            let contents = match fs::read_to_string(&self.file) {
                Ok(val) => val,
                Err(_) => panic!("Contents file is empty and therefore doesn't contain any values"),
            };
            if contents.contains(config) {
                for line in contents.lines() {
                    if line.contains(config) && line.contains("[") && line.contains("]") {
                        return Some(ConfigType::Divider);
                    } else if line.contains(config) && line.contains('"') && line.contains("=") {
                        return Some(ConfigType::Config);
                    }
                }
            }
            None
        }

        pub fn remove_config(&self, config: &str) {
            let contents = match fs::read_to_string(&self.file) {
                Ok(val) => val,
                Err(e) => panic!("Error:\n{e}\n"),
            };
            let final_contents = contents
                .lines()
                .filter(|line| !line.contains(config))
                .fold(String::new(), |prefix, suffix| prefix + "\n" + suffix);

            if let Err(e) = fs::write(&self.file, final_contents) {
                panic!("Error: {e}")
            }
        }

        pub fn new_divider(&self, divider: String) {
            let contents = match fs::read_to_string(&self.file) {
                Ok(val) => val,
                Err(e) => panic!("Error: {e}"),
            };
            if let Err(e) = fs::write(&self.file, format!("{}\n{}\n", contents, divider)) {
                eprintln!("Error: {e}")
            }
        }

        pub fn get_divider(&self, config: &str) -> Option<String> {
            let contents = match fs::read_to_string(&self.file) {
                Ok(val) => val,
                Err(_) => "".to_string(),
            };
            let mut divider = String::new();

            for line in contents.lines() {
                if line.contains("[") && line.contains("]") {
                    divider = line.to_string();
                } else if line.contains(config) {
                    return Some(divider);
                }
            }
            None
        }

        pub fn get_all_dividers(&self) -> Vec<String> {
            let contents = match fs::read_to_string(&self.file) {
                Ok(val) => val,
                Err(_) => "".to_string(),
            };
            let mut dividers = Vec::new();

            for line in contents.lines() {
                if line.contains("[") && line.contains("]") {
                    dividers.push(line.to_string());
                }
            }
            dividers
        }
    }
}
