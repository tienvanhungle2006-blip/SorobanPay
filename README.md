# SorobanPay\
<img width="2559" height="1599" alt="Screenshot 2026-03-26 155828" src="https://github.com/user-attachments/assets/0cf8fdad-27cf-4177-bb59-f01ccf540798" />


# 🛡️ Soroban Escrow Smart Contract

Một hợp đồng thông minh (Smart Contract) đơn giản và an toàn được xây dựng trên nền tảng **Stellar Soroban**. Dự án này cho phép thực hiện giao dịch trung gian (Escrow) giữa Khách hàng (Client) và Người làm tự do (Freelancer) bằng các token tiêu chuẩn SEP-41.

## 📝 Tính năng chính

* **Khởi tạo (Init):** Thiết lập cố định địa chỉ Client, Freelancer và loại Token giao dịch.
* **Nạp tiền an toàn:** Client nạp tiền vào hợp đồng, tiền được khóa an toàn bởi mã nguồn minh bạch.
* **Giải ngân tự động:** Chỉ Client mới có quyền phê duyệt giải ngân tiền cho Freelancer sau khi nghiệm thu công việc.
* **Tối ưu lưu trữ:** Cơ chế xóa dữ liệu sau khi hoàn thành để tiết kiệm chi phí lưu trữ trên mạng lưới (Storage footprint).

## 🚀 Hướng dẫn sử dụng

### 1. Cấu hình môi trường
Đảm bảo bạn đã cài đặt `stellar-cli` và cấu hình network `testnet`.

```bash
# Thêm network testnet nếu chưa có
stellar network add --rpc-url https://soroban-testnet.stellar.org:443 --network-passphrase "Test SDF Network ; September 2015" testnet
```

### 2. Triển khai hợp đồng (Deploy)
```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/escrow_contract.wasm \
  --source-account <TEN_GIAO_DICH_CUA_BAN> \
  --network testnet
```
*Lưu lại `Contract ID` trả về (Ví dụ: `CDBJK...`).*

### 3. Khởi tạo giao dịch (Init)
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account <ACCOUNT_NAME> \
  --network testnet \
  -- init \
  --client <ADDRESS_CLIENT> \
  --freelancer <ADDRESS_FREELANCER> \
  --token <ADDRESS_TOKEN>
```

### 4. Nạp tiền (Deposit)
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account <ACCOUNT_NAME> \
  --network testnet \
  -- deposit_simple \
  --amount 1000000000 # (Ví dụ 100 token với 7 chữ số thập phân)
```

### 5. Giải ngân (Release)
Khi công việc hoàn tất, Client gọi lệnh này:
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account <ACCOUNT_NAME> \
  --network testnet \
  -- release \
  --client <ADDRESS_CLIENT>
```

## ⚠️ Lưu ý quan trọng về lỗi `InvalidAction`

Nếu bạn gặp lỗi `VM call trapped: UnreachableCodeReached`, hãy kiểm tra các điều sau:
1. **Dữ liệu trống:** Bạn đã gọi hàm `init` hoặc `create_escrow` chưa? Hàm `release` sẽ lỗi nếu không tìm thấy dữ liệu trong storage.
2. **Quyền truy cập:** Tham số `--client` truyền vào hàm `release` phải khớp chính xác với địa chỉ đã lưu lúc `init`.
3. **Số dư:** Hợp đồng phải có đủ số dư token để thực hiện lệnh chuyển tiền cho Freelancer.

## 🛠 Cấu trúc thư mục
* `src/lib.rs`: Chứa toàn bộ logic hợp đồng thông minh.
* `Cargo.toml`: Cấu hình các dependency của Rust và Soroban SDK.

