use setter::config::Config;

fn main() {
    let mut setting = Config {
        setting: String::from("foo"),
        mode: String::from("bar2"),
        file: String::from("config.toml"),
        divider: String::from("[main]"),
    };
    let another_setting = Config {
        setting: String::from("foo2"),
        mode: String::from("bar2"),
        file: String::from("config.toml"),
        divider: String::from("[nmain]"),
    };

    setting.init();

    setting.setting = "new_val".to_string();

    setting.change_val("foo", "new_value");
    let new = setting.read_config("foo").unwrap();
    println!("{new}");

    another_setting.write_config();
}
