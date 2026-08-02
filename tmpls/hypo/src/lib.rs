use std::convert::Infallible;
use tmpls::{BigTable, Teams};

use hypo::*;

#[derive(Debug, Default)]
pub struct Benchmark;

impl tmpls::Benchmark for Benchmark {
    type Output = String;
    type Error = Infallible;

    fn big_table(
        &mut self,
        output: &mut Self::Output,
        input: &BigTable,
    ) -> Result<(), Self::Error> {
        output.clear();
        table!(
            input
                .table
                .iter()
                .map(|row| tr!(row.iter().map(|col| td!(*col))))
        )
        .render(output);
        Ok(())
    }

    fn teams(&mut self, output: &mut Self::Output, input: &Teams) -> Result<(), Self::Error> {
        output.clear();
        html!(
            head!(title!(input.year)),
            body!(
                h1!("CLS ", input.year),
                ul!(input.teams.iter().enumerate().map(|(idx, team)| {
                    li!(
                        class = (idx == 0).then_some("champion"),
                        b!(&team.name as &str),
                        ": ",
                        team.score
                    )
                }))
            )
        )
        .render(output);
        Ok(())
    }
}
