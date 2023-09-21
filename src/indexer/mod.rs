mod contracts;

use contracts::legal_document_manager;
use ethers::{
    core::types::Address,
    providers::{Http, Middleware, Provider},
};
use std::sync::Arc;
use tokio::time;

pub async fn index_event(chain_rpc_url: String, legal_document_address: String) {
    let provider = Provider::<Http>::try_from(chain_rpc_url).unwrap();
    let client = Arc::new(provider);
    let contract_address: Address = legal_document_address.parse().unwrap();
    let contract =
        legal_document_manager::LegalDocumentManager::new(contract_address, client.clone());

    log::info!("Indexer started");

    loop {
        let latest_sync_block: u64 = tokio::fs::read_to_string("latest_block")
            .await
            .unwrap()
            .parse()
            .unwrap();
        let latest_block = client.get_block_number().await.unwrap();

        let events = contract
            .events()
            .from_block(latest_sync_block)
            .to_block(latest_block)
            .query()
            .await
            .unwrap();

        println!("Read from {} to {}", latest_sync_block, latest_block);
        println!("Events number {}", events.len());

        // for e in events {
        //     match e {
        //         erc20::ERC20Events::ApprovalFilter(e) => {
        //             println!("Approval: {} {} {}", e.owner, e.spender, e.value)
        //         }
        //         erc20::ERC20Events::TransferFilter(e) => {
        //             println!("Transfer: {} {} {}", e.from, e.to, e.value)
        //         }
        //         _ => {}
        //     }
        // }

        println!("-----------------------------------------------------------------------");

        tokio::fs::write("latest_block", (latest_block + 1).to_string().as_bytes())
            .await
            .unwrap();

        time::sleep(time::Duration::from_secs(5)).await;
    }
}
