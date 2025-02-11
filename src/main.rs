use setter::config_pub::Config;

fn main() {
    let new_setting = Config {
        setting: String::from("foo2"),
        mode: String::from("bar2"),
        file: String::from("config.txt"),
        divider: String::from("[main]"),
    };

    new_setting.write_config();
    let value = new_setting.read_config();
    println!(
        "Setting value: {}, Set val: {}",
        value.unwrap(),
        new_setting.mode
    );
}
