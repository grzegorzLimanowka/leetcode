use std::{fmt::Display, str::FromStr};

use futures::future::BoxFuture;
use log::LevelFilter;
use sqlx::{
    any::AnyTransactionManager,
    database::{HasArguments, HasStatement, HasStatementCache, HasValueRef},
    Arguments, Column, ConnectOptions, Connection, Database, Error, Row, Statement,
    TransactionManager, TypeInfo, Value, ValueRef,
};
use url::Url;

#[derive(Debug, Hash)]
struct EmptyDb {}

#[derive(Debug, Hash)]
struct EmptyStatement {}

#[derive(Default, Debug, Hash)]
struct EmptyArgument {}

#[derive(Debug, Hash)]
struct EmptyRow {}

impl Row for EmptyRow {
    type Database = EmptyDb;

    fn columns(&self) -> &[<Self::Database as Database>::Column] {
        todo!()
    }

    fn try_get_raw<I>(
        &self,
        index: I,
    ) -> Result<<Self::Database as sqlx::database::HasValueRef<'_>>::ValueRef, sqlx::Error>
    where
        I: sqlx::ColumnIndex<Self>,
    {
        todo!()
    }
}

impl<'q> Arguments<'q> for EmptyArgument {
    type Database = EmptyDb;

    fn reserve(&mut self, additional: usize, size: usize) {
        todo!()
    }

    fn add<T>(&mut self, value: T)
    where
        T: 'q + Send + sqlx::Encode<'q, Self::Database> + sqlx::Type<Self::Database>,
    {
        todo!()
    }
}

impl<'q> Statement<'q> for EmptyStatement {
    type Database = EmptyDb;

    fn to_owned(&self) -> <Self::Database as HasStatement<'static>>::Statement {
        todo!()
    }

    fn sql(&self) -> &str {
        todo!()
    }

    fn parameters(&self) -> Option<sqlx::Either<&[<Self::Database as Database>::TypeInfo], usize>> {
        todo!()
    }

    fn columns(&self) -> &[<Self::Database as Database>::Column] {
        todo!()
    }

    fn query(
        &self,
    ) -> sqlx::query::Query<
        '_,
        Self::Database,
        <Self::Database as sqlx::database::HasArguments<'_>>::Arguments,
    > {
        todo!()
    }

    fn query_with<'s, A>(&'s self, arguments: A) -> sqlx::query::Query<'s, Self::Database, A>
    where
        A: sqlx::IntoArguments<'s, Self::Database>,
    {
        todo!()
    }

    fn query_as<O>(
        &self,
    ) -> sqlx::query::QueryAs<
        '_,
        Self::Database,
        O,
        <Self::Database as sqlx::database::HasArguments<'_>>::Arguments,
    >
    where
        O: for<'r> sqlx::FromRow<'r, <Self::Database as Database>::Row>,
    {
        todo!()
    }

    fn query_as_with<'s, O, A>(
        &'s self,
        arguments: A,
    ) -> sqlx::query::QueryAs<'s, Self::Database, O, A>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::Database as Database>::Row>,
        A: sqlx::IntoArguments<'s, Self::Database>,
    {
        todo!()
    }

    fn query_scalar<O>(
        &self,
    ) -> sqlx::query::QueryScalar<
        '_,
        Self::Database,
        O,
        <Self::Database as sqlx::database::HasArguments<'_>>::Arguments,
    >
    where
        (O,): for<'r> sqlx::FromRow<'r, <Self::Database as Database>::Row>,
    {
        todo!()
    }

    fn query_scalar_with<'s, O, A>(
        &'s self,
        arguments: A,
    ) -> sqlx::query::QueryScalar<'s, Self::Database, O, A>
    where
        (O,): for<'r> sqlx::FromRow<'r, <Self::Database as Database>::Row>,
        A: sqlx::IntoArguments<'s, Self::Database>,
    {
        todo!()
    }

    fn column<I>(&self, index: I) -> &<Self::Database as Database>::Column
    where
        I: sqlx::ColumnIndex<Self>,
    {
        self.try_column(index).unwrap()
    }

    fn try_column<I>(&self, index: I) -> Result<&<Self::Database as Database>::Column, sqlx::Error>
    where
        I: sqlx::ColumnIndex<Self>,
    {
        Ok(&self.columns()[index.index(self)?])
    }
}

#[derive(Clone, Debug)]
struct EmptyConnectOptions {}

impl FromStr for EmptyConnectOptions {
    type Err = sqlx::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl ConnectOptions for EmptyConnectOptions {
    type Connection = EmptyConnection;

    fn from_url(url: &Url) -> Result<Self, sqlx::Error> {
        todo!()
    }

    fn connect(&self) -> BoxFuture<'_, Result<Self::Connection, sqlx::Error>>
    where
        Self::Connection: Sized,
    {
        todo!()
    }

    fn log_statements(self, level: LevelFilter) -> Self {
        todo!()
    }

    fn log_slow_statements(self, level: LevelFilter, duration: std::time::Duration) -> Self {
        todo!()
    }
}

struct EmptyConnection {}

// struct EmptyConnectOptions {}

impl Connection for EmptyConnection {
    type Database = EmptyDb;

    type Options = EmptyConnectOptions;

    fn close(self) -> BoxFuture<'static, Result<(), sqlx::Error>> {
        todo!()
    }

    fn ping(&mut self) -> BoxFuture<'_, Result<(), sqlx::Error>> {
        todo!()
    }

    fn begin(&mut self) -> BoxFuture<'_, Result<sqlx::Transaction<'_, Self::Database>, sqlx::Error>>
    where
        Self: Sized,
    {
        todo!()
    }

    fn shrink_buffers(&mut self) {
        todo!()
    }

    fn transaction<'a, F, R, E>(&'a mut self, callback: F) -> BoxFuture<'a, Result<R, E>>
    where
        for<'c> F: FnOnce(&'c mut sqlx::Transaction<'_, Self::Database>) -> BoxFuture<'c, Result<R, E>>
            + 'a
            + Send
            + Sync,
        Self: Sized,
        R: Send,
        E: From<sqlx::Error> + Send,
    {
        Box::pin(async move {
            let mut transaction = self.begin().await?;
            let ret = callback(&mut transaction).await;

            match ret {
                Ok(ret) => {
                    transaction.commit().await?;

                    Ok(ret)
                }
                Err(err) => {
                    transaction.rollback().await?;

                    Err(err)
                }
            }
        })
    }

    fn cached_statements_size(&self) -> usize
    where
        Self::Database: sqlx::database::HasStatementCache,
    {
        0
    }

    fn clear_cached_statements(&mut self) -> BoxFuture<'_, Result<(), sqlx::Error>>
    where
        Self::Database: sqlx::database::HasStatementCache,
    {
        Box::pin(async move { Ok(()) })
    }

    fn connect(url: &str) -> BoxFuture<'static, Result<Self, sqlx::Error>>
    where
        Self: Sized,
    {
        let options = url.parse();

        Box::pin(async move { Ok(Self::connect_with(&options?).await?) })
    }

    fn connect_with(options: &Self::Options) -> BoxFuture<'_, Result<Self, sqlx::Error>>
    where
        Self: Sized,
    {
        options.connect()
    }

    fn close_hard(self) -> BoxFuture<'static, Result<(), Error>> {
        todo!()
    }

    fn flush(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        todo!()
    }

    fn should_flush(&self) -> bool {
        false
    }
}

struct EmptyTransactionManager;

impl TransactionManager for EmptyTransactionManager {
    type Database = EmptyDb;

    fn begin(
        conn: &mut <Self::Database as Database>::Connection,
    ) -> BoxFuture<'_, Result<(), Error>> {
        todo!()
    }

    fn commit(
        conn: &mut <Self::Database as Database>::Connection,
    ) -> BoxFuture<'_, Result<(), Error>> {
        todo!()
    }

    fn rollback(
        conn: &mut <Self::Database as Database>::Connection,
    ) -> BoxFuture<'_, Result<(), Error>> {
        todo!()
    }

    fn start_rollback(conn: &mut <Self::Database as Database>::Connection) {
        todo!()
    }
}

#[derive(Debug)]
struct EmptyColumn;

impl Column for EmptyColumn {
    type Database = EmptyDb;

    fn ordinal(&self) -> usize {
        todo!()
    }

    fn name(&self) -> &str {
        todo!()
    }

    fn type_info(&self) -> &<Self::Database as Database>::TypeInfo {
        todo!()
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct EmptyTypeInfo;

impl Display for EmptyTypeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("YOYO")
    }
}

impl TypeInfo for EmptyTypeInfo {
    fn is_null(&self) -> bool {
        todo!()
    }

    fn name(&self) -> &str {
        todo!()
    }
}

struct EmptyValue;
struct EmptyValueRef;

impl<'a> ValueRef<'a> for EmptyValueRef {
    type Database = EmptyDb;

    fn to_owned(&self) -> <Self::Database as Database>::Value {
        todo!()
    }

    fn type_info(&self) -> std::borrow::Cow<'_, <Self::Database as Database>::TypeInfo> {
        todo!()
    }

    fn is_null(&self) -> bool {
        todo!()
    }
}

impl Value for EmptyValue {
    type Database = EmptyDb;

    fn as_ref(&self) -> <Self::Database as HasValueRef<'_>>::ValueRef {
        todo!()
    }

    fn type_info(&self) -> std::borrow::Cow<'_, <Self::Database as Database>::TypeInfo> {
        todo!()
    }

    fn is_null(&self) -> bool {
        todo!()
    }
}

impl Database for EmptyDb {
    type Connection = EmptyConnection;

    type TransactionManager = EmptyTransactionManager;

    type Row = EmptyRow;

    type QueryResult = ();

    type Column = EmptyColumn;

    type TypeInfo = EmptyTypeInfo;

    type Value = EmptyValue;

    const NAME: &'static str = "asdf";

    const URL_SCHEMES: &'static [&'static str] = &["sadf"];
}

impl<'q> HasStatement<'q> for EmptyDb {
    type Database = Self;

    // type Statement = Statement<'q, Self::Database>;
    type Statement = EmptyStatement;
}

impl HasStatementCache for EmptyDb {
    //
}

impl<'q> HasArguments<'q> for EmptyDb {
    type Database = Self;

    type Arguments = EmptyArgument;

    type ArgumentBuffer = ();
}

impl<'q> HasValueRef<'q> for EmptyDb {
    type Database = Self;

    type ValueRef = EmptyValueRef;
}
