#[macro_export]
macro_rules! dprintln{
    ($format: expr) =>{
        println!("{} {}","[Debug]".bright_yellow(),$format);
    };
    ($format: expr; $arg: expr) =>{
        println!("{} {}{}", "[Debug]".bright_yellow(), $format, $arg);
    }
}
