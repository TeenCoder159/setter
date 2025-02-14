# Official documentation of the setter crate

## Setter is used to store configurations into a file.

### List of functions not working as of now:

```rust
fn not_working(){
    remove_config();
}
```

### Usage:

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

Some example functions:
For the latest working functions, see the main.rs in the github REPO, # Note: Not all functions may be functional


```rust
fn main(){
    setting.init(); // creates the file (faster and best for creating the first setting)
    setting.write_config(); // adds a new conifguration in the respective divider
    let value = read_config("foo", "[DIVDER]"); // reads the value stored for the setting 'foo' inside the [DIVIDER] and returns an Option<String>
    setting.new_divider("[packages]"); // Creates a new divider for the setting struct / file
    setting.get_divider("foo") // Returns the divider which has the setting 'foo' (only the first divider)

    // Check if a config exists and if it does, then what kind of a config it is (requires
    use setter::config::ConfigType;

    match new_setting.config_exists("[main]".to_string()).unwrap() {
        ConfigType::Divider => println!("Found a Divider"),
        ConfigType::Config => println!("Found a configuration"),
    }
}
---


