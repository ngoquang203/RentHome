use solana_client::rpc_client::RpcClient;
use solana_program::program_pack::Pack;
use solana_program::system_instruction::create_account;
use solana_program::system_program;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::system_instruction::transfer;
use solana_sdk::transaction::Transaction;


fn main() {
    // Khởi tạo một client Solana RPC
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    let rpc_client = RpcClient::new(rpc_url);

    // Khởi tạo một keypair
    let payer = Keypair::new();

    // Khởi tạo một keypair cho người nhận
    let recipient = Keypair::new();

    // Số dư của người gửi trước khi chuyển
    let initial_balance = rpc_client
        .get_balance(&payer.pubkey())
        .expect("Lỗi khi lấy số dư");

    // Tạo một giao dịch chuyển tiền từ người gửi đến người nhận
    let transfer_instruction = transfer(&payer.pubkey(), &recipient.pubkey(), 100);

    // Xây dựng transaction
    let mut transaction = Transaction::new_with_payer(
        &[transfer_instruction],
        Some(&payer.pubkey()),
    );

    // Gửi giao dịch
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Lỗi khi lấy latest blockhash");
    
    let result = rpc_client.send_and_confirm_transaction_with_spinner_and_config(
        &transaction,
        CommitmentConfig::processed(),rpc_url
    );

    match result {
        Ok(_) => println!("Giao dịch thành công!"),
        Err(err) => eprintln!("Lỗi: {:?}", err),
    }

    // In số dư sau khi chuyển
    let final_balance = rpc_client
        .get_balance(&payer.pubkey())
        .expect("Lỗi khi lấy số dư");
    println!("Số dư của người gửi sau khi chuyển: {}", final_balance);
}