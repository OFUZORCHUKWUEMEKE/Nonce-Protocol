// use litesvm::LiteSVM;
// use solana_program::{message::Message, pubkey::Pubkey, system_instruction::transfer};
// use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

// fn test() {
//     let from_keypair = Keypair::new();
//     let from = from_keypair.pubkey();
//     let to = Pubkey::new_unique();

//     let mut svm = LiteSVM::new();
//     svm.airdrop(&from, 10_000).unwrap();

//     let instruction = transfer(&from, &to, 64);
//     let tx = Transaction::new(
//         &[&from_keypair],
//         Message::new(&[instruction], Some(&from)),
//         svm.latest_blockhash(),
//     );
//     let tx_res = svm.send_transaction(tx).unwrap();

//     let from_account = svm.get_account(&from);
//     let to_account = svm.get_account(&to);
//     assert_eq!(from_account.unwrap().lamports, 4936);
//     assert_eq!(to_account.unwrap().lamports, 64);
// }
