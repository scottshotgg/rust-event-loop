use crossbeam_channel::{after, select, tick, unbounded};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let (msgs, output) = unbounded();
    // let (count_in, count_out) = unbounded();
    let ticker = tick(Duration::from_secs(1));
    // let start = Instant::now();

    // Sender
    thread::spawn(move || loop {
        // let val = String::from("hi");
        msgs.send("").unwrap();
    });

    // Recver
    thread::spawn(move || {
        // let timeout = after(Duration::from_secs(4));
        let mut count = 0;
        let mut total = 0;

        loop {
            select! {
                // recv(timeout) -> _ => {
                //     // println!("{}", count);
                //     count_in.send(count).unwrap();
                // }
                recv(ticker) -> _ => {
                    println!("Count: {}", count);
                    // count_in.send(count).unwrap();
                    total = total + count;
                    count = 0;
                    println!("Total: {}", total);
                    println!();
                }
                recv(output) -> _ => {
                    count = count + 1;
                    // countIn.send(count).unwrap();
                }
            }
        }
    });

    loop {
        // select! {
        //     recv(count_out) -> count => {
        //         match count {
        //             Ok(c) => {
        //                 println!("Count: {}", c)
        //             },
        //             Err(e) => {
        //                 println!("Error: {}", e)
        //             }
        //         }
        //     }
        // }
    }
}
