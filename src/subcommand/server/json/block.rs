use std::sync::Arc;

use super::{handle_json_result, types::BlockJson};
use crate::{
  subcommand::server::error::{OptionExt, ServerResult},
  Index,
};

pub(super) async fn get_latest_block(index: Arc<Index>) -> ServerResult<String> {
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

pub(super) async fn get_latest_block_id(index: Arc<Index>) -> ServerResult<String> {
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
