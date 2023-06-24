struct UserRole {
    admin: String,
    buyer: String,
}

struct TupleSturct(i32, i32, i32);

#[derive(Debug)]
enum AccessRoles {
    Admin,
    Customer,
}

enum RequestCode {
    Success(u32),
    ClientError(u32),
    ServerError(u32),
}

fn get_enums(success_code: u32) {
    let success = RequestCode::Success(success_code);

    println!("this is the successcode {}", success_code)
}

fn get_username(first_name: String, second_name: String) -> String {
    let new_string = format!(
        "hello my first name is {} and second name is {}",
        first_name, second_name
    );

    new_string
}

fn get_struct() {
    let customer = UserRole {
        admin: "whoami".to_string(),
        buyer: "emmanuel".to_string(),
    };

    let get_customer = customer.admin + " " + &customer.buyer;
    print!("user is {}", get_customer);

    let tuple1: TupleSturct = TupleSturct(1, 2, 3);

    let assign_one = tuple1.0;
    if assign_one == 1 {
        println!("this is the index of {}", assign_one);
    } else {
        println!("sorry no match for this");
    }
}

fn main() {
    let callback = get_username("obiabo".to_string(), "emmanuel".to_string());
    println!("{}", callback);

    get_struct();
    get_enums(200);
}
