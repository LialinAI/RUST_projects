
// Для определения структуры указывается ключевое слово struct и её название.
// Название должно описывать значение частей данных, сгруппированных вместе. 
//Далее, в фигурных скобках для каждой новой части данных поочерёдно определяются
// имя части данных и её тип. Каждая пара имя: тип называется полем.

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Кортежные структуры: структуры без именованных полей для создания разных типов
#[derive(Debug)]
struct Point(i32, i32, i32); // данная кортежная структура создана для опредления координат точки в трехмерном пространстве


// единично-подобная структура
// Единично-подобные структуры могут быть полезны, когда требуется реализовать типаж
// для некоторого типа, но у вас нет данных, которые нужно хранить в самом типе.
struct AlwaysEqual;



fn main() {

    let mut user1 = build_user("jkl@ya.ru".to_string(), "TYUI".to_string()); //экземпляр структуры 

    user1.email = String::from("anotheremail@example.com"); // обновление поля email экземпляра класса user1

    println!("{:?}", user1);

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    }; // Создание экземпляра структуры из экземпляра другой структуры с помощью синтаксиса обновления структуры
    // в этом случае данные типа string будут затерты из экземпляра класса user1

    println!("{:?}", user2);

    // println!("{:?}", user1); // получим ошибку

    let origin = Point(0, 0, 0);

    println!("{:?}", origin);

    let sub = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User { //функция для создания экземпляра класса
    User {
        active: true,
        username: username, // username,- сокращенная инициализация поля 
        email: email, // email, - сокращенная инициализация поля 
        sign_in_count: 1,
    }

}