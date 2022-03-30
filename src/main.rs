// create struct
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// define tuple struct
struct Color(i32, i32, i32);
fn main() {
    // create an instance of struct user
    let user1 = User {
        email: String::from("user1@gmail.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 1,
    };

    println!("user info is {:#?} ", user1);
    // Get the value of the emil of user1
    let email = user1.email;
    println!("email value is {}", email);
    // define a mutable user
    let mut user2 = User {
        email: String::from("user1@gmail.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 1,
    };

    // display the value of active before change
    println!("value is {} ", user2.active);

    // change the state of the active to false for user2
    user2.active = false;

    // display the value of active after change
    println!("value is {} ", user2.active);

    // create instance using build_user function
    let user3 = build_user(String::from("user3@gmail.com"), String::from("user3"));
    println!("user info is {:#?} ", user3);

    // create instance using struct update syntax
    let user4 = User {
        email: String::from("user4@gmail.com"),
        username: String::from("user4"),
        ..user1
    };
    println!("user info is {:#?} ", user4);

    // create instance of tuple struct
    let blue = Color(0, 0, 0255);

    // get a specific value from tuple stuct
    println!("value is {} ", blue.2)
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
