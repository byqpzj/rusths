use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KLine {
    #[serde(skip)]
    pub time: DateTime<Local>,
    #[serde(rename(deserialize = "1"))]
    pub time_int: i32,
    #[serde(rename(deserialize = "7"))]
    pub open: f64,
    #[serde(rename(deserialize = "8"))]
    pub high: f64,
    #[serde(rename(deserialize = "9"))]
    pub low: f64,
    #[serde(rename(deserialize = "11"))]
    pub close: f64,
    #[serde(rename(deserialize = "13"))]
    pub volume: i64, // 成交量
    #[serde(rename(deserialize = "19"))]
    pub amount: f64, // 成交额
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThsOrderBook{
    pub orderlevel: i32,
    pub ordersque: Vec<i32>,
    pub price:f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    pub code: String,
    pub name: String,
    pub price: f64,
    pub change: f64,
    pub volume: i64,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockData {
    pub code: String,
    pub name: String,
    pub change: f64,
    pub volume: i64,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData {
    pub time: DateTime<Local>,
    pub price: f64,
    pub volume: i64,
    pub bs_flag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookData {
    pub price: f64,
    pub volume: i64,
    pub order_count: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpoData {
    pub code: String,
    pub name: String,
    pub price: f64,
    pub pe: f64,
    pub shares: i64,
    pub date: String,
} 