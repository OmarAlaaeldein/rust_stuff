fn main(){
    let a:u16=4;
    let b:u16=3;
    println!("Hello world");
    println!("sum of {} and {} is {}",a,b,sum_unsigned(a,b));
}
fn sum_unsigned(a:u16,b:u16)->u16
{   
    return a+b;
}