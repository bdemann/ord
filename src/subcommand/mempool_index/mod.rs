use std::collections::HashMap;

use bitcoin::{OutPoint, Transaction, TxOut, Txid};
use bitcoincore_rpc::{Auth, Client, RpcApi};

use crate::InscriptionId;

// TODO this is completely stand in code it will probably be changed quite a lot
// as we figure out what is going on.
pub struct MempoolIndex {
  pub transactions: Vec<Transaction>,
  pub outpoints: Vec<OutPoint>,
  pub inscriptions_ids: Vec<InscriptionId>,
  pub test: Vec<u64>,
}

impl MempoolIndex {
  pub fn new() -> Self {
    Self {
      transactions: vec![],
      outpoints: vec![],
      inscriptions_ids: vec![],
      test: vec![],
    }
  }

  pub fn update(&mut self) {
    // Clear it out
    // Completely rebuild every time
    self.inscriptions_ids = vec![];
    self.outpoints = vec![];
    self.transactions = vec![];
    self.test.push(match self.test.last() {
      Some(last) => last + 1,
      None => 0,
    })
  }

  pub fn get_all_transactions_from_mempool() -> Result<Transaction, String> {
    Err("".to_string())
  }
}

fn run_bitcoin_test() -> Result<(), String> {
  let rpc_url = "http://127.0.0.1:8332/";
  let rpc_user = "username";
  let rpc_password = "password";

  let rpc_client = Client::new(
    rpc_url,
    Auth::UserPass(rpc_user.to_string(), rpc_password.to_string()),
  )
  .map_err(|err| err.to_string())?;

  let txids = get_mempool_txids(&rpc_client)?;

  println!("Found {} transactions in the mempool", txids.len());

  // let txids: &Vec<Txid> = &txids[..10].into();

  let transactions = get_mempool_transactions(&rpc_client, &txids)?;

  println!(
    "Successfully converted {} Txids to Transactions",
    transactions.len()
  );

  let outputs = get_mempool_outputs(&transactions);

  println!("We found {} outputs", outputs.len());

  Ok(())
}

// fn get_inscription_ids(outputs: &HashMap<OutPoint, TxOut>) -> Vec<InscriptionId> {}

fn do_things_to_transaction(transaction: &Transaction) {
  let my_input = &transaction.input;
}

fn get_mempool_outputs(transactions: &Vec<Transaction>) -> HashMap<OutPoint, TxOut> {
  let mut result = HashMap::new();
  for transaction in transactions {
    for (vout, output) in transaction.output.iter().enumerate() {
      let outpoint = OutPoint {
        txid: transaction.txid(),
        vout: vout as u32,
      };
      result.insert(outpoint, output.clone());
    }
  }
  result
}

fn get_mempool_transactions(
  client: &Client,
  txids: &Vec<Txid>,
) -> Result<Vec<Transaction>, String> {
  let transactions: Vec<Transaction> = txids.iter().fold(vec![], |mut acc, txid| {
    match client.get_raw_transaction(txid, None) {
      Ok(transaction) => acc.push(transaction),
      Err(err) => println!("WARN: {} is weird: {}", txid, err),
    };
    acc
  });

  println!(
    "Of {} txids {} transactions were found",
    txids.len(),
    transactions.len()
  );
  Ok(transactions)
}

fn get_mempool_txids(client: &Client) -> Result<Vec<Txid>, String> {
  // Create an RPC client
  // Send the getrawmempool RPC command
  return client.get_raw_mempool().map_err(|err| err.to_string());
}
