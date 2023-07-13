fn main() {

    // ex1();
    // ex2();
    // loop_in_loop();
    // while_loop();
    // for_loop();
    for_with_range();
    
}

//Ключевое слово loop говорит Rust выполнять блок кода снова и 
//снова до бесконечности или пока не будет явно приказано остановиться.
fn ex1(){
    loop {
        println!("again!"); 
    }
}

fn ex2(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    // Перед циклом мы объявляем переменную с именем counter и инициализируем её значением 0.
    // Затем мы объявляем переменную с именем result для хранения значения, возвращаемого из цикла.
    // На каждой итерации цикла мы добавляем 1 к переменной counter, а затем проверяем, равняется ли
    // 10 переменная counter. Когда это происходит, мы используем ключевое слово break со значением counter * 2.
    // После цикла мы ставим точку с запятой для завершения инструкции, присваивающей значение result. Наконец, мы
    // выводим значение в result, равное в данном случае 20.
    println!("The result is {result}");
}

fn loop_in_loop(){
    let mut count = 0;
    'counting_up: loop { // метка цикла
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // метка цикла
            }
            remaining -= 1;
        }

        count += 1;
    }

    // Если у вас есть циклы внутри циклов, break и continue применяются к самому
    // внутреннему циклу в этой цепочке. При желании вы можете создать метку цикла,
    // которую вы затем сможете использовать с break или continue для указания, что
    // эти ключевые слова применяются к помеченному циклу, а не к самому внутреннему цикл

    println!("End count = {count}");
}

fn while_loop(){
    let mut number = 3;

    while number != 0 { // пока number не равен 0, цикл продолжается
        println!("{number}!");

        number -= 1; // счетчик уменьшающий number
    }

    println!("LIFTOFF!!!");
}

fn for_loop(){
    let a = [10, 20, 30, 40, 50];

    for element in a { // перебор элементов коллекции a с помощью цикла for
        println!("the value is: {element}");
    }
}

fn for_with_range(){
    for number in (1..4).rev() { // пример использования цикла for вместо while
        println!("{number}!");
    }

    //Безопасность и компактность циклов for делают их наиболее часто используемой конструкцией цикла в Rust.
    // Безопасны они, т.к. не могут выйти за пределы коллекции 
    // (1..4) означет диапазон для цикла от 1 до 4 НЕ ВКЛЮЧИТЕЛЬНО
    // rev() - функция для разворота диапозона в обратном порядке
    
    println!("LIFTOFF!!!");
}