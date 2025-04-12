// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.


#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // The check `my_option.is_none()` followed by `my_option.unwrap()` is problematic.
    // Since my_option is always None here, the unwrap would always panic.
    // We remove this block as it doesn't serve a useful purpose in this context.
    // If the intention was to do something if it's Some, `if let Some(_) = my_option` would be appropriate.

    let my_arr = &[
        -1, -2, -3, // Add comma here
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // `Vec::resize` returns (), not the Vec. To create an empty Vec:
    let mut my_vec = vec![1, 2, 3, 4, 5];
    my_vec.clear(); // Or my_vec.resize(0, 0); if you need a specific default value (not applicable here)
    println!("This Vec is empty, see? {:?}", my_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
