//! Implementations of the [`Connection`] trait for various built-in types
// TODO: impl Connection for all `Read + Write` (blocked on specialization)

#[cfg(feature = "alloc")]
mod boxed;

#[cfg(feature = "std")]
mod tcpstream;

#[cfg(all(feature = "std", unix))]
mod unixstream;

use crate::conn::Connection;
use crate::conn::ConnectionExt;

impl<C:Connection<Error = E> + ?Sized, E> Connection for &mut C {
    type Error = E;

    async fn write(&mut self, byte: u8) -> Result<(), Self::Error> {
        (**self).write(byte).await
    }

    async fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        (**self).write_all(buf).await
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        (**self).flush().await
    }

    fn on_session_start(&mut self) -> Result<(), Self::Error> {
        (**self).on_session_start()
    }
}

impl<C:ConnectionExt<Error = E> + ?Sized, E> ConnectionExt for &mut C {
    async fn read(&mut self) -> Result<u8, Self::Error> {
        (**self).read().await
    }

    fn peek(&mut self) -> Result<Option<u8>, Self::Error> {
        (**self).peek()
    }
}
