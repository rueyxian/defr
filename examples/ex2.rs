use defr::defr;

fn main() {
    let path = std::path::Path::new("./a-weird-file-that-should-not-exists-am-i-right-right-right");

    let _ = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap();
    println!("file created");

    defr! {
        std::fs::remove_file(path).unwrap();
        println!("file removed");
    }

    let _ = std::process::Command::new("open")
        .arg("./")
        .spawn()
        .unwrap();

    std::thread::sleep(std::time::Duration::from_millis(3000));

    panic!("panic attack!");
}
