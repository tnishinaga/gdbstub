use super::prelude::*;
use crate::protocol::commands::ext::SectionOffsets;

impl<T: Target, C: Connection> GdbStubImpl<T, C> {
    pub(crate) async fn handle_section_offsets(
        &mut self,
        res: &mut ResponseWriter<'_, C>,
        target: &mut T,
        command: SectionOffsets,
    ) -> Result<HandlerStatus, Error<T::Error, C::Error>> {
        let ops = match target.support_section_offsets() {
            Some(ops) => ops,
            None => return Ok(HandlerStatus::Handled),
        };

        crate::__dead_code_marker!("section_offsets", "impl");

        let handler_status = match command {
            SectionOffsets::qOffsets(_cmd) => {
                use crate::target::ext::section_offsets::Offsets;

                match ops.get_section_offsets().map_err(Error::TargetError)? {
                    Offsets::Sections { text, data, bss } => {
                        res.write_str("Text=").await?;
                        res.write_num(text).await?;

                        res.write_str(";Data=").await?;
                        res.write_num(data).await?;

                        // "Note: while a Bss offset may be included in the response,
                        // GDB ignores this and instead applies the Data offset to the Bss section."
                        //
                        // While this would suggest that it's OK to omit `Bss=` entirely, recent
                        // versions of GDB seem to require that `Bss=` is present.
                        //
                        // See https://github.com/bminor/binutils-gdb/blob/master/gdb/remote.c#L4149-L4159
                        let bss = bss.unwrap_or(data);
                        res.write_str(";Bss=").await?;
                        res.write_num(bss).await?;
                    }
                    Offsets::Segments { text_seg, data_seg } => {
                        res.write_str("TextSeg=").await?;
                        res.write_num(text_seg).await?;

                        if let Some(data) = data_seg {
                            res.write_str(";DataSeg=").await?;
                            res.write_num(data).await?;
                        }
                    }
                }
                HandlerStatus::Handled
            }
        };

        Ok(handler_status)
    }
}
