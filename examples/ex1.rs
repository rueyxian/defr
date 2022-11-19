use defr::defr;

fn main() {
    // ANSI escape code: https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters
    defr! {
        println!("world");
        println!("\x1b[0m"); // reset
    }
    println!("\x1b[1;41;93m"); // set
    println!("hello");
}
