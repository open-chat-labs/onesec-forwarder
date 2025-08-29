use aws_sdk_ssm::Client;
use onesec_forwarder_lambda_core::NextBlockHeightStore;
use std::str::FromStr;

const PARAMETER_NAME: &str = "next_block_height";

pub struct ParameterStoreClient {
    aws_client: aws_sdk_ssm::Client,
}

impl ParameterStoreClient {
    pub fn new(aws_client: Client) -> Self {
        ParameterStoreClient { aws_client }
    }
}

impl NextBlockHeightStore for ParameterStoreClient {
    async fn get(&self) -> Result<u64, String> {
        let get_parameter_response = self
            .aws_client
            .get_parameter()
            .name(PARAMETER_NAME)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let value = get_parameter_response
            .parameter
            .and_then(|p| p.value)
            .ok_or("Next block height parameter value not found".to_string())?;

        u64::from_str(&value).map_err(|e| e.to_string())
    }

    async fn set(&mut self, value: u64) -> Result<(), String> {
        self.aws_client
            .put_parameter()
            .name(PARAMETER_NAME)
            .value(value.to_string())
            .send()
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
