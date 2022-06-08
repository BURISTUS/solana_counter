#![cfg(feature = "test-bpf")]

use std::assert_eq;

use borsh::BorshDeserialize;
use solana_sdk::{
        account::Account,
        instruction::{AccountMeta, Instruction},
        signature::Signer,
        transaction::Transaction,
        pubkey::Pubkey,
    };
use std::mem;
use solana_helloworld::{GreetingsCounter, process_instruction};
use solana_program_test::*;


#[tokio::test]
async fn test_hello_world() {
    let program_id = Pubkey::new_unique();
    let greetings_pubkey = Pubkey::new_unique();

    let mut program_test = ProgramTest::new(
        "solana_helloworld",
        program_id,
        processor!(process_instruction)
    );

    program_test.add_account(
        greetings_pubkey,
        Account{
            lamports: 5,
            data: vec![0_u8; mem::size_of::<u32>()],
            owner: program_id,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    let greeted_account = banks_client
        .get_account(greetings_pubkey)
        .await
        .expect("get account")
        .expect("account not found");
    assert_eq!(
        GreetingsCounter::try_from_slice(&greeted_account.data).unwrap().counter, 0
    );

      let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[0], // ignored but makes the instruction unique in the slot
            vec![AccountMeta::new(greetings_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();


    let greeted_account = banks_client
        .get_account(greetings_pubkey)
        .await
        .expect("get account")
        .expect("account not found");
    assert_eq!(
        GreetingsCounter::try_from_slice(&greeted_account.data).unwrap().counter, 1
    );

      let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[1], // ignored but makes the instruction unique in the slot
            vec![AccountMeta::new(greetings_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let greeted_account = banks_client
        .get_account(greetings_pubkey)
        .await
        .expect("get account")
        .expect("account not found");
    assert_eq!(
        GreetingsCounter::try_from_slice(&greeted_account.data).unwrap().counter, 2
    );


}
