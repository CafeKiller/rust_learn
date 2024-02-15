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

}

// 将此行代码加入后就不会出现: `Rectangle` doesn't implement `std::fmt::Display` 错误了
#[derive(Debug)] 
struct Rectangle {
    width: u32,
    height: u32,
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

*/