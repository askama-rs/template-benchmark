use std::convert::Infallible;

use hypertext::{GlobalAttributes, Renderable, html_elements, maud};
use tmpls::{BigTable, Teams};

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
        *output = maud! {
            table {
                @for row in &input.table {
                    tr {
                        @for col in row {
                            td { (*col) }
                        }
                    }
                }
            }
        }
        .render()
        .0;
        Ok(())
    }

    fn teams(&mut self, output: &mut Self::Output, input: &Teams) -> Result<(), Self::Error> {
        *output = maud! {
            html {
                head {
                    title { (input.year) }
                }
                body {
                    h1 { "CLS " (input.year) }
                    ul {
                        @for (idx, team) in input.teams.iter().enumerate() {
                            li.champion[idx == 0] {
                                b { (team.name.as_str()) } ": " (team.score)
                            }
                        }
                    }
                }
            }
        }
        .render()
        .0;
        Ok(())
    }
}
