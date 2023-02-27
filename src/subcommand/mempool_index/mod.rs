use std::collections::HashMap;

use bitcoin::{Transaction, Txid};
use bitcoincore_rpc::{Client, RpcApi};
use anyhow::Error;

use crate::ParsedEnvelope;
use crate::InscriptionId;
use crate::Inscription;

// TODO this is completely stand in code it will probably be changed quite a lot
// as we figure out what is going on.
pub struct MempoolIndex {
  pub client: Client,
  pub transactions: HashMap<Txid, Transaction>,
  pub inscriptions: HashMap<InscriptionId, Inscription>,
}

type Result<T = (), E = Error> = std::result::Result<T, E>;

impl MempoolIndex {
  
  pub fn new(client: Client) -> Self {
    Self {
      client,
      transactions: HashMap::new(),
      inscriptions: HashMap::new(),
    }
  }

  pub fn update(&mut self) -> Result<(), String> {
  
    // Get the txids of all transactions in the mempool.
    let txids = self.client.get_raw_mempool().map_err(|err| err.to_string())?;
    println!("Found {} Txids in the mempool", txids.len());
  
    // Get the transactions of all txids in the mempool.
    self.transactions = get_mempool_transactions(&self.client, &txids)?;
    println!("Successfully converted {} Txids to Transactions", self.transactions.len());
  
    // Get the inscriptions of all transactions in the mempool.
    self.inscriptions = get_mempool_inscriptions(&self.transactions);
    println!("We found {} inscriptions", self.inscriptions.len());
  
    Ok(())
  }

}

fn get_mempool_transactions(
  client: &Client,
  txids: &Vec<Txid>,
) -> Result<HashMap<Txid, Transaction>, String> {

  let transactions: HashMap<Txid, Transaction> = txids.iter()
    .filter_map(|txid| {
      client.get_raw_transaction(txid, None)
        .map(|transaction| (*txid, transaction))
        .map_err(|err| {
          println!("WARN: {} is weird: {}", txid, err);
        })
        .ok()
    })
    .collect();

  println!(
    "Of {} txids {} transactions were found",
    txids.len(),
    transactions.len()
  );
  Ok(transactions)
}

fn get_mempool_inscriptions(
  transactions: &HashMap<Txid, Transaction>
) -> HashMap<InscriptionId, Inscription> {
  let mut result = HashMap::new();
  
  // Deduce the inscriptions from the transactions.
  for (txid, transaction) in transactions {
  
    let mut id_counter = 0;
    let mut envelopes = ParsedEnvelope::from_transaction(transaction).into_iter().peekable();
    
    while let Some(envelope) = envelopes.peek() {
      
      let inscription_id = InscriptionId {
        txid: txid.clone(),
        index: id_counter,
      };

      result.insert(inscription_id, envelope.payload.clone());

      envelopes.next();
      id_counter += 1;
    }
  }

  result
}
