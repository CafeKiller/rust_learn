fn main() {
    println!("Hello, Cargo!");
}

/* 
    知识点:
    cargo new [projectName]     创建cargo项目
    
    cargo build                 构建cargo项目, 第一次运行时会生成cargo.lock文件;
                                cargo.lock文件负责追踪项目依赖的精确版本

    cargo run                   构建并运行cargo项目

    cargo check                 检查代码, 该命令的运行速度比build快.

    cargo build --release       同样是构建项目, 但主要用在即将发布时的程序上; 
                                编译时会进行优化, 但编译时间会较长.
*/