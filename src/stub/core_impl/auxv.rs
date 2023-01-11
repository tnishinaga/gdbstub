use super::prelude::*;
use crate::protocol::commands::ext::Auxv;

impl<T: Target, C: Connection> GdbStubImpl<T, C> {
    pub(crate) async fn handle_auxv(
        &mut self,
        res: &mut ResponseWriter<'_, C>,
        target: &mut T,
        command: Auxv<'_>,
    ) -> Result<HandlerStatus, Error<T::Error, C::Error>> {
        let ops = match target.support_auxv() {
            Some(ops) => ops,
            None => return Ok(HandlerStatus::Handled),
        };

        crate::__dead_code_marker!("auxv", "impl");

        let handler_status = match command {
            Auxv::qXferAuxvRead(cmd) => {
                let ret = ops
                    .get_auxv(cmd.offset, cmd.length, cmd.buf)
                    .handle_error()?;
                if ret == 0 {
                    res.write_str("l").await?;
                } else {
                    res.write_str("m").await?;
                    // TODO: add more specific error variant?
                    res.write_binary(cmd.buf.get(..ret).ok_or(Error::PacketBufferOverflow)?).await?;
                }
                HandlerStatus::Handled
            }
        };

        Ok(handler_status)
    }
}
