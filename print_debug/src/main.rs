// struct UnPrintable(i32);

// #[derive(Debug)] を書くと、fmt::Debugトレイトを自動で実装してくれるため、
// 型の変数を{:?}を用いてデバッグ出力できるようになる
// #[derive(Debug)]
// struct DebugPrintable(i32);

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {
    println!("Hello, world!");

    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    println!("Now {:?} will print!", Structure(3));
    println!("Now {:#?} will print!", Structure(3));

    println!("Now {:?} will print!", Deep(Structure(7)));
    println!("Now {:#?} will print!", Deep(Structure(7)));
}
