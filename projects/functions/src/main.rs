//---------- Функции
// fn main() {
//     another_function();
//     println!("Hello, world!");
//     // another_function();
// }

// fn another_function() {
//     println!("Еще одна функция.")
// }
//---------- Параметры функций
// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("Значение x равно {}", x);
// }

// fn main() {
//     another_function(5, 6);
// }

// fn another_function(x: i32, y: i32) {
//     println!("Значение x равно {}", x);
//     println!("Значение y равно {}", y);
// }


//---------- Инструкции и выражения в телах функций
// fn main() {
//     let x = 5;

//     let y = {

//         let x = 3;
        
//         x + 1
//         //Выражения не включают конечные точки с  запятой. Если вы добавляете точку
//         //с запятой в конец выражения, то превращаете его в инструкцию, которая в таком
//         //случае не возвращает значение
//     };

//     println!("Значение y равно {}", y);
// }

//---------- Функции с возвращаемыми значениями
// fn five() -> i32 {
//     5 // <- выражение, значение которого мы хотим вернуть
// }

// fn main() {
//     let x = five();
//     println!("Значение x равно {}", x);
// }

// fn main() {
//     let x = plus_one(5);
//     println!("Значение x равно {}", x);
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }

//Управление потоком (if, else)

// fn main() {
//     let number = 7;

//     if number < 5 {
//         println!("Condition was true")
//     } else {
//         println!("Condition was false")
//     }
// }

// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("Number divides by 4")
//     } else if number % 3 == 0 {
//         println!("Number divides by 3")
//     } else if number % 2 == 0 {
//         println!("Number divides by 2")
//     } else {
//         println!("Number doesn't divide by 4, 3 and 2")
//     }
// }

fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        "six"
    };

    println!("The number is {}", number)
}