use {
    lotic::pinocchio::{error::ProgramError, Address},
    mollusk_svm::{result::Check, Mollusk},
    solana_sdk::{
        account,
        instruction::{AccountMeta, Instruction},
    },
};

#[test]
fn test_signer_success() {
    let program_id = Address::from_str_const("2JF8AjwkmCz6brkAkJf8NEEKhg89a8KrTuDZiZ5cVdS2");
    let mollusk = Mollusk::new(&program_id, "../../target/deploy/test_constraint_signer");

    let data_account = Address::new_unique();
    let account = Address::new_unique();
    let accounts = vec![
        (account, account::Account::default()),
        (data_account, account::Account::default()),
    ];

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(data_account, false),
            AccountMeta::new(account, true),
        ],
        data: vec![0],
    };

    mollusk.process_and_validate_instruction(&instruction, &accounts, &[Check::success()]);
}

#[test]
fn test_signer_failure() {
    let program_id = Address::from_str_const("2JF8AjwkmCz6brkAkJf8NEEKhg89a8KrTuDZiZ5cVdS2");
    let mollusk = Mollusk::new(&program_id, "../../target/deploy/test_constraint_signer");

    let data_account = Address::new_unique();
    let account = Address::new_unique();
    let accounts = vec![
        (account, account::Account::default()),
        (data_account, account::Account::default()),
    ];

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(data_account, true),
            AccountMeta::new(account, false),
        ],
        data: vec![0],
    };

    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::MissingRequiredSignature)],
    );
}
