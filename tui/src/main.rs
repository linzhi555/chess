use tokio;
use tui::{Areas, Event, Ui};

async fn deal_func(event: Event, areas: Areas) -> Areas {
    let mut areas = areas;

    match event {
        Event::StringInput(x) => {
            areas.message.clear();
            areas.message.push_str(x.as_str());
        }
        _ => {}
    }

    areas
}

async fn example2() {
    let mut ui = Ui::new(deal_func);
    ui.run().await;
}

fn main() {
    let multi_threaded_runtime = tokio::runtime::Runtime::new().unwrap();
    multi_threaded_runtime.block_on(example2());
}
