fn main() {

    let s1 = String::from("hello");

    let len = calculate_length(&s1); // передаем в функцию неизменяемую ссылку
    // Тем самым мы не берем ответственность за значение переменной

    println!("The length of '{}' is {}.", s1, len);

    //////

    let mut s = String::from("hello");

    change(&mut s);

    ///////

    // let r1 = &mut s; // не может быть двух изменяемых ссылок на одну и ту же переменную
    // let r2 = &mut s;

    ///////

    // {
    //     let r1 = &mut s;
    // } // r1 goes out of scope here, so we can make a new reference with no problems.

    // let r2 = &mut s;

    /////////

    // let r1 = &s; // no problem (две неизменяемые ссылку могут существовать в одной области видимости)
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    /////////

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);


    // Из функции нельзя возвращать ссылку на переменую созданную внутри функции, так как возникнет висячая ссылка и переменная не верентся
    // Поэтому из функции мы возвращаем саму переменную, а не ссылку на нее



}

fn calculate_length(s: &String) -> usize { //принимаем ссылку типа String
    s.len()
    // s.push_str(", world"); // не прокатит, так как мы изменяем ссылку, а она в данном примере неизменяемая
} // выходим изобласти видимости

fn change(some_string: &mut String) {
    some_string.push_str(", world");
    println!( "{}", some_string);
}







