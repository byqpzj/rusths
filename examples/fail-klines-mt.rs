use rusths::ths::{THS, Adjust, Interval};
use std::thread;

/// 多线程获取K线数据的示例，失败了，竞争资源太激烈， [klines] 官方限频50ms/次间隔
fn main() {
    let mut handles = vec![];
    
    for i in 0..4 {
        let handle = thread::spawn(move || {
            // 在每个线程中创建新的 THS 实例
            let mut ths = THS::new(None).expect("Failed to create THS instance");
            ths.connect().expect("Failed to connect to server");
            
            let klines = ths.klines(
                "USHA600795",
                Some("2024-01-01 00:00:00"),
                Some("2025-01-01 00:00:00"),
                Adjust::FORWARD,
                Interval::MIN_5,
                10
            ).expect("Failed to get klines in thread");

            println!("{:?}", klines.payload.result);

            // 在线程结束前断开连接
            ths.disconnect().expect("Failed to disconnect");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}