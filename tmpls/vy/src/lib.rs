use std::convert::Infallible;

use tmpls::{BigTable, Output, Teams};
use vy::{Buffer, IntoHtml, b, body, h1, head, html, li, table, td, title, tr, ul};

#[derive(Debug, Default)]
pub struct Benchmark;

impl tmpls::Benchmark for Benchmark {
    type Output = WrappedBuffer;
    type Error = Infallible;

    fn big_table(
        &mut self,
        output: &mut Self::Output,
        input: &BigTable,
    ) -> Result<(), Self::Error> {
        table!(
            input
                .table
                .iter()
                .map(|row| tr!(row.iter().map(|col| td!(*col))))
        )
        .escape_and_write(&mut output.0);
        Ok(())
    }

    fn teams(&mut self, output: &mut Self::Output, input: &Teams) -> Result<(), Self::Error> {
        html!(
            head!(title!(input.year)),
            body!(
                h1!("CLS ", input.year),
                ul!(input.teams.iter().enumerate().map(|(idx, team)| {
                    li!(
                        class = (idx == 0).then_some("champion"),
                        b!(team.name.as_str(), ": ", team.score)
                    )
                }))
            )
        )
        .escape_and_write(&mut output.0);
        Ok(())
    }
}

#[derive(Default)]
pub struct WrappedBuffer(Buffer);

impl Output for WrappedBuffer {
    fn clear(&mut self) {
        unsafe { self.0._set_len(0) };
    }

    fn as_bytes(&self) -> &[u8] {
        self.0.as_str().as_bytes()
    }
}
