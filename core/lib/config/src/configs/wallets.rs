use zksync_basic_types::{Address, H256};
use zksync_crypto_primitives::PackedEthSignature;

#[derive(Debug, Clone)]
pub struct Wallet {
    address: Address,
    private_key: Option<H256>,
}

impl Wallet {
    pub fn from_address(address: Address) -> Self {
        Self {
            address,
            private_key: None,
        }
    }

    pub fn from_private_key(private_key: H256, address: Option<Address>) -> anyhow::Result<Self> {
        let calculated_address = PackedEthSignature::address_from_private_key(&private_key)?;
        if let Some(address) = address {
            if calculated_address != address {
                anyhow::bail!("Malformed wallet, address doesn't correspond private_key")
            }
        }

        Ok(Self {
            address: calculated_address,
            private_key: Some(private_key),
        })
    }

    pub fn address(&self) -> Address {
        self.address
    }
    pub fn private_key(&self) -> Option<H256> {
        self.private_key
    }
}

#[derive(Debug, Clone)]
pub struct EthSender {
    pub operator: Wallet,
    pub blob_operator: Option<Wallet>,
}

#[derive(Debug, Clone)]
pub struct StateKeeper {
    pub fee_account: Wallet,
}

#[derive(Debug, Clone)]
pub struct Wallets {
    pub eth_sender: Option<EthSender>,
    pub state_keeper: Option<StateKeeper>,
}