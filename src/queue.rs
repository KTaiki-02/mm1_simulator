//! # Queue モジュール
//!
//! このモジュールは MM1 シミュレーションにおける待ち行列の管理を行います。
//! 顧客の到着・退去を記録し、平均長などの統計情報を提供します。

use crate::customer::{self, Customer};

/// 顧客の待ち行列を表す構造体。
/// - `customers`: 現在キューにいる顧客のリスト（到着順）
/// - `length_log`: 時間とキュー長の履歴（平均長計算に使用）
pub struct Queue {
    pub customers: Vec<Customer>,
    pub length_log: Vec<(f64, usize)>,
}

impl Queue {
    /// 新しい空のキューを作成します。
    pub fn new() -> Self {
        Self {
            customers: Vec::new(),
            length_log: Vec::new(),
        }
    }

    /// 顧客をキューに追加します。
    ///
    /// # 引数
    /// - `customer`: 追加する顧客
    /// - `current_time`: 現在時刻（履歴記録に使用）
    pub fn enqueue(&mut self, customer: Customer, current_time: f64) {
        self.customers.push(customer);
        self.length_log.push((current_time, self.customers.len()));
    }

    /// 現在のキュー長を返します。
    pub fn current_length(&self) -> usize {
        self.customers.len()
    }

    /// キューが空かどうかを返します。
    pub fn is_empty(&self) -> bool {
        self.customers.is_empty()
    }

    /// キューの先頭の顧客を参照します（取り出しはしません）。
    pub fn peek(&self) -> Option<&Customer> {
        self.customers.first()
    }

    /// キューの先頭の顧客を取り出します。
    ///
    /// # 引数
    /// - `current_time`: 現在時刻（履歴記録に使用）
    ///
    /// # 戻り値
    /// - `Some(Customer)`：取り出した顧客
    /// - `None`：キューが空の場合
    pub fn dequeue(&mut self, current_time: f64) -> Option<Customer> {
        let removed = if !self.is_empty() {
            Some(self.customers.remove(0))
        } else {
            None
        };
        self.length_log.push((current_time, self.customers.len()));
        removed
    }

    /// シミュレーション全体の平均キュー長を計算します。
    ///
    /// # 引数
    /// - `total_time`: シミュレーションの総時間
    ///
    /// # 戻り値
    /// - 平均キュー長（面積 / 時間）
    pub fn average_length(&self, total_time: f64) -> f64 {
        let mut area = 0.0;
        for i in 1..self.length_log.len() {
            let (t0, l0) = self.length_log[i - 1];
            let (t1, _) = self.length_log[i];
            area += (t1 - t0) * (l0 as f64);
        }
        area / total_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue_and_dequeue() {
        let mut q = Queue::new();
        let c1 = Customer::new(1, 0.0);
        let c2 = Customer::new(2, 1.0);

        q.enqueue(c1, 0.0);
        q.enqueue(c2, 1.0);

        assert_eq!(q.current_length(), 2);
        assert!(!q.is_empty());
        assert_eq!(q.peek(), Some(&c1));

        let removed = q.dequeue(2.0);
        assert_eq!(removed, Some(c1));
        assert_eq!(q.current_length(), 1);
    }

    #[test]
    fn test_average_length() {
        let mut q = Queue::new();
        let c1 = Customer::new(1, 0.0);
        let c2 = Customer::new(2, 1.0);

        q.enqueue(c1, 0.0);
        q.enqueue(c2, 1.0);
        q.dequeue(3.0);
        q.dequeue(5.0);

        let avg = q.average_length(6.0);
        assert!((avg - 1.166).abs() < 0.01);
    }
}
