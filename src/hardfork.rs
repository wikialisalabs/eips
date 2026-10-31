//! Hardfork definitions and activation logic.

/// All known Ethereum mainnet hardforks in chronological order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Hardfork {
    /// Frontier (genesis).
    Frontier,
    /// Homestead — EIP-2, EIP-7, EIP-8.
    Homestead,
    /// Tangerine Whistle — EIP-150.
    TangerineWhistle,
    /// Spurious Dragon — EIP-155, EIP-160, EIP-161.
    SpuriousDragon,
    /// Byzantium — EIP-100, EIP-140, EIP-196, EIP-197, EIP-198, EIP-211, EIP-214, EIP-649, EIP-658.
    Byzantium,
    /// Constantinople — EIP-145, EIP-1014, EIP-1052, EIP-1234, EIP-1283.
    Constantinople,
    /// Istanbul — EIP-152, EIP-1108, EIP-1344, EIP-1884, EIP-2028, EIP-2200.
    Istanbul,
    /// Berlin — EIP-2565, EIP-2718, EIP-2929, EIP-2930.
    Berlin,
    /// London — EIP-1559, EIP-3198, EIP-3529, EIP-3541, EIP-3554.
    London,
    /// Merge (Paris) — EIP-3675, EIP-4399.
    Paris,
    /// Shanghai — EIP-3651, EIP-3855, EIP-3860, EIP-4895.
    Shanghai,
    /// Cancun — EIP-1153, EIP-4788, EIP-4844, EIP-5656, EIP-6780, EIP-7516.
    Cancun,
    /// Prague — EIP-2537, EIP-2935, EIP-6110, EIP-7002, EIP-7251, EIP-7549, EIP-7702.
    Prague,
}

impl Hardfork {
    /// The canonical name of this hardfork.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Frontier => "Frontier",
            Self::Homestead => "Homestead",
            Self::TangerineWhistle => "TangerineWhistle",
            Self::SpuriousDragon => "SpuriousDragon",
            Self::Byzantium => "Byzantium",
            Self::Constantinople => "Constantinople",
            Self::Istanbul => "Istanbul",
            Self::Berlin => "Berlin",
            Self::London => "London",
            Self::Paris => "Paris",
            Self::Shanghai => "Shanghai",
            Self::Cancun => "Cancun",
            Self::Prague => "Prague",
        }
    }
}

/// Activation configuration for a single network.
#[derive(Debug, Clone)]
pub struct HardforkConfig {
    /// Hardfork to check.
    pub hardfork: Hardfork,
    /// Block number at which this hardfork activates (None = not yet scheduled).
    pub block: Option<u64>,
    /// Unix timestamp at which this hardfork activates (used for post-Merge forks).
    pub timestamp: Option<u64>,
}

impl HardforkConfig {
    /// Returns `true` if this hardfork is active at the given `block` and `timestamp`.
    pub fn is_active_at(&self, block: u64, timestamp: u64) -> bool {
        let block_ok = self.block.map_or(true, |b| block >= b);
        let ts_ok = self.timestamp.map_or(true, |t| timestamp >= t);
        block_ok && ts_ok
    }
}

/// Mainnet hardfork schedule (block numbers / timestamps as of 2026).
pub fn mainnet_schedule() -> Vec<HardforkConfig> {
    vec![
        HardforkConfig { hardfork: Hardfork::Frontier,         block: Some(0),        timestamp: None },
        HardforkConfig { hardfork: Hardfork::Homestead,        block: Some(1_150_000), timestamp: None },
        HardforkConfig { hardfork: Hardfork::London,           block: Some(12_965_000),timestamp: None },
        HardforkConfig { hardfork: Hardfork::Paris,            block: Some(15_537_394),timestamp: None },
        HardforkConfig { hardfork: Hardfork::Shanghai,         block: None,            timestamp: Some(1_681_338_455) },
        HardforkConfig { hardfork: Hardfork::Cancun,           block: None,            timestamp: Some(1_710_338_135) },
        HardforkConfig { hardfork: Hardfork::Prague,           block: None,            timestamp: Some(1_746_000_000) },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hardfork_ordering() {
        assert!(Hardfork::Frontier < Hardfork::Prague);
        assert!(Hardfork::London < Hardfork::Cancun);
    }

    #[test]
    fn activation_check() {
        let cfg = HardforkConfig {
            hardfork: Hardfork::London,
            block: Some(12_965_000),
            timestamp: None,
        };
        assert!(cfg.is_active_at(13_000_000, 0));
        assert!(!cfg.is_active_at(12_000_000, 0));
    }

    #[test]
    fn mainnet_schedule_not_empty() {
        assert!(!mainnet_schedule().is_empty());
    }
}
