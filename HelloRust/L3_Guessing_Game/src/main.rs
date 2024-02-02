use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    /* 猜数字游戏 */
    println!("猜数游戏开始!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("神秘数字是{}", secret_number);

    loop {
        println!("猜一个数!");
    
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess).expect("无法读取行");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue
        };

        println!("你猜测的数是: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small"),
            Ordering::Greater => println!("To big"),
            Ordering::Equal => {
                println!("You WIN!!!!");
                break;
            },
        }
    }
}

/* 
    知识点:
    use 关键字表示引用;
    std::io     io库默认是在标准库中的, std就是标准库.
    默然情况下rust会引入 prelude (预导入模块) 提前导入到每个程序的全局作用域中, 
    如果你需要使用模块刚好不在 prelude中, 就需要通过use来导入模块了.


    let 是变量定义关键字; 默然情况下所有的变量都是不可变的
    mut 关键字, 指定变量是可变值的;


    String::new()   返回一个字符串实例, 标准库提供, 内部使用UTF-8
    :: 表示调用的是关联函数, 关联函数不是通过具体的实例调用的 (有点类似于Java和C#中的静态方法)

    & 取地址符号,和C语言是类似的,表示一个引用;
    引用在rust中默认也是不可修改, 所以需要 &mut 

    expect() 会返回一个 io::Result , Result本质就是一个枚举, 其有两个值: Ok, Err
    返回Ok表示成功, 同时会返回值; 如果返回的是Err就表示失败了, 在Err会附带错误信息.


    // ===== ===== ===== ===== ===== ===== ===== ===== ===== //
    https://crates.io/  Rust carte的管理网站

    rand::Rng   是一个trait(有点类似于其他语言的接口), 内置了很多方法.
    
    rand::thread_rng() 会返回一个ThreadRag, 这是一个随机数生成器(该随机数生成器是在本地线程空间的)
    gen_range(1, 101)   获取一个随机数, 范围包括1, 不包括101

    std::cmp::Ordering 是个枚举类型, 该枚举有三个值, Less小于 Greater大于 Equal等于
    
    match 类似于一个匹配机, 会更具表达式中的值, 匹配一个arm(手臂), 

    guess.cmp() 一个比较方法, 会对比传入的值, 并返回一个Ordering


    let guess: u32 = guess.trim().parse()
    在rust里是允许对变量进行再声明的, 其通过隐藏(shadow)原变量的方式实现;一般用于实现变量的类型转换.
    trim() 方法, 清除字符串空白; parse() 将字符串转换为 数值(int)
                                                    

*/