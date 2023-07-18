// Создадим новую библиотеку (библиотечный крейт) с именем restaurant выполнив команду cargo new restaurant --lib;

// Мы определяем модуль, начиная с ключевого слова mod, затем определяем название модуля
// (в данном случае front_of_house) и размещаем фигурные скобки вокруг тела модуля.
// Внутри модулей, можно иметь другие модули, как в случае с модулями hosting и serving.
// Модули также могут содержать определения для других элементов, таких как структуры,
// перечисления, константы, типажи

// Модуль сам по себе является исключительно контейнером

// Дерево также показывает, что некоторые модули являются братьями (siblings) друг для друга,
// то есть они определены в одном модуле; hosting и serving это братья которые определены внутри
// front_of_house. Если модуль A содержится внутри модуля B, мы говорим, что модуль A является
// *потомком * (child) модуля B, а модуль B является *родителем * (parent) модуля A. Обратите внимание,
// что родителем всего дерева модулей является неявный модуль с именем restaurant.

// Хотя front_of_house не является общедоступным, но поскольку функция eat_at_restaurant определена в том же модуле,
// что и front_of_house (то есть, eat_at_restaurant и front_of_house являются потомками одного родителя), мы можем ссылаться
// на front_of_house из eat_at_restaurant.

mod front_of_house {
    // pub используется для доступа к разделу hosting, так как по умолчанию он является приватным
    // но мы не получаем доступ к содержимому модуля, так как функции все еще являются приватными
    // поэтому для доступа к функции необходимо тоже добавить ключевое слово pub
    pub mod hosting { 
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// Функция fix_incorrect_order находится в модуле back_of_house, поэтому мы можем
// использовать super для перехода к родительскому модулю модуля back_of_house, который
// в этом случае является crate, корневым модулем. В этом модуле мы ищем deliver_order и
// находим его. Успех! Мы думаем, что модуль back_of_house и функция deliver_order, скорее
// всего, останутся в тех же родственных отношениях друг с другом, и должны будут перемещены
// вместе, если мы решим реорганизовать дерево модулей крейта. Поэтому мы использовали super,
// чтобы в будущем у нас было меньше мест для обновления кода, если этот код будет перемещён в другой модуль.

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    ////////

    //Если мы используем pub перед определением структуры, мы делаем структуру
    // общедоступной, но поля структуры по-прежнему остаются приватными.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {

        // поскольку back_of_house::Breakfast имеет приватное поле, то структура должна
        // предоставить публичную ассоциированную функцию, которая создаёт экземпляр Breakfast
        // (мы назвали её summer). Если Breakfast не имел бы такой функции, мы бы не могли создать
        // экземпляр Breakfast внутри eat_at_restaurant2, потому что мы не смогли бы установить
        // значение приватного поля seasonal_fruit в функции eat_at_restaurant2.
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    /////////

    //  если мы сделаем общедоступным перечисление, то все его варианты будут общедоступными.
    // Нужно только указать pub перед ключевым словом enum

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant3() { 

    // Поскольку мы сделали общедоступным перечисление Appetizer, то можно использовать
    // варианты Soup и Salad в функции eat_at_restaurant.

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

pub fn eat_at_restaurant2() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Поскольку поле toast в структуре back_of_house::Breakfast является открытым,
    // то в функции eat_at_restaurant2 можно писать и читать поле toast, используя точечную нотацию. 

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal

    // мы не можем использовать поле seasonal_fruit в eat_at_restaurant2,
    // потому что seasonal_fruit является приватным.

    // meal.seasonal_fruit = String::from("blueberries");
}

// мы подключили модуль crate::front_of_house::hosting в область действия функции eat_at_restaurant
// use создаёт псевдоним только для той конкретной области, в которой это объявление use и находится
use crate::front_of_house::hosting; 

// неправильное подключение с точки зрения понимания, т.к.
// Указание родительского модуля при вызове функции (как в примере выше) даёт понять,
// что функция не определена локально, но в то же время сводя к минимуму повторение полного пути. 
use crate::front_of_house::hosting::add_to_waitlist;

//С другой стороны, при подключении структур, перечислений и других элементов используя use,
// идиоматически правильным будет указывать полный путь.

//////

//До этого изменения внешний код должен был вызывать функцию add_to_waitlist
// , используя путь restaurant::front_of_house::hosting::add_to_waitlist() . Теперь,
// когда это объявление pub use повторно экспортировало модуль hosting из корневого
// модуля, внешний код теперь может использовать вместо него путь restaurant::hosting::add_to_waitlist() .

// Используя pub use , мы можем написать наш код с одной структурой, но сделать общедоступной другую структуру.
// Благодаря этому наша библиотека хорошо организована для программистов, работающих над библиотекой

//pub use crate::front_of_house::hosting;



pub fn eat_at_restaurant() {
    // Absolute path
    // абсолютный путь - это полный путь, начинающийся от корневого модуля крейта;
    // для кода из внешнего крейта абсолютный путь начинается с имени крейта, а для
    // кода из текущего крейта он начинается с литерала crate.
    crate::front_of_house::hosting::add_to_waitlist(); // вроде бы crate = restaurant

    // Relative path
    // относительный путь начинается с текущего модуля и использует ключевые слова self,
    // super или идентификатор в текущем модуле.
    front_of_house::hosting::add_to_waitlist();

    // мы подключили модуль crate::front_of_house::hosting в область действия функции eat_at_restaurant,
    // поэтому нам достаточно только указать hosting::add_to_waitlist для вызова функции add_to_waitlist внутри eat_at_restaurant. 
    hosting::add_to_waitlist();
}


////////////////////////

// Как видите, использование имени родительских модулей позволяет различать два типа Result.
// Если бы вместо этого мы указали use std::fmt::Result и use std::io::Result, мы бы имели два
// типа Result в одной области видимости, и Rust не смог бы понять какой из двух Result мы имели
// в виду, когда нашёл бы их употребление в коде.
use std::fmt;
use std::io;

// fn function1() -> fmt::Result {
  
//     // --snip--
// }

// fn function2() -> io::Result<()> {
    
//     // --snip--
// }


//Есть другое решение проблемы добавления двух типов с одинаковыми именами в одну
// и ту же область видимости используя use: после пути можно указать as и новое
// локальное имя (псевдоним) для типа. 

use std::fmt::Result;
use std::io::Result as IoResult;

// fn function3() -> Result {
   
//     // --snip--
// }

// fn function4() -> IoResult<()> {
  
//     // --snip--
// }

use rand::Rng; //используем внешний пакет, который скачали добавив в Cargo.toml (rand = "0.8.5")

/////////

// Обычное использование use
// use std::cmp::Ordering;
// use std::io;

//Использование вложенного пути для use, чтобы уменьшить количество строк кода
//use std::{io, cmp::Ordering};


//Обычный use
// use std::io;
// use std::io::Write;

//Можно использовать вложенный путь на любом уровне
//use std::io::{self, Write};

//Если мы хотим включить в область видимости все общедоступные элементы, определённые в пути,
// мы можем указать этот путь, за которым следует оператор *:
use std::collections::*;



