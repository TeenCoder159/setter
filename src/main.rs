use setter::config::{read_config, Config, ConfigType};

fn main() {
    let mut new_setting = Config {
        setting: String::from("foo2"),
        mode: String::from("bar2"),
        file: String::from("config.toml"),
        divider: String::from("[main]"),
    };

    new_setting.init();

    new_setting.setting = String::from("Another Value");

    new_setting.write_config();
    let value = read_config("Another Value".to_string());
    println!(
        "Setting value: {}, Set val: {}",
        value.unwrap(),
        new_setting.mode
    );
    new_setting.new_divider("[packages]".to_string());
    let contents = std::fs::read_to_string(&new_setting.file).expect("Error occured");
    println!("{contents}");
    match new_setting.config_exists("[main]".to_string()).unwrap() {
        ConfigType::Divider => println!("Found a Divider"),
        ConfigType::Config => println!("Found a configuration"),
    }
}
