# Welcome to the repository of setter, a Configuration tool made in rust

Setter is a project of mine to help me read and write configuration files for my other project, lion AKA lion-cli

Setter can be used in various different places as many projects require a config file for saving things like workspace settings, project settings, currently open files and etc.

Setter is a new project and will be available on crates.io once it is ready


Example:
```rust
use setter::config::{Config, ConfigType};

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
    let value = new_setting.read_config();
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
        _ => eprintln!("Didn't find anything"),
    }
}
```
```rust

fn main(){

    write_config() // creates a configuration (setting = "val")

    read_config() // reads a config with the setting value of the struct

    conifg_exists("config".to_string()) // returns what kind of config it is (Some(Divider), Some(Config), or None)

    init() // Best for creating the settings for the first time as it directly creates the file

}
```
