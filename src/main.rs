struct Philosopher {
    name: String,
}

impl Philosopher {
    // newはRustにおいて特に意味はないが、構造体の新しいインスタンスを生成する関数としてよく使われる
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }
}

fn main() {
    // new 関数を用意しないと以下のように呼び出さないといけなくなる
    // let p1 = Philosopher { name: "Judith Butler".to_string() };
    let p1 = Philosopher::new("Judith Butler");
    let p2 = Philosopher::new("Gilles Deleuze");
    let p3 = Philosopher::new("Karl Marx");
    let p4 = Philosopher::new("Emma Goldman");
    let p5 = Philosopher::new("Michel FouCault");

    

}
