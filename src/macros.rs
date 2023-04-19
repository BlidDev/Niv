

#[macro_export]
macro_rules! gerr {
    ($($arg:tt)*) => {
       Err(Box::new(GError::make(&format!($($arg)*)))) 
    };
}
