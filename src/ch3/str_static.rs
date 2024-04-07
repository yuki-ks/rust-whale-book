fn echo(s: &'static str) {
    println!("{}", s);
}

fn main() {
    echo("愚かな人でも黙っていると");
    echo("賢いとみられる");

    // let s = String::from("テスト");
    // echo(&s);
    // -> error
}
