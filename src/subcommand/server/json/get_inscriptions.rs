use bitcoin::{Block, OutPoint, Transaction};
use std::sync::Arc;

use super::{build_inscription, types::InscriptionJson};
use crate::{subcommand::server::error::ServerResult, templates::PageConfig, Index, InscriptionId};

// Method 1: get all of the inscriptions in one go
pub(super) fn get_inscriptions_from_block(
  block: &Block,
  index: &Arc<Index>,
  page_config: &Arc<PageConfig>,
) -> ServerResult<Vec<InscriptionJson>> {
  let transactions = &block.txdata;

  transactions
    .iter()
    .try_fold(vec![], |mut acc, transaction| {
      acc.extend(get_inscriptions_for_transaction(
        transaction,
        index,
        page_config,
      )?);
      Ok(acc)
    })
}

// Method 2: get the inscriptions by count
pub(super) fn get_paginated_inscriptions_from_block(
  block: &Block,
  index: &Arc<Index>,
  page_config: &Arc<PageConfig>,
  start: usize,
  count: usize,
) -> ServerResult<Vec<InscriptionJson>> {
  let inscription_ids = get_inscription_ids_for_block(block, index)?;
  let end_index = start + count;
  let page_of_inscriptions = &inscription_ids[start..end_index];
  get_inscriptions_from_inscription_ids(page_of_inscriptions, index, page_config)
}

// Method 3 get the transactions and just get the inscriptions for each transaction
pub(super) fn get_transactions_from_block(block: &Block) -> &Vec<Transaction> {
  return &block.txdata;
}

pub(super) fn get_inscriptions_for_transaction(
  transaction: &Transaction,
  index: &Arc<Index>,
  page_config: &Arc<PageConfig>,
) -> ServerResult<Vec<InscriptionJson>> {
  let inscription_ids = get_inscription_ids_for_transaction(transaction, index)?;
  get_inscriptions_from_inscription_ids(&inscription_ids, index, page_config)
}

// Helper functions
fn get_inscription_ids_for_transaction(
  transaction: &Transaction,
  index: &Arc<Index>,
) -> ServerResult<Vec<InscriptionId>> {
  transaction
    .output
    .iter()
    .enumerate()
    .try_fold(Vec::new(), |mut acc, (vout, _)| {
      let outpoint = OutPoint::new(transaction.txid(), vout as u32);
      let inscriptions_ids = index.get_inscriptions_on_output(outpoint)?;
      acc.extend(inscriptions_ids);
      Ok(acc)
    })
}

fn get_inscription_ids_for_block(
  block: &Block,
  index: &Arc<Index>,
) -> ServerResult<Vec<InscriptionId>> {
  let transactions = get_transactions_from_block(block);

  transactions
    .iter()
    .try_fold(vec![], |mut acc, transaction| {
      let inscription_ids = get_inscription_ids_for_transaction(transaction, index)?;
      acc.extend(inscription_ids);
      Ok(acc)
    })
}

fn get_inscriptions_from_inscription_ids(
  inscription_ids: &[InscriptionId],
  index: &Arc<Index>,
  page_config: &Arc<PageConfig>,
) -> ServerResult<Vec<InscriptionJson>> {
  inscription_ids
    .iter()
    .map(|inscription_id| {
      build_inscription::build_inscription(inscription_id, &index, &page_config)
    })
    .collect()
}

// fn get_inscriptions_from_block(
//   block: &Block,
//   index: &Arc<Index>,
//   page_config: &Arc<PageConfig>,
// ) -> ServerResult<Vec<InscriptionJson>> {
//   let transactions = &block.txdata;

//   transactions.iter().fold(Ok(vec![]), |acc, transaction| {
//     let mut acc = acc?;
//     let output_results: ServerResult<Vec<InscriptionJson>> = transaction
//       .output
//       .iter()
//       .enumerate()
//       .fold(Ok(vec![]), |acc, (vout, _)| {
//         let mut acc = acc?;
//         let outpoint = OutPoint::new(transaction.txid(), vout as u32);
//         let inscriptions_ids = index.get_inscriptions_on_output(outpoint)?;
//         let inscriptions: ServerResult<Vec<InscriptionJson>> =
//           inscriptions_ids
//             .iter()
//             .fold(Ok(vec![]), |acc, inscription_id| {
//               let mut acc = acc?;
//               acc.push(build_inscription(inscription_id, &index, &page_config)?);
//               Ok(acc)
//             });
//         acc.extend(inscriptions?);
//         Ok(acc)
//       });
//     acc.extend(output_results?);
//     Ok(acc)
//   })
// }
