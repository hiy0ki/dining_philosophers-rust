use std::thread;
use std::time::Duration;

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

    fn eat(&self) {
        println!("{} is eating", self.name);

        thread::sleep(Duration::from_millis(1000));
       
        println!("{} is done eating", self.name);
    }
}

fn main() {
    let philosophers = vec![
        // new 関数を用意しないと以下のように呼び出さないといけなくなる
        // let p1 = Philosopher { name: "Judith Butler".to_string() };
        Philosopher::new("Judith Butler"),
        Philosopher::new("Gilles Deleuze"),
        Philosopher::new("Karl Marx"),
        Philosopher::new("Emma Goldman"),
        Philosopher::new("Michel FouCault"),
    ];

    for p in &philosophers {
        p.eat();
    }

}
