//! This quick-and-dirty implementation is error-ridden and unsafe!
//! DO NOT USE IN PRODUCTION!

mod lifetimeless;

use std::convert::Infallible;

use pollster::block_on;
use tmpls::{BigTable, Teams};
use yew::{Html, LocalServerRenderer, function_component, html};
use yew_autoprops::autoprops;

use crate::lifetimeless::Lifetimeless;

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
        lifetimeless!(input: &BigTable);
        let app = LocalServerRenderer::<BigAppleApp>::with_props(BigAppleAppProps { input });
        block_on(app.render_to_string(output));
        Ok(())
    }

    fn teams(&mut self, output: &mut Self::Output, input: &Teams) -> Result<(), Self::Error> {
        lifetimeless!(input: &Teams);
        let app = LocalServerRenderer::<TeamsApp>::with_props(TeamsAppProps { input });
        block_on(app.render_to_string(output));
        Ok(())
    }
}

#[autoprops]
#[function_component]
fn BigAppleApp(input: &Lifetimeless<BigTable>) -> Html {
    let BigTable { table } = &**input;
    html! {
        <table>
            { for table.iter().map(|row| html! {
                <tr>
                    { for row.iter().map(|col| html! {
                        <td>{ col }</td>
                    }) }
                </tr>
            }) }
        </table>
    }
}

#[autoprops]
#[function_component]
fn TeamsApp(input: &Lifetimeless<Teams>) -> Html {
    let Teams { year, teams } = &**input;
    html! {
        <html>
            <head>
                <title>{ year }</title>
            </head>
            <body>
                <h1>{"CSL "}{ year }</h1>
                <ul>
                    { for teams.iter().enumerate().map(|(idx, team)| html! {
                        <li class={ (idx == 0).then_some("champion") }>
                            <b>{ team.name.as_str() }</b>
                            { ": " }
                            { team.score }
                        </li>
                    }) }
                </ul>
            </body>
        </html>
    }
}
