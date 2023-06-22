use std::string;

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }
}

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

    // String型は動的メモリで取得している
    // Rustはスマートポインタの仕様を使ってる
    let mut s = String::from("hello");
    let cs = "hello";
    println!("{}",s);

    s.push_str(", world!");

    println!("{}",s);

    let s1 = String::from("hello");

    

    let s2 = s1.clone();

    println!("{}, world!", s1);

    takes_ownership(s1);    
    
    let x = 5;
    makes_copy(x);

    // String::from();と一緒
    let mut s1 = gives_ownership();
    
    let s2 = String::from("hello");

    // s2の所有権を関数に入れてそれを返している=所有権はs2->s3に移った。
    let s3 = takes_and_gives_back(s2);

    let a = calculate_length(&s1);
    println!("{}",a);

    change(&mut s1);

    let mut ss =  String::from("test fawe");

    println!("{}",first_word(&ss)) ;

    s.clear();

    let rect1 = Rectangle{width:30,height:50};
    println!("rect1 is {:#?}",rect1);

    ShowRectangle(&rect1);
    
    println!("{}",rect1.area());
}

fn test_function (x: i32) -> i32
{
    println!("test {}",x);
    x
}

fn takes_ownership(some_string: String)
{
    println!("{}",some_string);
}

fn makes_copy(some_integer: i32)
{
    println!("{}",some_integer);
}

fn gives_ownership() -> String{
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String)->String
{
    a_string
}

// 参照にすれば、メモリ解放までは行われない。
// その中身をいじれる
fn calculate_length(s:&String) -> usize
{
    s.len()
}


fn change(some_string: &mut String)
{
    some_string.push_str(", world");
}

fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();

    for(i,&item) in bytes.iter().enumerate()
    {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn ShowRectangle(rect :&Rectangle)
{
    println!("{:#?}",rect);
}