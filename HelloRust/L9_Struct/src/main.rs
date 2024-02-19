struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        username: String::from("test123"),
        email: String::from("123456@qq.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("user.username => {}", user1.username);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    // 省略写法
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);


    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    
    // 使用元组进行重构
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area1(rect1)
    );

    // 使用结构体重构：赋予更多意义
    let rect1 = Rectangle { width: 30, height: 50 };
    println!(
        "The area of the rectangle is {} square pixels.", 
        area2(&rect1)
    );
    
    // 通过派生 trait 增加实用功能
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is {:?}", rect1); // 此处直接运行会产生错误: `Rectangle` doesn't implement `std::fmt::Display`
    // 此处需要使用 {:?} 占位符


    // 然后在 main 中将我们先前调用 area 方法并传递 rect1 作为参数的地方，
    // 改成使用 方法语法（method syntax）在 Rectangle 实例上调用 area 方法。
    // 方法语法获取一个实例并加上一个点号，后跟方法名、圆括号以及任何参数。
    let rect1 = Rectangle { width: 30, height: 50 };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


    let sq = Rectangle::square(3);

}

// 将此行代码加入后就不会出现: `Rectangle` doesn't implement `std::fmt::Display` 错误了
#[derive(Debug)] 
struct Rectangle {
    width: u32,
    height: u32,
}


impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
/* 
    知识点:

        定义并实例化结构体

            定义结构体，需要使用 struct 关键字并为整个结构体提供一个名字。
            结构体的名字需要描述它所组合的数据的意义。
            接着，在大括号中，定义每一部分数据的名字和类型，我们称为 字段（field）。

            一旦定义了结构体后，为了使用它，通过为每个字段指定具体值来创建这个结构体的"实例"。
            创建一个实例需要以结构体的名字开头，接着在大括号中使用 key: value 键-值对的形式提供字段，
            其中 key 是字段的名字，value 是需要存储在字段中的数据值。
            实例中字段的顺序不需要和它们在结构体中声明的顺序一致。
            换句话说，结构体的定义就像一个类型的通用模板，而实例则会在这个模板中放入特定数据来创建这个类型的值。


        使用结构体更新语法从其他实例创建实例
            使用旧实例的大部分值但改变其部分值来创建一个新的结构体实例通常是很有帮助的。
            这可以通过 结构体更新语法（struct update syntax）实现。


        使用没有命名字段的元组结构体来创建不同的类型
            也可以定义与元组类似的结构体，称为 元组结构体（tuple structs）。
            元组结构体有着结构体名称提供的含义，但没有具体的字段名，只有字段的类型。
            当你想给整个元组取一个名字，并使元组成为与其他元组不同的类型时，
            元组结构体是很有用的，这时像常规结构体那样为每个字段命名就显得多余和形式化了。

            要定义元组结构体，以 struct 关键字和结构体名开头并后跟元组中的类型。

        
        没有任何字段的类单元结构体
            我们也可以定义一个没有任何字段的结构体！
            它们被称为"类单元结构体"（unit-like structs）因为它们类似于 ()，即 unit 类型。
            类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用。


        方法
            方法与函数类似：它们使用 fn 关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法时会执行的代码。
            不过方法与函数是不同的，因为它们在结构体的上下文中被定义（或者是枚举或 trait 对象的上下文). 
            并且它们第一个参数总是 self，它代表调用该方法的结构体实例。


            定义方法
                impl Rectangle {
                    fn area(&self) -> u32 {
                        self.width * self.height
                    }
                }
                
                为了使函数定义于 Rectangle 的上下文中，我们开始了一个 impl 块（impl 是 implementation 的缩写）。
                接着将 area 函数移动到 impl 大括号中，并将签名中的第一个（在这里也是唯一一个）参数和函数体中其他地方的对应参数改成 self。

                在 area 的签名中，使用 &self 来替代 rectangle: &Rectangle，
                因为该方法位于 impl Rectangle 上下文中所以 Rust 知道 self 的类型是 Rectangle。
                注意仍然需要在 self 前面加上 &，就像 &Rectangle 一样。方法可以选择获取 self 的所有权，
                或者像我们这里一样不可变地借用 self，或者可变地借用 self，就跟其他参数一样。
                

                这里选择 &self 的理由跟在函数版本中使用 &Rectangle 是相同的：我们并不想获取所有权，只希望能够读取结构体中的数据，而不是写入。
                如果想要在方法中改变调用方法的实例，需要将第一个参数改为 &mut self。
                通过仅仅使用 self 作为第一个参数来使方法获取实例的所有权是很少见的；
                这种技术通常用在当方法将 self 转换成别的实例的时候，这时我们想要防止调用者在转换之后使用原始的实例。


                使用方法替代函数，除了可使用方法语法和不需要在每个函数签名中重复 self 的类型之外，其主要好处在于组织性。
                我们将某个类型实例能做的所有事情都一起放入 impl 块中，而不是让将来的用户在我们的库中到处寻找 Rectangle 的功能。


                关于 -> 运算符到哪去了？
                    在 C/C++ 语言中，有两个不同的运算符来调用方法：. 直接在对象上调用方法，
                    而 -> 在一个对象的指针上调用方法，这时需要先解引用（dereference）指针。
                    换句话说，如果 object 是一个指针，那么 object->something() 就像 (*object).something() 一样。

                    Rust 并没有一个与 -> 等效的运算符；相反，Rust 有一个叫 自动引用和解引用（automatic referencing and dereferencing）的功能。
                    方法调用是 Rust 中少数几个拥有这种行为的地方。
                    
                    他是这样工作的：当使用 object.something() 调用方法时，Rust 会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配。
                    也就是说，这些代码是等价的：
                        p1.distance(&p2);
                        (&p1).distance(&p2);

                    第一行看起来简洁的多。
                    这种自动引用的行为之所以有效，是因为方法有一个明确的接收者———— self 的类型。
                    在给出接收者和方法名的前提下，Rust 可以明确地计算出方法是仅仅读取（&self），做出修改（&mut self）或者是获取所有权（self）。
                    事实上，Rust 对方法接收者的隐式借用让所有权在实践中更友好。


                关联函数
                    impl 块的另一个有用的功能是：允许在 impl 块中定义 不 以 self 作为参数的函数。
                    这被称为 关联函数（associated functions），因为它们与结构体相关联。
                    它们仍是函数而不是方法，因为它们并不作用于一个结构体的实例。
                    你已经使用过 String::from 关联函数了。

                
                多个 impl 块
                    每个结构体都允许拥有多个 impl 块。
*/