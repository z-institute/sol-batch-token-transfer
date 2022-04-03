mod util;
use dotenv::dotenv;
use std::env;
use solana_client::rpc_client::RpcClient;
use spl_token;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use spl_associated_token_account::{create_associated_token_account,get_associated_token_address};
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct Record {
    pub address: String,
    pub tx_hash: String,
}


fn main() {
    dotenv().ok();
    let records = util::read_from_file("data.csv").unwrap();

    let mut wtr = csv::Writer::from_path("done.csv").unwrap();
    wtr.write_record(&["address", "amount", "tx_hash"]).unwrap();

    for record in records.into_iter() {
        println!("{:?}", record);
        // record.address
        // record.amount

        let key_pair:Keypair = Keypair::from_base58_string(&env::var("KEY").unwrap());
        let mut ins: Vec<Instruction> = vec![];
        let wallet_publickey = key_pair.pubkey();
        let fee_payer = Some(&wallet_publickey);
        let signers: Vec<&Keypair> = vec![&key_pair];
        // change RPC endpoint here
        let rpc_url: String = env::var("NETWORK_RPC").unwrap();
        let commitment = CommitmentConfig::confirmed();
        let rpc_client = RpcClient::new_with_commitment(&rpc_url, commitment);
        let recent = rpc_client
        .get_latest_blockhash()
        .expect("failed to get recent blockhash");

        let receiver_pub = util::get_pub(&record.address);
        let token_mint_pub = util::get_pub(&env::var("TOKEN_MINT_ADDR").unwrap());

        let ata = get_associated_token_address(&receiver_pub,&token_mint_pub);
        let ata_sender = get_associated_token_address(&wallet_publickey,&token_mint_pub);
        let create_ata_ins = create_associated_token_account(&wallet_publickey , &receiver_pub, &token_mint_pub);
        // creat if associated token account does not exist
        if rpc_client.get_account_with_commitment(&ata, CommitmentConfig::processed()).unwrap().value.is_none() {
            ins.push(create_ata_ins);
        }
        ins.push(spl_token::instruction::transfer(
            &spl_token::id(),
            &ata_sender,
            &ata,
            &wallet_publickey,
            &[],
            spl_token::ui_amount_to_amount(record.amount.parse::<f64>().unwrap(), env::var("TOKEN_DECIMAL").unwrap().parse::<u8>().unwrap()), // convert to raw amount according to the token decimal
        ).unwrap());

        let mut tx = Transaction::new_with_payer(&ins, fee_payer);
        tx.sign(&signers, recent);
        // let messagee = encode(tx.message_data());

        // let simulation = rpc_client.simulate_transaction(&tx);
        // println!("{:?}", simulation);
        let send = rpc_client.send_and_confirm_transaction_with_spinner(&tx);
        // println!(
        //     "tx: {:?} \nmint:{:?}\nresult:{:?}",
        //     messagee,
        //     wallet_publickey,
        //     send
        // );
        let tx_hash = send.unwrap();

        println!("Tx hash: {:?}", &tx_hash);
        wtr.serialize((record.address, record.amount, tx_hash.to_string())).unwrap();

    }
    wtr.flush().unwrap();

    // println!("Hello, world!");
}
