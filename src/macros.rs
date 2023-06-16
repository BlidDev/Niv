
#[macro_export]

macro_rules! gerr {

    ($($arg:tt)*) => {
       Err(Box::new(GError::make(&format!($($arg)*)))) 
    };

}

#[macro_export]
macro_rules! sgerr {
    ($res:pat, $thing: expr, $($arg:tt),*) => {
        let $res = $thing else {
            return gerr!($($arg),*);
        };
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


#[macro_export]
macro_rules! make_wrapper {
    ($name:ident, $fun:expr, $($p : literal),*) => {

        #[allow(unused_variables)]
        pub fn $name(args : Vec<Type>, glb : &mut Globals, scp : &Scope, qr : &QueryW,
            cnv : &mut Option<Canvas>
            ) -> Result<Type, ERROR> {
            
            $fun($(unstringify!($p)),*)
        }

    };
}


#[macro_export]
macro_rules! make_wrappers {
    ($($name:ident, $fun : expr => [$($p : literal),*]),*) => {
        
        $(
            make_wrapper!($name,$fun, $($p),*);
        )*
    };
}
