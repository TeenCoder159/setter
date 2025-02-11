pub mod config_pub {

    use std::fs;

    pub struct Config {
        pub setting: String,
        pub mode: String,
        pub file: String,
    }

    impl Config {
        pub fn write_config(&self) {
            match std::fs::write(
                &self.file,
                format!("{} = \"{}\"\n", self.setting, self.mode),
            ) {
                Err(e) => panic!("Error: {e}"),
                _ => {}
            }
        }

        pub fn read_config(&self) -> String {
            let contents = fs::read_to_string(&self.file).unwrap();
            let (_, after) = contents
                .split_once(&format!("{} = ", self.setting))
                .unwrap();
            let (line, _) = after.split_once("\n").unwrap();
            let (value, _) = line.split_once('"').unwrap().1.split_once('"').unwrap();
            value.to_string()
        }
    }
}
