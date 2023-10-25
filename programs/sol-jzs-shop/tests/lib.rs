use anchor_lang::{system_program, InstructionData};
// use borsh::BorshDeserialize;
use anchor_lang::ToAccountMetas;
use sol_jzs_shop;
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
// use std::mem;

#[tokio::test]
async fn testing_initialize() {
    let mut program = ProgramTest::new(
        "sol_jzs_shop",
        sol_jzs_shop::id(),
        processor!(sol_jzs_shop::entry),
    );

    let system_program = system_program::ID;
    let token_program_id = anchor_spl::token::ID;
    // let rent = solana_program::sysvar::rent::ID;
    // let associated_token_program_id = anchor_spl::associated_token::ID;

    let owner = Keypair::new();

    program.add_account(
        owner.pubkey(),
        Account {
            lamports: 1_000_000_000,
            ..Account::default()
        },
    );
    // let store = Pubkey::find_program_address(&[b"seed"], &sol_jzs_shop::id()).0;
    let store = Pubkey::find_program_address(
        &[&token_program_id.to_bytes(), &owner.pubkey().to_bytes()],
        &sol_jzs_shop::id(),
    )
    .0;
    // println!("{store}");

    let initalize_ix = Instruction {
        program_id: sol_jzs_shop::id(),
        data: sol_jzs_shop::instruction::InitializeStore {
            initial_price: 2,
            // token_mint: token_program_id,
        }
        .data(),
        accounts: sol_jzs_shop::accounts::InitializeStore {
            store,
            user: owner.pubkey(),
            system_program,
        }
        .to_account_metas(Some(true)),
    };
    // println!("{:#?}", initalize_ix);
    let mut program_context = program.start_with_context().await;

    let mut initalize_tx = Transaction::new_with_payer(&[initalize_ix], Some(&owner.pubkey()));
    let recent_blockhash = program_context.last_blockhash.clone();

    initalize_tx.partial_sign(&[&owner], recent_blockhash);
    println!("{:#?}", initalize_tx);
    let result = program_context
        .banks_client
        .process_transaction_with_metadata(initalize_tx)
        // .process_transaction(initalize_tx)
        .await
        .unwrap();
    println!("{:#?}", result);
    assert_eq!(
        &result.metadata.unwrap().return_data.unwrap().data[4..],
        "Store initialized".as_bytes()
    );

    // test update price
    let price = 200;
    let price_update_ix = Instruction {
        program_id: sol_jzs_shop::ID,
        // data: sol_jzs_shop::instruction::Initialize {}.data(),
        data: sol_jzs_shop::instruction::UpdatePrice { price }.data(),
        accounts: sol_jzs_shop::accounts::UpdatePrice {
            store,
            user: owner.pubkey(),
        }
        .to_account_metas(None),
    };
    // let mut program_context = program.start_with_context().await;

    let mut price_update_tx =
        Transaction::new_with_payer(&[price_update_ix], Some(&owner.pubkey()));
    let recent_blockhash = program_context.last_blockhash.clone();

    price_update_tx.partial_sign(&[&owner], recent_blockhash);
    // println!("{:#?}", initalize_tx);
    let result = program_context
        .banks_client
        .process_transaction_with_metadata(price_update_tx)
        // .process_transaction(initalize_tx)
        .await
        .unwrap();
    println!("{:#?}", result);
    let returned = result.metadata.unwrap().return_data.unwrap().data;
    assert_eq!(returned, &[price as u8]);
}
