use setter::config_pub::Config;

fn main() {
    let new_setting = Config {
        setting: String::from("foo"),
        mode: String::from("bar"),
        file: String::from("config.txt"),
    };

    new_setting.write_config();
    let value = new_setting.read_config();
    println!("Setting value: {}, Set val: {}", value, new_setting.mode);
}
