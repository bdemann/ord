use bitcoin::Txid;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

use crate::{subcommand::server::error::ServerResult, Index, InscriptionId};

use super::{
  block::get_latest_block, get_inscriptions::get_inscription_ids_for_block, handle_json_result,
  types::BlockJson,
};

#[derive(Deserialize, Serialize, Clone)]
pub(super) struct InscriptionIdJson {
  pub inscription_id: InscriptionId,
  pub transaction_id: Txid,
}

#[derive(Deserialize, Serialize, Clone)]
pub(super) struct TransactionsWithInscriptionId {
  pub transaction_id: Txid,
  pub inscription_ids: Vec<InscriptionId>,
}

pub(super) fn create_inscription_json(inscription_id: &InscriptionId) -> InscriptionIdJson {
  InscriptionIdJson {
    inscription_id: inscription_id.clone(),
    transaction_id: inscription_id.txid,
  }
}

pub(super) fn get_inscription_ids(block_index: u64, index: &Arc<Index>) -> ServerResult<String> {
  let block_option = index.get_block_by_height(block_index)?;
  let block = match block_option {
    Some(block) => block,
    None => return Ok("[]".to_string()),
  };

  let inscriptions_ids: Vec<InscriptionIdJson> = get_inscription_ids_for_block(&block, index)?
    [0..100]
    .into_iter()
    .map(|thing| create_inscription_json(thing))
    .collect();
  Ok(handle_json_result(serde_json::to_string(&inscriptions_ids)))
}

pub(super) fn get_inscription_ids_by_transaction(
  block_index: u64,
  index: &Arc<Index>,
) -> ServerResult<String> {
  let block_option = index.get_block_by_height(block_index)?;
  let block = match block_option {
    Some(block) => block,
    None => return Ok("[]".to_string()),
  };

  let inscriptions_ids: Vec<InscriptionIdJson> = get_inscription_ids_for_block(&block, index)?
    [0..100]
    .into_iter()
    .map(|thing| create_inscription_json(thing))
    .collect();

  let mut map: HashMap<Txid, Vec<InscriptionId>> = HashMap::new();

  for inscription_id in inscriptions_ids {
    map
      .entry(inscription_id.transaction_id)
      .or_insert_with(Vec::new)
      .push(inscription_id.inscription_id)
  }

  // Transform the hashmap into the desired Vec of TransactionsWithInscriptionId
  let result: Vec<TransactionsWithInscriptionId> = map
    .into_iter()
    .map(
      |(transaction_id, inscription_ids)| TransactionsWithInscriptionId {
        transaction_id,
        inscription_ids,
      },
    )
    .collect();
  Ok(handle_json_result(serde_json::to_string(&result)))
}
