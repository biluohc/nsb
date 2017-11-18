
/*!
# 暖手宝

## 安装
```rust
cargo install --git  https://github.com/biluohc/nsb
```

## 使用
`nsb 死循环线程数量`, 默认是机器线程数目的一半.

## Ps: 是不是要加个循环的频率(插入sleep)?

*/
extern crate signalbool;
use signalbool::SignalBool;
extern crate num_cpus;

use std::thread::{sleep, spawn};
use std::process::exit;
use std::time::Duration;
use std::env::args;


fn main() {
    let help = || {
        println!("Usage:\n  nsb <Thread_number[Default: 1]>");
        exit(1);
    };

    let thread_number = args()
        .nth(1)
        .and_then(|s| s.parse::<usize>().map_err(|_| help()).ok())
        .unwrap_or_else(default_number);
    let sb = SignalBool::new(&[signalbool::Signal::SIGINT], signalbool::Flag::Restart)
        .map_err(|e| eprintln!("Register Signal failed: {:?}", e))
        .unwrap();

    loops(thread_number);

    println!(
        "nsb running on {} threads\nHit CTRL-C to stop the server",
        thread_number
    );
    loop {
        sleep(Duration::from_millis(1));
        if sb.caught() {
            break;
        }
    }
}

fn loops(number: usize) {
    // if the loop is empty, process will been kill(Address boundary error), why?
    (0..number).into_iter().for_each(|_| {
        let _ = spawn(move || loop {
            if args().count() > std::usize::MAX {
                println!("{}", args().count());
            }
        });
    })
}

fn default_number() -> usize {
    let cpus = num_cpus::get();
    if cpus < 2 {
        1
    } else {
        cpus / 2
    }
}
