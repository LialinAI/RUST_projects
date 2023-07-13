fn main() {

    ex1();
    ex2();
    ex3();

}

fn ex1(){
    let number = 3; // неизменяемая переменная

    if number < 5 { // Если number < 5
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Если number < 5 напечатется condition was true,
    // В любом другом случае выведется condition was false
    // В данном примере будет выполнено условие if
}

fn ex2(){
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2"); // условие не проходит, тк выполнилось пердыдущее, хотя 6 делится на 2
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // else if предполагает, что если какаое-то из верхних условий было выполнено, то
    // все условия, которые ниже проверяться не будут
}

fn ex3(){
    let condition = true;
    let number = if condition { 5 } else { 6 }; // короткая запись 

    // выдаст ошибку из-за несовместимости типов данных
    // let number = if condition { 5 } else { "six" };


    println!("The value of number is: {number}");
}

