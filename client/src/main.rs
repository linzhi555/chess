use client::{Areas, Event, Ui};
fn main() {
    let x = "Info: ";
    let f = |event: Event, area: &mut Areas| {
        area.message.clear();
        area.message.push_str(x);

        match event {
            Event::StringInput(s) => {
                area.message.push_str("new input");
                area.message.push_str(s.as_str());
            }

            Event::GridClick(x, y) => {
                area.message.push_str("click event");
                area.message.push_str(format!("{} {}", x, y).as_str())
            }
        }
    };

    let mut ui = Ui::new(Box::new(f));
    ui.run();
}
