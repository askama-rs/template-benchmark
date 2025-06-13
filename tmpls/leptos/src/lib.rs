use std::convert::Infallible;

use leptos::prelude::*;
use leptos::tachys::view::Position;
use tmpls::{BigTable, Teams};

#[derive(Debug, Default)]
pub struct Benchmark;

impl tmpls::Benchmark for Benchmark {
    type Output = String;
    type Error = Infallible;

    #[allow(deprecated)]
    fn big_table(
        &mut self,
        output: &mut Self::Output,
        input: &BigTable,
    ) -> Result<(), Self::Error> {
        let BigTable { table } = input;
        let view = view! {
            <table>
                { table.iter().map(|row| view! {
                    <tr>
                        { row.iter().map(|col| view! {
                            <td>{ *col }</td>
                        }).collect::<Vec<_>>() }
                    </tr>
                }).collect::<Vec<_>>() }
            </table>
        };
        view.to_html_with_buf(output, &mut Position::FirstChild, true, false, vec![]);
        Ok(())
    }

    fn teams(&mut self, output: &mut Self::Output, input: &Teams) -> Result<(), Self::Error> {
        let Teams { year, ref teams } = *input;
        let view = view! {
            <html>
                <head>
                    <title>{ year }</title>
                </head>
                <body>
                    <h1>"CSL "{ year }</h1>
                    <ul>
                        { teams.iter().enumerate().map(|(idx, team)| view! {
                            <li class=(idx == 0).then_some("champion")>
                                <b>{ team.name.as_str() }</b>
                                ": "
                                { team.score }
                            </li>
                        }).collect::<Vec<_>>() }
                    </ul>
                </body>
            </html>
        };
        view.to_html_with_buf(output, &mut Position::FirstChild, true, false, vec![]);
        Ok(())
    }
}
