use std::io;
// use std::any;

fn main() {
    // start_f_and_c();
    // start_fibo();
    start_song();
    
}

fn start_song() {
    let numbers: [&str; 12]= ["первый","второй","третий","четвертый","пятый","шестой","седьмой","восьмой","девятый","десятый","одиннадцатый","двенадцатый"];

    let mut strings: [&str; 12] = ["Двенадцать прыгающих лордов,","Одиннадцать танцующих дам,","Десять трубящих трубачей,","Девять стучащих барабанщиков,","Восьмерых доярок,","Семь плавающих лебедей,","Шесть гусынь, несущих яйца,","Пять золотых колец,","Четырех певчих птиц,","Трех фаверолей,","Двух горлиц","И куропатку на грушевом дереве."];

    let first_part: [&str; 3] = ["На","день Рождества","Моя любовь подарила мне"];

    let single: &str = "Куропатку на грушевом дереве.";

    let mut n: u32 = 0;
    let mut y: usize = 0;
    let mut x: usize = 0;
    
    strings.reverse(); // преобразование массива в обратный порядок
    // for k in strings{
    //     print!("{} ", k);
    // }
    
    for i in 0..11 { // цикл для куплетов
        
        if i == 0 { // условие для исполенения блока 1 раз
            for k in first_part{ // цикл для получения слов из массива
                if n == 0 { // условие для исполнения блока 1 раз (вывод "На первый")
                    print!("{} ", k);
                    print!("{} ", numbers[i]);
                }
                else {
                    println!("{}", k); // вывод остальной части массива
                }
                n+=1;
            }
            println!("{}\n", single); //вывод отдельной строки
        }

        n=0;
        y+=1; //счетчик
        x=0;

        while x < y{ // цикл для вывода определенного количества строк из  массива
            
            if  n == 0 { // блок для вывода "На {номер дня} день Рождества Моя любовь подарила мне"
                for k in first_part { // 
                    if n == 0 {
                        print!("{} ", k);
                        print!("{} ", numbers[i+1]);
                    }
                    else {
                        println!("{}", k);
                    }
                    n+=1;
                }
            }
            println!("{}", strings[y-x]); //вывод определенной строки из массива
            x+=1;//счетчик
        }
        println!("{}\n", strings[0]); // Вывод последней строки каждого куплета
        
    }
}   
    


fn start_fibo() {
    println!("Программа для нахождения n-го числа Фиббоначи");

    loop {
    println!("Введите n");
    
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: u32 = match number.trim().parse()  {
        Ok(number) => number,
        Err(_) => continue,
        };
    
    let answ: u32 = fibo(number); // присваивание переменной вывод функции  
    println!("Ответ: {}", answ);
    break; // досрочный выход из цикла
    }


}

fn fibo(number: u32) -> u32 {
    let zero_number: u32 = 0;
    let one_number: u32 = 1;

    if number == 0{
        return zero_number
    }
        
    if number == 1 {
        return one_number
    }
    
    return fibo(number-1)+fibo(number-2) // рекурсия 

}

fn start_f_and_c(){
    println!("Программа для перевода температуры из шкалы Фаренгейта в шкалу Цельсия и наоборот");

    loop {
    println!("Введите в какую шкалу вы хотите перевести температуру\n1 - из Цельсия в Фаренгейта\n2 - из Фаренгейта в Цельсия");
    
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: u32 = match number.trim().parse()  {
        Ok(number) => number,
        Err(_) => continue,
        };

    // print_type_of(&number)

    if number==1 || number==2 {  // если пользователь правильно воспользовался меню
        let mut temp = String::new();
    
        println!("Введите температуру");
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");
    
        
        let temp: f32 = match temp.trim().parse()  {
            Ok(temp) => temp,
            Err(_) => continue,
            };
    
        fahrenheit_and_celsius(number, temp); //запуск функции рассчета градусов
        // number - номер пункта меню, temp - температура     
        }
    else { // если пользователь неправильно воспользовался меню, то цикл запускается заново
        continue;
        }

    break;
    }
}

fn fahrenheit_and_celsius(scale: u32, temperature: f32){
    if scale == 2{ //пункт меню
        let temperature = (5.0*(temperature-32.0))/9.0; // перевод в градусы Цельсия
        println!("Температура в градусах Цельсия: {}", temperature);
    }
    else if scale == 1 { // пункт меню
        let temperature = (9.0*temperature)/5.0+32.0; // перевод в градусы Фаренгейта
        println!("Температура в градусах Фаренгейта: {}", temperature);

    }
    else {
        println!("Нет такого варианта ответа");
    }
}

