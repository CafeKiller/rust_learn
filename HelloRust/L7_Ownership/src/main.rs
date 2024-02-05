fn main() {
    println!("Hello, world!");

    { // s 在这里无效, 它尚未声明
        let s = "hello"; // 从此处起，s 是有效的
        
        println!("s = {}", s); // 使用 s
    } // 此作用域已结束，s 不再有效

    // println!("s = {}", s); // s不再作用域内. 此处代码运行会报错

    let mut str = String::from("Hello");
    // 这两个冒号（::）是运算符，允许将特定的 from 函数置于 String 类型的命名空间（namespace）下，而不需要使用类似 string_from 这样的名字。

    str.push_str(", World!!!");

    println!("str = {}", str);

    let x = 5;
    let y = x;

    println!("x={}, y={}.", x, y);

    let s1 = String::from("Hello");
    let s2 = s1;

    // println!("s1 = {}, s2 = {}", s1, s2); // s1 已经被移动了, 无法进行访问.  此处代码运行会报错
    println!("s2 = {}", s2);

    
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2); // s2 克隆了s1的值, 此时s1的值没有被移动, 所以可以正常访问.


}


/* 
    知识点:

        所有权（系统）是 Rust 最为与众不同的特性，它让 Rust 无需垃圾回收（garbage collector）即可保障内存安全。

        什么是所有权？
            Rust 的核心功能（之一）是 所有权（ownership）。

            所有运行的程序都必须管理其使用计算机内存的方式。
            一些语言中具有垃圾回收机制，在程序运行时不断地寻找不再使用的内存；
            在另一些语言中，程序员必须亲自分配和释放内存。

            Rust 则选择了第三种方式：
                通过所有权系统管理内存，编译器在编译时会根据一系列的规则进行检查。
                在运行时，所有权系统的任何功能都不会减慢程序。
        
        
        所有权规则
            1、Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
            2、值有且只有一个所有者。
            3、当所有者（变量）离开作用域，这个值将被丢弃。


        变量作用域
            { // s 在这里无效, 它尚未声明
                let s = "hello"; // 从此处起，s 是有效的
                
                println!("s = {}", s); // 使用 s
            } // 此作用域已结束，s 不再有效
            
            - 当 s 进入作用域 时，它就是有效的。
            - 这一直持续到它 离开作用域 为止。

        
        String 类型
            字符串值被硬编码进程序里。
            字符串字面值是很方便的，不过他们并不适合使用文本的每一种场景。
            原因之一就是他们是不可变的。
            另一个原因是并不是所有字符串的值都能在编写代码时就知道

            Rust 有第二个字符串类型，String。
            这个类型被分配到堆上，所以能够存储在编译时未知大小的文本。
            可以使用 from 函数基于字符串字面值来创建 String.

        
        内存与分配
            就字符串字面值来说，我们在编译时就知道其内容，所以文本被直接硬编码进最终的可执行文件中。
            这使得字符串字面值快速且高效。不过这些特性都只得益于字符串字面值的不可变性。
            
            不过我们不能为了每一个在编译时大小未知的文本而将一块内存放入二进制文件中，并且它的大小还可能随着程序运行而改变。

            对于 String 类型，为了支持一个可变，可增长的文本片段，需要在堆上分配一块在编译时未知大小的内存来存放内容。这意味着：
                - 必须在运行时向操作系统请求内存。
                - 需要一个当我们处理完 String 时将内存返回给操作系统的方法。

                第一部分由我们完成：当调用 String::from 时，它的实现 (implementation) 请求其所需的内存。
                这在编程语言中是非常通用的。

                然而，第二部分实现起来就各有区别了。
                在有 垃圾回收（garbage collector，GC）的语言中， GC 记录并清除不再使用的内存，而我们并不需要关心它。
                没有 GC 的话，识别出不再使用的内存并调用代码显式释放就是我们的责任了，跟请求内存的时候一样。

                从历史的角度上说正确处理内存回收曾经是一个困难的编程问题。
                    如果忘记回收了会浪费内存。
                    如果过早回收了，将会出现无效变量。
                    如果重复回收，这也是个 bug(二次回收)。

                >> Rust 采取了一个不同的策略：__内存在拥有它的变量离开作用域后就被自动释放__。

                    { // s 在这里无效, 它尚未声明
                        let s = "hello"; // 从此处起，s 是有效的
                        
                        println!("s = {}", s); // 使用 s
                    } // 此作用域已结束，s 不再有效

                    这是一个将 String 需要的内存返回给操作系统的很自然的位置：
                        当 s 离开作用域的时候。当变量离开作用域，Rust 为我们调用一个特殊的函数。
                        这个函数叫做 drop，在这里 String 的作者可以放置释放内存的代码。
                        Rust 在结尾的 } 处自动调用 drop。
        

        变量与数据交互的方式（一）：移动
            Rust 中的多个变量可以采用一种独特的方式与同一数据交互。
            

        变量与数据交互的方式（二）：克隆
            如果我们 确实 需要深度复制 String 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 clone 的通用函数。

        
        只在栈上的数据：拷贝
            Rust 有一个叫做 Copy trait 的特殊注解，可以用在类似整型这样的存储在栈上的类型上
            如果一个类型拥有 Copy trait，一个旧的变量在将其赋值给其他变量后仍然可用。
            Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait。
            如果我们对其值离开作用域时需要特殊处理的类型使用 Copy 注解，将会出现一个编译时错误。
*/