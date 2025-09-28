/// 顧客を表す構造体
///
/// `Customer` は到着時刻、サービス開始・終了時刻を持ち、
/// 待ち時間や滞在時間を計算するメソッドを提供する。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Customer {
    pub customer_id: u64,
    pub arrival_time: f64,
    pub service_start_time: Option<f64>,
    pub service_end_time: Option<f64>,
}
impl Customer {
    /// 新しい顧客を生成する
    ///
    /// `customer_id` と `arrival_time` を指定し、サービス時刻は未定義で初期化される。
    pub fn new(customer_id: u64, arrival_time: f64) -> Self {
        Customer {
            customer_id,
            arrival_time,
            service_start_time: None,
            service_end_time: None,
        }
    }
    /// 待ち時間を返す
    ///
    /// サービス開始時刻が未定義の場合は `None` を返す。
    pub fn wait_time(&self) -> Option<f64> {
        match self.service_start_time {
            Some(start) => Some(start - self.arrival_time),
            None => None,
        }
    }
    /// サービス時間を返す
    ///
    /// 開始・終了時刻が両方定義されている場合のみ計算される。
    pub fn service_time(&self) -> Option<f64> {
        match (self.service_start_time, self.service_end_time) {
            (Some(start), Some(end)) => Some(end - start),
            _ => None,
        }
    }
    /// システム内の滞在時間を返す
    ///
    /// サービス終了時刻が未定義の場合は `None` を返す。
    pub fn total_time_in_system(&self) -> Option<f64> {
        match self.service_end_time {
            Some(end) => Some(end - self.arrival_time),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_customer_initialization() {
        let c = Customer::new(1, 5.0);
        assert_eq!(c.customer_id, 1);
        assert_eq!(c.arrival_time, 5.0);
        assert!(c.service_start_time.is_none());
        assert!(c.service_end_time.is_none());
    }

    #[test]
    fn test_wait_time() {
        let c = Customer {
            customer_id: 2,
            arrival_time: 3.0,
            service_start_time: Some(5.0),
            service_end_time: None,
        };
        assert_eq!(c.wait_time(), Some(2.0));
    }

    #[test]
    fn test_service_time() {
        let c = Customer {
            customer_id: 3,
            arrival_time: 2.0,
            service_start_time: Some(4.0),
            service_end_time: Some(7.0),
        };
        assert_eq!(c.service_time(), Some(3.0));
    }

    #[test]
    fn test_total_time_in_system() {
        let c = Customer {
            customer_id: 4,
            arrival_time: 1.0,
            service_start_time: Some(2.0),
            service_end_time: Some(6.0),
        };
        assert_eq!(c.total_time_in_system(), Some(5.0));
    }

    #[test]
    fn test_incomplete_times() {
        let c = Customer {
            customer_id: 5,
            arrival_time: 0.0,
            service_start_time: None,
            service_end_time: None,
        };
        assert_eq!(c.wait_time(), None);
        assert_eq!(c.service_time(), None);
        assert_eq!(c.total_time_in_system(), None);
    }
}
