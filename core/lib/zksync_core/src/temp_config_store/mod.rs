use anyhow::Context as _;
use zksync_config::{
    configs::{
        api::{HealthCheckConfig, MerkleTreeApiConfig, Web3JsonRpcConfig},
        chain::{
            CircuitBreakerConfig, MempoolConfig, NetworkConfig, OperationsManagerConfig,
            StateKeeperConfig,
        },
        fri_prover_group::FriProverGroupConfig,
        house_keeper::HouseKeeperConfig,
        wallets::{EthSender, StateKeeper, Wallet, Wallets},
        FriProofCompressorConfig, FriProverConfig, FriProverGatewayConfig,
        FriWitnessGeneratorConfig, FriWitnessVectorGeneratorConfig, GeneralConfig,
        PrometheusConfig, ProofDataHandlerConfig, WitnessGeneratorConfig,
    },
    ApiConfig, ContractVerifierConfig, DBConfig, ETHConfig, ETHWatchConfig, GasAdjusterConfig,
    ObjectStoreConfig, PostgresConfig,
};
use zksync_protobuf::{read_optional, repr::ProtoRepr, ProtoFmt};

use crate::{consensus, proto};

pub fn decode_yaml<T: ProtoFmt>(yaml: &str) -> anyhow::Result<T> {
    let d = serde_yaml::Deserializer::from_str(yaml);
    let this: T = zksync_protobuf::serde::deserialize(d)?;
    Ok(this)
}

pub fn decode_yaml_repr<T: ProtoRepr>(yaml: &str) -> anyhow::Result<T::Type> {
    let d = serde_yaml::Deserializer::from_str(yaml);
    let this: T = zksync_protobuf::serde::deserialize_proto(d)?;
    this.read()
}
//
// TODO (QIT-22): This structure is going to be removed when components will be responsible for their own configs.
/// A temporary config store allowing to pass deserialized configs from `zksync_server` to `zksync_core`.
/// All the configs are optional, since for some component combination it is not needed to pass all the configs.
#[derive(Debug, PartialEq)]
pub struct TempConfigStore {
    pub postgres_config: Option<PostgresConfig>,
    pub health_check_config: Option<HealthCheckConfig>,
    pub merkle_tree_api_config: Option<MerkleTreeApiConfig>,
    pub web3_json_rpc_config: Option<Web3JsonRpcConfig>,
    pub circuit_breaker_config: Option<CircuitBreakerConfig>,
    pub mempool_config: Option<MempoolConfig>,
    pub network_config: Option<NetworkConfig>,
    pub contract_verifier: Option<ContractVerifierConfig>,
    pub operations_manager_config: Option<OperationsManagerConfig>,
    pub state_keeper_config: Option<StateKeeperConfig>,
    pub house_keeper_config: Option<HouseKeeperConfig>,
    pub fri_proof_compressor_config: Option<FriProofCompressorConfig>,
    pub fri_prover_config: Option<FriProverConfig>,
    pub fri_prover_group_config: Option<FriProverGroupConfig>,
    pub fri_prover_gateway_config: Option<FriProverGatewayConfig>,
    pub fri_witness_vector_generator: Option<FriWitnessVectorGeneratorConfig>,
    pub fri_witness_generator_config: Option<FriWitnessGeneratorConfig>,
    pub prometheus_config: Option<PrometheusConfig>,
    pub proof_data_handler_config: Option<ProofDataHandlerConfig>,
    pub witness_generator_config: Option<WitnessGeneratorConfig>,
    pub api_config: Option<ApiConfig>,
    pub db_config: Option<DBConfig>,
    pub eth_sender_config: Option<ETHConfig>,
    pub eth_watch_config: Option<ETHWatchConfig>,
    pub gas_adjuster_config: Option<GasAdjusterConfig>,
    pub object_store_config: Option<ObjectStoreConfig>,
}

#[derive(Debug)]
pub struct Secrets {
    pub consensus: Option<consensus::Secrets>,
}

impl ProtoFmt for Secrets {
    type Proto = proto::Secrets;
    fn read(r: &Self::Proto) -> anyhow::Result<Self> {
        Ok(Self {
            consensus: read_optional(&r.consensus).context("consensus")?,
        })
    }

    fn build(&self) -> Self::Proto {
        Self::Proto {
            consensus: self.consensus.as_ref().map(|x| x.build()),
        }
    }
}

impl TempConfigStore {
    pub fn general(&self) -> GeneralConfig {
        GeneralConfig {
            postgres_config: self.postgres_config.clone(),
            api_config: self.api_config.clone(),
            contract_verifier: self.contract_verifier.clone(),
            circuit_breaker_config: self.circuit_breaker_config.clone(),
            mempool_config: self.mempool_config.clone(),
            operations_manager_config: self.operations_manager_config.clone(),
            state_keeper_config: self.state_keeper_config.clone(),
            house_keeper_config: self.house_keeper_config.clone(),
            proof_compressor_config: self.fri_proof_compressor_config.clone(),
            prover_config: self.fri_prover_config.clone(),
            prover_gateway: self.fri_prover_gateway_config.clone(),
            witness_vector_generator: self.fri_witness_vector_generator.clone(),
            prover_group_config: self.fri_prover_group_config.clone(),
            witness_generator: self.fri_witness_generator_config.clone(),
            prometheus_config: self.prometheus_config.clone(),
            proof_data_handler_config: self.proof_data_handler_config.clone(),
            db_config: self.db_config.clone(),
            eth: self.eth_sender_config.clone(),
        }
    }

    pub fn wallets(&self) -> Wallets {
        let eth_sender = self.eth_sender_config.as_ref().and_then(|x| {
            let operator = x
                .sender
                .private_key()
                .and_then(|operator| Wallet::from_private_key(operator, None).ok());
            let blob_operator = x
                .sender
                .private_key_blobs()
                .and_then(|operator| Wallet::from_private_key(operator, None).ok());
            operator.map(|operator| EthSender {
                operator,
                blob_operator,
            })
        });
        #[allow(deprecated)]
        let state_keeper = self
            .state_keeper_config
            .as_ref()
            .map(|state_keeper| StateKeeper {
                fee_account: Wallet::from_address(
                    state_keeper
                        .fee_account_addr
                        .clone()
                        .expect("Must be presented in env variables"),
                ),
            });
        Wallets {
            eth_sender,
            state_keeper,
        }
    }
}
