#[derive(Default, Debug, Clone)]
pub struct CalendarDay {
    pub date: String,
    pub day: i32,
    pub is_weekend: bool,
    pub is_holiday: bool,
    pub holiday_text: String,
    pub day_name: String,
}

#[cfg(target_os = "android")]
sling::include_modules!();

#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();

    let ui = AppWindow::new().unwrap();
    ui.run().unwrap();
}
