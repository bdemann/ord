use std::sync::Arc;

use bitcoin::{block, BlockHash};

use super::{
  get_inscriptions::get_inscription_ids_for_block, handle_json_result, types::BlockJson,
};
use crate::{
  subcommand::server::error::{OptionExt, ServerResult},
  Index,
};

pub(super) async fn get_latest_block(index: Arc<Index>) -> ServerResult<String> {
  let block_hash = get_latest_block_hash(&index)?;

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
  let block_hash = get_latest_block_hash(&index)?;

  Ok(handle_json_result(serde_json::to_string(&get_block_id(
    block_hash, &index,
  )?)))
}

// pub(super) async fn get_latest_block_with_cursed_inscription(
//   index: Arc<Index>,
// ) -> ServerResult<String> {
//   let latest_block_hash = get_latest_block_hash(&index)?;
//   let mut block_id = get_block_id(latest_block_hash, &index)?;

//   while !has_cursed_inscription(block_id, &index) {
//     block_id -= 1;
//   }

//   return Ok("".to_string());
// }

// fn has_cursed_inscription(block_id: usize, index: &Arc<Index>) -> ServerResult<bool> {
//   let block = index.get_block_by_height(block_id as u64)?.unwrap();
//   let inscription_ids = get_inscription_ids_for_block(&block, index)?;

//   inscription_ids.iter().map(|id| id.)

//   return false;
// }

fn get_latest_block_hash(index: &Arc<Index>) -> ServerResult<BlockHash> {
  Ok(
    index
      .blocks(1)?
      .iter()
      .map(|(_, block_hash)| block_hash)
      .collect::<Vec<_>>()[0]
      .clone(),
  )
}

fn get_block_id(block_hash: BlockHash, index: &Arc<Index>) -> ServerResult<usize> {
  let block_info = index
    .block_header_info(block_hash)?
    .ok_or_not_found(|| format!("block {block_hash}"))?;

  Ok(block_info.height)
}
