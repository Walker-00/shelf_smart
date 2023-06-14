use sled;

fn main() {
    let db = sled::open("idk").unwrap();
    db.insert(b"yo!", b"v1");
    println!("{:?}", db.get(b"yo!"));
}
