fn hold_my_vec<T>(_: Vec<T>) {}

fn main() {
    let v = vec![2, 3, 5, 0, 11, 13, 17];
    hold_my_vec(v);
    let element = v.get(3);

    println!("I got this element from the vector: {:?}", element);
    println!("Omg, this feels so good and illegal 🤤")
}
