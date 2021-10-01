use syn::Field;

use crate::relational::{filters::FilterType, helpers::field_name};

use super::{
    db::Database,
    filters::{FilterFieldType, Filters, IntFilters, StringFilters},
    sql::Sql,
};

#[derive(Clone)]
pub struct FieldDetails {
    pub field: Field,
    pub alias: Option<String>,
}

impl FieldDetails {
    pub fn field_name(&self) -> String {
        field_name(self.field.clone())
    }

    pub fn alias(&self) -> String {
        if self.alias.is_some() {
            self.alias.clone().unwrap()
        } else {
            self.field_name()
        }
    }

    pub fn r#as(mut self, alias: String) -> Self {
        self.alias = Some(alias);
        self
    }
}

impl IntFilters<i32> for (i32, FieldDetails) {
    fn gt(&self, other: i32) -> Filters {
        Filters {
            field: self.1.clone(),
            filter_type: FilterType::GreaterThan,
            ty: FilterFieldType::I32(other),
            not: false,
            and: Box::new(None),
        }
    }

    fn gte(&self, other: i32) -> Filters {
        Filters {
            field: self.1.clone(),
            filter_type: FilterType::GreaterThanOrEqual,
            ty: FilterFieldType::I32(other),
            not: false,
            and: Box::new(None),
        }
    }

    fn lt(&self, other: i32) -> Filters {
        Filters {
            field: self.1.clone(),
            filter_type: FilterType::LessThan,
            ty: FilterFieldType::I32(other),
            not: false,
            and: Box::new(None),
        }
    }

    fn lte(&self, other: i32) -> Filters {
        Filters {
            field: self.1.clone(),
            filter_type: FilterType::LessThanOrEqual,
            ty: FilterFieldType::I32(other),
            not: false,
            and: Box::new(None),
        }
    }

    fn eq(&self, other: i32) -> Filters {
        Filters {
            field: self.1.clone(),
            filter_type: FilterType::EqualTo,
            ty: FilterFieldType::I32(other),
            not: false,
            and: Box::new(None),
        }
    }
}

impl StringFilters<String> for (String, FieldDetails) {
    fn eq(&self, other: String) -> Filters {
        Filters {
            field: self.1.clone(),
            filter_type: FilterType::EqualTo,
            ty: FilterFieldType::String(other),
            not: false,
            and: Box::new(None),
        }
    }

    fn like(&self, other: String) -> Filters {
        Filters {
            field: self.1.clone(),
            filter_type: FilterType::Like,
            ty: FilterFieldType::String(other),
            not: false,
            and: Box::new(None),
        }
    }

    fn ilike(&self, other: String) -> Filters {
        Filters {
            field: self.1.clone(),
            filter_type: FilterType::ILike,
            ty: FilterFieldType::String(other),
            not: false,
            and: Box::new(None),
        }
    }
}

pub struct Distinct(pub FieldDetails);

impl Sql for FieldDetails {
    fn to_sql<DB>(self, db: &DB) -> String
    where
        DB: Database,
    {
        format!(
            "{} as {}{}{}",
            self.field_name(),
            db.backtick_open(),
            self.alias(),
            db.backtick_close()
        )
    }
}

impl Sql for Vec<FieldDetails> {
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

impl Sql for Distinct {
    fn to_sql<DB>(self, db: &DB) -> String
    where
        DB: Database,
    {
        format!("distinct {}", self.0.to_sql(db))
    }
}
