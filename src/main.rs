use std::str::FromStr;
use fuel_gql_client::client::{FuelClient, PageDirection, PaginationRequest};
use fuel_gql_client::client::schema::block::{Header};
use fuel_gql_client::fuel_tx::{Receipt};
use fuel_gql_client::prelude::ContractId;

pub fn serialize_data(_b: &Header, r: Receipt) {
    let log_id = r.rb().unwrap() as u64;
    let data = r.data().unwrap().to_vec();

    if log_id == 4 {
        println!("This is mint event");
        println!("{:?}", data);

    }

    if log_id == 6 {
        println!("This is transfer event");
        println!("{:?}", data);
    }
}

pub async fn run() {
    let my_contract = ContractId::from_str("8ebf2b98f443137a9920930180af22fef99043fa8005f93c34b4fee2805a8672").unwrap();
    let client = FuelClient::from_str("127.0.0.1:4000").unwrap();

    let blocks = client.blocks(PaginationRequest {
        cursor: None,
        results: 5,
        direction: PageDirection::Backward,
    }).await.unwrap();

    for b in blocks.results {
        for t in b.transactions {
            let transaction_id = &t.id.to_string();
            let receipts = client.receipts(transaction_id).await.unwrap();

            for r in receipts {
                match r {
                    Receipt::LogData { id, .. } => {
                        if id == my_contract {
                            serialize_data(&b.header, r.clone());
                        } else {
                            println!("Skipping not a matching contract")
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    run().await;
}
