use std::string;

fn main() {
    println!("Hello, world!");

    let guess: u32 = "42".parse().expect("a");

    let tup = (500,6.4,1);
    let (x,y,z) = tup;
    println!("{},{},{}",tup.0,tup.1,tup.2);
    
    let a: [i32; 5] = [1,2,3,4,5];
    
    println!("{}",test_function(23));

    if a[0] == 1{

    }else if a[0] ==2{

    }

    let condition = true;

    // let number = if condition {5} else {"six"};
    // ↑これはエラー

    let mut count = 0;
    'counting_up: loop
    {
        count+=1;
        println!("{}",count);
        if count == 4
        {
            break 'counting_up;
        }
    }

    for i in (1..4).rev(){
        println!("{}",i);
    }

    let mut s = String::from("hello");
    let cs = "hello";
    println!("{}",s);

    s.push_str(", world!");

    println!("{}",s);


}

fn test_function (x: i32) -> i32
{
    println!("test {}",x);
    x
}