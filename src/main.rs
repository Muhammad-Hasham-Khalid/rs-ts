fn _basics() {
    let data = vec![1, 2, 3];
    let mut iter = data.iter().map(|x| x + 1);

    let mut collected_values = vec![1, 2, 3];

    while let Some(x) = iter.next() {
        collected_values.push(x);
    }

    println!("{:?}", collected_values);
}

fn read_file_and_print() {
    let file = std::fs::read_to_string("lines").unwrap();

    file.lines()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .skip(2)
        .take(2)
        .for_each(|(_, line)| println!("{}", line));
}

fn main() {
    read_file_and_print();
}
