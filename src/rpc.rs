use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EpochInfo {
    pub epoch: u64,
    pub slot_index: u64,
    pub slots_in_epoch: u64,
    pub absolute_slot: u64
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceSample {
    // pub num_slots: u64, (Unused for now but could be useful in the future)
    pub num_transactions: u64,
    pub sample_period_secs: u64,
    pub slot: u64
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockProduction {
    // Key: validator identity as a base-58 encoded string, Value: (# of leader slots, # of blocks produced)
    pub by_identity: std::collections::HashMap<String, SlotStats>,
    // First and last slot of block production information (inclusive)
    pub range: SlotRange,
}

#[derive(Debug, Deserialize)]
pub struct SlotStats {
    pub assigned: u64,
    pub produced: u64
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlotRange {
    pub first_slot: u64,
    pub last_slot: u64
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteAccount {
    pub node_pubkey: String,
    pub activated_stake: u64,
    pub commission: u8,
    pub last_vote: u64,
    pub root_slot: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteAccountsResponse {
    pub current: Vec<VoteAccount>,
    pub delinquent: Vec<VoteAccount>
}

pub async fn get_epoch_info() -> Result<EpochInfo, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client.post("https://api.mainnet-beta.solana.com")
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getEpochInfo",
            "params": []
        }))
        .send()
        .await?;
    
    let json: serde_json::Value = res.json().await?;
    let result: EpochInfo = serde_json::from_value(json["result"].clone())?;
    Ok(result)
}

pub async fn get_performance_samples() -> Result<Vec<PerformanceSample>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client.post("https://api.mainnet-beta.solana.com")
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getRecentPerformanceSamples",
            "params": [1]
        }))
        .send()
        .await?;
    
    let json: serde_json::Value = res.json().await?;
    let result: Vec<PerformanceSample> = serde_json::from_value(json["result"].clone())?;
    Ok(result)
}

pub async fn get_block_production() -> Result<BlockProduction, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client.post("https://api.mainnet-beta.solana.com")
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getBlockProduction",
            "params": []
        }))
        .send()
        .await?;
    
    let json: serde_json::Value = res.json().await?;
    let result: BlockProduction = serde_json::from_value(json["result"]["value"].clone())?;
    Ok(result)
}

pub async fn get_vote_accounts() -> Result<VoteAccountsResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client.post("https://api.mainnet-beta.solana.com")
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getVoteAccounts",
            "params": []
        }))
        .send()
        .await?;
    
    let json: serde_json::Value = res.json().await?;
    let result: VoteAccountsResponse = serde_json::from_value(json["result"].clone())?;
    Ok(result)
}