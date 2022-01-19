// =======================
//      Parent Type
// =======================
#[macro_export]
macro_rules! parent_type(
  ($discriminant:ty = $from_value:expr) => {
    paste::item! {
      parent_type!($discriminant = $from_value, [<$discriminant:snake>]);
    }
  };

  ($discriminant:ty = $from_value:expr, $func_base:ident) => {
    paste::item! {
      pub fn [<get_ $func_base>](&self) -> $discriminant {
        self.$from_value
      }
    }
  };
);

// =======================
//      Child Type
// =======================
#[macro_export]
macro_rules! child_type(
  ($parent:ty, $discriminant:ty = $value:expr) => {
    paste::item! {
      child_type!($parent, $discriminant = $value, [<$discriminant:snake>]);
    }
  };

  ($parent:ty, $discriminant:ty = $from:expr, $func_base:ident) => {
    paste::item! {
      pub fn [<get_ $func_base>](&self) -> $discriminant {
        $from
      }
    }

    pub fn get_parent(
      &self,
      conn: &crate::db::DbConnection,
    ) -> diesel::prelude::QueryResult<$parent> {
      use diesel::prelude::*;

      match <Self as diesel::associations::BelongsTo<$parent>>::foreign_key(&self) {
        None => Err(diesel::result::Error::NotFound),
        Some(id) => {
          <$parent as diesel::associations::HasTable>::table()
            .find(id)
            .get_result::<$parent>(conn.get())
        }
      }
    }

    pub fn get_parent_optional(
     &self,
      conn: &crate::db::DbConnection,
    ) -> diesel::prelude::QueryResult<$parent> {
      use diesel::prelude::*;

      match <Self as diesel::associations::BelongsTo<$parent>>::foreign_key(&self) {
        None => Err(diesel::result::Error::NotFound),
        Some(id) => {
          <$parent as diesel::associations::HasTable>::table()
            .find(id)
            .get_result::<$parent>(conn.get())
        }
      }
    }
  };
);
