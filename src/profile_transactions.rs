// #[macro_use]
use lazy_static::lazy_static;

use std::{collections::HashSet, env, path::PathBuf, rc::Rc};
use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoin::{blockdata::transaction};
use bitcoincore_rpc::bitcoin::Txid;

use super::graph::Graph;

lazy_static! {
    static ref RPC_CLIENT: Client = {
        dotenv::dotenv().ok();
        let rpc_user: String = env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set");
        let cookie_url: String = env::var("BITCOIN_COOKIE_URL").expect("BITCOIN_COOKIE_URL must be set");
        let rpc_password: String =
            env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set");
        let rpc_port: String = env::var("BITCOIN_RPC_PORT").expect("BITCOIN_RPC_PORT must be set");
        let rpc_url: String = format!("http://{rpc_user}:{rpc_password}@127.0.0.1:{rpc_port}/");
        let cookie_file: PathBuf = PathBuf::from(cookie_url.clone());
        Client::new(&rpc_url, Auth::CookieFile(cookie_file)).unwrap()
    };
}

pub fn build_transaction_graph(start_height: u64, end_height: u64) -> Graph<Txid> {
    // Every Transaction has a set of Inputs and outputs
    // Each Input refers to an output of some earlier transaction
    // We say a Transaction A funds Transaction B if an ouput of A is an input of B
    // Build a graph where nodes represents Txid and an edge (t1, t2) is in the graph
    // if the transaction t1 funds transaction t2
    const TX_COUNT: usize = 20;
    let some_value = Box::new(TX_COUNT);

    let rpc_client: &Client = &*RPC_CLIENT;
    let transactions = rpc_client.list_transactions(
        None, 
        Some(*some_value),
        None, 
        None
    ).unwrap();
    println!("Transactions {:?}", transactions);

    let mut graph = Graph::new();

    for tx in transactions.iter() {
        let txid = tx.info.txid;
        let blockhash = tx.info.blockhash;
        let tx_details = rpc_client
            .get_raw_transaction(
                &txid, 
                blockhash.as_ref()
            ).unwrap();
        println!("Tx Details {:?}", tx_details);
        graph.insert_vertex(txid);
        
        for input in tx_details.input.iter() {
            let out_id = input.previous_output.txid;
            graph.insert_edge(txid, out_id)
        }
    }

    // How to visualize the graph data 
    graph
}