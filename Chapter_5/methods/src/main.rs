#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//Чтобы определить функцию в контексте Rectangle, мы создаём блок
// impl (implementation - реализация) для Rectangle. Всё в impl будет связано
// с типом Rectangle. Затем мы перемещаем функцию area внутрь фигурных скобок impl
// и меняем первый (и в данном случае единственный) параметр на self в сигнатуре и в теле.
// В main, где мы вызвали функцию area и передали rect1 в качестве аргумента, теперь мы можем
// использовать синтаксис метода для вызова метода area нашего экземпляра Rectangle. Синтаксис
// метода идёт после экземпляра: мы добавляем точку, за которой следует имя метода, круглые скобки и любые аргументы.

impl Rectangle { // может быть несколько блоков impl для одной и той же структуры (для удобства)
    //Мы выбрали &self здесь по той же причине, по которой использовали &Rectangle
    // в версии кода с функцией: мы не хотим брать структуру во владение, мы просто
    // хотим прочитать данные в структуре, а не писать в неё.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //Метод с несколькими параметрами
    //кроме self передаем еще ссылку на другой экземпляр класса для сравнения их ширины и высоты
    fn can_hold(&self, other: &Rectangle) -> bool { 
        self.width > other.width && self.height > other.height
    }


    //Ассоциированные функции, не являющиеся методами, часто используются для конструкторов,
    // возвращающих новый экземпляр структуры. Их часто называют new, но new не является специальным
    // именем и не встроена в язык. Например, мы можем предоставить ассоциированную функцию с именем
    // square, которая будет иметь один параметр размера и использовать его как ширину и высоту, что 
    // упростит создание квадратного Rectangle, вместо того, чтобы указывать одно и то же значение дважды

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    //  вызов асоциированной функции для создания структуры Rectangle с параметрами
    // width: 100, height: 100 (квадрат)
    let rect3 = Rectangle::square(100); 

    println!(
        "{:?}\n",
        rect3
    );

    println!(
        "The area of the rectangle is {} square pixels.\n",
        rect1.area() // считаем площадь с помощью метода area структуры Recatangle
    );

    // проверяем поместится ли один прямоугольник в другой
    // с помощью метода can_hold структуры Recatangle
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); 
}
