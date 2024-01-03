use std::sync::Arc;

use crate::{
  subcommand::server::error::{OptionExt, ServerResult},
  templates::ServerConfig,
  Index, InscriptionId,
};

use super::types::{InscriptionJson, SatJson};

pub(super) fn build_inscription(
  inscription_id: &InscriptionId,
  index: &Arc<Index>,
  server_config: &Arc<ServerConfig>,
) -> ServerResult<InscriptionJson> {
  let inscription_id = inscription_id.clone();

  let entry = index
    .get_inscription_entry(inscription_id)?
    .ok_or_not_found(|| format!("inscription {inscription_id}"))?;

  let inscription = index
    .get_inscription_by_id(inscription_id)?
    .ok_or_not_found(|| format!("inscription {inscription_id}"))?;

  let satpoint = index
    .get_inscription_satpoint_by_id(inscription_id)?
    .ok_or_not_found(|| format!("inscription {inscription_id}"))?;

  let transaction = match index.get_transaction(satpoint.outpoint.txid) {
    Ok(transaction) => transaction,
    Err(_) => match index.gettransaction(satpoint.outpoint.txid) {
      Ok(transaction) => transaction,
      Err(_) => None,
    },
  };

  let output = match &transaction {
    Some(transaction) => Some(
      transaction
        .output
        .clone()
        .into_iter()
        .nth(satpoint.outpoint.vout.try_into().unwrap())
        .ok_or_not_found(|| format!("inscription {inscription_id} current transaction output"))?,
    ),
    None => None,
  };

  let address = match &output {
    Some(output) => server_config
      .chain
      .address_from_script(&output.script_pubkey)
      .ok(),
    None => None,
  }
  .map(|address| address.to_string());

  let output_value = match &output {
    Some(output) => Some(output.value),
    None => None,
  };

  let previous = if let Some(previous) = entry.inscription_number.checked_sub(1) {
    if let Ok(previous) = index.get_inscription_id_by_inscription_number(previous) {
      previous
    } else {
      None
    }
  } else {
    None
  };

  let next = index.get_inscription_id_by_inscription_number(entry.inscription_number + 1)?;
  let sat_json = match entry.sat {
    Some(sat) => Some(SatJson {
      number: sat.n(),
      decimal: sat.decimal(),
      degree: sat.degree(),
      percentile: sat.percentile(),
      name: sat.name(),
      cycle: sat.cycle(),
      epoch: sat.epoch(),
      period: sat.period(),
      block: sat.height(),
      offset: sat.third(),
      rarity: sat.rarity(),
    }),
    None => None,
  };

  let genesis_transaction_id = inscription_id.txid;
  let genesis_transaction = index.get_transaction(genesis_transaction_id)?;
  let genesis_output = match genesis_transaction {
    Some(transaction) => transaction
      .output
      .into_iter()
      .nth(satpoint.offset.try_into().unwrap()),
    None => None,
  };
  let original_owner = match genesis_output {
    Some(genesis_output) => server_config
      .chain
      .address_from_script(&genesis_output.script_pubkey)
      .ok(),
    None => None,
  }
  .map(|address| address.to_string());

  Ok(InscriptionJson {
    chain: server_config.chain,
    genesis_fee: entry.fee,
    genesis_height: entry.height,
    inscription_id: inscription_id.clone(),
    next,
    number: entry.inscription_number,
    previous,
    sat: sat_json,
    satpoint,
    timestamp: entry.timestamp,
    address,
    output_value,
    output,
    content_len: inscription.content_length(),
    transaction: inscription_id.txid.to_string(),
    location: satpoint.to_string(),
    offset: satpoint.offset,
    content_type: inscription.content_type().map(|bytes| bytes.to_string()),
    original_owner,
  })
}
