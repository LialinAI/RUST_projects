#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {

    // Если значение равно Some, мы распечатываем значение в варианте Some, привязывая
    // значение к переменной max в шаблоне. Мы не хотим ничего делать со значением None.
    // Чтобы удовлетворить выражение match, мы должны добавить _ => () после обработки первой
    // и единственной ветки, и добавление шаблонного кода раздражает.

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    //Синтаксис if let позволяет скомбинировать if и let в менее многословную конструкцию,
    // и затем обработать значения соответствующе только одному шаблону, одновременно игнорируя все остальные.

    // Код в блоке if let не запускается, если значение не соответствует шаблону.

    let config_max2 = Some(3u8);
    if let Some(max) = config_max2 {
        println!("The maximum is configured to be {}", max);
    }

    // два примера выше идентичны друг другу, но написаны через разные конструкции

    let coin = Coin::Quarter(UsState::Alabama);
    // let coin = Coin::Penny;

    let mut count = 0;
    match coin {
        Coin::Quarter(ref state) => println!("State quarter from {:?}!", state),
        _ => {count += 1;
            println!("match: {}", count);
        }
    }

    // Можно добавлять else к if let. Блок кода, который находится внутри else аналогичен
    // по смыслу блоку кода ветки связанной с шаблоном _ выражения match

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
        println!("if let: {}", count);   
    }

    // два примера выше идентичны друг другу, но написаны через разные конструкции
}
