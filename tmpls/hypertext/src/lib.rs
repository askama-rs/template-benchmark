use std::convert::Infallible;
use std::mem::take;

use hypertext::Buffer;
use hypertext::prelude::*;
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
        let mut buffer = Buffer::dangerously_from_string(take(output));
        maud! {
            table {
                @for row in &input.table {
                    tr {
                        @for col in row {
                            td { (col) }
                        }
                    }
                }
            }
        }
        .render_to(&mut buffer);
        *output = buffer.rendered().into_inner();
        Ok(())
    }

    fn teams(&mut self, output: &mut Self::Output, input: &Teams) -> Result<(), Self::Error> {
        let mut buffer = Buffer::dangerously_from_string(take(output));
        maud! {
            html {
                head {
                    title { (input.year) }
                }
                body {
                    h1 { "CLS " (input.year) }
                    ul {
                        @for (idx, team) in input.teams.iter().enumerate() {
                            li.champion[idx == 0] {
                                b { (team.name) } ": " (team.score)
                            }
                        }
                    }
                }
            }
        }
        .render_to(&mut buffer);
        *output = buffer.rendered().into_inner();
        Ok(())
    }
}
