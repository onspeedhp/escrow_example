# Escrow Starter

Anchor escrow starter để clone về sửa tiếp cho flow buyer-seller.

Repo này đã có sẵn form cơ bản:

- `init_escrow`: buyer lock token vào vault.
- `confirm_receipt`: buyer xác nhận nhận hàng, seller nhận 100%.
- `resolve_partial`: buyer nhập số lượng thực nhận, seller nhận một phần, buyer được refund phần còn lại.
- `claim_timeout`: seller claim nếu quá deadline mà buyer không phản hồi.

## Tech Stack

- Solana + Anchor `0.30.1`
- Rust smart contract
- TypeScript tests
- SPL Token

## Cài Đặt

```bash
git clone https://github.com/onspeedhp/escrow_example.git
cd escrow_example
yarn install
```

Cần có:

- Rust
- Solana CLI
- Anchor CLI `0.30.1`
- Node.js/Yarn

Nếu đang dùng `avm`:

```bash
avm use 0.30.1
```

## Env

Tạo file `.env` từ mẫu:

```bash
cp .env.example .env
```

`PRIVATE_KEY` là base58 secret key của ví dev dùng trong test để mint token/fund account.

## Chạy Nhanh

Build program:

```bash
yarn build
```

Chạy test local:

```bash
yarn test
```

Format code:

```bash
yarn lint:fix
```

## File Nên Xem Trước

- `programs/escrow-example/src/state.rs`: state của escrow account.
- `programs/escrow-example/src/instructions/init_escrow.rs`: tạo escrow và lock token vào vault.
- `programs/escrow-example/src/instructions/confirm_receipt.rs`: giải ngân 100%.
- `programs/escrow-example/src/instructions/resolve_partial.rs`: giải ngân một phần.
- `programs/escrow-example/src/instructions/claim_timeout.rs`: claim sau timeout.
- `tests/escrow-example.ts`: ví dụ gọi program bằng TypeScript.

## Flow Hiện Có

```text
buyer -> init_escrow -> token vào vault PDA
buyer -> confirm_receipt -> seller nhận 100%
buyer -> resolve_partial(qty) -> seller nhận theo qty, buyer nhận refund
seller -> claim_timeout -> seller nhận 100% sau deadline
```

## Ghi Chú

- Với hackathon, nên chạy trên localnet/devnet trước.
- Nếu chưa kịp dùng USDC thật, có thể dùng test SPL token giống test hiện tại.
- Đây là starter, chưa phải production escrow.
- Nếu cần chặt hơn, thêm bước seller cùng ký `resolve_partial`.
