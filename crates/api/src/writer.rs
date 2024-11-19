use std::io;

// use bytes::BytesMut;
use tokio::{
    io::{AsyncWriteExt, BufWriter},
    net::tcp::OwnedWriteHalf,
};

use crate::frame::Frame;

/// Send and receive `IBFrame` values from a remote peer.
///
/// When implementing networking protocols, a message on that protocol is
/// often composed of several smaller messages known as frames. The purpose of
/// `Connection` is to read and write frames on the underlying `TcpStream`.
///
/// To read frames, the `Connection` uses an internal buffer, which is filled
/// up until there are enough bytes to create a full frame. Once this happens,
/// the `Connection` creates the frame and returns it to the caller.
///
/// When sending frames, the frame is first encoded into the write buffer.
/// The contents of the write buffer are then written to the socket.
#[derive(Debug)]
pub struct Writer {
    // The `TcpStream`. It is decorated with a `BufWriter`, which provides write
    // level buffering. The `BufWriter` implementation provided by Tokio is
    // sufficient for our needs.
    stream: BufWriter<OwnedWriteHalf>,
    // The buffer for reading frames. Unfortunately, Tokio's `BufReader`
    // currently requires you to empty its buffer before you can ask it to
    // retrieve more data from the underlying stream, so we have to manually
    // implement buffering. This should be fixed in Tokio v0.3.
    // buffer: BytesMut,

    //`true` if no message from server has been received yet, `false` otherwise.
    // never_received: bool,
}

impl Writer {
    /// Create a new `Connection`, backed by `socket`. Read and write buffers
    /// are initialized.
    pub fn new(socket: OwnedWriteHalf) -> Writer {
        Writer {
            stream: BufWriter::new(socket),
            // Default to a 4KB read buffer. For the use case of mini redis,
            // this is fine. However, real applications will want to tune this
            // value to their specific use case. There is a high likelihood that
            // a larger read buffer will work better.
            //       buffer:         BytesMut::with_capacity(32 * 1024),
            // never_received: true,
        }
    }

    /// Write a single `IBFrame` value to the underlying stream.
    ///
    /// The `IBFrame` value is written to the socket using the various `write_*`
    /// functions provided by `AsyncWrite`. Calling these functions directly on
    /// a `TcpStream` is **not** advised, as this will result in a large number
    /// of syscalls. However, it is fine to call these functions on a
    /// *buffered* write stream. The data will be written to the buffer.
    /// Once the buffer is full, it is flushed to the underlying socket.
    pub async fn write_frame(&mut self, frame: &Frame) -> io::Result<()> {
        // Arrays are encoded by encoding each entry. All other frame types are
        // considered literals. For now, mini-redis is not able to encode
        // recursive frame structures. See below for more details.
        tracing::trace!("writing frame: {:?}", frame);
        match frame {
            Frame::Array(val) => {
                // Iterate and encode each entry in the array.
                for entry in &**val {
                    self.write_value(entry).await?;
                }
            },
            // The frame type is a literal. Encode the value directly.
            _ => self.write_value(frame).await?,
        }

        // Ensure the encoded frame is written to the socket. The calls above
        // are to the buffered stream and writes. Calling `flush` writes the
        // remaining contents of the buffer to the socket.
        self.stream.flush().await
    }

    /// Write a single `IBFrame` value to the underlying stream.
    ///
    /// The `IBFrame` value is written to the socket using the various `write_*`
    /// functions provided by `AsyncWrite`. Calling these functions directly on
    /// a `TcpStream` is **not** advised, as this will result in a large number
    /// of syscalls. However, it is fine to call these functions on a
    /// *buffered* write stream. The data will be written to the buffer.
    /// Once the buffer is full, it is flushed to the underlying socket.
    pub async fn write_raw(&mut self, msg: &[u8]) -> io::Result<()> {
        self.stream.write_all(msg).await?;
        self.stream.flush().await
    }

    /// Write a frame literal to the stream
    async fn write_value(&mut self, frame: &Frame) -> io::Result<()> {
        match frame {
            Frame::Bulk(val) => {
                let _len = val.len();

                self.stream.write_all(val).await?;
            },
            // Encoding an `Array` from within a value cannot be done using a
            // recursive strategy. In general, async fns do not support
            // recursion. Mini-redis has not needed to encode nested arrays yet,
            // so for now it is skipped.
            Frame::Array(_val) => unreachable!(),
        }

        Ok(())
    }
}
