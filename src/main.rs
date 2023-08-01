mod data;
mod mock_data;
mod ui;
mod utils;

fn mock() {
    ui::inout::print_fee(mock_data::fetch_fee());
    ui::inout::print_block(&mock_data::fetch_block());
    ui::inout::print_difficulty(&mock_data::fetch_block());
    ui::inout::print_halving(&mock_data::fetch_block());
}

#[tokio::main]
async fn main() {
    let mock_data = false;
    if mock_data {
        mock();
        return;
    }

    let fees = data::fetch_fee().await;
    let block = data::fetch_block().await;
    ui::inout::print_fee(fees);
    ui::inout::print_block(&block);
    ui::inout::print_difficulty(&block);
    ui::inout::print_halving(&block);
}
