//---------- Переменные
// fn main() {
//     let mut x = 5;
//     println!("Значение x равно {}", x);
//     x = 6;
//     println!("Значение x равно {}", x);
// }


//---------- Затенение
// fn main() {
//     let x = 5;
//     let x = x + 1;
//     let x = x * 2;
//     println!("Значение x равно {}", x);
// }

// fn main() {
//     let spaces = " ";
//     let spaces = spaces.len();
// }


//---------- Числовые операции
// fn main() {
//     let sum = 5 + 10;
//     println!("sum: {}", sum);

//     let difference = 95.5 - 4.3;
//     println!("diff: {}", difference);

//     let product = 4 * 30;
//     println!("prod: {}", product);

//     let quotient = 56.7 / 32.2;
//     println!("quot: {}", quotient);

//     let remainder = 43 % 5;
//     println!("rem: {}", remainder);
// }


//---------- Кортежи
// fn main() {
//     //let tup: (i32, f64, u8) = (500, 6.4, 1);
//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("Значение y равно {}", y);
// }

// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);
//     let five_hundred = x.0;
//     let six_point_four = x.1;
//     let one = x.2;

//     println!("x.0: {}", five_hundred);
//     println!("x.1: {}", six_point_four);
//     println!("x.2: {}", one);
// }


//---------- Массивы
//fn main() {
    //use std::fmt;
    //let a = [1, 2, 3, 4, 5];
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // println!("array: {:?}", a);
    // println!("array length: {:?}", a.len());

    // let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    // println!("year: {:?}", months);
// Доступ к элементам массива
//     let first = a[0];
//     let second = a[1];
//     println!("index1: {:?}", first);
//     println!("index2: {:?}", second);
// }
// Недействительный доступ к элементу массива
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;
    let element = a[index];
    println!("Значение элемента равно {}", element);
}