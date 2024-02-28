
// 定义一个 IP 枚举
#[derive(Debug)]
enum IPAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IPAddr {
    kind: IPAddrKind,
    address: String,
}

// IpAddr 枚举的新定义表明了 V4 和 V6 成员都关联了 String 值
#[derive(Debug)]
enum IPAddr2 {
    V4(String),
    V6(String),
}


#[derive(Debug)]
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Quit 没有关联任何数据。
// Move 包含一个匿名结构体。
// Write 包含单独一个 String。
// ChangeColor 包含三个 i32。
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {

    // 注意枚举的成员位于其标识符的命名空间中，并使用两个冒号分开。
    let four = IPAddrKind::V4;
    let six = IPAddrKind::V6;
    println!("V4 = {:?}, V6 = {:?}", four, six);


    let home = IPAddr {
        kind: IPAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IPAddr {
        kind: IPAddrKind::V6,
        address: String::from("::1"),
    };
    println!("home = {:?}, loopback = {:?}", home, loopback);


    //我们可以使用一种更简洁的方式来表达相同的概念，仅仅使用枚举并将数据直接放进每一个枚举成员而不是将枚举作为结构体的一部分。
    // IpAddr 枚举的新定义表明了 V4 和 V6 成员都关联了 String 值
    // 我们直接将数据附加到枚举的每个成员上，这样就不需要一个额外的结构体了。
    let home = IPAddr2::V4(String::from("127.0.0.1"));
    let loopback: IPAddr2 = IPAddr2::V6(String::from("::1"));
    println!("home = {:?}, loopback = {:?}", home, loopback);


    // 用枚举替代结构体还有另一个优势：每个成员可以处理不同类型和数量的数据。
    // IPv4 版本的 IP 地址总是含有四个值在 0 和 255 之间的数字部分。
    // 如果我们想要将 V4 地址存储为四个 u8 值而 V6 地址仍然表现为一个 String，这就不能使用结构体了。
    let home = IpAddr3::V4(127, 0, 0, 1);    
    let loopback = IpAddr3::V6(String::from("::1"));
    println!("home = {:?}, loopback = {:?}", home, loopback);

}

fn route(ip_type:IPAddrKind) {
    println!("route push {:?}", ip_type);
} 




/* 

    关于空值问题

        空值的问题在于当你尝试像一个非空值那样使用一个空值，会出现某种形式的错误。因为空和非空的属性无处不在，非常容易出现这类错误。
        然而，空值尝试表达的概念仍然是有意义的：空值是一个因为某种原因目前无效或缺失的值。
        问题不在于概念而在于具体的实现。为此，Rust 并没有空值，不过它确实拥有一个可以编码存在或不存在概念的枚举。
        这个枚举是 Option<T>

        enum Option<T> {
            Some(T),
            None,
        }


        Option<T> 枚举是如此有用以至于它甚至被包含在了 prelude 之中，你不需要将其显式引入作用域。
        另外，它的成员也是如此，可以不需要 Option:: 前缀来直接使用 Some 和 None。
        即便如此 Option<T> 也仍是常规的枚举，Some(T) 和 None 仍是 Option<T> 的成员。


        let some_number = Some(5);
        let some_string = Some("a string");
        let absent_number: Option<i32> = None;
        如果使用 None 而不是 Some，需要告诉 Rust Option<T> 是什么类型的，因为编译器只通过 None 值无法推断出 Some 成员保存的值的类型。

        当有一个 Some 值时，我们就知道存在一个值，而这个值保存在 Some 中。
        当有个 None 值时，在某种意义上，它跟空值具有相同的意义：并没有一个有效的值。

        简而言之，因为 Option<T> 和 T（这里 T 可以是任何类型）是不同的类型，编译器不允许像一个肯定有效的值那样使用 Option<T>

*/