use std::net::UdpSocket;
use std::time::Duration;
use std::thread;


fn main() -> std::io::Result<()> {
    // UDP 소켓 생성 및 로컬 주소에 바인딩
    let socket = UdpSocket::bind("0.0.0.0:8787")?;

    // 서버의 주소 설정
    let server_address = "127.0.0.1:7878";

    loop {
        // 서버에 메시지 전송
        socket.send_to(b"Hello, server!", server_address)?;

        println!("send message.");

        // 1초 동안 대기
        thread::sleep(Duration::from_secs(1));
    }
}
