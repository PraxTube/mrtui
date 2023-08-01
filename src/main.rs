mod data;
mod mock_data;
mod ui;
mod utils;

fn mock() {
    ui::inout::print_fee(mock_data::fetch_fee());
    ui::inout::print_blocks(&mock_data::fetch_blocks());
    ui::inout::print_difficulty(&mock_data::fetch_blocks()[0]);
    ui::inout::print_halving(&mock_data::fetch_blocks()[0]);
}

#[tokio::main]
async fn main() {
    let mock_data = true;
    if mock_data {
        mock();
        return;
    }

    let fees = data::fetch_fee().await;
    let blocks = data::fetch_blocks(None).await;
    ui::inout::print_fee(fees);
    ui::inout::print_blocks(&blocks);
    ui::inout::print_difficulty(&blocks[0]);
    ui::inout::print_halving(&blocks[0]);
}
