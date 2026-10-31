//! EIP-1559: Fee market change — base fee + priority fee model.

/// EIP-1559 transaction fields.
#[derive(Debug, Clone)]
pub struct Eip1559Transaction {
    /// Chain ID (replay protection).
    pub chain_id: u64,
    /// Maximum fee per gas the sender is willing to pay.
    pub max_fee_per_gas: u128,
    /// Maximum priority fee (miner tip) per gas.
    pub max_priority_fee_per_gas: u128,
    /// Gas limit.
    pub gas_limit: u64,
}

/// Calculate the effective gas price for a transaction given the current base fee.
///
/// `effective_gas_price = min(max_fee_per_gas, base_fee + max_priority_fee_per_gas)`
pub fn effective_gas_price(tx: &Eip1559Transaction, base_fee: u128) -> u128 {
    (base_fee + tx.max_priority_fee_per_gas).min(tx.max_fee_per_gas)
}

/// Check if a transaction can be included at the given base fee.
pub fn can_include(tx: &Eip1559Transaction, base_fee: u128) -> bool {
    tx.max_fee_per_gas >= base_fee
}

/// Calculate the next block's base fee (EIP-1559 formula).
///
/// `next_base_fee = base_fee * (1 + (gas_used - gas_target) / gas_target / 8)`
pub fn next_base_fee(base_fee: u128, gas_used: u64, gas_target: u64) -> u128 {
    if gas_used == gas_target {
        return base_fee;
    }
    if gas_used > gas_target {
        let delta = gas_used - gas_target;
        let increase = base_fee * delta as u128 / gas_target as u128 / 8;
        base_fee + increase.max(1)
    } else {
        let delta = gas_target - gas_used;
        let decrease = base_fee * delta as u128 / gas_target as u128 / 8;
        base_fee.saturating_sub(decrease)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tx(max_fee: u128, priority: u128) -> Eip1559Transaction {
        Eip1559Transaction {
            chain_id: 1,
            max_fee_per_gas: max_fee,
            max_priority_fee_per_gas: priority,
            gas_limit: 21_000,
        }
    }

    #[test]
    fn effective_price_capped_at_max_fee() {
        // base=10, priority=5 → 15 > max_fee=12 → capped at 12
        assert_eq!(effective_gas_price(&tx(12, 5), 10), 12);
    }

    #[test]
    fn effective_price_uncapped() {
        // base=10, priority=2 → 12 <= max_fee=20
        assert_eq!(effective_gas_price(&tx(20, 2), 10), 12);
    }

    #[test]
    fn can_include_at_base_fee() {
        assert!(can_include(&tx(100, 0), 100));
        assert!(!can_include(&tx(99, 0), 100));
    }

    #[test]
    fn base_fee_increases_when_full() {
        let base = 1_000_000_000u128; // 1 gwei
        let target = 15_000_000u64;
        let next = next_base_fee(base, target * 2, target);
        assert!(next > base);
    }

    #[test]
    fn base_fee_stays_when_at_target() {
        let base = 1_000_000_000u128;
        let target = 15_000_000u64;
        assert_eq!(next_base_fee(base, target, target), base);
    }
}
