#[macro_export]
macro_rules! sql_enum {
  ($v:vis $name:ident $type:tt) => {
    #[repr(i32)]
    #[derive(
      Debug, Copy, Clone, Eq, PartialEq, Hash,
      Serialize_repr, Deserialize_repr, EnumIter,
      FromSqlRow, FromPrimitive, AsExpression,
    )]
    #[sql_type = "Integer"]
    $v enum $name $type
    enum_internals!($name);
  };
}

#[allow(unused)]
macro_rules! enum_internals {
  ($($t:ty)*) => ($(

    // Required packages
    use diesel::{
      backend::Backend,
      deserialize::{self, FromSql},
      serialize::{self, Output, ToSql},
      sql_types::Integer,
    };
    use std::io::Write;

    // Convert to SQL
    impl<DB> ToSql<Integer, DB> for $t
    where
      DB: Backend,
      i32: ToSql<Integer, DB>,
    {
      fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        (*self as i32).to_sql(out)
      }
    }

    // Convert from SQL
    impl<DB> FromSql<Integer, DB> for $t
    where
      DB: Backend,
      i32: FromSql<Integer, DB>,
    {
      fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let number = i32::from_sql(bytes)?;
        num::FromPrimitive::from_i32(number).ok_or_else(
          || format!("diesel::deserialize::FromSql<Integer> for {} --- Invalid variant index {}", stringify!($t), number).into()
        )
      }
    }
  )*)
}
