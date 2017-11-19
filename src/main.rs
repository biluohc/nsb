
/*!
# 暖手宝

## 安装
```rust
cargo install --git  https://github.com/biluohc/nsb
```

## 使用
`hw 死循环线程数量`, 默认是机器线程数目的一半.

## Ps: 是不是要加个循环的频率(插入sleep)?

*/
extern crate signalbool;
use signalbool::{SignalBool, Signal, Flag};
extern crate num_cpus;

use std::thread::{sleep, spawn};
use std::time::Duration;
use std::process::exit;
use std::env::args;


fn main() {
    let help = || {
        println!("Usage:\n  hw <Thread_number[Default: half of the number of CPUs]>");
        exit(1);
    };
    let default = || {
        let cpus = num_cpus::get();
        if cpus < 2 { 1 } else { cpus / 2 }
    };

    let number = args()
        .nth(1)
        .and_then(|s| s.parse::<usize>().map_err(|_| help()).ok())
        .unwrap_or_else(default);

    let sb = SignalBool::new(&[Signal::SIGINT], Flag::Restart)
        .map_err(|e| eprintln!("Register Signal failed: {:?}", e))
        .unwrap();

    // if the loop is empty, process will crach(coredump), see more issue #1.
    for _ in 0..number {
        let _ = spawn(move || loop {
            if args().count() > std::usize::MAX {
                println!("{}", args().count());
            }
        });
    }

    println!(
        "hw running on {} threads\nHit CTRL-C to stop the server",
        number
    );

    // wait Ctrl+C singal
    loop {
        sleep(Duration::from_millis(1));
        if sb.caught() {
            break;
        }
    }
}
