use super::Payment;
use super::Status;

use chrono::{NaiveDateTime, Utc};

/// Convenience methods
impl Payment {
    // Is the payment url expired :
    // - > 5 days
    // - (nowpayments > 7 days)
    pub fn is_expired(&self) -> bool {
        let now: NaiveDateTime = Utc::now().naive_utc();
        let diff = now - self.created_at;
        diff.num_days() > 4
    }
    pub fn is_used(&self) -> bool {
        vec![Status::Confirming, Status::Confirmed, Status::Sending].contains(&self.status)
    }
    pub fn is_finished(&self) -> bool {
        vec![
            Status::Finished,
            Status::PartiallyPaid,
            Status::Failed,
            Status::Refunded,
        ]
        .contains(&self.status)
    }
    // The payment status is unknown.
    pub fn is_unknown(&self) -> bool {
        vec![Status::Unknown].contains(&self.status)
    }
}
