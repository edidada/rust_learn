use std::{sync::mpsc, thread, time::Duration};

struct Queue {
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Self {
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) {
    // TODO: We want to send `tx` to both threads. But currently, it is moved
    // into the first thread. How could you solve this problem?

    // 克隆 tx 以创建第二个发送端
    let tx2 = tx.clone();

    // 先解构取出两半，避免闭包整体捕获 q 导致部分移动错误
    let Queue { first_half, second_half } = q;

    let handle1 = thread::spawn(move || {
        for val in first_half {
            println!("Sending {val:?}");
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    let handle2 = thread::spawn(move || {
        for val in second_half {
            println!("Sending {val:?}");
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    // 等待两个线程结束，确保资源及时清理、panic 能传播回调用方
    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn main() {
    // You can optionally experiment here.
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();

    send_tx(queue, tx);

    // 接收所有消息
    let mut received = Vec::new();
    for msg in rx {
        println!("收到: {}", msg);
        received.push(msg);

        if received.len() == 10 {
            break;
        }
    }

    received.sort();
    println!("收到的所有消息: {:?}", received);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threads3() {
        let (tx, rx) = mpsc::channel();
        let queue = Queue::new();

        send_tx(queue, tx);

        let mut received = Vec::with_capacity(10);
        for value in rx {
            received.push(value);
        }

        received.sort();
        assert_eq!(received, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
