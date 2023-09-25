mod contracts;

use contracts::legal_document_manager::{self, LegalDocumentManagerEvents};
use deadpool_postgres::Pool;
use ethers::{
    core::types::Address,
    prelude::LogMeta,
    providers::{Http, Middleware, Provider},
};
use std::sync::Arc;
use tokio::time;

use crate::models::onchain_officer_model::{create_onchain_officer, CreateOnchainOfficerInfo};

pub async fn index_event(chain_rpc_url: String, legal_document_address: String, db_pool: Pool) {
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

        if latest_block.as_u64() < latest_sync_block + 1 {
            continue;
        };

        let events = contract
            .events()
            .from_block(latest_sync_block + 1)
            .to_block(latest_block)
            .query_with_meta()
            .await
            .unwrap();

        println!("Read from {} to {}", latest_sync_block, latest_block);
        println!("Events number {}", events.len());

        for e in events {
            match e {
                (LegalDocumentManagerEvents::OfficerCreatedFilter(event), meta) => {
                    let _ = &handle_officer_created(&db_pool, event, meta).await;
                }
                _ => {}
            }
        }

        println!("-----------------------------------------------------------------------");

        tokio::fs::write("latest_block", (latest_block).to_string().as_bytes())
            .await
            .unwrap();

        time::sleep(time::Duration::from_secs(5)).await;
    }
}

async fn handle_officer_created(
    db_pool: &Pool,
    event: legal_document_manager::OfficerCreatedFilter,
    _meta: LogMeta,
) {
    println!("get OfficerCreated event");
    if let Ok(client) = db_pool.get().await {
        let onchain_address = event
            .officer_address
            .as_fixed_bytes()
            .iter()
            .fold("0x".to_owned(), |acc, byte| acc + &format!("{:02x}", byte));

        let officer_info = CreateOnchainOfficerInfo {
            onchain_address,
            name: event.info.name,
            date_of_birth: event.info.date_of_birth,
            sex: event.info.sex,
        };

        let _ = create_onchain_officer(&client, &officer_info).await;
    }
}
