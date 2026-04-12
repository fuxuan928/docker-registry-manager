//! Locale models for UI translations

use serde::{Deserialize, Serialize};

/// Supported application locale preferences.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Locale {
    #[default]
    System,
    En,
    ZhHans,
    ZhHant,
}
