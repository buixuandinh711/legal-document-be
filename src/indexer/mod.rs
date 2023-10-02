mod contracts;

use contracts::legal_document_manager::{self, LegalDocumentManagerEvents};
use deadpool_postgres::Pool;
use ethers::{
    core::types::Address,
    prelude::LogMeta,
    providers::{Http, Middleware, Provider},
    types::H160,
};
use std::sync::Arc;
use tokio::time;

use crate::models::{
    division_model::{create_division, CreateDivisionInfo},
    onchain_officer_model::{create_onchain_officer, CreateOnchainOfficerInfo},
    position_model::{create_position, CreatePositionInfo},
};

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
        let mut latest_block = client.get_block_number().await.unwrap().as_u64();

        if latest_block < latest_sync_block + 1 {
            continue;
        };

        if latest_block - latest_sync_block > 1000 {
            latest_block = latest_sync_block + 1000;
        }

        let events = contract
            .events()
            .from_block(latest_sync_block + 1)
            .to_block(latest_block)
            .query_with_meta()
            .await
            .unwrap();

        log::info!(
            "Read from {} to {}, event found: {}",
            latest_sync_block + 1,
            latest_block,
            events.len()
        );

        for e in events {
            match e {
                (LegalDocumentManagerEvents::OfficerCreatedFilter(event), meta) => {
                    handle_officer_created(&db_pool, event, meta).await;
                }
                (LegalDocumentManagerEvents::DivisionCreatedFilter(event), meta) => {
                    handle_division_created(&db_pool, event, meta).await;
                }
                (LegalDocumentManagerEvents::PositionCreatedFilter(event), meta) => {
                    handle_position_created(&db_pool, event, meta).await;
                }
                _ => {}
            }
        }

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
    if let Ok(client) = db_pool.get().await {
        let onchain_address = to_string_address(&event.officer_address);

        let officer_info = CreateOnchainOfficerInfo {
            onchain_address,
            name: event.info.name,
            date_of_birth: event.info.date_of_birth,
            sex: event.info.sex,
        };

        let _ = create_onchain_officer(&client, &officer_info).await;
    }
}

async fn handle_division_created(
    db_pool: &Pool,
    event: legal_document_manager::DivisionCreatedFilter,
    _meta: LogMeta,
) {
    if let Ok(client) = db_pool.get().await {
        let division_info = CreateDivisionInfo {
            onchain_id: event.division_id,
            name: event.name,
            onchain_supervisory_id: event.supervisory_div_id,
        };
        let _ = create_division(&client, &division_info).await;
    }
}

async fn handle_position_created(
    db_pool: &Pool,
    event: legal_document_manager::PositionCreatedFilter,
    _meta: LogMeta,
) {
    if let Ok(client) = db_pool.get().await {
        let position_info = CreatePositionInfo {
            officer_address: to_string_address(&event.officer_address),
            division_onchain_id: event.division_id,
            position_index: event.position_index.as_usize() as i16,
            name: event.position_info.name,
            role: event.position_info.role,
        };

        let _ = create_position(&client, &position_info).await;
    }
}

fn to_string_address(address: &H160) -> String {
    address
        .as_fixed_bytes()
        .iter()
        .fold("0x".to_owned(), |acc, byte| acc + &format!("{:02x}", byte))
}
