#![allow(unused)]

use hayabusa::prelude::Discriminator;
use litesvm::LiteSVM;
use solana_sdk::{
    account::Account, instruction::{AccountMeta, Instruction}, pubkey::Pubkey, signature::Keypair, signer::Signer, system_program, transaction::Transaction, pubkey,
};
use spl_token::{state::{Account as TokenAccount, Mint}, solana_program::program_pack::Pack};

#[test]
fn integration() {
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../target/deploy/counter_program.so");

    let program_id = pubkey!("HPoDm7Kf63B6TpFKV7S8YSd7sGde6sVdztiDBEVkfuxz");

    svm.add_program(program_id, program_bytes);

    let keypair = Keypair::new();
    let user = keypair.pubkey();

    svm.airdrop(&user, 1_000_000_000_000).unwrap();

    let counter_account_data = pack_zc_account(CounterAccount { counter: 0 });
    let counter_account_pk = Pubkey::new_unique();
    let counter_account = Account {
        lamports: svm.minimum_balance_for_rent_exemption(counter_account_data.len()),
        data: counter_account_data,
        owner: program_id,
        executable: false,
        rent_epoch: 0,
    };

    svm.set_account(counter_account_pk, counter_account).unwrap();

    let ix_data = {
        const UPDATE_COUNTER_DISCRIMINATOR: [u8; 8] = [231, 120, 160, 18, 72, 164, 104, 62];
        let mut data = UPDATE_COUNTER_DISCRIMINATOR.to_vec();
        data.extend_from_slice(&1u64.to_le_bytes());
        data
    };

    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(user, true),
            AccountMeta::new(counter_account_pk, false),
        ],
        data: ix_data,
    };

    let tx = Transaction::new_signed_with_payer(&[ix], Some(&user), &[&keypair], svm.latest_blockhash());

    let res = svm.send_transaction(tx);

    println!("Transaction result: {:#?}", res);

}

#[test]
fn integration2() {
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../target/deploy/counter_program.so");

    let program_id = pubkey!("HPoDm7Kf63B6TpFKV7S8YSd7sGde6sVdztiDBEVkfuxz");

    svm.add_program(program_id, program_bytes);

    let keypair = Keypair::new();
    let user = keypair.pubkey();

    let target_keypair = Keypair::new();
    let target = target_keypair.pubkey();

    svm.airdrop(&user, 1_000_000_000_000).unwrap();

    let ix_data = {
        const INITIALIZE_COUNTER_DISCRIMINATOR: [u8; 8] = [184, 155, 169, 181, 122, 145, 244, 45];
        let data = INITIALIZE_COUNTER_DISCRIMINATOR.to_vec();
        data
    };

    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(user, true),
            AccountMeta::new(target, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: ix_data,
    };

    let tx = Transaction::new_signed_with_payer(&[ix], Some(&user), &[&keypair, &target_keypair], svm.latest_blockhash());

    let res = svm.send_transaction(tx);

    println!("Transaction result: {:#?}", res);

}

fn pack_zc_account<T: bytemuck::NoUninit + Discriminator>(account: T) -> Vec<u8> {
    let mut data = T::DISCRIMINATOR.to_vec();
    data.extend_from_slice(bytemuck::bytes_of(&account));
    data
}

#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy, Discriminator)]
#[repr(C)]
struct CounterAccount {
    counter: u64,
}