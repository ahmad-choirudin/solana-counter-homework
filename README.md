# solana-counter-homework

A beginner Solana program (Rust, native `solana-program`) that stores a counter on-chain.

## Instructions

Instruction data uses Borsh: one byte for the variant, then a `u32` amount in little-endian.

| Variant | Byte | Effect |
| --- | --- | --- |
| `Increment(u32)` | `0` | Adds the amount to the counter (fails on overflow) |
| `Decrement(u32)` | `1` | Subtracts the amount (fails with custom error `0` on underflow) |

Example: `[0, 5, 0, 0, 0]` increments by 5.

## Account

The counter account must be owned by this program, writable, and allocated with at least 8 bytes (`CounterState::SIZE`). A zero-filled account is a valid counter starting at 0.

## Build and test

```bash
cargo check
cargo test
```

Build for deployment with your Solana CLI's SBF build tool (for example `cargo build-sbf`).

## Notes

Anyone can send instructions to a counter account that this program owns; there is no authority check. This is a homework project and has not been audited.

## License

MIT
