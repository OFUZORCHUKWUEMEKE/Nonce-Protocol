use anchor_lang::{prelude, solana_program::native_token::sol_to_lamports};
use litesvm::LiteSVM;
use anchor_spl::token_interface::spl_token_2022::{
    extension::StateWithExtensions, solana_program::program_pack::Pack,
    state::Account as SplAccount,
};
use anchor_spl::{
    associated_token::spl_associated_token_account,
    token_2022::spl_token_2022::{self},
};
pub use nonce;
use solana_program;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

pub struct NonceUtils {
    svm_engine: LiteSVM,
    pub payer: Pubkey,
}

pub const ACTORS: [&str; 3] = ["admin", "emeke", "ekenem"];

impl NonceUtils {
    pub fn new() -> Self {
        let svm_engine = LiteSVM::new()
            .with_sigverify(false)
            .with_blockhash_check(false)
            .with_transaction_history(0)
            .with_spl_programs();

        let mut env = Self {
            svm_engine,
            payer: Pubkey::default(),
        };

        env
    }

    pub fn airdrop(&mut self, pubkey: &Pubkey) {
        self.svm_engine
            .airdrop(pubkey, sol_to_lamports(100.0))
            .unwrap();
    }
}

pub fn make_address(string: &str) -> Pubkey {
    let mut array: [u8; 32] = [0; 32];

    for (index, byte) in string.bytes().enumerate() {
        array[index] = byte;
    }

    Pubkey::new_from_array(array)
}

pub trait InstructionGenerator {
    fn accounts(&self) -> Vec<AccountMeta>;
    fn instruction(&self) -> Instruction;
}

trait AccountMetaVecExt {
    fn append_payer(&mut self, payer: Pubkey) -> &mut Self;
    fn append_system_program(&mut self) -> &mut Self;
    fn append_token_program(&mut self) -> &mut Self;
    fn append_associated_token_program(&mut self) -> &mut Self;
}

impl AccountMetaVecExt for Vec<AccountMeta> {
    fn append_payer(&mut self, payer: Pubkey) -> &mut Self {
        self.push(AccountMeta::new_readonly(payer, false));

        self
    }

    fn append_system_program(&mut self) -> &mut Self {
        self.push(AccountMeta::new_readonly(
            solana_sdk::system_program::ID,
            false,
        ));
        self
    }

    fn append_token_program(&mut self) -> &mut Self {
        self.push(AccountMeta::new_readonly(anchor_spl::token::ID, false));

        self
    }

    fn append_associated_token_program(&mut self) -> &mut Self {
        self.push(AccountMeta::new_readonly(
            spl_associated_token_account::id(),
            false,
        ));

        self
    }
}
