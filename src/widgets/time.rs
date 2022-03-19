use chrono::Utc;
use chrono::DateTime;

use super::Widget;

// Why does this "#[derive(Copy, Clone)]" not fix the error:
// error[E0507]: cannot move out of `**widget` which is behind a shared reference
//   --> src/main.rs:21:13
//    |
// 21 |             widget.display_text();
//    |             ^^^^^^ move occurs because `**widget` has type `dyn Widget`, which does not implement the `Copy` trait
pub struct Time { }

impl Widget for Time {

    fn name(&self) -> String {
        String::from("time")
    }

    fn display_text(&self) -> String {
        let current_time: DateTime<Utc> = Utc::now();
        format!("{}", current_time.format("%d.%m.%Y %H:%M:%S"))
    }

}
