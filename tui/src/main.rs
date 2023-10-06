use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tui::Ui;
fn main() {

    let(tx,rx)=mpsc::channel();
        let mut ui = Ui::new();
    let handle = thread::spawn(move || {
        ui.run(tx);
    });
    
    let mut buffer = String::new();
    while !handle.is_finished() {
        if let Ok(temp)=rx.recv_timeout(Duration::from_millis(100)){
            buffer.push_str(temp.as_str());
        }
    }
    println!("the result is {}",buffer)

}

