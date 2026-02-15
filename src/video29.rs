pub fn my_main() {
    let name = String::from("bubblegum");
    let len = get_length(&name);

    println!("Длинна '{name}' равна {len} символам.");
    dbg!(&name, &len);
    dbg!(&name, &len);

    // modify_text(&name); // так не получиться name не mutable
}

fn get_length(text: &String) -> usize {
    text.chars().count()

}
/*
fn modify_text(text: &String) {
    text.push_str("!"); // not mutable
}
*/