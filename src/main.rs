use web3_rpc::model::Tag;
use web3_rpc::web3::Web3;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rpc = Web3::new("http://127.0.0.1:8545".to_string());
    let r = rpc.web3_client_version().await?;
    println!("{:?}", r);
    let r = rpc.web3_sha3("0x68656c6c6f20776f726c64").await?;
    println!("{:?}", r);

    let r = rpc.net_version().await?;
    println!("{:?}", r);

    let r = rpc.net_listening().await?;
    println!("{:?}", r);

    let r = rpc.net_peer_count().await?;
    println!("{:?}", r);

    let r = rpc.eth_protocol_version().await?;
    println!("{:?}", r);

    let r = rpc.eth_syncing().await?;
    println!("{:?}", r);

    let r = rpc.eth_coinbase().await?;
    println!("{:?}", r);

    let r = rpc.eth_mining().await?;
    println!("{:?}", r);

    let r = rpc.eth_hashrate().await?;
    println!("{:?}", r);

    let r = rpc.eth_gas_price().await?;
    println!("{:?}", r);

    let r = rpc.eth_accounts().await?;
    println!("{:?}", r);

    let r = rpc
        .eth_get_balance(
            "0x846c4dc9f4e2514206ef179eaa0bcfae007e37d2",
            Some(Tag::Latest),
        )
        .await?;
    println!("{:?}", r);

    let r = rpc
        .eth_get_storage_at("0x295a70b2de5e3953354a6a8344e616ed314d7251", "0x0", None)
        .await?;
    println!("{:?}", r);

    let r = rpc
        .eth_get_transaction_count("0x846c4dc9f4e2514206ef179eaa0bcfae007e37d2", None)
        .await?;
    println!("{:?}", r);

    let r = rpc
        .eth_get_block_transaction_count_by_hash(
            "0xe812a49745d691961893d7cfd3902d78d710751bab872f12215ee23f27f3efa9",
        )
        .await?;
    println!("{:?}", r);

    Ok(())
}