/*
    Struct :

    assosiated functions :
        - not tied to instance of struct
        - do not take &self argument

    struct methods
        - take the &self argument
        - these are the functions



*/
fn main() {
    println!("Hello, world!");

    let user1: User = User {
        username: String::from("User1"),
        email: String::from("user1@email.com"),
        sign_in_count: 32,
        active: true,
    };

    println!("User1 is : {:#?}", user1);
    println!(
        "username : {}\nemail : {}\nsign_in_count : {}\nisactive : {}",
        user1.username, user1.email, user1.sign_in_count, user1.active
    );

    // types of struct
    /*
       tuple struct : structs without name fields
       color(i32,i32,i32)
    */

    // creating rectangle and calculating its area
    let rect1: Rectangle = Rectangle {
        width: 32,
        height: 64,
        color: Color(255, 123, 32),
    };

    println!("Rectangle having the area : {}", rect1.area());
    println!(
        "This rectangle has the color : {}, {}, {}",
        rect1.color.0, rect1.color.1, rect1.color.2
    );

    // create the square from it
    let sq1: Rectangle = Rectangle::square(32);
    println!(
        "this square has the area : {} sq. unit, having the side {} units",
        sq1.area(),
        sq1.height
    );
}

// derive debug trait allows the syntax {:#?}, it can only be applied to the structs and its impementations
#[derive(Debug)]
struct User {
    // named fields : fields with names
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u64,
    height: u64,
    color: Color,
}

impl Rectangle {
    fn area(&self) -> u64 {
        self.width * self.height
    }
}

impl Rectangle {
    fn square(side: u64) -> Rectangle {
        Rectangle {
            width: side,
            height: side,
            color: Color(0, 0, 0),
        }
    }
}

// tuple structs
#[derive(Debug)]
struct Color(i32, i32, i32);

// unit structs : structs without any fields
