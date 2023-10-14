use client::Ui;
fn main() {
    let x = "Info: ";
    let f = |s: &str, message: &mut String| {
        message.clear();
        message.push_str(x);
        message.push_str(s);
    };

    let mut ui = Ui::new(Box::new(f));
    ui.run();
}
