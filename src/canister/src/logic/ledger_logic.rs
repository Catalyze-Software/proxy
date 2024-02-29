use crate::CATALYZE_MULTI_SIG;
use candid::Principal;
use canister_types::models::api_error::ApiError;
use ic_ledger_types::{
    query_archived_blocks, query_blocks, AccountIdentifier, Block, BlockIndex, GetBlocksArgs,
    Tokens, DEFAULT_SUBACCOUNT, MAINNET_LEDGER_CANISTER_ID,
};

pub struct Ledger {}

impl Ledger {
    // This method checks if the transaction is send and received from the given principal
    pub async fn validate_transaction(
        principal: Principal,
        block_index: BlockIndex,
    ) -> Result<Tokens, ApiError> {
        // Get the block
        let block = Self::get_block(block_index).await;
        match block {
            Ok(block) => {
                // Check if the block has a transaction
                if let Some(operation) = block.transaction.operation {
                    if let ic_ledger_types::Operation::Transfer {
                        from,
                        to,
                        amount,
                        fee: _, // Ignore fee
                    } = operation
                    {
                        if from != Self::principal_to_account_identifier(principal) {
                            return Err(ApiError::bad_request().add_message("Invalid from adrress"));
                        }
                        if to
                            != Self::principal_to_account_identifier(
                                Principal::from_text(CATALYZE_MULTI_SIG.to_string()).unwrap(),
                            )
                        {
                            return Err(ApiError::bad_request().add_message("Invalid to address"));
                        }
                        return Ok(amount);
                    } else {
                        // Not a transfer
                        return Err(ApiError::bad_request()
                            .add_message("This block does not contain a transfer"));
                    }
                } else {
                    // No operation
                    return Err(
                        ApiError::bad_request().add_message("No operation found in the block")
                    );
                }
            }
            // No block
            Err(err) => return Err(ApiError::bad_request().add_message("No block found")),
        }
    }

    async fn get_block(block_index: BlockIndex) -> Result<Block, ApiError> {
        let args = GetBlocksArgs {
            start: block_index,
            length: 1,
        };

        match query_blocks(MAINNET_LEDGER_CANISTER_ID, args.clone()).await {
            Ok(blocks_result) => {
                if blocks_result.blocks.len() >= 1 {
                    match blocks_result.blocks.into_iter().next() {
                        Some(block) => return Ok(block),
                        None => return Err(ApiError::bad_request().add_message("No block found")),
                    }
                }

                if let Some(func) = blocks_result.archived_blocks.into_iter().find_map(|b| {
                    (b.start <= block_index && (block_index - b.start) < b.length)
                        .then(|| b.callback)
                }) {
                    match query_archived_blocks(&func, args).await {
                        Ok(range) => match range {
                            Ok(_range) => match _range.blocks.into_iter().next() {
                                Some(block) => return Ok(block),
                                None => {
                                    return Err(ApiError::bad_request()
                                        .add_message("No archived block found"))
                                }
                            },
                            Err(err) => {
                                return Err(ApiError::bad_request().add_message(&err.to_string()))
                            }
                        },
                        _ => (),
                    }
                }
            }
            Err((_, err)) => return Err(ApiError::bad_request().add_message(&err)),
        }
        Err(ApiError::bad_request().add_message("No block found"))
    }

    fn principal_to_account_identifier(principal: Principal) -> AccountIdentifier {
        AccountIdentifier::new(&principal, &DEFAULT_SUBACCOUNT)
    }
}
