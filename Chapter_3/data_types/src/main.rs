fn main() {
    let x = 2.0; // f64, где f - float

    let y: f32 = 3.0; // f32
    println!("{} {}", x, y);

    // addition
    let sum = 5 + 10;
    println!("{}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("{}", difference);

    // multiplication
    let product = 4 * 30;
    println!("{}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("{}", quotient);

    let truncated = -5 / 3; // Results in -1
    println!("{}", truncated);

    // remainder
    let remainder = 11 % 4;
    println!("{}", remainder);

    let t = true;
    println!("{}", t);

    let f: bool = false; // with explicit type annotation
    println!("{}", f);

    let c = 'z';
    println!("{}", c);

    let z: char = 'ℤ'; // with explicit type annotation
    println!("{}", z);

    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1); 
    let (x, y, z) = tup; // присвоили переменным значения кортежа

    // tup - кортеж - универсальный способ объединения нескольких значений с различными типами
    // в один составной тип. Кортежи имеют фиксированную длину: после объявления они не могут увеличиваться или уменьшаться в размерах.

    println!("The value of y is: {y}"); // выводим y, то есть 2 значение в кортеже

    let five_hundred = tup.0; // записываем в переменную первое значение кортежа
    println!("The value of the first element of tuple is: {five_hundred}");

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    // months - массив - В отличие от кортежа, каждый элемент массива должен иметь один и тот же тип. 
    // В отличие от массивов в некоторых других языках, массивы в Rust имеют фиксированную длину.

    println!("The last month of the year is: {}", months[11]) // вывод 12 элемента массива
}
