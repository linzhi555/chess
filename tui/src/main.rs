use std::thread;
use std::time::Duration;
use tui::Ui;
fn main() {
    let (mut ui, tx, rx) = Ui::new();
    let handle = thread::spawn(move || {
        ui.run();
    });

    let mut buffer = String::new();
    while !handle.is_finished() {
        if let Ok(temp) = rx.recv_timeout(Duration::from_millis(100)) {
            let mut reply = "i have received".to_string();
            reply.push_str(temp.as_str());
            tx.send(reply).unwrap();
            buffer.push_str(temp.as_str())
        }
    }
    println!("the result is {}", buffer)
}
