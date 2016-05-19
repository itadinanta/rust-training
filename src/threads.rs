use std::thread;
use std::sync::mpsc::channel;

fn main() {
    // let's talk via a channel
    let (tx, rx) = channel();

    let receiver = thread::spawn(move || println!("{}, world!", rx.recv().unwrap()));
    let _sender = thread::spawn(move || tx.send("Hello").unwrap());

    match receiver.join() {
        Ok(_) => println!("Ok"),
        Err(_) => println!("Nope..."),
    }
}
