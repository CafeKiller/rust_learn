fn main() {
    println!("Hello, world!");
    another_function();
    function_1(10);
    let x = function_2();

    let y = {
        let x = 20 + 90;
        x - 9
    };
    println!("main:  y={}, x={}", y, x); 
    
    let five = five(); 
    println!("five => {}", five);
}

fn another_function(){
    println!("Hello, another_function");
}

fn function_1(x: i32) {
    println!("x = {}", x)
}

fn function_2() -> i32 {
    println!("function_2 start");
    100
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

/* 
    知识点

    Rust 代码中的函数和变量名使用snake风格。在snake，所有字母都是小写并使用下划线分隔单词。
    Rust 不关心函数定义于何处，只要定义了就行。

    在定义函数时, 参数类型必须指定
    要求在函数定义中提供类型注解，意味着编译器不需要你在代码的其他地方注明类型来指出你的意图。

    包含语句和表达式的函数体
        函数体由一系列的语句和一个可选的结尾表达式构成。
        Rust 是一门基于表达式（expression-based）的语言

        语句（Statements）是执行一些操作但不返回值的指令。
        表达式（Expressions）计算并产生一个值。

        let y = 6;  是一个语句。
        5 + 6       是一个表达式, 产生的值是 11

        函数调用是一个表达式。
        宏调用是一个表达式。
        我们用来创建新作用域的大括号（代码块），{}，也是一个表达式
            let y = {
                let x = 3;
                x + 1
            }
        注意结尾没有分号的那一行 x+1，与你见过的大部分代码行不同。
        表达式的结尾没有分号。
        如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。

    
    
    具有返回值的函数
        函数可以向调用它的代码返回值。
        我们并不对返回值命名，但要在箭头（->）后声明它的类型。
        在 Rust 中，函数的返回值等同于函数体最后一个表达式的值。
        使用 return 关键字和指定值，可从函数中提前返回；但大部分函数隐式的返回最后的表达式。

        fn plus_one(x: i32) -> i32 {
            x + 1;
        }
        以上的代码编译时会报错
            主要的错误信息，“mismatched types”（类型不匹配）
            函数 plus_one 的定义说明它要返回一个 i32 类型的值，不过语句并不会返回值，使用空元组 () 表示不返回值。
            因为不返回值与函数定义相矛盾，从而出现一个错误。

        在输出中，Rust 提供了一条信息，可能有助于纠正这个错误：它建议删除分号，这会修复这个错误。
*/
