

#[macro_export]
macro_rules! gerr {
    ($($arg:tt)*) => {
       Err(Box::new(GError::make(&format!($($arg)*)))) 
    };
}

#[macro_export]
macro_rules! commands {
    (($q:expr), {$($k:ident => ($f:expr,$n:expr)),* $(,)?}) => {
        {
            $(
                //add_command(&mut $q, &mut $a,stringify!($k),$f,$n);
                add_command(&mut $q, $f, stringify!($k), $n);
            )*
        }
    };
}


