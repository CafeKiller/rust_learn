// file: Rust学习_Lession11_match控制流和if let控制流


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// #################################################### //

#[derive(Debug)] 
enum UsState {
    Alabama,
    Alaska,
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents2(coin: Coin2) -> u8 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("[println!] State quarter from {:?}!", state);
            25
        },
    }
}

// #################################################### //


fn main() {
    let v1 = Coin::Quarter;

    println!("v1 is {}", value_in_cents(v1));

    let v2 = Coin2::Quarter(UsState::Alabama);

    println!("v2 is {}", value_in_cents2(v2));

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }


    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // 我们想要对 Some(3) 匹配进行操作但是不想处理任何其他 Some<u8> 值或 None 值。
    // 为了满足 match 表达式（穷尽性）的要求，必须在处理完这唯一的成员后加上 _ => ()，这样也要增加很多样板代码。

    let some_u8_value = Some(3);
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let mut count = 0;
    match v2 {
        Coin2::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    
    let mut count = 0;
    if let Coin2::Quarter(state) = v2 {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}



/*

知识点:

    Rust 有一个叫做 match 的极为强大的控制流运算符，它允许我们将一个值与一系列的模式相比较，并根据相匹配的模式执行相应代码。
    模式可由字面值、变量、通配符和许多其他内容构成；
    match 的力量来源于模式的表现力以及编译器检查，它确保了所有可能的情况都得到处理。

    可以把 match 表达式想象成某种硬币分类器：硬币滑入有着不同大小孔洞的轨道，每一个硬币都会掉入符合它大小的孔洞。
    同样地，值也会通过 match 的每一个模式，并且在遇到第一个 “符合” 的模式时，值会进入相关联的代码块并在执行中被使用。

    绑定值的模式
    匹配分支的另一个有用的功能是可以绑定匹配的模式的部分值。这也就是如何从枚举成员中提取值的。

    _ 通配符
    Rust 也提供了一个模式用于不想列举出所有可能值的场景。
    例如，u8 可以拥有 0 到 255 的有效的值，如果我们只关心 1、3、5 和 7 这几个值，
    就并不想必须列出 0、2、4、6、8、9 一直到 255 的值。
    所幸我们不必这么做：可以使用特殊的模式 _ 替代


    if let 语法让我们以一种不那么冗长的方式结合 if 和 let，来处理只匹配一个模式的值而忽略其他模式的情况。

    if let 获取通过等号分隔的一个模式和一个表达式。
    它的工作方式与 match 相同，这里的表达式对应 match 而模式则对应第一个分支。

    使用 if let 意味着编写更少代码，更少的缩进和更少的样板代码。
    然而，这样会失去 match 强制要求的穷尽性检查。
    match 和 if let 之间的选择依赖特定的环境以及增加简洁度和失去穷尽性检查的权衡取舍。

    换句话说，可以认为 if let 是 match 的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值。

    可以在 if let 中包含一个 else。
    else 块中的代码与 match 表达式中的 _ 分支块中的代码相同，这样的 match 表达式就等同于 if let 和 else。

*/


