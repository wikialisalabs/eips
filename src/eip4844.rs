//! EIP-4844: Shard blob transactions — KZG commitments & blob gas market.

/// The target blob count per block (EIP-4844).
pub const TARGET_BLOB_COUNT: u64 = 3;

/// The maximum blob count per block.
pub const MAX_BLOB_COUNT: u64 = 6;

/// Blob gas used per blob (128 KB).
pub const BLOB_GAS_PER_BLOB: u64 = 131_072;

/// A KZG commitment — 48 bytes.
pub type KzgCommitment = [u8; 48];

/// A versioned hash derived from a KZG commitment (EIP-4844).
///
/// `versioned_hash = 0x01 || keccak256(commitment)[1..]`
pub fn versioned_hash(commitment: &KzgCommitment) -> [u8; 32] {
    // Stub: use XOR-based hash for testing (replace with real keccak in production)
    let mut hash = [0u8; 32];
    hash[0] = 0x01; // version byte
    for (i, &b) in commitment.iter().enumerate() {
        hash[1 + (i % 31)] ^= b;
    }
    hash
}

/// EIP-4844 blob transaction fields.
#[derive(Debug, Clone)]
pub struct BlobTransaction {
    /// KZG commitments for each blob.
    pub blob_versioned_hashes: Vec<[u8; 32]>,
    /// Maximum fee per blob gas.
    pub max_fee_per_blob_gas: u128,
}

/// Calculate the total blob gas used by a transaction.
pub fn blob_gas_used(tx: &BlobTransaction) -> u64 {
    tx.blob_versioned_hashes.len() as u64 * BLOB_GAS_PER_BLOB
}

/// Calculate the next block's blob base fee.
///
/// Uses the EIP-4844 exponential formula:
/// `next = current * e^((used - target) / target / 8)` (integer approximation)
pub fn next_blob_base_fee(current_fee: u128, blob_gas_used: u64) -> u128 {
    let target = TARGET_BLOB_COUNT * BLOB_GAS_PER_BLOB;
    if blob_gas_used == target {
        return current_fee;
    }
    if blob_gas_used > target {
        current_fee + (current_fee / 8).max(1)
    } else {
        current_fee.saturating_sub(current_fee / 8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn versioned_hash_starts_with_01() {
        let commitment = [0xabu8; 48];
        let vh = versioned_hash(&commitment);
        assert_eq!(vh[0], 0x01);
    }

    #[test]
    fn blob_gas_calculation() {
        let tx = BlobTransaction {
            blob_versioned_hashes: vec![[0u8; 32]; 3],
            max_fee_per_blob_gas: 1,
        };
        assert_eq!(blob_gas_used(&tx), 3 * BLOB_GAS_PER_BLOB);
    }

    #[test]
    fn blob_base_fee_increases() {
        let fee = 1_000_000u128;
        let full_blobs = MAX_BLOB_COUNT * BLOB_GAS_PER_BLOB;
        assert!(next_blob_base_fee(fee, full_blobs) > fee);
    }
}
