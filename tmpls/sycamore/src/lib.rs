//! This quick-and-dirty implementation is error-ridden and unsafe!
//! DO NOT USE IN PRODUCTION!

use std::convert::Infallible;

use sycamore::prelude::*;
use sycamore::render_to_string_in_scope;
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
        // SAFETY: this is NOT safe, don't copy this code!
        let input = unsafe { std::mem::transmute::<&BigTable, &'static BigTable>(input) };
        let BigTable { table } = input;

        *output = create_root(|| ()).run_in(|| {
            render_to_string_in_scope(|| {
                view! {
                    table {
                        ({ table.iter().map(|row| view! {
                            tr {
                                ({ row.iter().map(|&col| view! {
                                    td { (col) }
                                }).collect::<Vec<View>>() })
                            }
                        }).collect::<Vec<View>>() })
                    }
                }
            })
        });
        Ok(())
    }

    fn teams(&mut self, output: &mut Self::Output, input: &Teams) -> Result<(), Self::Error> {
        // SAFETY: this is NOT safe, don't copy this code!
        let input = unsafe { std::mem::transmute::<&Teams, &'static Teams>(input) };
        let Teams { year, teams } = input;

        *output = create_root(|| ()).run_in(|| {
            render_to_string_in_scope(|| {
                view! {
                    html {
                        head {
                            title { (*year) }
                        }
                        body {
                            h1 {
                                "CSL " (*year)
                            }
                            ul {
                                ({ teams.iter().enumerate().map(|(idx, team)| view! {
                                    li(class = (idx == 0).then_some("active")) {
                                        b { (team.name.as_str()) }
                                        ": "
                                        (team.score)
                                    }
                                }).collect::<Vec<View>>() })
                            }
                        }
                    }
                }
            })
        });
        Ok(())
    }
}
