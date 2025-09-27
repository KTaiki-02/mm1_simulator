//! # event.rs
//!
//! このモジュールはイベント駆動型シミュレーションにおける
//! イベント構造体とイベント種別を定義します。
//!
//! ## 主な構造体
//! - `Event`: 時間、種別、顧客ID、サービス時間を持つイベント
//! - `EventType`: 到着、サービス開始、サービス終了の3種
//!
//! ## トレイト実装
//! - `PartialEq`, `Eq`: `time` に基づく等価性
//! - `PartialOrd`, `Ord`: `time` に基づく順序比較（`NaN` 非許容）
//!
//! ## 注意点
//! - `f64` の比較には `NaN` を含まない前提で `unwrap()` を使用
//! - `BinaryHeap` や `.sort()` に対応するため `Ord` を明示的に実装



/// use crate::customer;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EventType {
    Arrival,
    ServiceStart,
    ServiceEnd,
}
#[derive(Debug, Clone, Copy)]
pub struct Event {
    pub time: f64,
    pub event_type: EventType,
    pub customer_id: u64,
    pub service_time: Option<f64>,
}
impl Event {
    fn new(time: f64, event_type: EventType, customer_id: u64) -> Self {
        Event {
            time,
            event_type,
            customer_id,
            service_time: None,
        }
    }
}

/// イベントの等価性を定義するトレイト実装。
/// `Event` は `time` フィールドのみを比較対象とし、
/// 他のフィールド（event_type, customer_id など）は無視する。
impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

/// `PartialEq` が完全な等価性を持つことを宣言する。
/// `Eq` 自体はメソッドを持たないため、空の実装で十分。
impl Eq for Event {}

/// イベントの時間順による大小比較を定義するトレイト実装。
/// `f64` の `partial_cmp()` を使うため、
/// `NaN` を含むと `None` を返す可能性がある。
/// そのため戻り値は `Option<Ordering>`。
impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.time.partial_cmp(&other.time)
    }
}

/// 完全な順序比較を定義するトレイト実装。
/// `partial_cmp()` の結果が `None` になることはないと設計者が保証し、
/// `unwrap()` によって `Ordering` を取得する。
/// `NaN` を含まない前提で使うこと。
impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_creation() {
        let event = Event::new(10.5, EventType::Arrival, 1);
        assert_eq!(event.time, 10.5);
        assert_eq!(event.event_type, EventType::Arrival);
        assert_eq!(event.customer_id, 1);
        assert!(event.service_time.is_none());
    }

    #[test]
    fn test_event_ordering() {
        let e1 = Event::new(5.0, EventType::Arrival, 1);
        let e2 = Event::new(10.0, EventType::ServiceStart, 2);
        let e3 = Event::new(7.5, EventType::ServiceEnd, 3);

        let mut events = vec![e2, e3, e1];
        events.sort();

        assert_eq!(events[0].time, 5.0);
        assert_eq!(events[1].time, 7.5);
        assert_eq!(events[2].time, 10.0);
    }
}
