fn get_username(first_name: String, second_name: String) -> String {
    let new_string = format!(
        "hello my first name is {} and second name is {}",
        first_name, second_name
    );

    new_string
}

fn main() {
    get_username("obiabo".to_string(), "emmanuel".to_string());
 
}
