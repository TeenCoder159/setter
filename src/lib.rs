pub mod config_pub {

    use std::fs;

    pub struct Config {
        pub setting: String,
        pub mode: String,
        pub file: String,
        pub divider: String,
    }

    impl Config {
        pub fn divider(&self, divider: String) {
            let contents = match fs::read_to_string(&self.file) {
                Ok(val) => val,
                Err(e) => panic!("Error: {e}"),
            };
            match fs::write(&self.file, format!("{}\n{}\n", contents, divider)) {
                Err(e) => eprintln!("Error: {e}"),
                _ => {}
            }
        }
        pub fn write_config(&self) {
            let contents = match fs::read_to_string(&self.file) {
                Ok(val) => val,
                _ => "".to_string(),
            };
            let mut final_contents =
                format!("{}\n{} = \"{}\"", self.divider, self.setting, self.mode);
            if contents.contains(&self.divider) {
                if !contents.is_empty() {
                    let (prefix, suffix) = match contents.split_once(&self.divider) {
                        Some((a, b)) => (a, b),
                        None => ("", ""),
                    };

                    final_contents = if prefix.is_empty() {
                        format!("{final_contents}\n{suffix}")
                    } else {
                        format!("{prefix}\n{final_contents}\n{suffix}")
                    };
                }
            } else {
                final_contents = format!("{contents}\n{final_contents}");
            }
            let mut writing_content = "".to_string();
            for line in final_contents.lines() {
                if !line.is_empty() {
                    writing_content.push_str(format!("{line}\n").as_str());
                }
            }

            match fs::write(&self.file, writing_content) {
                Err(e) => panic!("Error: {e}"),
                _ => {}
            }
        }

        pub fn read_config(&self) -> Option<String> {
            let contents = fs::read_to_string(&self.file).unwrap();
            for line in contents.lines() {
                if line.contains(&self.setting) {
                    let (value, _) = line.split_once('"').unwrap().1.split_once('"').unwrap();
                    return Some(value.to_string());
                }
            }
            None
        }

        pub fn init(&self) {
            match fs::write(
                &self.file,
                format!("{} = \"{}\"\n", self.setting, self.mode),
            ) {
                Err(e) => panic!("Error: {e}"),
                _ => {}
            }
        }
    }
}

pub fn divider() {}
