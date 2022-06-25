fn main() {
    let a = [10, 20, 30, 40, 50, ];
    let mut index = 0;
    while index<5 {
        println!("Значение равно {}", a[index]);
        index = index+1;
    }
    println!("а тут for!");
    let b = a;
    for element in a.iter() {
        println!("Значение равно {}", element);
    }
    println!("Обратный отчет");
    for elem in (1..4).rev(){
        println!("{}", elem);
    }
    println!("{}",myfunc(44))
}
fn myfunc(i:i32)->String{
        let s:&str= "Hello";
    let s1 = String::from("Hello!!!");
    s1
}
