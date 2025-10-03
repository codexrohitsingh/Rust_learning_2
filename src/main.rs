// Write a functino is_even that takes a number as an imput and retuns true if it is even 
// fn main() {
//     println!("{}",is_even(22));

// }

// fn is_even(a:i32)->bool{
//     if a%2==0
//     {
//         return true;
//     }
//     else{
//         return false;
//     }
// }


// ------------------------------------------------------------------------------------

// Write a function fib that finds the fibbonacci of a number it takes as input 

// fn main(){
//     is_fibonacci(5);
// }

// fn is_fibonacci(n:i32)->i32{
//     let mut first_number= 0; 
//     let mut second_number =1;
//     if n==0{
//         return 0;
//     }
//     if n==1{
//         return 1;
//     }
//     // let mut third_number :i32=first_number+second_number;
//     let mut third_number=first_number+second_number;
//     for _i in 2..n{
//         third_number=first_number+second_number;
//         first_number=second_number;
//         second_number=third_number;
//     }
//     print!("{}",third_number);
//     return third_number;
// }

// ------------------------------------------------------------------------------------


// Question 3 
// Write a function get_string_length that takes a string as as input and return its
// length


fn main(){
    // let string_name =  get_string_length("Hi there ");
    let str = String::from("Hello");
    print!("{}",get_string_length(str));
    // print!("{:?}",string_name);

}

fn get_string_length(name:String)->usize{
     let nam= name.chars().count();
    // print!("{}",nam);
    return  nam;

}

