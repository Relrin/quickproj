pub mod git;
pub mod local;
pub mod traits;

pub use self::git::GitInstaller;
pub use self::local::LocalInstaller;
pub use self::traits::Installer;
