pub mod bump;
pub mod list;
pub mod error;
mod shell;

use clap::{ArgEnum, Subcommand};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Component {
    Major,
    Minor,
    Patch
}

#[derive(Subcommand)]
pub enum Commands {
    List {},
    Bump {
        /// Name of application inside a monorepo
        name: String,

        #[clap(short, long, arg_enum, default_value = "minor")]
        component: Component
    }
}
