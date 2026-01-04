use std::process::Command;

fn main() {
    println!("🔧 X-Task: 扩展构建后执行,可以接受cargo build后传递的部分参数");
    let mut args = std::env::args()
        .skip(1)
        .filter(|arg| arg != "build")
        .collect::<Vec<String>>();

    args.insert(0, "build".to_string());
    args.push("-p".to_string());
    args.push("testing".to_string());
    println!("🔧 X-Task: 执行参数: {:?}", args);

    let status = Command::new("cargo")
        .args(&args)
        .status()
        .expect("Failed to build wasm target");
    assert!(status.success());

    
    println!("🔧 X-Task: 构建完成,开始bundle执行");

    let is_release = args.iter().any(|arg| arg == "--release");
    let target = args
        .iter()
        .find(|arg| arg.starts_with("--target"))
        .map(|arg| arg.split("=").nth(1).unwrap().to_string());

    // 找寻此次构建的二进制目录,根据target和is_release判断
    let build_dir = target
        .map(|target| {
            format!(
                "target/{}/{}",
                target,
                if is_release { "release" } else { "debug" }
            )
        })
        .unwrap_or_else(|| format!("target/{}", if is_release { "release" } else { "debug" }));
    println!("🔧 X-Task: 原始构建目录: {}", build_dir);
}
