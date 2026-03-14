//! TAO and Alpha balance types.
//!
//! TAO has 9 decimal places (1 TAO = 1_000_000_000 RAO).

use serde::{Deserialize, Serialize};
use std::fmt;

/// One TAO in RAO (smallest unit).
pub const RAO_PER_TAO: u64 = 1_000_000_000;

/// Represents a TAO balance stored as RAO (u64).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Balance {
    rao: u64,
}

impl Balance {
    pub const ZERO: Self = Self { rao: 0 };

    /// Create from RAO (smallest unit).
    pub fn from_rao(rao: u64) -> Self {
        Self { rao }
    }

    /// Create from TAO (floating point, truncated to RAO precision).
    pub fn from_tao(tao: f64) -> Self {
        Self {
            rao: (tao * RAO_PER_TAO as f64) as u64,
        }
    }

    /// Return value in RAO.
    pub fn rao(&self) -> u64 {
        self.rao
    }

    /// Return value in TAO.
    pub fn tao(&self) -> f64 {
        self.rao as f64 / RAO_PER_TAO as f64
    }

    /// Human-readable string like "1.234567890 τ".
    pub fn display_tao(&self) -> String {
        format!("{:.9} τ", self.tao())
    }
}

impl fmt::Display for Balance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.4} τ", self.tao())
    }
}

impl std::ops::Add for Balance {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            rao: self.rao.saturating_add(rhs.rao),
        }
    }
}

impl std::ops::Sub for Balance {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            rao: self.rao.saturating_sub(rhs.rao),
        }
    }
}

/// Represents an Alpha token balance for a subnet's native token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct AlphaBalance {
    raw: u64,
}

impl AlphaBalance {
    pub const ZERO: Self = Self { raw: 0 };

    pub fn from_raw(raw: u64) -> Self {
        Self { raw }
    }

    pub fn raw(&self) -> u64 {
        self.raw
    }
}

impl fmt::Display for AlphaBalance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.4} α", self.raw as f64 / RAO_PER_TAO as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tao_conversion() {
        let b = Balance::from_tao(1.5);
        assert_eq!(b.rao(), 1_500_000_000);
        assert!((b.tao() - 1.5).abs() < 1e-9);
    }

    #[test]
    fn saturating_ops() {
        let a = Balance::from_rao(10);
        let b = Balance::from_rao(20);
        assert_eq!((a - b).rao(), 0); // saturating
    }
}
