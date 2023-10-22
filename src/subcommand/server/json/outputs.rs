use std::sync::Arc;

use bitcoin::{Block, BlockHash, OutPoint};

use crate::{index::Index, subcommand::server::error::ServerResult, templates::PageConfig};

use super::{handle_json_result, types::OutputJson};

pub(super) async fn outputs_for_block_by_hash(
  page_config: Arc<PageConfig>,
  index: Arc<Index>,
  block_hash: BlockHash,
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
  page_config: Arc<PageConfig>,
  index: Arc<Index>,
  block_index: u64,
) -> ServerResult<String> {
  let block_option = index.get_block_by_height(block_index)?;
  let block = match block_option {
    Some(block) => block,
    None => return Ok("[]".to_string()),
  };

  let outputs = get_outputs_from_block(&block, &index, &page_config);

  Ok(handle_json_result(serde_json::to_string(&outputs?)))
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
              script_pubkey: output.script_pubkey.to_string(),
              address: page_config
                .chain
                .address_from_script(&output.script_pubkey)
                .ok()
                .map(|address| address.to_string()),
              transaction: transaction.txid(),
            });
          }
          Ok(acc)
        });
    acc.extend(output_results?);
    Ok(acc)
  })
}
