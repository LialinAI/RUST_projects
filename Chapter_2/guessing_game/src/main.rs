use std::io; //включаем в область видимости io из библиотеки std
use rand::Rng; //включаем в область видимости Rng из библиотеки rand
use std::cmp::Ordering; //включаем в область видимости cmp::Ordering из библиотеки std

fn main() {
    println!("Guess the number!"); // печатает текст,  восклицательный знак означет, что это макрос 

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // функция thred_rng предоставляет нам генератор случайных чисел
    // gen_range - генерирует случайное число в диапозоне, в нашем случае от 1 до 100  

    // println!("The secret number is: {secret_number}"); // печать сгенерированного
    // случайного числа

    // loop - бесконечный цикл исаользующийся для запуска программы заново при
    // неправильном вводе данных от пользователя 
    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); 

    // let используется для создания НЕИЗМЕНЯЕМОЙ переменной
    // let mut используется для создания ИЗМЕНЯЕМОЙ переменной
    // Синтаксис :: в строке ::new указывает, что new является ассоциированной функцией типа String
    // Ассоциированная функция — это функция, реализованная для типа, в данном случае String

    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

            // read_line получает ввод от пользователя и записывает в строку 
            // & - ссылка, добавляя mut получаем изменяемую ссылку 
            // excpect - печать текста при возникновении ошибки  

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // trim - удалит любые пробельные символы в начале и конце строки для того,
        // чтобы мы могли сопоставить строку с u32, который содержит только числовые данные.

        // Метод parse строк преобразует строку в другой тип.
        // Здесь мы используем его для преобразования строки в число

        // Исрользуем match, чтобы перейти от аварийного завершения при ошибке к обработке ошибки.
        // То есть если строку преобразовать в число можно, то продолжаем выполнение программы
        // Если строку в число преобразовать нельзя, то цикл начинается сначала (continue)

        println!("You guessed: {guess}");

        // Тип Ordering является ещё одним перечислением и имеет варианты Less, Greater и Equal
        // Это три возможных исхода при сравнении двух величин.
        // Метод cmp сравнивает два значения и может вызываться для всего, что можно сравнить
        // Он принимает ссылку на все, что требуется сравнить: здесь сравнивается guess с secret_number
        // В результате возвращается вариант перечисления Ordering, которое мы ввели в область видимости с помощью оператора use
        // Для принятия решения о том, что делать дальше, мы используем выражение match, определяющее, какой вариант Ordering был возвращён
        // из вызова cmp со значениями guess и secret_numbe

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            }
        }
    }
}   
