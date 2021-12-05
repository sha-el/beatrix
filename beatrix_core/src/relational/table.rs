use super::{db::Database, field::FieldDetails, helpers::format_name, select::Select, sql::Sql};

#[derive(Debug, Clone)]
pub struct TableDetails {
    pub name: String,
    pub alias: Option<String>,
}

impl TableDetails {
    pub fn table_name(&self) -> String {
        format_name(&self.name)
    }

    pub fn alias(&self) -> String {
        if self.alias.is_some() {
            self.alias.clone().unwrap()
        } else {
            self.table_name()
        }
    }

    pub fn r#as(mut self, alias: String) -> Self {
        self.alias = Some(alias);
        self
    }
}

pub trait Table {
    fn table_details() -> TableDetails;
    fn fields() -> Vec<FieldDetails>;
    fn select() -> Select;
}

impl Sql for TableDetails {
    fn to_sql<DB>(self, db: &DB) -> String
    where
        DB: Database,
    {
        format!(
            "{}{}{} as {}{}{}",
            db.backtick_open(),
            self.table_name(),
            db.backtick_close(),
            db.backtick_open(),
            self.alias(),
            db.backtick_close()
        )
    }
}

impl Sql for Vec<TableDetails> {
    fn to_sql<DB>(self, db: &DB) -> String
    where
        DB: Database,
    {
        self.into_iter()
            .map(|f| f.to_sql(db))
            .collect::<Vec<String>>()
            .join(", ")
    }
}
