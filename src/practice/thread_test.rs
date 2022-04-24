use std::thread;
use std::sync::mpsc;
use std::time::{Instant,Duration};

pub fn thread_test(){
    let start = Instant::now();

    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move ||{
        let vals = vec![
            String::from("a 자식 스레드가"),
            String::from("a 안녕하세요"),
            String::from("a 라고"),
            String::from("a 인사합니다."),
        ];

        for val in vals {
            tx1.send(val).expect("문자열 데이터가 아닙니다.");
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move ||{
        let vals = vec![
            String::from("b 그리고"),
            String::from("b 더 많은"),
            String::from("b 메시지를"),
            String::from("b 보냅니다."),
        ];

        for val in vals {
            tx.send(val).expect("문자열 데이터가 아닙니다.");
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("수신 : {}", received);
    }

    println!("\nexec time : {:?}", start.elapsed());
}