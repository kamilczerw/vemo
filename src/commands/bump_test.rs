use std::collections::HashMap;
use crate::commands::bump::run_v2;
use crate::cfg::{Config, DEFAULT_TAG_FORMAT};
use crate::commands::Component;

#[test]
fn bump_should_bump() {
    let config = Config {
        format: DEFAULT_TAG_FORMAT.to_string(),
        debug: false,
        gh_token: None,
        apps: HashMap::new()
    };

    let result = run_v2(config, "test", &Component::Patch);

    assert_eq!(result.is_ok(), true);
}
