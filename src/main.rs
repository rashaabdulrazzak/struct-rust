// create struct
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
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
}
