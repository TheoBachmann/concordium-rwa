use super::error::Error;
use concordium_cis2::TokenIdVec;
pub use concordium_rwa_utils::cis2_conversions::Rate;
use concordium_rwa_utils::{cis2_schema_types, cis2_types};
use concordium_std::*;

pub type ContractResult<R> = Result<R, Error>;
pub type TokenAmount = cis2_types::SftTokenAmount;
pub type TokenId = cis2_types::SftTokenId;
pub type NftTokenAmount = cis2_types::NftTokenAmount;
pub type NftTokenId = TokenIdVec;
pub type NftTokenUId = cis2_schema_types::TokenUId<NftTokenId>;
pub type NftTokenOwnerUId = cis2_schema_types::TokenOwnerUId<NftTokenId>;
pub type ContractTransferParams = concordium_cis2::TransferParams<TokenId, TokenAmount>;
pub type ContractBalanceOfQueryParams = concordium_cis2::BalanceOfQueryParams<TokenId>;
pub type ContractBalanceOfQuery = concordium_cis2::BalanceOfQuery<TokenId>;
pub type ContractBalanceOfQueryResponse = concordium_cis2::BalanceOfQueryResponse<TokenAmount>;

/// Represents the metadata URL and hash of a token.
#[derive(SchemaType, Serial, Clone, Deserial)]
pub struct ContractMetadataUrl {
    pub url:  String,
    pub hash: Option<String>,
}

impl From<ContractMetadataUrl> for MetadataUrl {
    fn from(val: ContractMetadataUrl) -> Self {
        MetadataUrl {
            url:  val.url,
            hash: {
                if let Some(hash) = val.hash {
                    let mut hash_bytes = [0u8; 32];
                    match hex::decode_to_slice(hash, &mut hash_bytes) {
                        Ok(_) => Some(hash_bytes),
                        Err(_) => None,
                    }
                } else {
                    None
                }
            },
        }
    }
}
