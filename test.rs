use chrono::{DateTime, Local, NaiveDate};
fn main() {
    let now = Local::now();
    let today = now.date_naive();
    println!("{}", today);
}
