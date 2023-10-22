use tokio;
use tui::{Areas, Event, Ui};

async fn deal_func(event: Event, areas: Areas) -> Areas {
    let mut areas = areas;

    match event {
        Event::StringInput(x) => {
            areas.message.clear();
            areas.message.push_str(x.as_str());
        }
        Event::GridClick(x, y) => {
            areas.grid_area.selected = true;
            areas.grid_area.select_x = x;
            areas.grid_area.select_y = y;
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
