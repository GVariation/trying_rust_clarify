/*
fn hello(name: &str) {
    println!("Привет, {name}");
}

fn repeat(text: &str, times: usize) {
    println!("{}", text.repeat(times));
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    return celsius * 9.0 / 5.0 + 32.0;
}

fn add(a: i32, b: i32) -> isize {
    println!("Adding {a} + {b}");
    (a + b) as isize
}

fn remember() {
    //так не работает
    //let var = (let var2 = 1);
    let number: i8 = 10;
    let pi: f32 = 3.1415;
    let turned_on: bool = false;
    let chars: char = '';

    //let coordinates: (f32, f32) = (1.5, 2.5);
    //let people: [&str; 3] = ["bg", "thx", "silmarion"];

    println!("{number} {pi} {turned_on} {chars}");
    println!("{}", isize::MAX);
    println!("{}", isize::MIN);

    let a: f64 = 0.1;
    let b: f64 = 0.2;
    let sum: f64 = a + b;
    println!("sum={}", sum);

    let money: i32 = 5_000;
    println!("money > 0 = {}", money > 0);

    if money > 0 {
        println!("Ну что там с деньгами?");
    }

    //let letter = 'z';
    //let omega: char = 'Ω';
    let heart: char = '♡';
    
    println!("heart = {}", heart);

    let data: (u8, f32, bool) = (10, 3.5, false);
    let (n, d, b) = data;

    println!("{n}, {d}, {b}.");
    let first = data.0;
    let second = data.1;
    let last = data.2;

    println!("Первый элемент: {}", data.0);
    println!("Второй элемент: {}", second);
    println!("Первый и третий элемент через запятую: {}, {}",first, last);

    let coordinates: (f32, f32) = (2.5, 1.5);
    //let (x, y) = coordinates;
    //let x = coordinates.0;
    //let y = coordinates.1;
    //let empty: () = ();
    println!("Координаты: {:?}", coordinates);

    //let numbers = [1, 2, 3, 4, 5];
    let days = ["Пн", "Вт", "Ср", "Чт", "Пт", "Сб", "Вс"];
    //let numbers: [u8; 3] = [1, 2, 3];
    //println!("Числа:{:?}", numbers);
    //let repeat = ["Bob"; 5];
    //println!("repeat={:?}", repeat);
    let first = days[0];
    let last = days[6];
    println!("Первый и последный день: {first}, {last}.");

    hello("Лёлик");
    hello("Болик");

    repeat("lol", 5);

    println!("celsius_to_fahrenheit(20.0)={:?}", celsius_to_fahrenheit(20.0));
    
    let result = add(3,4);
    dbg!(result);

    dbg!(20 + 50);
}

/* 
коментарий
в
несколько
строк
*/

fn get_rating(movie: &str) -> i32 {
    // Movie API Docs: https//www.movie-ratings.com/docs
    let rating: i32 = get_movie_data(movie);
    rating
}

// Это простенькая функция используется для тестирования
fn get_movie_data(movie: &str) -> i32 {
    10
}

fn vid15_controlflow() {
    //check_length("qwertyuiop123");
    //check_length("123");

    if long_enought("qwertyuiop123") {
        println!("Пароль достаточно длинный");
    } else {
        println!("Пароль слишком короткий");
    }
}

fn check_length(password: &str) {
    let length = password.len();

    if length >= 10 {
        println!("'{password}' достаточно длинный");
    } else {
        println!("'{password}' не достаточно длинный");
    }
}

fn long_enought(password: &str) -> bool {
    // .len считает длинну в байтах
    let length = password.len();

    if length >= 10 {
        true
    } else {
        false
    }
    // вместо блока if .. else выше, можно записать одной строкой:
    // length >= 10
}

fn long_enought_char_count() {
    // как "правильно" посчиатать символы
    let letgth = password.chars().count();
}

fn main() {
    dbg!(get_response("Привет чел"));
    dbg!(get_response("Как дел"));
    dbg!(get_response("Норм"));
    dbg!(get_response("Это что сейчас было?!"));

}

fn get_response(input: &str) -> &str {
    let lowered: String = input.to_lowercase();

    if lowered.contains("привет") {
        "И тебе привет"
    } else if lowered.contains("как дел") {
        "Норм, сам как?"
    } else if lowered.contains("норм") {
        "эт хорошо"
    } else {
        "Я не понимаю..."
    }
}
fn main() {
    analyze_number(5);
}

fn analyze_number(n: i32) {
    if n > 10 {
        println!("{n} больше чем 10");
    } else if n > 0 {
        println!("{n} больше чем 0");
    } else {
        println!("{n} крутое число");
    }
}

fn main() {
    let n = 10;
    let odd_even = if n % 2 == 0 {"четное"} else {"не четное"};
    dbg!(odd_even);
    
    // не сработает следующий код, состояния if else должны быть одного типа
    // let is_connected = false;
    // let result = if is_connected {"connected"} else {-1};
    // dbg!(result);
}
*/

fn main() {
    /*
    // будет выполняться бесконечно (пока не кончиться размер у переменной)
    let mut n = 0;
    loop {
        n += 1;
        println!("n={:?}", n);
    }
    */

    /*
    let mut counter = 5;
    loop {
        println!("Счёт: {counter}");
        counter -=1;

        if counter == 0 {
            println!("Мы досчитали до 0");
            break;
        }
    }
    println!("Выполняется код после loop")
    */
    let mut counter = 0;
    let result = loop {
        println!("Счёт: {counter}");
        counter += 1;

        if counter == 5 {
            break counter;
        }
    };   
    println!("result={:?}", result)
}