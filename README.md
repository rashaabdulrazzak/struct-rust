# Struct in Rust

What are Struct?
it is a custom
datatype to group related data together
and name that data in a meaningful way.  
What are the difference between Tuple and Struct?

- both hold multiple related values.

- Like tuples, the pieces of a struct can be different types.
- Unlike with tuples, in a struct you’ll name each piece of data so it’s clear what the values mean.
- Adding these names means that structs are more flexible than tuples: you don’t have to rely on the order of the data to specify or access the values of an instance.

Define a struct:

```
struct User{
username: String,
 email: String,
sign_in_count: u64,
active: bool,
}
```

How to use the Struct?

we need to create instance of the struct to use it.

when creating instance there is no need to We don’t have to specify the fields in the same order in which we declared them in the struct.

Ways of creatig instance:

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

```

3. Use an exsiting instance to create a new one

```

```

##### How to get a specific value from a Struct

we use dot notation to get a specific value from a Struct
example if we want the username of user1 we can type `user1.username`

##### How we change a specific value in a Struct

if the struct instance is defined as mut variable we can change the specific value from the Struct using the dot notation also.
in the following example we will create a new instance of the user struct and we will make it mutable :

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
