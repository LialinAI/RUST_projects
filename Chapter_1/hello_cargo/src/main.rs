fn main() {
    println!("Hello, world!");
}

// Компиляция и запуск - это отдельные шаги

// Первая строка TOML, [package], является заголовочной секцией, которая
// указывает что следующие инструкции настраивают пакет

// Последняя строка TOML, [dependencies] является началом секции для
// списка любых зависимостей вашего проекта.

//  cargo build - собирает проект
//  cargo run - скомпилировать код, и затем запустить полученный исполняемый файл
//  Cargo check - быстро проверяет ваш код, чтобы убедиться, что он компилируется,
//  но не создаёт исполняемый файл

// cargo build --release - компиляция с оптимизацией для релизной версии
