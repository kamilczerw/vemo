use crate::commands::shell::git::Tag;
use crate::git::GitProvider;
use crate::git::provider::error::GitProviderError;

pub struct Provider {
}

impl Provider {
    pub fn new() -> Self {
        Self {}
    }
}

impl GitProvider for Provider {
    fn find_latest_tag(&self, app_name: &str) -> Result<Option<Tag>, GitProviderError> {
        todo!()
    }

    fn release(&self, name: &str, tag: Tag, body: String) -> Result<(), GitProviderError> {
        todo!()
    }
}
