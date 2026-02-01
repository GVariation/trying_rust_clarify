fn main() {
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
}