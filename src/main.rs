use setter::config::{read_config, read_or, Config, ConfigType};

fn main() {
    let new_setting = Config {
        setting: String::from("foo2"),
        mode: String::from("bar2"),
        file: String::from("config.toml"),
        divider: String::from("[main]"),
    };

    let string = read_or("Something", "Hiya".to_string()).unwrap();
    println!("{string}");
}
