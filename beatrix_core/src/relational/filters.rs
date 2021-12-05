use std::ops::Not;

use tokio_postgres::types::ToSql;

use super::{db::Database, field::FieldDetails, sql::Sql};

#[derive(Clone)]
pub enum FilterType {
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    EqualTo,
    In,
    Between,
    Like,
    ILike,
}

impl Sql for FilterType {
    fn to_sql<DB>(self, _: &DB) -> String
    where
        DB: Database,
    {
        match self {
            FilterType::GreaterThan => ">".to_string(),
            FilterType::GreaterThanOrEqual => ">=".to_string(),
            FilterType::LessThan => "<".to_string(),
            FilterType::LessThanOrEqual => "<=".to_string(),
            FilterType::EqualTo => "=".to_string(),
            FilterType::In => "in".to_string(),
            FilterType::Between => "between".to_string(),
            FilterType::Like => "like".to_string(),
            FilterType::ILike => "ilike".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum FilterFieldType {
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U32(u32),
    F32(f32),
    F64(f64),
    String(String),
}

#[derive(Clone)]
pub struct Filters {
    pub field: FieldDetails,
    pub filter_type: FilterType,
    pub ty: FilterFieldType,
    pub not: bool,
    pub and: Box<Option<Filters>>,
}

impl Not for Filters {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        self.not = !self.not;
        self
    }
}

impl FilterFieldType {
    pub fn to_sql(&self) -> &dyn ToSql {
        match self {
            FilterFieldType::Bool(v) => v as &dyn ToSql,
            FilterFieldType::I8(v) => v as &dyn ToSql,
            FilterFieldType::I16(v) => v as &dyn ToSql,
            FilterFieldType::I32(v) => v as &dyn ToSql,
            FilterFieldType::I64(v) => v as &dyn ToSql,
            FilterFieldType::U32(v) => v as &dyn ToSql,
            FilterFieldType::F32(v) => v as &dyn ToSql,
            FilterFieldType::F64(v) => v as &dyn ToSql,
            FilterFieldType::String(v) => v as &dyn ToSql,
        }
    }
}

impl Sql for Vec<Filters> {
    fn to_sql<DB>(self, db: &DB) -> String
    where
        DB: Database,
    {
        self.iter()
            .enumerate()
            .map(|(i, filter)| {
                format!(
                    "{} {} {} ${}",
                    if filter.not { "not" } else { "" },
                    filter.field.field_name(),
                    filter.filter_type.clone().to_sql(db),
                    i + 1
                )
            })
            .collect::<Vec<String>>()
            .join(" or ")
    }
}

pub trait IntFilters<Rhs: ?Sized = Self> {
    fn gt(&self, other: Rhs) -> Filters;
    fn gte(&self, other: Rhs) -> Filters;
    fn lt(&self, other: Rhs) -> Filters;
    fn lte(&self, other: Rhs) -> Filters;
    fn eq(&self, other: Rhs) -> Filters;
}

pub trait StringFilters<Rhs: ?Sized = Self> {
    fn eq(&self, other: Rhs) -> Filters;
    fn like(&self, other: Rhs) -> Filters;
    fn ilike(&self, other: Rhs) -> Filters;
}
