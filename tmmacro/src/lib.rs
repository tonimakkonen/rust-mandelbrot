
#[macro_export]
macro_rules! file_name_and_content {
        ($a:tt)=>{
            ($a, include_str!($a))
        };
    }