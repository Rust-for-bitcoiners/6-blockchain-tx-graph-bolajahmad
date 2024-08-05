mod graph;
mod profile_transactions;

fn main() {
    profile_transactions::build_transaction_graph(0, 10);
}
