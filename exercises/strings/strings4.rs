// strings4.rs

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
// No hints this time!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue"); // &str
    string("red".to_string()); // string
    string(String::from("hi")); // string
    string("rust is fun!".to_owned()); // owned, == "XXX".clone()
    string("nice weather string".into()); // 值到值的转换，会消耗原有值
    string_slice("nice weather slice".into()); // 值到值的转换，会消耗原有值
    // let data:String = "initial contents".into();
    // let data2:&str = "initial contents".into();
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
