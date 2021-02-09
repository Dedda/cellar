type ConvertResult<T> = Result<T, String>;

pub fn value_as_int(value: &str) -> ConvertResult<i64> {
    match value.parse() {
        Ok(int) => Ok(int),
        Err(_) => Err(format!("Cannot parse `{}` as integer", value)),
    }
}

pub fn value_as_float(value: &str) -> ConvertResult<f64> {
    match value.parse() {
        Ok(float) => Ok(float),
        Err(_) => Err(format!("Cannot parse `{}` as float", value)),
    }
}