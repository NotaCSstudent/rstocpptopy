// build.rs

extern crate cc;
//extern crate cc;
fn rstocpp()
{
    cc::Build::new()
    .file("bar.cpp")
    .compile("bar");
   
}




fn main()
{
 
    rstocpp();
    
}