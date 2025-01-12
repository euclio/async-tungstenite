use async_std::net::{TcpListener, TcpStream};
use async_std::task;
use async_tungstenite::{accept_async, client_async};

#[async_std::test]
async fn handshakes() {
    let (tx, rx) = futures::channel::oneshot::channel();

    let f = async move {
        let listener = TcpListener::bind("127.0.0.1:12345").await.unwrap();
        tx.send(()).unwrap();
        while let Ok((connection, _)) = listener.accept().await {
            let stream = accept_async(connection).await;
            stream.expect("Failed to handshake with connection");
        }
    };

    task::spawn(f);

    rx.await.expect("Failed to wait for server to be ready");
    let tcp = TcpStream::connect("127.0.0.1:12345")
        .await
        .expect("Failed to connect");
    let url = url::Url::parse("ws://localhost:12345/").unwrap();
    let _stream = client_async(url, tcp)
        .await
        .expect("Client failed to connect");
}
