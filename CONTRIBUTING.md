# Hướng dẫn Đóng góp — `eips`

Mỗi thay đổi trong **`alloy-eips`** liên quan trực tiếp đến quy chuẩn Ethereum on-chain — độ chính xác là ưu tiên số 1.

---

## Mục lục

1. [Môi trường Phát triển](#1-môi-trường-phát-triển)
2. [Cấu trúc Repository](#2-cấu-trúc-repository)
3. [Quy trình Đóng góp](#3-quy-trình-đóng-góp)
4. [Thêm EIP mới](#4-thêm-eip-mới)
5. [Thêm Hardfork mới](#5-thêm-hardfork-mới)
6. [Viết Tests](#6-viết-tests)
7. [Commit Convention](#7-commit-convention)
8. [Pull Request Checklist](#8-pull-request-checklist)

---

## 1. Môi trường Phát triển

```bash
git clone https://github.com/wikialisalabs/eips.git
cd eips
rustup show
cargo build
cargo test
```

---

## 2. Cấu trúc Repository

```
eips/
├── src/
│   ├── lib.rs          # Crate root
│   ├── hardfork.rs     # Hardfork enum + activation logic
│   ├── eip1559.rs      # EIP-1559 fee market
│   └── eip4844.rs      # EIP-4844 blob transactions
├── Cargo.toml
├── README.md
└── CONTRIBUTING.md
```

---

## 3. Quy trình Đóng góp

1. Mỗi EIP nên có Issue riêng với link tới EIP spec chính thức trên [eips.ethereum.org](https://eips.ethereum.org).
2. Fork → branch `feat/eip-<number>` (e.g. `feat/eip-7702`).
3. Implement + tests (xem mục 6).
4. Chạy CI cục bộ.
5. Mở PR với link EIP spec trong description.

---

## 4. Thêm EIP mới

1. Tạo file `src/eip<number>.rs`.
2. Add `pub mod eip<number>;` vào `src/lib.rs`.
3. Implement structs / functions theo EIP spec chính thức.
4. Viết test với **numeric examples** lấy từ chính spec EIP.

**Template:**

```rust
//! EIP-XXXX: <tên EIP>
//!
//! Specification: https://eips.ethereum.org/EIPS/eip-XXXX

// ... implementation

#[cfg(test)]
mod tests {
    use super::*;

    /// Test case from EIP-XXXX spec, section Y.
    #[test]
    fn spec_example_1() { ... }
}
```

---

## 5. Thêm Hardfork mới

1. Thêm variant vào `Hardfork` enum trong `src/hardfork.rs` — **đúng thứ tự chronological**.
2. Thêm entry vào `mainnet_schedule()` với block number / timestamp chính xác từ [etherscan.io](https://etherscan.io).
3. Cập nhật `name()` match arm.
4. Viết test `hardfork_ordering()` vẫn pass.

---

## 6. Viết Tests

Mọi số liệu trong tests phải có nguồn gốc rõ ràng:

```rust
/// Test case: EIP-1559 spec example — https://eips.ethereum.org/EIPS/eip-1559
#[test]
fn eip1559_spec_example() { ... }
```

```bash
cargo test
cargo test --all-features
```

---

## 7. Commit Convention

```
feat(eips): add EIP-7702 set-code transaction support
feat(hardfork): add Osaka hardfork to mainnet schedule
fix(eip1559): correct next_base_fee when gas_used < target
test(eip4844): add blob base fee calculation from spec
docs(eip1559): clarify effective_gas_price formula comment
```

---

## 8. Pull Request Checklist

```
[ ] cargo fmt --all                          → no diff
[ ] cargo clippy --all-features -- -D warnings → 0 warnings
[ ] cargo test --all-features                → all pass
[ ] EIP spec linked in PR description        ✓
[ ] Numeric test values sourced from spec    ✓
[ ] Hardfork order maintained (if applicable)✓
[ ] PR linked to Issue                       ✓
```

---

## Liên hệ

- **Email:** root@wikialisalabs.axvn.vn
- **Issues:** https://github.com/wikialisalabs/eips/issues

*Cảm ơn bạn đã đóng góp! 🦀📜*
