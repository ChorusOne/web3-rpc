use crate::client::{Client, Error};
use crate::model::{Block, JsonRpcResult, Receipt, Tag, Transaction};
use error_stack::{Report, ResultExt};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Clone)]
pub struct Web3 {
    pub client: Client,
}

impl Web3 {
    pub fn new(url: String) -> Self {
        Web3 {
            client: Client::new(url),
        }
    }

    pub fn new_with_client(url: String, client: reqwest_middleware::ClientWithMiddleware) -> Self {
        Web3 {
            client: Client::new_with_client(url, client),
        }
    }

    fn parse_json<'de, T>(entity_str: &'de str) -> Result<T, Report<Error>>
    where
        T: Deserialize<'de>,
    {
        let jd = &mut serde_json::Deserializer::from_str(entity_str);
        let entity: T = serde_path_to_error::deserialize(jd).change_context(Error::FailedToDeserialize)?;
        Ok(entity)
    }

    // web3
    pub async fn web3_client_version(&self) -> Result<JsonRpcResult<String>, Report<Error>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "net_version", "params": [], "id": "101" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn web3_sha3(&self, sha3: &str) -> Result<JsonRpcResult<String>, Report<Error>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "web3_sha3", "params": [sha3], "id": "102" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    // net
    pub async fn net_version(&self) -> Result<JsonRpcResult<String>, Report<Error>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "net_version", "params": [], "id": "401" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn net_listening(&self) -> Result<JsonRpcResult<bool>, Report<Error>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "net_listening", "params": [], "id": "402" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<bool> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn net_peer_count(&self) -> Result<JsonRpcResult<i64>, Report<Error>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "net_peerCount", "params": [], "id": "303" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<i64> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    // eth
    pub async fn eth_protocol_version(&self) -> Result<JsonRpcResult<String>, Report<Error>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "eth_protocolVersion", "params": [], "id": "304" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_syncing(&self) -> Result<JsonRpcResult<bool>, Report<Error>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "eth_syncing", "params": [], "id": "305" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<bool> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_coinbase(&self) -> Result<JsonRpcResult<String>, Report<Error>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "eth_coinbase", "params": [], "id": "306" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_mining(&self) -> Result<JsonRpcResult<bool>, Report<Error>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "eth_mining", "params": [], "id": "307" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<bool> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_hashrate(&self) -> Result<JsonRpcResult<String>, Report<Error>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "eth_hashrate", "params": [], "id": "308" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_gas_price(&self) -> Result<JsonRpcResult<String>, Report<Error>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "eth_gasPrice", "params": [], "id": "309" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_accounts(&self) -> Result<JsonRpcResult<Vec<String>>, Report<Error>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "eth_accounts", "params": [], "id": "310" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<Vec<String>> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_balance(
        &self,
        address: &str,
        tag: Option<Tag>,
    ) -> Result<JsonRpcResult<String>, Report<Error>> {
        let mut t = String::from(Tag::Latest);
        if let Some(tag) = tag {
            t = String::from(tag);
        }
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_getBalance", "params": [address, t], "id": "311" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_storage_at(
        &self,
        data: &str,
        quantity: &str,
        tag: Option<Tag>,
    ) -> Result<JsonRpcResult<String>, Report<Error>> {
        let mut t = String::from(Tag::Latest);
        if let Some(tag) = tag {
            t = String::from(tag);
        }
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_getStorageAt", "params": [data, quantity, t], "id": "312" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_transaction_count(
        &self,
        address: &str,
        tag: Option<Tag>,
    ) -> Result<JsonRpcResult<String>, Report<Error>> {
        let mut t = String::from(Tag::Latest);
        if let Some(tag) = tag {
            t = String::from(tag);
        }
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_getTransactionCount", "params": [address, t], "id": "313" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_block_transaction_count_by_hash(
        &self,
        hash: &str,
    ) -> Result<JsonRpcResult<String>, Report<Error>> {
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_getBlockTransactionCountByHash", "params": [hash], "id": "314" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_block_transaction_count_by_number(
        &self,
        number: &str,
    ) -> Result<JsonRpcResult<String>, Report<Error>> {
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_getBlockTransactionCountByNumber", "params": [number], "id": "315" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_uncle_count_by_block_hash(
        &self,
        hash: &str,
    ) -> Result<JsonRpcResult<String>, Report<Error>> {
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_getUncleCountByBlockHash", "params": [hash], "id": "316" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_uncle_count_by_block_number(
        &self,
        number: &str,
    ) -> Result<JsonRpcResult<String>, Report<Error>> {
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_getUncleCountByBlockNumber", "params": [number], "id": "317" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_code(
        &self,
        address: &str,
        tag: Option<Tag>,
    ) -> Result<JsonRpcResult<String>, Report<Error>> {
        let mut t = String::from(Tag::Latest);
        if let Some(tag) = tag {
            t = String::from(tag);
        }
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_getCode", "params": [address, t], "id": "318" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_sign(
        &self,
        address: &str,
        data: &str,
    ) -> Result<JsonRpcResult<String>, Report<Error>> {
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_sign", "params": [address, data], "id": "319" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_send_transaction(
        &self,
        from: &str,
        to: &str,
        gas: &str,
        gas_price: &str,
        value: &str,
        data: &str,
    ) -> Result<JsonRpcResult<String>, Report<Error>> {
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_sendTransaction", "params": [{
            "from": from,
            "to": to,
            "gas": gas,
            "gasPrice": gas_price,
            "value": value,
            "data": data
        }], "id": "320" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_send_raw_transaction(
        &self,
        hash: &str,
    ) -> Result<JsonRpcResult<String>, Report<Error>> {
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_sendRawTransaction", "params": [hash], "id": "321" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_call(&self, data: Value) -> Result<JsonRpcResult<String>, Report<Error>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "eth_call", "params": [data], "id": "322" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_estimate_gas(&self, data: Value) -> Result<JsonRpcResult<String>, Report<Error>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "eth_estimateGas", "params": [data], "id": "323" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_block_by_hash(
        &self,
        hash: &str,
        obj: bool,
    ) -> Result<JsonRpcResult<Block>, Report<Error>> {
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_getBlockByHash", "params": [hash, obj], "id": "324" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<Block> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_block_by_number(
        &self,
        number: &str,
        obj: bool,
    ) -> Result<JsonRpcResult<Block>, Report<Error>> {
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_getBlockByNumber", "params": [number, obj], "id": "325" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<Block> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_transaction_by_hash(
        &self,
        hash: &str,
    ) -> Result<JsonRpcResult<Transaction>, Report<Error>> {
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_getTransactionByHash", "params": [hash], "id": "326" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<Transaction> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_block_number(&self) -> Result<JsonRpcResult<String>, Report<Error>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "eth_blockNumber", "params": [], "id": "327" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_transaction_receipt(
        &self,
        hash: &str,
    ) -> Result<JsonRpcResult<Receipt>, Report<Error>> {
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_getTransactionReceipt", "params": [hash], "id": "328" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<Receipt> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_transaction_by_block_hash_and_index(
        &self,
        hash: &str,
        index: &str,
    ) -> Result<JsonRpcResult<Transaction>, Report<Error>> {
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_getTransactionByBlockHashAndIndex", "params": [hash, index], "id": "329" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<Transaction> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_transaction_by_block_number_and_index(
        &self,
        number: &str,
        index: &str,
    ) -> Result<JsonRpcResult<Transaction>, Report<Error>> {
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_getTransactionByBlockNumberAndIndex", "params": [number, index], "id": "330" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<Transaction> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_uncle_by_block_hash_and_index(
        &self,
        hash: &str,
        index: &str,
    ) -> Result<JsonRpcResult<Block>, Report<Error>> {
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_getUncleByBlockHashAndIndex", "params": [hash, index], "id": "331" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<Block> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_uncle_by_block_number_and_index(
        &self,
        hash: &str,
        index: &str,
    ) -> Result<JsonRpcResult<Block>, Report<Error>> {
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_getUncleByBlockNumberAndIndex", "params": [hash, index], "id": "332" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<Block> = Self::parse_json(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_compilers(&self) -> Result<JsonRpcResult<Vec<String>>, Report<Error>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "eth_getCompilers", "params": [], "id": "333" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<Vec<String>> = Self::parse_json(result.as_str())?;

        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn get_block_transaction_count_by_hash() {
        let rpc = Web3::new("https://mainnet.infura.io/v3/xxx".to_string());
        let r = rpc
            .eth_get_block_transaction_count_by_hash(
                "0xe812a49745d691961893d7cfd3902d78d710751bab872f12215ee23f27f3efa9",
            )
            .await;
        println!("{:?}", r);
    }
}
