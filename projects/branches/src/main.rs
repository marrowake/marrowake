// Управление потоком, Выражения if
// fn main() {
//     let number = 7;

//     if number < 5 {
//         println!("условие было истинным");
//     } else {
//         println!("условие было ложным");
//     }
// }

// Обработка нескольких условий с помощью else if
// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("число делится на 4");
//     } else if number % 3 == 0 {
//         println!("число делится на 3");
//     } else if number % 2 == 0 {
//         println!("число делится на 2");
//     } else {
//         println!("число не делится на 4, 3 и 2");
//     }
// }

// Использование выражения if в инструкции let
// fn main() {
//     let condition = true;
//     let number = if condition {
//         5
//     } else {
//         6
//     };
//     println!("Значение числа равно {}", number);
// }

// fn main() {
//     let condition = true;
//     let number = if condition {
//         5
//     } else {
//         "six"
//     };
//     print!("Значение числа равно {}", number);
// }