use setter::config::{Config, ConfigType};

fn main() {
    let setting = Config {
        setting: String::from("foo"),
        mode: String::from("bar2"),
        file: String::from("config.toml"),
        divider: String::from("[main]"),
    };

    setting.init();

    setting.change_val("new_value");
    let new = setting.read_config("foo").unwrap();
    println!("{new}");
}
