use crate::commands::shell::git::GitProvider;

#[derive(Debug)]
pub enum GitClientError {
    ///
    UnexpectedError(String),

    /// Git provider API token is missing
    MissingToken(GitProvider),

    /// Git provider is not supported
    UnsupportedProvider(GitProvider),
}
