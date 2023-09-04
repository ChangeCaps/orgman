mod data;
mod delegate;
mod network;
mod ui;

pub use data::*;
pub use delegate::*;
pub use network::*;
pub use ui::*;

use ori::prelude::*;

#[tokio::main]
async fn main() {
    App::new(AppData::ui, AppData::init())
        .title("Org Maid")
        .theme(Palette::dark)
        .delegate(AppDelegate::default())
        .run();
}
