use anyhow::Context as _;
use zksync_basic_types::{L1ChainId, L2ChainId};
use zksync_config::{configs, configs::genesis::SharedBridge};
use zksync_protobuf::{repr::ProtoRepr, required};

use crate::{parse_h160, parse_h256, proto::genesis as proto};

impl ProtoRepr for proto::Genesis {
    type Type = configs::GenesisConfig;
    fn read(&self) -> anyhow::Result<Self::Type> {
        let prover = required(&self.prover).context("prover")?;
        let shared_bridge = if let Some(shared_bridge) = &self.shared_bridge {
            Some(SharedBridge {
                bridgehub_proxy_addr: required(&shared_bridge.bridgehub_proxy_addr)
                    .and_then(|x| parse_h160(x))
                    .context("bridgehub_proxy_addr")?,
                state_transition_proxy_addr: required(&shared_bridge.state_transition_proxy_addr)
                    .and_then(|x| parse_h160(x))
                    .context("state_transition_proxy_addr")?,
                transparent_proxy_admin_addr: required(&shared_bridge.transparent_proxy_admin_addr)
                    .and_then(|x| parse_h160(x))
                    .context("transparent_proxy_admin_addr")?,
            })
        } else {
            None
        };
        Ok(Self::Type {
            protocol_version: required(&self.genesis_protocol_version)
                .map(|x| *x as u16)
                .context("protocol_version")?,
            genesis_root_hash: required(&self.genesis_root)
                .and_then(|x| parse_h256(x))
                .context("genesis_root_hash")?,
            rollup_last_leaf_index: *required(&self.genesis_rollup_leaf_index)
                .context("rollup_last_leaf_index")?,
            genesis_commitment: required(&self.genesis_batch_commitment)
                .and_then(|x| parse_h256(x))
                .context("genesis_commitment")?,
            bootloader_hash: required(&self.bootloader_hash)
                .and_then(|x| parse_h256(x))
                .context("bootloader_hash")?,
            default_aa_hash: required(&self.default_aa_hash)
                .and_then(|x| parse_h256(x))
                .context("default_aa_hash")?,
            l1_chain_id: required(&self.l1_chain_id)
                .map(|x| L1ChainId(*x))
                .context("l1_chain_id")?,
            l2_chain_id: required(&self.l2_chain_id)
                .and_then(|x| L2ChainId::try_from(*x).map_err(|a| anyhow::anyhow!(a)))
                .context("l2_chain_id")?,
            recursion_node_level_vk_hash: required(&prover.recursion_node_level_vk_hash)
                .and_then(|x| parse_h256(x))
                .context("recursion_node_level_vk_hash")?,
            recursion_leaf_level_vk_hash: required(&prover.recursion_leaf_level_vk_hash)
                .and_then(|x| parse_h256(x))
                .context("recursion_node_level_vk_hash")?,
            recursion_circuits_set_vks_hash: required(&prover.recursion_circuits_set_vks_hash)
                .and_then(|x| parse_h256(x))
                .context("recursion_node_level_vk_hash")?,
            recursion_scheduler_level_vk_hash: required(&prover.recursion_scheduler_level_vk_hash)
                .and_then(|x| parse_h256(x))
                .context("recursion_node_level_vk_hash")?,
            fee_account: required(&self.fee_account)
                .and_then(|x| parse_h160(x))
                .context("fee_account")?,
            shared_bridge,
            dummy_prover: *required(&prover.dummy_verifier).context("dummy_prover")?,
        })
    }

    fn build(this: &Self::Type) -> Self {
        let shared_bridge = this
            .shared_bridge
            .as_ref()
            .map(|shared_bridge| proto::SharedBridge {
                bridgehub_proxy_addr: Some(shared_bridge.bridgehub_proxy_addr.as_bytes().into()),
                state_transition_proxy_addr: Some(
                    shared_bridge.state_transition_proxy_addr.as_bytes().into(),
                ),
                transparent_proxy_admin_addr: Some(
                    shared_bridge.transparent_proxy_admin_addr.as_bytes().into(),
                ),
            });

        Self {
            genesis_root: Some(this.genesis_root_hash.as_bytes().into()),
            genesis_rollup_leaf_index: Some(this.rollup_last_leaf_index),
            genesis_batch_commitment: Some(this.genesis_root_hash.as_bytes().into()),
            genesis_protocol_version: Some(this.protocol_version as u32),
            default_aa_hash: Some(this.genesis_root_hash.as_bytes().into()),
            bootloader_hash: Some(this.genesis_root_hash.as_bytes().into()),
            fee_account: Some(this.fee_account.as_bytes().into()),
            l1_chain_id: Some(this.l1_chain_id.0),
            l2_chain_id: Some(this.l2_chain_id.as_u64()),
            prover: Some(proto::Prover {
                recursion_scheduler_level_vk_hash: Some(
                    this.recursion_scheduler_level_vk_hash.as_bytes().into(),
                ),
                recursion_node_level_vk_hash: Some(
                    this.recursion_node_level_vk_hash.as_bytes().into(),
                ),
                recursion_leaf_level_vk_hash: Some(
                    this.recursion_leaf_level_vk_hash.as_bytes().into(),
                ),
                recursion_circuits_set_vks_hash: Some(
                    this.recursion_circuits_set_vks_hash.as_bytes().into(),
                ),
                dummy_verifier: Some(this.dummy_prover),
            }),
            shared_bridge,
        }
    }
}
