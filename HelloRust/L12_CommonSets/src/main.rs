// Rust 学习第十二章 常见的集合



// 向量 Vector 相关
fn learn_vector() {
    
    // 创建一个空的 Vector 存储 i32 类型的数据
    let _vec: Vec<i32> = Vec::new();
    
    // 此处通过 rust 定义的 vec! 宏来初始化 Vector, 可以直接推断出 Vector 的存储类型
    let _vec = vec![1, 2, 233, 666];
    println!("[learn_vector] vec = {:?}", _vec);

    // 更新 Vector
    let mut _vec = Vec::new();
    _vec.push(233);
    _vec.push(322);
    _vec.push(666);
    _vec.push(999);
    println!("[learn_vector] vec = {:?}", _vec);

    // 关于 Vector 的销毁
    // 需要的注意的是: Vector 在离开其作用域之后就会被丢弃


    // 获取 Vector 的值
    // 1、通过索引获取
    let num1: &i32 = &_vec[1]; 
    println!("[learn_vector] num1 = {}", num1);

    // 2、通过 get 方法
    match _vec.get(2) {
        Some(third) => println!("[learn_vector] vec.get(2) = {}", third),
        None => println!("[learn_vector] is not element"),
    };
    // 需要注意的是: 使用索引的方式获取元素, 其下标是从0开始的. 且返回的是一个引用
    // 而使用 get 方式获取的则是一个 option<&T>
    // 还有就是当访问超出范围的下标时, 第一种方式会 panic, 而第二种方式则是会执行 None 分支


    // 注意: 当程序获取到一个有效的引用后, 借用检查器就来确保 Vector 内容的这个引用和任何其他引用保持有效 
    let mut _vec = vec![1, 2, 3];
    let _first = &_vec[0]; 
    _vec.push(500); // 此处就会出现问题, 因为上面的代码里我们已经获取其引用了
    println!("[learn_vector] vec = {:?}", _vec);


    // 遍历 Vector 中的所有元素
    let _vec = vec![22, 33, 444, 888, 1024];
    for item in &_vec {
        println!("[learn_vector] {}, ", item);
    }
    // 可变的引用
    let mut _vec2 = vec![222, 333, 666, 999];
    for item in &mut _vec2 {
        *item += 1;
        println!("[learn_vector] {}, ", item);
    }

    // 使用枚举来存储更多的类型
    #[derive(Debug)]
    enum MoreType {
        Int(i32),
        Float(f64),
        Text(String),        
    }
    let row = vec![
        MoreType::Int(3),
        MoreType::Float(3.1415926),
        MoreType::Text(String::from("I is cafe")), 
    ];
    println!("[learn_vector] row = {:?}", row);


}// <- vec 在此处就会被丢弃



fn main() {
    println!("Hello, rust!");

    learn_vector()
}

