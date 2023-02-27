use std::{sync::Arc};
use axum::{Extension, extract::Path};
use bitcoin::{TxOut};
use serde::{Deserialize, Serialize};

use crate::{Index, Sat, SatPoint, Chain, InscriptionId, templates::PageConfig, deserialize_from_str::DeserializeFromStr};
use super::{server::Server, error::{ServerResult, OptionExt, ServerError}};

#[derive(Deserialize, Serialize, Clone)]
struct InscriptionJson {
  pub(crate) inscription_id: InscriptionId,
  address: String,
  output_value: u64,
  pub(crate) sat: Option<Sat>,
  preview: String,
  content: String,
  content_len: usize,
  pub(crate) genesis_height: u64,
  pub(crate) genesis_fee: u64,
  pub(crate) timestamp: u32,
  transaction: String,
  location: String,
  pub(crate) output: TxOut,
  offset: u64,
  end: bool,
  pub(crate) chain: Chain,
  body: Option<Vec<u8>>,
  content_type: Option<String>,
  pub(crate) next: Option<InscriptionId>,
  pub(crate) number: u64,
  pub(crate) previous: Option<InscriptionId>,
  pub(crate) satpoint: SatPoint,
}

impl Server {

  pub(super) async fn hello_world(Extension(index): Extension<Arc<Index>>) -> ServerResult<String> {
    let block_str: Vec<_> = index.blocks(100).unwrap().iter().map(|(_, block_hash)| block_hash.to_string()).collect();
    Ok(format!("{:?}", block_str))
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
      ))
    }
    let inscription_ids: Vec<_> = (start..=end).map(|n| index.get_inscription_id_by_inscription_number(n).unwrap().unwrap()).collect();

    let inscription_json: Vec<InscriptionJson> = inscription_ids.iter().fold(Ok(vec![]), |acc: ServerResult<_>, inscription_id| {
    let inscription_id = inscription_id.clone();
    let acc = acc?;

    let entry = index
      .get_inscription_entry(inscription_id)?
      .ok_or_not_found(|| format!("inscription {inscription_id}"))?;

    let inscription = index
      .get_inscription_by_id(inscription_id)?
      .ok_or_not_found(|| format!("inscription {inscription_id}"))?;

    let satpoint = index
      .get_inscription_satpoint_by_id(inscription_id)?
      .ok_or_not_found(|| format!("inscription {inscription_id}"))?;

    let output = index
      .get_transaction(satpoint.outpoint.txid)?
      .ok_or_not_found(|| format!("inscription {inscription_id} current transaction"))?
      .output
      .into_iter()
      .nth(satpoint.outpoint.vout.try_into().unwrap())
      .ok_or_not_found(|| format!("inscription {inscription_id} current transaction output"))?;

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

    let thing = InscriptionJson {
        chain: page_config.chain,
        genesis_fee: entry.fee,
        genesis_height: entry.height,
        inscription_id: inscription_id.clone(),
        next,
        number: entry.number,
        output,
        previous,
        sat: entry.sat,
        satpoint,
        timestamp: entry.timestamp,
        address: "todo".to_string(),
        output_value: 0,
        preview: "todo".to_string(),
        content: "todo".to_string(),
        content_len: 0,
        transaction: "todo".to_string(),
        location: "todo".to_string(),
        offset: 0,
        end: true,
        // body: inscription.body().map(|bytes| bytes.to_vec()), // BODY is so large it's hard to see what's going on so we are commenting out for readability in tests.
        body: Some(vec![]),
        content_type: inscription.content_type().map(|bytes| bytes.to_string()),
      };

      Ok(vec![acc, vec![thing]].concat())
      // Ok(format!("{}{}", acc, serde_json::to_string(&thing).unwrap()))

    })?;

    Ok(serde_json::to_string(&inscription_json).unwrap())
  }

}
