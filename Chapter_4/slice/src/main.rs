fn main() {
    
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    let word2 = first_word(&s[6..11]); // передаем в функцию срез, чтобы получит второе слово строки 

    //срезы также можно делать для массивов
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    for i in slice{
        println!("{}", i);
    }
    
    println!("{}", word);
    println!("{}", word2);

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // представляем строку в виде байтов

    // iter — это метод, который возвращает каждый элемент в коллекции, а enumerate
    // оборачивает результат iter и вместо этого возвращает каждый элемент как часть кортежа.
    // Первый элемент кортежа, возвращаемый из enumerate, является индексом, а второй элемент —
    // ссылкой на элемент. Это немного удобнее, чем вычислять индекс самостоятельно.

    //Поскольку метод enumerate возвращает кортеж, мы можем использовать шаблоны для деструктурирования этого кортежа.
    // Мы подробнее обсудим шаблоны в Главе 6.. В цикле for мы указываем шаблон, имеющий i для индекса в кортеже и &item 
    // для одного байта в кортеже. Поскольку мы получаем ссылку на элемент из .iter().enumerate(), мы используем & в шаблоне.

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // возвращает по одному элементу из первого слова строки
        }
    }

    &s[..] // если строка состоит из одного слова
}
