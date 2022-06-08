use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg, pubkey::Pubkey,
    program_error::ProgramError,
};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct GreetingsCounter{
    pub counter: u32
}

entrypoint!(process_instruction);


pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let account = next_account_info(account_iter)?;

    if account.owner != program_id {
        msg!("Greeted account doesn't have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut greeting_counter = GreetingsCounter::try_from_slice(&account.data.borrow())?;
    greeting_counter.counter += 1;
    greeting_counter.serialize(&mut &mut account.data.borrow_mut()[..]);

    msg!("Greeted {} times ", greeting_counter.counter);

    Ok(())
}
#[cfg(test)]
mod test {
    use  super::*;
    use solana_program::clock::Epoch;
    use std::mem;


    #[test]
    fn test_transaction() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default()
        );

        let instruction_data: Vec<u8> = Vec::new();

        let accounts = vec![account];

        assert_eq!(
            GreetingsCounter::try_from_slice(&accounts[0].data.borrow()).unwrap().counter, 0
        );

        process_instruction(&program_id, &accounts, &instruction_data).unwrap();

        assert_eq!(
            GreetingsCounter::try_from_slice(&accounts[0].data.borrow()).unwrap().counter, 1
        );

        process_instruction(&program_id, &accounts, &instruction_data).unwrap();

        assert_eq!(
            GreetingsCounter::try_from_slice(&accounts[0].data.borrow()).unwrap().counter, 2
        );
    }
}
