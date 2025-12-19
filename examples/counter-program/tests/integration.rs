use litesvm::LiteSVM;
use {
    solana_keypair::Keypair,
    solana_instruction::Instruction,
    solana_transaction::Transaction,
    solana_native_token::LAMPORTS_PER_SOL,
    solana_pubkey::{pubkey, Pubkey},
    solana_signer::Signer,
    std::path::PathBuf,
};

fn read_program() -> Vec<u8> {
    let mut so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    so_path.push("target/deploy/counter_program.so");
    std::fs::read(so_path).unwrap()
}

const ID: Pubkey = pubkey!("HPoDm7Kf63B6TpFKV7S8YSd7sGde6sVdztiDBEVkfuxz");

#[test]
fn integration_test() {
    let mut svm = LiteSVM::new();
    let kp = Keypair::new();
    let kp_pk = kp.pubkey();

    svm.airdrop(&kp_pk, 10 * LAMPORTS_PER_SOL).unwrap();

    let program_bytes = read_program();

    svm.add_program(ID, &program_bytes).unwrap();

    let ix = Instruction {
        program_id: ID,
        accounts: vec![],
        data: vec![],
    };
    let hash = svm.latest_blockhash();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&kp_pk), &[&kp], hash);

    let res = svm.send_transaction(tx).unwrap();

    println!("meta: {:#?}", res);
}