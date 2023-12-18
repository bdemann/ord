use crate::{
  subcommand::server::error::{OptionExt, ServerResult},
  subcommand::mempool_index::MempoolIndex, InscriptionId,
};

use super::types::LightInscriptionJson;

pub(super) fn build_light_inscription(
  inscription_id: &InscriptionId,
  mempool_index: &MempoolIndex,
) -> ServerResult<LightInscriptionJson> {
  let inscription_id = inscription_id.clone();

  let inscription = mempool_index
    .inscriptions
    .get(&inscription_id)
    .ok_or_not_found(|| format!("inscription {inscription_id}"))?;

  Ok(LightInscriptionJson {
    inscription_id: inscription_id.clone(),
    content_len: inscription.content_length(),
    transaction: inscription_id.txid.to_string(),
    content_type: inscription.content_type().map(|bytes| bytes.to_string()),
  })
}
