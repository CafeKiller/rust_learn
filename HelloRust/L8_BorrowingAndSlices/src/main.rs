fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("s1 = {}, s1 len = {}", s1, len);

    let mut s2 = String::from("World, ");
    change_str(&mut s2);
    println!("s2 = {}", s2);


    let mut s = String::from("Coffee");
    let ss1 = &mut s;
    // let ss2 = &mut s; // 一个作用域内不允许有两个可变引用

    { 
        let ss2 = &mut s; // 不在一个作用域内了
    }

    // println!("{} and {}", ss1, ss2);


    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用
    let r3 = &mut s; // 没问题
    println!("{}", r3);


    // let reference_to_nothing = dangle(); 引用超出了作用域, 无法再使用



    // --------------------------------------------------------------
    // --------------------------------------------------------------


    let mut s = String::from("hello world");
    let word = first_word(&s);

    println!("word = {}", word); // 值是5
    s.clear(); // 此时会出现一个问题, 此处已经清除了s的值(也就是"")
    println!("word = {}", word); // 但在此处访问word时, 值依旧是 5


    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];



    let mut s = String::from("hello world");
    let word2 = first_word2(&s);

    println!("word2 = {}", word2); 
    // s.clear(); 通过切片处理后, 此时在此处使用就会报错了
    println!("word2 = {}", word2); 


    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

}

fn calculate_length(s: &String) -> usize { // s 是对 String 的引用
    // s.push_str("world"); 引用的值 默认是不允许修改, 此处代码运行会报错
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权

fn change_str(s: &mut String) {
    s.push_str("Hello"); // 
} 

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
//     // 因为 s 是在 dangle 函数内创建的，当 dangle 的代码执行完毕后，s 将被释放。不过我们尝试返回它的引用。
//     // 这意味着这个引用会指向一个无效的 String，这可不对！Rust 不会允许我们这么做。
// }

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

/* 
    知识点:
        引用与借用:
            &符号就是引用，它们允许你使用值但不获取其所有权。
            
            let s1 = String::from("hello");
            let len = calculate_length(&s1);
            &s1 语法让我们创建一个 指向 值 s1 的引用，但是并不拥有它。
            因为并不拥有这个值，当引用离开作用域时其指向的值也不会被丢弃。
            同理，函数签名使用&来表明参数s的类型是一个引用。
            变量 s 有效的作用域与函数参数的作用域一样，不过当引用离开作用域后并不丢弃它指向的数据，因为我们没有所有权。
            当函数使用引用而不是实际值作为参数，无需返回值来交还所有权，因为就不曾拥有所有权。

            我们将获取引用作为函数参数称为 借用（borrowing）。
            正如变量默认是不可变的，引用也一样。（默认）不允许修改引用的值。

        
        可变引用:
            首先，必须将 s 改为 mut。然后必须创建一个可变引用 &mut s 和接受一个可变引用 some_string: &mut String。
            不过可变引用有一个很大的限制：在特定作用域中的特定数据有且只有一个可变引用。

            这个限制允许可变性，不过是以一种受限制的方式允许。

            这个限制的好处是 Rust 可以在编译时就避免数据竞争。
            数据竞争（data race）类似于竞态条件，它可由这三个行为造成：
                - 两个或更多指针同时访问同一数据。
                - 至少有一个指针被用来写入数据。
                - 没有同步数据访问的机制。

            数据竞争会导致未定义行为，难以在运行时追踪，并且难以诊断和修复；
            Rust 避免了这种情况的发生，因为它甚至不会编译存在数据竞争的代码！

            类似的规则也存在于同时使用可变与不可变引用中。
            多个不可变引用是可以的，因为没有哪个只能读取数据的人有能力影响其他人读取到的数据。

            注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。

            不可变引用 r1 和 r2 的作用域在 println! 最后一次使用之后结束，这也是创建可变引用 r3 的地方。
            它们的作用域没有重叠，所以代码是可以编译的。


        悬垂引用（Dangling References）:
            在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个 悬垂指针（dangling pointer），所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。
            相比之下，在 Rust 中编译器确保引用永远也不会变成悬垂状态：
                当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。

        引用的规则: 
            在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
            引用必须总是有效的。


        Slice 类型:
            另一个没有所有权的数据类型是 slice。slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。


        字符串 slice:
            et s = String::from("hello world");
            let hello = &s[0..5];
            let world = &s[6..11];

            这类似于引用整个 String 不过带有额外的 [0..5] 部分。
            它不是对整个 String 的引用，而是对部分 String 的引用。
            
            可以使用一个由中括号中的 [starting_index..ending_index] 指定的 range 创建一个 slice，
            其中 starting_index 是 slice 的第一个位置，ending_index 则是 slice 最后一个位置的后一个值。

            在其内部，slice 的数据结构存储了 slice 的开始位置和长度，
            长度对应于 ending_index 减去 starting_index 的值。
            所以对于 let world = &s[6..11]; 的情况，
            world 将是一个包含指向 s 第 7 个字节（从 1 开始）的指针和长度值 5 的 slice。

            注意：
                字符串 slice range 的索引必须位于有效的 UTF-8 字符边界内，
                如果尝试从一个多字节字符的中间位置创建字符串 slice，则程序将会因错误而退出。
                出于介绍字符串 slice 的目的，本部分假设只使用 ASCII 字符集；
        
        
        字符串字面值就是 slice
            let s = "Hello, world!";
            这里 s 的类型是 &str：它是一个指向二进制程序特定位置的 slice。这也就是为什么字符串字面值是不可变的；&str 是一个不可变引用。

        
        字符串 slice 作为参数
        
        其他类型的 slice
            字符串 slice，正如你想象的那样，是针对字符串的。不过也有更通用的 slice 类型。

*/