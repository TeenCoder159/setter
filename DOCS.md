# Official documentation of the setter crate

## Setter is used to store configurations into a file.

Start by creating a struct from config::ConfigType

```rust
use setter::config::Config;

fn main(){
    let setting = Config {
        setting: String::from("foo"),
        mode: String::from("bar"),
        file: String::from("config.toml"),
        divider: String::from("[main]"),
    };
}


```

Next, call the different functions:

```rust
fn main(){
    setting.init(); // creates the file (faster and best for creating the first setting)
    setting.write_config(); // adds a new conifguration in the respective divider
    let value = read_config("foo".to_string()); // reads the value stored for the setting 'foo' and returns an Option<String>
    setting.new_divider("[packages]".to_string()); // Creates a new divider for the setting struct / file

    // Check if a config exists and if it does, then what kind of a config it is (requires
    use setter::config::ConfigType;

    match new_setting.config_exists("[main]".to_string()).unwrap() {
        ConfigType::Divider => println!("Found a Divider"),
        ConfigType::Config => println!("Found a configuration"),
    }
}


