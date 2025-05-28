use dioxus::prelude::*;
use dioxus::ssr::Renderer;
use tmpls::{BigTable, Teams};

#[derive(Debug, Default)]
pub struct Benchmark;

// there is no default `<html>` element
mod dioxus_elements {
    pub mod elements {
        pub use dioxus::prelude::dioxus_elements::elements::*;

        pub mod html {
            pub const TAG_NAME: &str = "html";
            pub const NAME_SPACE: Option<&str> = None;
        }
    }

    pub use dioxus::prelude::dioxus_elements::*;
    pub use elements::html;
}

impl tmpls::Benchmark for Benchmark {
    type Output = String;
    type Error = std::fmt::Error;

    fn big_table(
        &mut self,
        output: &mut Self::Output,
        input: &BigTable,
    ) -> Result<(), Self::Error> {
        let BigTable { table } = input;
        let vnode = rsx! {
            table {
                for row in table {
                    tr {
                        for col in row {
                            td { "{col}" }
                        }
                    }
                }
            }
        };
        Renderer::new().render_element_to(output, vnode)
    }

    fn teams(&mut self, output: &mut Self::Output, input: &Teams) -> Result<(), Self::Error> {
        let Teams { year, teams } = input;
        let vnode = rsx! {
            html {
                head {
                    title { "{year}" }
                }
                body {
                    h1 { "CSL {year}" }
                    ul {
                        for (idx, team) in teams.iter().enumerate() {
                            li {
                                class: if idx == 0 { "champion" },
                                b { "{ team.name }" } ": { team.score }"
                            }
                        }
                    }
                }
            }
        };
        Renderer::new().render_element_to(output, vnode)
    }
}
