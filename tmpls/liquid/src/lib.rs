use liquid::model::{ScalarCow, Value};
use liquid::{Error, Template, object};
use tmpls::{BigTable, Team, Teams};

pub struct Benchmark {
    big_table: Template,
    teams: Template,
}

impl Default for Benchmark {
    fn default() -> Self {
        let big_table = liquid::ParserBuilder::with_stdlib()
            .build()
            .unwrap()
            .parse(include_str!("../templates/big-table.html"))
            .unwrap();
        let teams = liquid::ParserBuilder::with_stdlib()
            .build()
            .unwrap()
            .parse(include_str!("../templates/teams.html"))
            .unwrap();
        Self { big_table, teams }
    }
}

impl tmpls::Benchmark for Benchmark {
    type Output = Vec<u8>;
    type Error = Error;

    fn big_table(
        &mut self,
        output: &mut Self::Output,
        input: &BigTable,
    ) -> Result<(), Self::Error> {
        let BigTable { table } = input;

        let table = Value::Array(
            table
                .iter()
                .map(|row| {
                    Value::Array(
                        row.iter()
                            .map(|&col| Value::Scalar(ScalarCow::new(col as i64)))
                            .collect(),
                    )
                })
                .collect(),
        );
        let globals = object!({
            "table": table,
        });

        self.big_table.render_to(output, &globals)
    }

    fn teams(&mut self, output: &mut Self::Output, input: &Teams) -> Result<(), Self::Error> {
        let Teams { year, ref teams } = *input;

        let teams = Value::Array(
            teams
                .iter()
                .map(|&Team { ref name, score }| {
                    Value::Object(object!({
                        "name": name.as_str(),
                        "score": score,
                    }))
                })
                .collect(),
        );
        let globals = object!({
            "year": year,
            "teams": teams,
        });

        self.teams.render_to(output, &globals)
    }
}
