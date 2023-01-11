use super::prelude::*;
use crate::protocol::commands::ext::MonitorCmd;

use crate::protocol::ConsoleOutput;

impl<T: Target, C: Connection> GdbStubImpl<T, C> {
    pub(crate) async fn handle_monitor_cmd<'a>(
        &mut self,
        res: &mut ResponseWriter<'_, C>,
        target: &mut T,
        command: MonitorCmd<'a>,
    ) -> Result<HandlerStatus, Error<T::Error, C::Error>> {
        let ops = match target.support_monitor_cmd() {
            Some(ops) => ops,
            None => return Ok(HandlerStatus::Handled),
        };

        crate::__dead_code_marker!("monitor_cmd", "impl");

        let handler_status = match command {
            MonitorCmd::qRcmd(cmd) => {
                let use_rle = ops.use_rle();

                let mut err: Result<_, Error<T::Error, C::Error>> = Ok(());
                let mut callback = |msg: &[u8]| {
                    // TODO: replace this with a try block (once stabilized)
                    let e = (async || {
                        let mut res = ResponseWriter::new(res.as_conn(), use_rle);
                        res.write_str("O").await?;
                        res.write_hex_buf(msg).await?;
                        res.flush().await?;
                        Ok(())
                    })();

                    if let Err(e) = e {
                        err = Err(e)
                    }
                };

                ops.handle_monitor_cmd(cmd.hex_cmd, ConsoleOutput::new(&mut callback))
                    .map_err(Error::TargetError)?;
                err?;

                HandlerStatus::NeedsOk
            }
        };

        Ok(handler_status)
    }
}
