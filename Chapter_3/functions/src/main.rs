fn main() {
    println!("Hello, world!");

    another_function(6, 'h');
    // Поскольку в программе определена another_function,
    // её можно вызвать из функции main.
    // Передаем в нее 2 аргумента
    statments_and_expressions();

    let five = 5;
    println!("Result of func plus_one: {}", plus_one(five));
}

fn another_function(x: u32, y: char) { // принимает 2 аргумента и выводит их
    println!("Value of x and y: {} {} ", x, y);

}

fn statments_and_expressions() {
    let y = {
        let x = 3; //statement оператор
        x + 1 //expression выражение
    };

    // Операторы — это инструкции, которые выполняют какое-либо действие и не возвращают значения.
    // Выражения вычисляются до результирующего значения. Давайте рассмотрим несколько примеров.

    println!("The value of y is: {y}");
}

fn plus_one(x: i32) -> i32 { // -> u32 означает тип данных возвращаемый функцией
    x + 1 // это то, что возвращает функция (последняя строка указанная в функции)
    // ключевое слово return можно не указывать
}

//все что заканчиваться на ; - оператор, если ; отсутствует, то это выражение