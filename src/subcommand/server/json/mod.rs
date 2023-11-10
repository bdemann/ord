pub mod block;
mod build_inscription;
mod get_inscriptions;
mod inscription;
mod outputs;
mod troubleshooting;
mod types;

use axum::{extract::Path, Extension};
use bitcoin::BlockHash;
use serde_json::Error;
use std::sync::Arc;

use super::{error::ServerResult, server::Server};
use crate::{
  deserialize_from_str::DeserializeFromStr, templates::PageConfig, Index, InscriptionId,
};

impl Server {
  pub(super) async fn latest_block(
    Extension(index): Extension<Arc<Index>>,
  ) -> ServerResult<String> {
    block::get_latest_block(index).await
  }

  pub(super) async fn latest_block_id(
    Extension(index): Extension<Arc<Index>>,
  ) -> ServerResult<String> {
    block::get_latest_block_id(index).await
  }

  pub(super) async fn inscription_json_by_id(
    Extension(page_config): Extension<Arc<PageConfig>>,
    Extension(index): Extension<Arc<Index>>,
    Path(DeserializeFromStr(inscription_id)): Path<DeserializeFromStr<InscriptionId>>,
  ) -> ServerResult<String> {
    inscription::inscription_json_by_id(page_config, index, inscription_id).await
  }

  pub(super) async fn inscription_json_by_index(
    Extension(page_config): Extension<Arc<PageConfig>>,
    Extension(index): Extension<Arc<Index>>,
    Path(DeserializeFromStr(inscription_index)): Path<DeserializeFromStr<i64>>,
  ) -> ServerResult<String> {
    inscription::inscription_json_by_index(page_config, index, inscription_index).await
  }

  pub(super) async fn outputs_for_block_by_hash(
    Extension(page_config): Extension<Arc<PageConfig>>,
    Extension(index): Extension<Arc<Index>>,
    Path(DeserializeFromStr(block_hash)): Path<DeserializeFromStr<BlockHash>>,
  ) -> ServerResult<String> {
    outputs::outputs_for_block_by_hash(page_config, index, block_hash).await
  }

  pub(super) async fn test() -> ServerResult<String> {
    Ok("working with troubleshooting: first 100".to_string())
  }

  pub(super) async fn outputs_for_block(
    Extension(page_config): Extension<Arc<PageConfig>>,
    Extension(index): Extension<Arc<Index>>,
    Path(DeserializeFromStr(block_index)): Path<DeserializeFromStr<u64>>,
  ) -> ServerResult<String> {
    outputs::outputs_for_block(page_config, index, block_index).await
  }

  pub(super) async fn inscriptions_for_block(
    Extension(page_config): Extension<Arc<PageConfig>>,
    Extension(index): Extension<Arc<Index>>,
    Path(DeserializeFromStr(block_index)): Path<DeserializeFromStr<u64>>,
  ) -> ServerResult<String> {
    inscription::inscriptions_for_block(page_config, index, block_index).await
  }

  pub(super) async fn inscriptions_for_block_by_hash(
    Extension(page_config): Extension<Arc<PageConfig>>,
    Extension(index): Extension<Arc<Index>>,
    Path(DeserializeFromStr(block_hash)): Path<DeserializeFromStr<BlockHash>>,
  ) -> ServerResult<String> {
    inscription::inscriptions_for_block_by_hash(page_config, index, block_hash).await
  }

  pub(super) async fn paginated_inscriptions_for_block(
    Extension(page_config): Extension<Arc<PageConfig>>,
    Extension(index): Extension<Arc<Index>>,
    Path((DeserializeFromStr(block_index), DeserializeFromStr(start), DeserializeFromStr(count))): Path<(DeserializeFromStr<u64>, DeserializeFromStr<usize>, DeserializeFromStr<usize>)>,
  ) -> ServerResult<String> {
    inscription::paginated_inscriptions_for_block(page_config, index, block_index, start, count)
      .await
  }

  pub(super) async fn paginated_inscriptions_for_block_by_hash(
    Extension(page_config): Extension<Arc<PageConfig>>,
    Extension(index): Extension<Arc<Index>>,
    Path((DeserializeFromStr(block_hash), DeserializeFromStr(start), DeserializeFromStr(count))): Path<(
      DeserializeFromStr<BlockHash>,
      DeserializeFromStr<usize>,
      DeserializeFromStr<usize>,
    )>,
  ) -> ServerResult<String> {
    inscription::paginated_inscriptions_for_block_by_hash(
      page_config,
      index,
      block_hash,
      start,
      count,
    )
    .await
  }

  pub(super) async fn inscription_count_for_block(
    Extension(index): Extension<Arc<Index>>,
    Path(DeserializeFromStr(block_index)): Path<DeserializeFromStr<u64>>,
  ) -> ServerResult<String> {
    inscription::inscription_count_for_block(index, block_index).await
  }

  pub(super) async fn inscription_count_for_block_by_hash(
    Extension(index): Extension<Arc<Index>>,
    Path(DeserializeFromStr(block_hash)): Path<DeserializeFromStr<BlockHash>>,
  ) -> ServerResult<String> {
    inscription::inscription_count_for_block_by_hash(index, block_hash).await
  }

  pub(super) async fn latest_inscription_json(
    Extension(page_config): Extension<Arc<PageConfig>>,
    Extension(index): Extension<Arc<Index>>,
  ) -> ServerResult<String> {
    inscription::latest_inscription_json(page_config, index).await
  }

  pub(super) async fn inscription_json(
    Extension(page_config): Extension<Arc<PageConfig>>,
    Extension(index): Extension<Arc<Index>>,
    Path((DeserializeFromStr(start), DeserializeFromStr(end))): Path<(
      DeserializeFromStr<i64>,
      DeserializeFromStr<i64>,
    )>,
  ) -> ServerResult<String> {
    inscription::inscription_json(page_config, index, start, end).await
  }

  pub(super) async fn inscription_ids_for_block(
    Extension(index): Extension<Arc<Index>>,
    Path(DeserializeFromStr(block_index)): Path<DeserializeFromStr<u64>>,
  ) -> ServerResult<String> {
    troubleshooting::get_inscription_ids(block_index, &index)
  }

  pub(super) async fn inscription_ids_for_block_by_transaction(
    Extension(index): Extension<Arc<Index>>,
    Path(DeserializeFromStr(block_index)): Path<DeserializeFromStr<u64>>,
  ) -> ServerResult<String> {
    troubleshooting::get_inscription_ids_by_transaction(block_index, &index)
  }
}

fn handle_json_result(result: Result<String, Error>) -> String {
  match result {
    Ok(json) => json,
    Err(err) => err.to_string(),
  }
}
