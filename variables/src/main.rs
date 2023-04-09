// Fix the error below with least amount of modification to the code
// fn main() {
//     let x: i32 = 5; // Uninitialized but used, ERROR !
//     let y: i32; // Uninitialized but also unused, only a Warning !

//     assert_eq!(x, 5);
//     println!("Success!");
// }
// fn main() {
//     let mut x = 1;
//     x += 2;

//     assert_eq!(x, 3);
//     println!("Success!");
// }
fn main() {
    let my_array: [i32; 5] = [34, 34, 34, 34, 34];
    println!("{:?}", my_array)
}
