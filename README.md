# Struct in Rust

##### What are Struct?

it is a custom datatype to group related data together and name that data in a meaningful way.

##### What are the difference between Tuple and Struct?

- both hold multiple related values.

- Like tuples, the pieces of a struct can be different types.
- Unlike with tuples, in a struct you’ll name each piece of data so it’s clear what the values mean.
- Adding these names means that structs are more flexible than tuples: you don’t have to rely on the order of the data to specify or access the values of an instance.

##### Define a struct:

```
struct User{
username: String,
 email: String,
sign_in_count: u64,
active: bool,
}
```

##### What are the type of Struct?

1. Default one when you define the struct and name its fields as in the above example.
2. Tuple Struct: like tuples they don't have names associated with their fields
   example of define and use of tuple struct

```
 struct Color(i32, i32, i32);
fn main() {
    let black = Color(0, 0, 0);

}
```

we can access its individual value using `.` followed by the index

##### How to use the Struct?

we need to create instance of the struct to use it.

when creating instance there is no need to We don’t have to specify the fields in the same order in which we declared them in the struct.

##### How many Ways do we have to create an instance of struct?

1. create the whole instance from scratch

```
fn main() {
    // create an instance of struct user
    let user1 = User {
        email: String::from("user1@gmail.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 1,
    };
}
```

2. Use a function to create the instance of struct

```
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

```

this function takes the email and username as parameter and assign the other value to the default value of the srtuct user.
To create an instance using the above function we write:

```
let user3 = build_user(String::from("user3@gmail.com"), String::from("user3"));

```

3. Use an exsiting instance to create a new one : called struct update syntax
   It’s often useful to create a new instance of a struct that includes most of the values from another instance, but changes some. You can do this using struct update syntax.

```
fn main() {
    // create an instance of struct user
    let user1 = User {
        email: String::from("user1@gmail.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 1,
    };
    // create instance using struct update syntax
    let user4 = User {
        email: String::from("user4@gmail.com"),
        username: String::from("user4"),
        ..user1
    };
    println!("user info is {:#?} ", user4);
}
```

**Note**
It is important to realize that if we create a new instance with struct update method but we let the new one take the username or email from the user1 then we won't be able to use user1 after that declartion.because the String in the username field of user1 was moved into user2. as the string is not copy trait

##### How to get a specific value from a Struct

we use dot notation to get a specific value from a Struct
example if we want the username of user1 we can type `user1.username`

##### How we change a specific value in a Struct

if the struct instance is defined as **mut** variable we can change the specific field value from the Struct using the dot notation. Always remember Rust doesn’t allow us to mark only certain fields as mutable.
In the following example we will create a new instance of the user struct and we will make it mutable :

```
fn main() {
   // create an instance of struct user
   let mut user2 = User {
       email: String::from("user1@gmail.com"),
       username: String::from("user1"),
       active: true,
       sign_in_count: 1,
   };
}
```

then we can change the status of active using dot notation like this `user2.active = false`

##### How to print a struct instance?

we use println! macro with `{:#?}` to pass the variable info into it
and to be able to use that we need to add `#[derive(Debug)]` before the struct. here is how we print the user1:

```
println!("user info is {:#?} ", user1);

```
