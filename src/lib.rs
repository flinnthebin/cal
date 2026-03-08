#[cfg(target_os = "android")]
slint::include_modules!();

use chrono::{DateTime, Datelike, Local};
use slint::ComponentHandle;

#[derive(Default, Debug, Clone)]
pub struct CalendarDay {
    pub date: String,
    pub day: i32,
    pub is_weekend: bool,
    pub is_holiday: bool,
    pub holiday_text: String,
    pub day_name: String,
}

#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    let info = get_current_date();
    let dateinfo = DateInfo {
        weekday: info.weekday.into(),
        year: info.year.into(),
        month: info.month.into(),
        day: info.day.into(),
    };
    let ui = Calendar::new().unwrap();
    ui.set_active_date(dateinfo);
    ui.run().unwrap();
}

fn get_current_date() -> DateInfo {
    let now: DateTime<Local> = Local::now();
    let weekday = now.weekday().to_string().into();
    let dateinfo = DateInfo {
        weekday,
        year: now.year().to_string().into(),
        month: get_month_name(now.month() as i32).into(),
        day: now.day().to_string().into(),
    };

    dateinfo
}

fn get_month_name(month_number: i32) -> String {
    match month_number {
        1 => "January".to_string(),
        2 => "February".to_string(),
        3 => "March".to_string(),
        4 => "April".to_string(),
        5 => "May".to_string(),
        6 => "June".to_string(),
        7 => "July".to_string(),
        8 => "August".to_string(),
        9 => "September".to_string(),
        10 => "October".to_string(),
        11 => "November".to_string(),
        12 => "December".to_string(),
        _ => "Unexpected Value".to_string(),
    }
}
