use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_types::SdkConfig;
use onesec_forwarder_lambda_core::ForwardingEventLogger;
use onesec_forwarder_types::{EvmAddress, IcpAccount, Token};
use std::collections::HashMap;

pub struct DynamoDbLogger {
    client: Client,
    table_name: &'static str,
}

impl DynamoDbLogger {
    pub fn new(aws_config: &SdkConfig, table_name: &'static str) -> Self {
        DynamoDbLogger {
            client: Client::new(aws_config),
            table_name,
        }
    }
}

impl ForwardingEventLogger for DynamoDbLogger {
    async fn log(
        &self,
        token: Token,
        evm_address: EvmAddress,
        icp_account: IcpAccount,
    ) -> Result<(), String> {
        let mut fields = HashMap::new();
        fields.insert("token".to_string(), AttributeValue::S(token.to_string()));
        fields.insert(
            "chain".to_string(),
            AttributeValue::S(evm_address.chain.to_string()),
        );
        fields.insert(
            "evm_address".to_string(),
            AttributeValue::S(evm_address.address),
        );

        match icp_account {
            IcpAccount::ICRC(icrc) => {
                fields.insert(
                    "icp_account_owner".to_string(),
                    AttributeValue::S(icrc.owner.to_string()),
                );

                if let Some(subaccount) = icrc.subaccount {
                    fields.insert(
                        "icp_account_subaccount".to_string(),
                        AttributeValue::S(format!("{subaccount:x?}")),
                    );
                }
            }
            IcpAccount::AccountId(account) => {
                fields.insert("icp_account_id".to_string(), AttributeValue::S(account));
            }
        }

        self.client
            .put_item()
            .table_name(self.table_name)
            .set_item(Some(fields))
            .send()
            .await
            .map_err(|e| e.into_service_error().to_string())?;

        Ok(())
    }
}
