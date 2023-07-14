fn main() {
    // Владение — это набор правил, определяющих, как программа на языке Rust управляет памятью.

    // Все данные, хранящиеся в стеке, должны иметь известный фиксированный размер. Данные, размер
    // которых во время компиляции неизвестен или может измениться, должны храниться в куче.

    // Помещение в стек происходит более быстро, чем выделение памяти в куче, потому что операционная
    // система не должна искать место для размещения информации — это место всегда на верхушке стека. 
    // Для сравнения, выделение памяти в куче куче требует больше работы, потому что операционная система
    // сначала должна найти участок памяти достаточного размера, а затем произвести некоторые действия для
    // подготовки к следующему выделению памяти.

    // Область видимости
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    //////

    let x = 5;
    let y = x;

    // Мы можем догадаться, что делает этот код: «привязать значение 5 к x;
    // затем сделать копию значения в x и привязать его к y». Теперь у нас есть две переменные:
    // x и y, и обе равны 5. Это то, что происходит на самом деле, потому что целые числа — это
    // простые значения с известным фиксированным размером, и эти два значения 5 помещаются в стек.

    let s1 = String::from("hello");
    let s2 = s1;

    // Когда мы присваиваем s1 значению s2, данные String копируются, то есть мы копируем указатель, длину и ёмкость,
    // которые находятся в стеке. Мы не копируем данные в куче, на которые указывает указатель.

    // Чтобы обеспечить безопасность памяти, после строки let s2 = s1; , Rust считает s1 более недействительным. Следовательно,
    // Rust не нужно ничего освобождать, когда s1 выходит за пределы области видимости. Посмотрите, что происходит, когда вы пытаетесь
    // использовать s1 после создания s2


    ////

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // В данном случае мы получим глубокую копию, то есть будет выделена еще одна область памяти под переменную  s2


    //////

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // Но этот код, кажется, противоречит тому, что мы только что узнали: у нас нет вызова clone, но x всё ещё действителен
    // и не был перемещён в y.

    // Причина в том, что такие типы, как целые числа, размер которых известен во время компиляции, полностью хранятся в стеке,
    // поэтому копии фактических значений создаются быстро. Это означает, что нет причин, по которым мы хотели бы предотвратить доступность
    // x после того, как создадим переменную y. Другими словами, здесь нет разницы между глубоким и поверхностным копированием, поэтому вызов
    // clone ничем не отличается от обычного поверхностного копирования, и мы можем его опустить.

    let f1 = gives_ownership();         // gives_ownership moves its return
    // value into s1

    let f2 = String::from("hello");     // f2 comes into scope

    let f3 = takes_and_gives_back(f2);  // f2 is moved into
                                        // takes_and_gives_back, which also
                                         // moves its return value into f3

} // Here, f3 goes out of scope and is dropped. f2 was moved, so nothing
  // happens. f1 goes out of scope and is dropped.


fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

let some_string = String::from("yours"); // some_string comes into scope

some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

a_string  // a_string is returned and moves out to the calling function
}

