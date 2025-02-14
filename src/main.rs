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

    let new = setting.read_config("CUSTOM", "nmain").unwrap();
    println!("{new}");
    println!("{}", setting.get_divider("foo").unwrap());

    another_setting.write_config();
}
