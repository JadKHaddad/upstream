use std::{
    pin::Pin,
    task::{Context, Poll},
};

use tokio::{
    io::{AsyncRead, AsyncWrite, ReadBuf},
    net::TcpStream,
};
use tokio_rustls::server::TlsStream;

/// XXX: We do not want to box the tls stream but clippy does not like it unboxed
pub enum ServerTcpStream {
    Plain(TcpStream),
    Tls(Box<TlsStream<TcpStream>>),
}

impl ServerTcpStream {
    pub fn plain(stream: TcpStream) -> Self {
        Self::Plain(stream)
    }

    pub fn tls(stream: TlsStream<TcpStream>) -> Self {
        Self::Tls(Box::new(stream))
    }
}

impl AsyncRead for ServerTcpStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            ServerTcpStream::Plain(s) => Pin::new(s).poll_read(cx, buf),
            ServerTcpStream::Tls(s) => Pin::new(s).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for ServerTcpStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        match self.get_mut() {
            ServerTcpStream::Plain(s) => Pin::new(s).poll_write(cx, buf),
            ServerTcpStream::Tls(s) => Pin::new(s).poll_write(cx, buf),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        match self.get_mut() {
            ServerTcpStream::Plain(s) => Pin::new(s).poll_flush(cx),
            ServerTcpStream::Tls(s) => Pin::new(s).poll_flush(cx),
        }
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        match self.get_mut() {
            ServerTcpStream::Plain(s) => Pin::new(s).poll_shutdown(cx),
            ServerTcpStream::Tls(s) => Pin::new(s).poll_shutdown(cx),
        }
    }
}
