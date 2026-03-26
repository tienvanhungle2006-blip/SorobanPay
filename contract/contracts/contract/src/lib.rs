#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, token, symbol_short, Symbol};

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    // Khởi tạo giao dịch giữ tiền
    pub fn create_escrow(
        env: Env,
        client: Address,
        freelancer: Address,
        token_address: Address,
        amount: i128,
    ) {
        // Xác thực quyền sở hữu của người gửi tiền
        client.require_auth();

        let token_client = token::Client::new(&env, &token_address);
        
        // Chuyển tiền từ Client vào Contract
        token_client.transfer(&client, &env.current_contract_address(), &amount);

        // Lưu trữ thông tin vào Storage của Contract
        env.storage().instance().set(&symbol_short!("amount"), &amount);
        env.storage().instance().set(&symbol_short!("freelan"), &freelancer);
        env.storage().instance().set(&symbol_short!("token"), &token_address);
    }

    // Khách hàng phê duyệt để giải ngân
    pub fn release(env: Env, client: Address) {
        // Chỉ khách hàng mới có quyền gọi hàm này
        client.require_auth();

        let freelancer: Address = env.storage().instance().get(&symbol_short!("freelan")).unwrap();
        let token_address: Address = env.storage().instance().get(&symbol_short!("token")).unwrap();
        let amount: i128 = env.storage().instance().get(&symbol_short!("amount")).unwrap();

        let token_client = token::Client::new(&env, &token_address);
        
        // Chuyển tiền từ Contract cho Freelancer
        token_client.transfer(&env.current_contract_address(), &freelancer, &amount);
        
        // Xóa dữ liệu để giải phóng dung lượng lưu trữ (Tùy chọn)
        env.storage().instance().remove(&symbol_short!("amount"));
    }

    // Hàm này dùng để "cài đặt" các địa chỉ cố định một lần duy nhất
    pub fn init(env: Env, client: Address, freelancer: Address, token: Address) {
        client.require_auth(); // Chỉ người quản trị/client mới được init
        
        env.storage().instance().set(&symbol_short!("client"), &client);
        env.storage().instance().set(&symbol_short!("freelan"), &freelancer);
        env.storage().instance().set(&symbol_short!("token"), &token);
    }

    // Hàm nạp tiền "siêu ngắn" - Không cần địa chỉ nào cả!
    pub fn deposit_simple(env: Env, amount: i128) {
        let client: Address = env.storage().instance().get(&symbol_short!("client")).unwrap();
        let token_addr: Address = env.storage().instance().get(&symbol_short!("token")).unwrap();
        
        client.require_auth();

        let token_client = token::Client::new(&env, &token_addr);
        token_client.transfer(&client, &env.current_contract_address(), &amount);
        
        env.storage().instance().set(&symbol_short!("amount"), &amount);
    }
}