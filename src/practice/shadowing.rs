fn ex1() {
    let z = 13;
    let x = {
        let y = 10;
        println!("y: {y}");
        z - y
    };
    println!("x: {x}");
}

fn ex2(){
    let a = 10;
    println!("이전: {a}");
    {
        let a = "hello";
        println!("내부 범위: {a}");

        let a = true;
        println!("내부 범위 섀도 처리됨: {a}");
    }

    println!("이후: {a}");
}

pub fn run() {
    println!("ex1");
    ex1();
    println!("ex2");
    ex2();
}