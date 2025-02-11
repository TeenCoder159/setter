# Welcome to the repository of setter, a Configuration tool made in rust

Setter is a project of mine to help me read and write configuration files for my other project, lion AKA lion-cli

Setter can be used in various different places as many projects require a config file for saving things like workspace settings, project settings, currently open files and etc.

Setter is a new project and will be available on crates.io once it is ready


Example:
```rust
use setter::config_pub::Config;

fn main() {
    let new_setting = Config {
        setting: String::from("foo"),
        mode: String::from("bar"),
        file: String::from("config.toml"),
        divider: String::from("[main]"),
    };

    new_setting.write_config();
    let value = new_setting.read_config();
    println!("Setting value: {}, Set val: {}", value, new_setting.mode);
}
```

This will allow you to create a section called `[main]` and store a configuration called foo with the value of bar related to that section
