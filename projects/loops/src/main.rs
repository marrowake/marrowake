// Повторение кода с помощью цикла loop
// fn main() {
//     loop {
//         println!("еще раз!");
//     }
// }

// Возвращение значений из циклов
// fn main() {
//     let mut counter = 0;
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//     println!("Результат равен {}", result);
// }

// Условные циклы с использованием while
// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{}", number);

//         number = number - 1;
//     }

//     println!("GO");
// }

// Осуществление цикла в коллекции с помощью for
// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("Значение равно {}", a[index]);

//         index = index + 1;
//     }
// }
// ИЛИ
// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a.iter() {
//         println!("Значение равно {}", element);
//     }
// }

