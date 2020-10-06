use chrono::{DateTime, NaiveDateTime, Utc};

use crate::object_data::DateTimeUtc;

#[test]
fn it_should_return_a_datetime_struct() {
    let date = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc);
    let date = DateTimeUtc::new(date);
    assert_eq!("Thu, 01 Jan 1970 00:01:01 +0000", date.get_date_as_string());
}
