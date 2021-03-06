pub mod bump;
pub mod list;
pub mod error;
pub mod shell;

use clap::{ArgEnum, Subcommand};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Component {
    Major,
    Minor,
    Patch
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all available applications.
    ///
    /// The format variable is used to filter out the tags in the monorepo.
    ///
    /// Default format is "{app_name}/v{version}", it can be changed using .vemo.toml or VEMO_FORMAT env variable.
    List {},
    Bump {
        /// Name of application inside a monorepo
        name: String,

        /// Version component which will be bumped
        #[clap(short, long, arg_enum, default_value = "minor")]
        component: Component
    }
}
