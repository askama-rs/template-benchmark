use std::fmt;
use std::mem::transmute;
use std::ops::Deref;

use liquid::model::{
    DisplayCow, KString, KStringCow, ObjectRender, ObjectSource, ObjectView, State, Value,
    ValueView,
};
use liquid::{Error, Object, Template, object};
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

        let globals = object!({
            "table": table,
        });
        self.big_table.render_to(output, &globals)
    }

    fn teams(&mut self, output: &mut Self::Output, input: &Teams) -> Result<(), Self::Error> {
        let Teams { year, ref teams } = *input;

        // SAFETY: `LiquidTeam` is transparent over `Team`
        let teams = unsafe { transmute::<&Vec<Team>, &Vec<LiquidTeam>>(teams) };

        let globals = object!({
            "year": year,
            "teams": teams,
        });

        self.teams.render_to(output, &globals)
    }
}

#[repr(transparent)]
struct LiquidTeam(Team);

impl Deref for LiquidTeam {
    type Target = Team;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl serde::Serialize for LiquidTeam {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl fmt::Debug for LiquidTeam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl ObjectView for LiquidTeam {
    fn as_value(&self) -> &dyn ValueView {
        self
    }

    fn size(&self) -> i64 {
        2
    }

    fn keys<'a>(&'a self) -> Box<dyn Iterator<Item = KStringCow<'a>> + 'a> {
        const KEYS: [KStringCow; 2] = [
            KStringCow::from_static("name"),
            KStringCow::from_static("score"),
        ];

        Box::new(KEYS.into_iter())
    }

    fn values<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn ValueView> + 'a> {
        let values: [&dyn ValueView; 2] = [&self.name, &self.score];
        Box::new(values.into_iter())
    }

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (KStringCow<'a>, &'a dyn ValueView)> + 'a> {
        let values: [(KStringCow, &dyn ValueView); 2] = [
            (KStringCow::from_static("name"), &self.name),
            (KStringCow::from_static("score"), &self.score),
        ];
        Box::new(values.into_iter())
    }

    fn contains_key(&self, index: &str) -> bool {
        matches!(index, "name" | "score")
    }

    fn get<'a>(&'a self, index: &str) -> Option<&'a dyn ValueView> {
        match index {
            "name" => Some(&self.name),
            "score" => Some(&self.score),
            _ => None,
        }
    }
}

impl ValueView for LiquidTeam {
    fn as_debug(&self) -> &dyn ::std::fmt::Debug {
        self
    }

    fn render(&self) -> DisplayCow<'_> {
        DisplayCow::Owned(Box::new(ObjectRender::new(self)))
    }

    fn source(&self) -> DisplayCow<'_> {
        DisplayCow::Owned(Box::new(ObjectSource::new(self)))
    }

    fn type_name(&self) -> &'static str {
        "object"
    }

    fn query_state(&self, state: State) -> bool {
        match state {
            State::Truthy => true,
            State::DefaultValue | State::Empty | State::Blank => false,
        }
    }

    fn to_kstr(&self) -> KStringCow<'_> {
        let s = ObjectRender::new(self).to_string();
        KStringCow::from_string(s)
    }

    fn to_value(&self) -> Value {
        let mut object = Object::new();
        object.insert(
            KString::from_static("name"),
            ValueView::to_value(&self.name),
        );
        object.insert(
            KString::from_static("score"),
            ValueView::to_value(&self.score),
        );
        Value::Object(object)
    }

    fn as_object(&self) -> Option<&dyn ObjectView> {
        Some(self)
    }
}
