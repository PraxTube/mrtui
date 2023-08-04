use clap::Parser;

mod data;
mod mock_data;
mod ui;
mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Mock data, used for debugging
    #[arg(short, long, default_value_t = false)]
    mock: bool,
}

fn mock() {
    ui::inout::print_blocks(&mock_data::fetch_blocks());
    ui::inout::print_halving(&mock_data::fetch_blocks()[0]);
    ui::inout::print_hashrate(&mock_data::fetch_hashrate());
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.mock {
        mock();
        return;
    }

    let blocks = data::fetch_blocks(None).await;
    let hashrate = data::fetch_hashrate().await;
    ui::inout::print_blocks(&blocks);
    ui::inout::print_halving(&blocks[0]);
    ui::inout::print_hashrate(&hashrate);
}
