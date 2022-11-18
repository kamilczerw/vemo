use std::{path::Path, env};

use super::Config;


#[test]
fn get_apps_config_should_parse_nested_keys() {
    let current_dir = env::current_dir().unwrap();
    let config_path = format!("{}/src/cfg/test.toml", current_dir.display());

    let settings = config::Config::builder()
        .add_source(config::File::from(Path::new(&config_path)));

    let cfg = settings.build().unwrap();
    let apps = Config::get_app_configs(cfg).unwrap();
    let mut keys = apps.keys();
    let mut values = apps.values();

    assert_eq!(apps.is_empty(), false);
    assert_eq!(keys.next(), Some(&String::from("b")));

    let app_b = values.next().unwrap();
    assert_eq!(app_b.path, Some(String::from("app_b")));


    assert_eq!(keys.next(), Some(&String::from("a")));

    let app_a = values.next().unwrap();
    assert_eq!(app_a.path, Some(String::from("app_a")));
}
