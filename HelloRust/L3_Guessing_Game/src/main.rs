use std::io;

fn main() {
    /* 猜数字游戏 */
    println!("猜数游戏开始!");

    println!("猜一个数!");
    
    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess).expect("无法读取行");

    println!("你猜测的数是: {}", guess);
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

*/