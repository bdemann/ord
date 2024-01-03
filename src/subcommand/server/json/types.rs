use bitcoin::{BlockHash, Transaction, TxOut, Txid};
use serde::{Deserialize, Serialize};

use crate::{
  decimal_sat::DecimalSat, degree::Degree, height::Height, Chain, Epoch, InscriptionId, Rarity,
  SatPoint,
};

pub(super) type AddressJson = String;
pub(super) type ScriptJson = String;

#[derive(Deserialize, Serialize, Clone)]
pub(super) struct LightInscriptionJson {
  pub inscription_id: InscriptionId,
  pub content_len: Option<usize>,
  pub transaction: String,
  pub content_type: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub(super) struct InscriptionJson {
  pub inscription_id: InscriptionId,
  pub address: Option<AddressJson>,
  pub output_value: Option<u64>,
  pub sat: Option<SatJson>,
  pub content_len: Option<usize>,
  pub genesis_height: u32,
  pub genesis_fee: u64,
  pub timestamp: u32,
  pub transaction: String,
  pub location: String,
  pub output: Option<TxOut>,
  pub offset: u64,
  pub chain: Chain,
  pub content_type: Option<String>,
  pub next: Option<InscriptionId>,
  pub number: i32,
  pub previous: Option<InscriptionId>,
  pub satpoint: SatPoint,
  pub original_owner: Option<AddressJson>,
}

#[derive(Deserialize, Serialize, Clone)]
pub(super) struct SatJson {
  pub number: u64,
  pub decimal: DecimalSat,
  pub degree: Degree,
  pub percentile: String,
  pub name: String,
  pub cycle: u32,
  pub epoch: Epoch,
  pub period: u32,
  pub block: Height,
  pub offset: u64,
  pub rarity: Rarity,
}

#[derive(Deserialize, Serialize, Clone)]
pub(super) struct BlockJson {
  pub hash: BlockHash,
  pub transactions: Vec<Transaction>,
}

#[derive(Deserialize, Serialize, Clone)]
pub(super) struct OutputJson {
  pub inscriptions: Vec<InscriptionId>,
  pub value: u64,
  pub script_pubkey: ScriptJson,
  pub address: Option<AddressJson>,
  pub transaction: Txid,
}

// Can we get rid of these? What were they used for?
#[derive(Deserialize, Serialize, Clone)]
struct DecimalJson {
  height: u64,
  offset: u64,
}

#[derive(Deserialize, Serialize, Clone)]
struct DegreeJson {
  hour: u64,
  minute: u64,
  second: u64,
  third: u64,
}
