//! Shared trait for VM-specific chain configuration registries.
//!
//! Both `ChainRegistry` (EVM) and `SolanaChainRegistry` (SVM) implement
//! `ChainRegistryOps`, which provides default implementations for
//! `from_file` and `load_with_defaults` — eliminating per-VM duplication
//! of filesystem I/O and user-override merging.

use std::format;
use std::path::Path;
use std::string::String;

/// Trait implemented by all VM-specific chain registries.
///
/// Provides a uniform interface for loading chain configuration from TOML,
/// merging user overrides, and resolving chain + token pairs. Default
/// methods handle filesystem I/O so implementors only need to provide
/// parsing, merging, and resolution logic.
pub trait ChainRegistryOps: Sized {
    /// The VM-specific error type returned by registry operations.
    type Error;

    /// Parses a registry from a TOML string.
    fn from_toml(content: &str) -> Result<Self, Self::Error>;

    /// Merges another registry into `self` (other takes precedence).
    fn merge(&mut self, other: Self);

    /// The filename for user-override config (e.g., `"chains.toml"` or
    /// `"solana-chains.toml"`), stored under `~/.config/morpheum/`.
    fn override_filename() -> &'static str;

    /// Constructs an error from a config-load message string.
    fn config_error(msg: String) -> Self::Error;

    /// Loads a registry from a TOML file on disk.
    fn from_file(path: &Path) -> Result<Self, Self::Error> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| Self::config_error(format!("failed to read {}: {e}", path.display())))?;
        Self::from_toml(&content)
    }

    /// Loads the default config, then merges user overrides from
    /// `~/.config/morpheum/<override_filename>` (if present).
    fn load_with_defaults(default_toml: &str) -> Result<Self, Self::Error> {
        let mut registry = Self::from_toml(default_toml)?;

        if let Some(config_dir) = dirs_next::config_dir() {
            let user_path = config_dir.join("morpheum").join(Self::override_filename());
            if user_path.exists() {
                let user = Self::from_file(&user_path)?;
                registry.merge(user);
            }
        }

        Ok(registry)
    }
}
