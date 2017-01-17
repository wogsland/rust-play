// this generates a compilation error becasue of the moved reference

fn main() {
    let s = "Nachoes, gladly.".to_string();
    let t1 = s;
    let t2 = s;
}
