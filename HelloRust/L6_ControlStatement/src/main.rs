fn main() {
    println!("Hello, world!");

    let number = 6;

    if number > 5 {
        println!("number > 5")
    } else {
        println!("number < 5")
    }

    /* if number {
        println!("number was three")
    } */

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let z = if number > 5 {
        -1
    } else {
        0
    };
    println!("z ===>>> {}", z);

    let mut counter= 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result = {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    };


    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

}

/* 
    知识点

    if 表达式
        if 表达式允许根据条件执行不同的代码分支。
        你提供一个条件并表示 “如果条件满足，运行这段代码；如果条件不满足，不运行这段代码。”

        所有的 if 表达式都以 if 关键字开头，其后跟一个条件。
        if 表达式中与条件关联的代码块有时被叫做 arms
        也可以包含一个可选的 else 表达式来提供一个在条件为假时应当执行的代码块. 
        如果不提供 else 表达式并且条件为假时，程序会直接忽略 if 代码块并继续执行下面的代码。

        值得注意的是代码中的条件 必须 是 bool 值。如果条件不是 bool 值，我们将得到一个错误。
            if number {
                println!("number was three");
            }
            这个错误表明 Rust 期望一个 bool 却得到了一个整数。
            不像 Ruby 或 JavaScript 这样的语言，Rust 并不会尝试自动地将非布尔值转换为布尔值。
            必须总是显式地使用布尔值作为 if 的条件。

        使用 else if 处理多重条件
            可以将 else if 表达式与 if 和 else 组合来实现多重条件。
            使用过多的 else if 表达式会使代码显得杂乱无章，所以如果有多于一个 else if 表达式，最好重构代码。

        在 let 语句中使用 if
            因为 if 是一个表达式，我们可以在 let 语句的右侧使用它

        
    使用循环重复执行
            Rust 有三种循环：loop、while 和 for。

            使用 loop 重复执行代码
                loop 关键字告诉 Rust 一遍又一遍地执行一段代码直到你明确要求停止。

                从循环返回
                    loop 的一个用例是重试可能会失败的操作，比如检查线程是否完成了任务。
                    然而你可能会需要将操作的结果传递给其它的代码。
                    如果将返回值加入你用来停止循环的 break 表达式，它会被停止的循环返回.

            while 条件循环
                在程序中计算循环的条件也很常见。当条件为真，执行循环。
                当条件不再为真，调用 break 停止循环。
                这个循环类型可以通过组合 loop、if、else 和 break 来实现；

                这种结构消除了很多使用 loop、if、else 和 break 时所必须的嵌套，这样更加清晰。当条件为真就执行，否则退出循环。
            
            使用 for 遍历集合
                for 循环的安全性和简洁性使得它成为 Rust 中使用最多的循环结构。
                即使是在想要循环执行代码特定次数时.

                大部分 Rustacean 也会使用 for 循环。
                这么做的方式是使用 Range，它是标准库提供的类型，用来生成从一个数字开始到另一个数字之前结束的所有数字的序列。


*/