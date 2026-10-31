# eips

> **Tầng 3 — Ecosystem Extensions** · Prague-ready EIP hardfork activation

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

---

## Tổng quan

`alloy-eips` cung cấp định nghĩa và logic kích hoạt cho tất cả Ethereum hardforks, từ Frontier đến **Prague**.

| Module | Nội dung |
|--------|----------|
| `hardfork` | `Hardfork` enum, `HardforkConfig`, `mainnet_schedule()` |
| `eip1559` | Fee market: `effective_gas_price()`, `next_base_fee()` |
| `eip4844` | Blob transactions: KZG commitments, `next_blob_base_fee()` |

---

## Cài đặt

```toml
[dependencies]
alloy-eips = { git = "https://github.com/wikialisalabs/eips" }
```

## Sử dụng nhanh

```rust
use alloy_eips::hardfork::{Hardfork, mainnet_schedule};
use alloy_eips::eip1559::{can_include, effective_gas_price, Eip1559Transaction};

// Kiểm tra hardfork activation
let schedule = mainnet_schedule();
let prague = schedule.iter().find(|c| c.hardfork == Hardfork::Prague).unwrap();
println!("Prague active: {}", prague.is_active_at(0, 1_800_000_000));

// EIP-1559 fee calculation
let tx = Eip1559Transaction {
    chain_id: 1,
    max_fee_per_gas: 100,
    max_priority_fee_per_gas: 2,
    gas_limit: 21_000,
};
let price = effective_gas_price(&tx, 90); // base_fee = 90
println!("Effective gas price: {}", price); // 92
```

---

## Kiểm thử

```bash
cargo test
cargo test --all-features
```

---

## Đóng góp

Xem [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Giấy phép

Apache-2.0 © 2026 Wikialisalabs Foundation
