use futures::{pin_mut, TryStreamExt};
use serde::{de::DeserializeOwned, Serialize};
use tokio_postgres::types::ToSql;

use super::{
    db::{DBType, Database},
    field::FieldDetails,
    filters::Filters,
    sql::Sql,
    table::TableDetails,
};

#[derive(Default, Clone)]
pub struct Select {
    pub(crate) fields: Vec<FieldDetails>,
    pub(crate) from: Vec<TableDetails>,
    pub(crate) filter: Vec<Filters>,
}

impl Select {
    pub fn new(fields: Vec<FieldDetails>, from: Vec<TableDetails>) -> Self {
        Self {
            fields,
            from,
            filter: Vec::new(),
        }
    }

    pub fn add_field(mut self, field: FieldDetails) -> Select {
        self.fields.push(field);
        self
    }

    pub fn fields(mut self, fields: Vec<FieldDetails>) -> Select {
        self.fields = fields;
        self
    }

    pub fn and_from<T>(mut self, table: TableDetails) -> Select {
        self.from.push(table);
        self
    }

    pub fn filter(mut self, filters: Filters) -> Self {
        self.filter.push(filters);
        self
    }

    pub async fn fetch_all<T, DB>(self, db: &DB) -> Result<Vec<T>, tokio_postgres::Error>
    where
        T: Serialize + DeserializeOwned,
        DB: Database,
    {
        let client = db.client();
        let sql = self.clone().to_sql(db);

        let response = match client {
            DBType::Postgres(client) => {
                client
                    .query_raw(
                        &client.prepare(&sql).await?,
                        self.filter.iter().map(|p| p.ty.to_sql() as &dyn ToSql),
                    )
                    .await?
            }
        };

        let mut result: Vec<T> = vec![];
        pin_mut!(response);
        while let Some(row) = response.try_next().await? {
            result.push(serde_postgres::from_row(&row).unwrap());
        }

        Ok(result)
    }

    pub async fn fetch_one<T, DB>(self, db: &DB) -> Result<Option<T>, tokio_postgres::Error>
    where
        T: Serialize + DeserializeOwned,
        DB: Database,
    {
        let client = db.client();
        let sql = self.clone().to_sql(db);

        let response = match client {
            DBType::Postgres(client) => {
                client
                    .query_raw(
                        &client.prepare(&sql).await?,
                        self.filter.iter().map(|p| p.ty.to_sql() as &dyn ToSql),
                    )
                    .await?
            }
        };

        pin_mut!(response);
        if let Some(row) = response.try_next().await? {
            let result = serde_postgres::from_row(&row).unwrap();
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }
}

impl Sql for Select {
    fn to_sql<DB>(self, db: &DB) -> String
    where
        DB: Database,
    {
        let mut query = format!(
            "select {} from {}",
            self.fields.to_sql(db),
            self.from.to_sql(db),
        );

        if !self.filter.is_empty() {
            query.push_str(&format!(" where {}", self.filter.to_sql(db)));
        }

        query.push(';');
        println!("{}", query);
        query
    }
}
