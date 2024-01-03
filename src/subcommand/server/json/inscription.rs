use bitcoin::BlockHash;
use std::sync::Arc;

use super::{build_inscription, get_inscriptions, handle_json_result, types::InscriptionJson};
use crate::{
  subcommand::server::error::{ServerError, ServerResult},
  templates::ServerConfig,
  Index, InscriptionId,
};

pub(super) async fn inscription_json_by_id(
  server_config: Arc<ServerConfig>,
  index: Arc<Index>,
  inscription_id: InscriptionId,
) -> ServerResult<String> {
  let inscription = build_inscription::build_inscription(&inscription_id, &index, &server_config)?;
  Ok(handle_json_result(serde_json::to_string(&inscription)))
}

pub(super) async fn inscription_json_by_index(
  server_config: Arc<ServerConfig>,
  index: Arc<Index>,
  inscription_index: i32,
) -> ServerResult<String> {
  Ok(
    match index.get_inscription_id_by_inscription_number(inscription_index)? {
      Some(inscription_id) => {
        let inscription =
          build_inscription::build_inscription(&inscription_id, &index, &server_config)?;
        handle_json_result(serde_json::to_string(&inscription))
      }
      None => "{}".to_string(),
    },
  )
}

pub(super) async fn latest_inscription_json(
  server_config: Arc<ServerConfig>,
  index: Arc<Index>,
) -> ServerResult<String> {
  let latest_inscription = index.get_latest_inscriptions_with_prev_and_next(1, None)?.0[0];
  let inscription =
    build_inscription::build_inscription(&latest_inscription, &index, &server_config)?;
  Ok(handle_json_result(serde_json::to_string(&inscription)))
}

pub(super) async fn inscription_json(
  server_config: Arc<ServerConfig>,
  index: Arc<Index>,
  start: i32,
  end: i32,
) -> ServerResult<String> {
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
        let inscription =
          build_inscription::build_inscription(&inscription_id, &index, &server_config)?;
        Ok(vec![acc, vec![inscription]].concat())
      })?;

  Ok(handle_json_result(serde_json::to_string(&inscription_json)))
}

pub(super) async fn inscriptions_for_block(
  server_config: Arc<ServerConfig>,
  index: Arc<Index>,
  block_index: u32,
) -> ServerResult<String> {
  let block_option = index.get_block_by_height(block_index)?;
  let block = match block_option {
    Some(block) => block,
    None => return Ok("[]".to_string()),
  };

  let inscriptions = get_inscriptions::get_inscriptions_from_block(&block, &index, &server_config);

  Ok(handle_json_result(serde_json::to_string(&inscriptions?)))
}

pub(super) async fn inscriptions_for_block_by_hash(
  server_config: Arc<ServerConfig>,
  index: Arc<Index>,
  block_hash: BlockHash,
) -> ServerResult<String> {
  let block_option = index.get_block_by_hash(block_hash)?;
  let block = match block_option {
    Some(block) => block,
    None => return Ok("[]".to_string()),
  };

  let inscriptions = get_inscriptions::get_inscriptions_from_block(&block, &index, &server_config);

  Ok(handle_json_result(serde_json::to_string(&inscriptions?)))
}

pub(super) async fn paginated_inscriptions_for_block(
  server_config: Arc<ServerConfig>,
  index: Arc<Index>,
  block_index: u32,
  start: usize,
  count: usize,
) -> ServerResult<String> {
  let block_option = index.get_block_by_height(block_index)?;
  let block = match block_option {
    Some(block) => block,
    None => return Ok("[]".to_string()),
  };

  let inscriptions = get_inscriptions::get_paginated_inscriptions_from_block(
    &block,
    &index,
    &server_config,
    start,
    count,
  );

  Ok(handle_json_result(serde_json::to_string(&inscriptions?)))
}

pub(super) async fn paginated_inscriptions_for_block_by_hash(
  server_config: Arc<ServerConfig>,
  index: Arc<Index>,
  block_hash: BlockHash,
  start: usize,
  count: usize,
) -> ServerResult<String> {
  let block_option = index.get_block_by_hash(block_hash)?;
  let block = match block_option {
    Some(block) => block,
    None => return Ok("[]".to_string()),
  };

  let inscriptions = get_inscriptions::get_paginated_inscriptions_from_block(
    &block,
    &index,
    &server_config,
    start,
    count,
  );

  Ok(handle_json_result(serde_json::to_string(&inscriptions?)))
}

pub(super) async fn inscription_count_for_block(
  index: Arc<Index>,
  block_index: u32,
) -> ServerResult<String> {
  let block_option = index.get_block_by_height(block_index)?;
  let block = match block_option {
    Some(block) => block,
    None => return Ok("[]".to_string()),
  };

  let inscription_count = get_inscriptions::get_inscription_count_on_block(&block, &index);

  Ok(handle_json_result(serde_json::to_string(
    &inscription_count?,
  )))
}

pub(super) async fn inscription_count_for_block_by_hash(
  index: Arc<Index>,
  block_hash: BlockHash,
) -> ServerResult<String> {
  let block_option = index.get_block_by_hash(block_hash)?;
  let block = match block_option {
    Some(block) => block,
    None => return Ok("[]".to_string()),
  };

  let inscriptions = get_inscriptions::get_inscription_count_on_block(&block, &index);

  Ok(handle_json_result(serde_json::to_string(&inscriptions?)))
}
