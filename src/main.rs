use std::net::TcpListner;

fn main() {
    let listner = TcpListner::bind("127.0.0.1:7878").unwrap();

    for stream in listner.incoming() {
        let stream = stream.unwrap();

        println!("接続が確率しました");
    }
}
