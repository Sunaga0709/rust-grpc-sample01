use chrono::Local;

pub fn get_timestamp() -> i32 {
    Local::now().timestamp() as i32
}
