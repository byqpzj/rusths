
use rusths::ths::{THS, Adjust, Interval};
use chrono::{Local, Timelike};

fn main() {
    // 初始化日志
    // 创建 THS 实例
    let mut ths = THS::new(None).expect("Failed to create THS instance");

    // 连接到服务器
    ths.connect().expect("Failed to connect to server");

    while true {
        let stocks = ths.order_book_bid("USHA600795").expect("Failed to get stock list");
        println!("订单簿: {:?}", stocks);
    }
    // 获取股票列表


    // 断开连接
    ths.disconnect().expect("Failed to disconnect");
} 