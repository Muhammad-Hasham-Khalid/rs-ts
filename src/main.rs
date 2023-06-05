fn _basics() {
    let data = vec![1, 2, 3];
    let mut iter = data.iter().map(|x| x + 1);

    let mut collected_values = vec![1, 2, 3];

    while let Some(x) = iter.next() {
        collected_values.push(x);
    }

    println!("{:?}", collected_values);
}

fn _read_file_and_print() {
    let file = std::fs::read_to_string("lines").unwrap();

    file.lines()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .skip(2)
        .take(2)
        .for_each(|(_, line)| println!("{}", line));
}

/*
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Color {
    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        }

        return false;
    }

    fn is_green_parts(&self) -> bool {
        match self {
            Color::Red => return false,
            Color::Green => return false,
            Color::Blue => return true,
            Color::Yellow => return true,
        }
    }
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
        Color::Yellow => println!("yellow"),
    }
}
*/

/*
struct Custom {
    age: usize,
    name: String,
}

enum Item {
    Number(usize),
    String(String),
    MyCustom(Custom),
}

fn append(items: &mut Vec<Item>) {
    items.push(Item::String("Hello, FEM".to_string()));
}
*/

fn _multiply(val: Option<usize>) -> Option<usize> {
    return Some(val? * 5);
}

fn main() {}
