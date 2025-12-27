/*
    enums : grouping related values into one type
        - enumerate a list of varients
        - when to use enum & struct
            - when to use enum :
                - when you have a fixed set of related values that can be grouped together
                - when you want to represent a value that can be one of several different types
                - eg : ipv4, ipv6 <-- only two versions

            - when to use struct :
                - when you want to group related data together into a single entity
                - when you want to create complex data types with multiple fields

    option enum : handling absence of value
        - enum Option<T> {
            Some(T),
            None,
        }
        - prevents null pointer exceptions
        - forces handling of absence of value at compile time
*/
fn main() {
    println!("Hello, world!");

    // accessing enums in main
    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));
    let _home_struct = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let _loopback_struct = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("Using struct to print info:");
    _home.print_info();
    _loopback.print_info();
    _home_struct.print_info();
    _loopback_struct.print_info();
}

// ip address enum
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
} // <-- directly defining the data inside the enum varients

impl IpAddr {
    fn print_info(&self) {
        match self {
            IpAddr::V4(a, b, c, d) => {
                println!(
                    "IPv4 Address individual values :\n\n  0 : {}\n  1 : {}\n  2 : {}\n  3 : {}\n",
                    a, b, c, d
                );
            }
            IpAddr::V6(addr) => {
                println!("IPv6 Address: {}", addr);
            }
        }
    }
}

enum IpAddrKind {
    V4,
    V6,
} // <-- just defining the varients without any data

// defining struct for ip address
struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

impl IpAddrStruct {
    fn print_info(&self) {
        match self.kind {
            IpAddrKind::V4 => println!("IPv4 Address: {}", self.address),
            IpAddrKind::V6 => println!("IPv6 Address: {}", self.address),
        }
    }
}
