struct UserRole {
    admin: String,
    buyer: String,
}

struct TupleSturct(i32, i32, i32);

enum ApiResponseCode {
    ResponseCode(u32),
}

fn get_enums(success_code: u32) {
    let response = ApiResponseCode::ResponseCode(success_code);

    match response {
        ApiResponseCode::ResponseCode(code) if code >= 500 && code < 600 => {
            let add: u32 = code * 50;
            print!("{}", add)
        }
        ApiResponseCode::ResponseCode(200) => print!(" Success"),
        ApiResponseCode::ResponseCode(400) => print!("Client Error"),
        _ => println!("Unknown response code: {:.2}", success_code),
    }
}

fn get_username(first_name: String, second_name: String) -> String {
    let new_string = format!(
        "hello my first name is {} and second name is {}",
        first_name, second_name
    );

    new_string
}

fn get_arrays() {
    let arr: (&str, &str) = ("hello", "second");
    print!("{} {}", arr.1, arr.0);
}

fn main() {
    let callback = get_username("obiabo".to_string(), "emmanuel".to_string());
    println!("{}", callback);

    get_enums(500);
    get_arrays()
}
