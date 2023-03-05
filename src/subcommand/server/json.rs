use axum::{extract::Path, Extension};
use bitcoin::{Address, Block, BlockHash, OutPoint, Script, Transaction, TxOut, Txid};
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::sync::Arc;

use super::{
  error::{OptionExt, ServerError, ServerResult},
  server::Server,
};
use crate::{
  decimal::Decimal, degree::Degree, deserialize_from_str::DeserializeFromStr, height::Height,
  templates::PageConfig, Chain, Epoch, Index, InscriptionId, Rarity, SatPoint,
};

#[derive(Deserialize, Serialize, Clone)]
struct InscriptionJson {
  inscription_id: InscriptionId,
  address: Option<Address>,
  output_value: Option<u64>,
  sat: Option<SatJson>,
  content_len: Option<usize>,
  genesis_height: u64,
  genesis_fee: u64,
  timestamp: u32,
  transaction: String,
  location: String,
  output: Option<TxOut>,
  offset: u64,
  chain: Chain,
  content_type: Option<String>,
  next: Option<InscriptionId>,
  number: u64,
  previous: Option<InscriptionId>,
  satpoint: SatPoint,
}

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

#[derive(Deserialize, Serialize, Clone)]
struct SatJson {
  number: u64,
  decimal: Decimal,
  degree: Degree,
  percentile: String,
  name: String,
  cycle: u64,
  epoch: Epoch,
  period: u64,
  block: Height,
  offset: u64,
  rarity: Rarity,
}

#[derive(Deserialize, Serialize, Clone)]
struct BlockJson {
  hash: BlockHash,
  transactions: Vec<Transaction>,
}

#[derive(Deserialize, Serialize, Clone)]
struct OutputJson {
  inscriptions: Vec<InscriptionId>,
  value: u64,
  script_pubkey: Script,
  address: Option<Address>,
  transaction: Txid,
}

impl Server {
  pub(super) async fn latest_block(
    Extension(index): Extension<Arc<Index>>,
  ) -> ServerResult<String> {
    let block_hash = index
      .blocks(1)?
      .iter()
      .map(|(_, block_hash)| block_hash)
      .collect::<Vec<_>>()[0]
      .clone();
    let transactions = match index.get_block_by_hash(block_hash)? {
      Some(block) => block.txdata,
      None => vec![],
    };
    let block_json = BlockJson {
      hash: block_hash.clone(),
      transactions,
    };
    Ok(handle_json_result(serde_json::to_string(&block_json)))
  }

  pub(super) async fn latest_block_id(
    Extension(index): Extension<Arc<Index>>,
  ) -> ServerResult<String> {
    let block_hash = index
      .blocks(1)?
      .iter()
      .map(|(_, block_hash)| block_hash)
      .collect::<Vec<_>>()[0]
      .clone();

    let block_info = index
      .block_header_info(block_hash)?
      .ok_or_not_found(|| format!("block {block_hash}"))?;

    Ok(handle_json_result(serde_json::to_string(
      &block_info.height,
    )))
  }

  pub(super) async fn inscription_json_by_id(
    Extension(page_config): Extension<Arc<PageConfig>>,
    Extension(index): Extension<Arc<Index>>,
    Path(DeserializeFromStr(inscription_id)): Path<DeserializeFromStr<InscriptionId>>,
  ) -> ServerResult<String> {
    let inscription = build_inscription(&inscription_id, &index, &page_config)?;
    Ok(handle_json_result(serde_json::to_string(&inscription)))
  }

  pub(super) async fn inscription_json_by_index(
    Extension(page_config): Extension<Arc<PageConfig>>,
    Extension(index): Extension<Arc<Index>>,
    Path(DeserializeFromStr(inscription_index)): Path<DeserializeFromStr<u64>>,
  ) -> ServerResult<String> {
    Ok(
      match index.get_inscription_id_by_inscription_number(inscription_index)? {
        Some(inscription_id) => {
          let inscription = build_inscription(&inscription_id, &index, &page_config)?;
          handle_json_result(serde_json::to_string(&inscription))
        }
        None => "{}".to_string(),
      },
    )
  }

  pub(super) async fn outputs_for_block_by_hash(
    Extension(page_config): Extension<Arc<PageConfig>>,
    Extension(index): Extension<Arc<Index>>,
    Path(DeserializeFromStr(block_hash)): Path<DeserializeFromStr<BlockHash>>,
  ) -> ServerResult<String> {
    let block_option = index.get_block_by_hash(block_hash)?;
    let block = match block_option {
      Some(block) => block,
      None => return Ok("[]".to_string()),
    };

    let outputs = get_outputs_from_block(&block, &index, &page_config);

    Ok(handle_json_result(serde_json::to_string(&outputs?)))
  }

  pub(super) async fn outputs_for_block(
    Extension(page_config): Extension<Arc<PageConfig>>,
    Extension(index): Extension<Arc<Index>>,
    Path(DeserializeFromStr(block_index)): Path<DeserializeFromStr<u64>>,
  ) -> ServerResult<String> {
    let block_option = index.get_block_by_height(block_index)?;
    let block = match block_option {
      Some(block) => block,
      None => return Ok("[]".to_string()),
    };

    let outputs = get_outputs_from_block(&block, &index, &page_config);

    Ok(handle_json_result(serde_json::to_string(&outputs?)))
  }

  pub(super) async fn inscriptions_for_block(
    Extension(page_config): Extension<Arc<PageConfig>>,
    Extension(index): Extension<Arc<Index>>,
    Path(DeserializeFromStr(block_index)): Path<DeserializeFromStr<u64>>,
  ) -> ServerResult<String> {
    let block_option = index.get_block_by_height(block_index)?;
    let block = match block_option {
      Some(block) => block,
      None => return Ok("[]".to_string()),
    };

    let inscriptions = get_inscriptions_from_block(&block, &index, &page_config);

    Ok(handle_json_result(serde_json::to_string(&inscriptions?)))
  }

  pub(super) async fn inscriptions_for_block_by_hash(
    Extension(page_config): Extension<Arc<PageConfig>>,
    Extension(index): Extension<Arc<Index>>,
    Path(DeserializeFromStr(block_hash)): Path<DeserializeFromStr<BlockHash>>,
  ) -> ServerResult<String> {
    let block_option = index.get_block_by_hash(block_hash)?;
    let block = match block_option {
      Some(block) => block,
      None => return Ok("[]".to_string()),
    };

    let inscriptions = get_inscriptions_from_block(&block, &index, &page_config);

    Ok(handle_json_result(serde_json::to_string(&inscriptions?)))
  }

  pub(super) async fn latest_inscription_json(
    Extension(page_config): Extension<Arc<PageConfig>>,
    Extension(index): Extension<Arc<Index>>,
  ) -> ServerResult<String> {
    let latest_inscription = index.get_latest_inscriptions(1, None)?[0];
    let inscription = build_inscription(&latest_inscription, &index, &page_config)?;
    Ok(handle_json_result(serde_json::to_string(&inscription)))
  }

  pub(super) async fn inscription_json(
    Extension(page_config): Extension<Arc<PageConfig>>,
    Extension(index): Extension<Arc<Index>>,
    Path((DeserializeFromStr(start), DeserializeFromStr(end))): Path<(
      DeserializeFromStr<u64>,
      DeserializeFromStr<u64>,
    )>,
  ) -> ServerResult<String> {
    if start == end {
      return Err(ServerError::BadRequest("empty range".to_string()));
    }
    if start > end {
      return Err(ServerError::BadRequest(
        "range start greater than range end".to_string(),
      ));
    }
    let inscription_ids: ServerResult<Vec<_>> = (start..=end).fold(Ok(vec![]), |acc, n| {
      let mut acc = acc?;
      if let Some(inscription_id) = index.get_inscription_id_by_inscription_number(n)? {
        acc.push(inscription_id);
      }
      Ok(acc)
    });

    let inscription_json: Vec<InscriptionJson> =
      inscription_ids?
        .iter()
        .fold(Ok(vec![]), |acc: ServerResult<_>, inscription_id| {
          let acc = acc?;
          let inscription = build_inscription(&inscription_id, &index, &page_config)?;
          Ok(vec![acc, vec![inscription]].concat())
        })?;

    Ok(handle_json_result(serde_json::to_string(&inscription_json)))
  }
}

fn build_inscription(
  inscription_id: &InscriptionId,
  index: &Arc<Index>,
  page_config: &Arc<PageConfig>,
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
    Some(output) => page_config
      .chain
      .address_from_script(&output.script_pubkey)
      .ok(),
    None => None,
  };

  let output_value = match &output {
    Some(output) => Some(output.value),
    None => None,
  };

  let previous = if let Some(previous) = entry.number.checked_sub(1) {
    Some(
      index
        .get_inscription_id_by_inscription_number(previous)?
        .ok_or_not_found(|| format!("inscription {previous}"))?,
    )
  } else {
    None
  };

  let next = index.get_inscription_id_by_inscription_number(entry.number + 1)?;
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

  Ok(InscriptionJson {
    chain: page_config.chain,
    genesis_fee: entry.fee,
    genesis_height: entry.height,
    inscription_id: inscription_id.clone(),
    next,
    number: entry.number,
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
  })
}

fn handle_json_result(result: Result<String, Error>) -> String {
  match result {
    Ok(json) => json,
    Err(err) => err.to_string(),
  }
}

fn get_outputs_from_block(
  block: &Block,
  index: &Arc<Index>,
  page_config: &Arc<PageConfig>,
) -> ServerResult<Vec<OutputJson>> {
  let transactions = &block.txdata;

  transactions.iter().fold(Ok(vec![]), |acc, transaction| {
    let mut acc = acc?;
    let output_results: ServerResult<Vec<OutputJson>> =
      transaction
        .output
        .iter()
        .enumerate()
        .fold(Ok(vec![]), |acc, (vout, output)| {
          let mut acc = acc?;
          let outpoint = OutPoint::new(transaction.txid(), vout as u32);
          let inscriptions = index.get_inscriptions_on_output(outpoint)?;
          if inscriptions.len() > 0 {
            acc.push(OutputJson {
              inscriptions,
              value: output.value,
              script_pubkey: output.script_pubkey.clone(),
              address: page_config
                .chain
                .address_from_script(&output.script_pubkey)
                .ok(),
              transaction: transaction.txid(),
            });
          }
          Ok(acc)
        });
    acc.extend(output_results?);
    Ok(acc)
  })
}

fn get_inscriptions_from_block(
  block: &Block,
  index: &Arc<Index>,
  page_config: &Arc<PageConfig>,
) -> ServerResult<Vec<InscriptionJson>> {
  let transactions = &block.txdata;

  transactions.iter().fold(Ok(vec![]), |acc, transaction| {
    let mut acc = acc?;
    let output_results: ServerResult<Vec<InscriptionJson>> = transaction
      .output
      .iter()
      .enumerate()
      .fold(Ok(vec![]), |acc, (vout, _)| {
        let mut acc = acc?;
        let outpoint = OutPoint::new(transaction.txid(), vout as u32);
        let inscriptions_ids = index.get_inscriptions_on_output(outpoint)?;
        let inscriptions: ServerResult<Vec<InscriptionJson>> =
          inscriptions_ids
            .iter()
            .fold(Ok(vec![]), |acc, inscription_id| {
              let mut acc = acc?;
              acc.push(build_inscription(inscription_id, &index, &page_config)?);
              Ok(acc)
            });
        acc.extend(inscriptions?);
        Ok(acc)
      });
    acc.extend(output_results?);
    Ok(acc)
  })
}
