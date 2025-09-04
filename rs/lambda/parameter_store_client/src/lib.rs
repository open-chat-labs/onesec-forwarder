use aws_sdk_ssm::Client;
use aws_types::SdkConfig;
use onesec_forwarder_lambda_core::NextBlockHeightStore;
use onesec_forwarder_types::EvmChain;
use std::str::FromStr;

pub struct ParameterStoreClient {
    aws_client: aws_sdk_ssm::Client,
}

impl ParameterStoreClient {
    pub fn new(aws_config: &SdkConfig) -> Self {
        ParameterStoreClient {
            aws_client: Client::new(aws_config),
        }
    }

    fn parameter_name(&self, chain: EvmChain) -> String {
        format!(
            "/onesec-forwarder/next-block-height/{}",
            chain.to_string().to_lowercase()
        )
    }
}

impl NextBlockHeightStore for ParameterStoreClient {
    async fn get(&self, chain: EvmChain) -> Result<u64, String> {
        let get_parameter_response = self
            .aws_client
            .get_parameter()
            .name(self.parameter_name(chain))
            .send()
            .await
            .map_err(|e| e.into_source().unwrap().to_string())?;

        let value = get_parameter_response
            .parameter
            .and_then(|p| p.value)
            .ok_or("Next block height parameter value not found".to_string())?;

        u64::from_str(&value).map_err(|e| e.to_string())
    }

    async fn set(&mut self, chain: EvmChain, value: u64) -> Result<(), String> {
        self.aws_client
            .put_parameter()
            .name(self.parameter_name(chain))
            .value(value.to_string())
            .overwrite(true)
            .send()
            .await
            .map_err(|e| e.into_source().unwrap().to_string())?;

        Ok(())
    }
}
