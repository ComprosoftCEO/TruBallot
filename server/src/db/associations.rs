// =======================
//       Model Base
// =======================
#[macro_export]
macro_rules! model_base(
  () => {
    model_base!(no update);

    // Save changes to database
    pub fn update(
      &self,
      conn: &crate::db::DbConnection,
    ) -> diesel::prelude::QueryResult<Self> {
      use diesel::prelude::*;

      self.save_changes::<Self>(conn.get())
    }

    // Insert as a new entry, or update the existing entry on conflict
    //   Uses the primary key to detect duplicate entries
    pub fn insert_or_update(
      &self,
      conn: &crate::db::DbConnection
    ) -> diesel::prelude::QueryResult<Self> {
      use diesel::prelude::*;
      use diesel::insert_into;

      insert_into(<Self as diesel::associations::HasTable>::table())
        .values(self)
        .on_conflict(<Self as diesel::associations::HasTable>::table().primary_key())
        .do_update()
        .set(self)
        .get_result::<Self>(conn.get())
    }
  };

  (order by $order:expr) => {
    model_base!();
    paste::item! {
      model_base!(@ order by crate::schema::$order);
    }
  };

  (no update, order by $order:expr) => {
    model_base!(no update);
    paste::item! {
      model_base!(@ order by crate::schema::$order);
    }
  };

  (no update) => {
    // Get all
    pub fn all(
      conn: &crate::db::DbConnection,
    ) -> diesel::prelude::QueryResult<Vec<Self>> {
      use diesel::prelude::*;

      <Self as diesel::associations::HasTable>::table()
        .get_results::<Self>(conn.get())
    }

    // Count all users
    pub fn count_all(
      conn: &crate::db::DbConnection,
    ) -> diesel::prelude::QueryResult<i64> {
      use diesel::prelude::*;

      <Self as diesel::associations::HasTable>::table()
        .count().get_result::<i64>(conn.get())
    }

    // Find from ID
    pub fn find(
      id: <&Self as diesel::associations::Identifiable>::Id,
      conn: &crate::db::DbConnection,
    ) -> diesel::prelude::QueryResult<Self> {
      use diesel::prelude::*;

      <Self as diesel::associations::HasTable>::table()
        .find(id)
        .get_result::<Self>(conn.get())
    }

    // Find optional from ID
    pub fn find_optional(
      id: <&Self as diesel::associations::Identifiable>::Id,
      conn: &crate::db::DbConnection,
    ) -> diesel::prelude::QueryResult<Option<Self>> {
      use diesel::prelude::*;

      <Self as diesel::associations::HasTable>::table()
        .find(id)
        .get_result::<Self>(conn.get())
        .optional()
    }

    // Create the new entry in the database
    pub fn insert(
      &self,
      conn: &crate::db::DbConnection,
    ) -> diesel::prelude::QueryResult<Self> {
      use diesel::prelude::*;
      use diesel::insert_into;

      insert_into(<Self as diesel::associations::HasTable>::table())
        .values(self)
        .get_result::<Self>(conn.get())
    }

    // Insert only if the entry is not in the database
    pub fn insert_ignore_conflicts(
      &self,
      conn: &crate::db::DbConnection,
    ) -> diesel::prelude::QueryResult<Option<Self>> {
      use diesel::prelude::*;
      use diesel::insert_into;

      insert_into(<Self as diesel::associations::HasTable>::table())
        .values(self)
        .on_conflict_do_nothing()
        .get_result::<Self>(conn.get())
        .optional()
    }

    // Create multiple entries in the database
    pub fn insert_list(
      entries: &Vec<Self>,
      conn: &crate::db::DbConnection,
    ) -> diesel::prelude::QueryResult<Vec<Self>> {
      use diesel::prelude::*;
      use diesel::insert_into;

      insert_into(<Self as diesel::associations::HasTable>::table())
        .values(entries)
        .get_results::<Self>(conn.get())
    }

    // Insert a list of values, but ignore duplicate values
    pub fn insert_list_ignore_conflicts(
      entries: &Vec<Self>,
      conn: &crate::db::DbConnection,
    ) -> diesel::prelude::QueryResult<Vec<Self>> {
      use diesel::prelude::*;
      use diesel::insert_into;

      insert_into(<Self as diesel::associations::HasTable>::table())
        .values(entries)
        .on_conflict_do_nothing()
        .get_results::<Self>(conn.get())
    }

    // Test if an item exists in the database
    pub fn exists(
      &self,
      conn: &crate::db::DbConnection,
    ) -> diesel::prelude::QueryResult<bool> {
      Self::find_optional(
        <&Self as diesel::associations::Identifiable>::id(&self),
        conn,
      ).map(|result| result.is_some())
    }

    // Test if an item exists by ID
    pub fn exists_from_id(
      id: <&Self as diesel::associations::Identifiable>::Id,
      conn: &crate::db::DbConnection,
    ) -> diesel::prelude::QueryResult<bool> {
      Self::find_optional(id, conn).map(|result| result.is_some())
    }

    // Reload from database
    pub fn reload(
      &mut self,
      conn: &crate::db::DbConnection,
    ) -> diesel::prelude::QueryResult<()> {
      use diesel::prelude::*;

      let new_value = <Self as diesel::associations::HasTable>::table()
        .find(
          <&Self as diesel::associations::Identifiable>::id(&self)
        )
        .get_result::<Self>(conn.get())?;

      *self = new_value;

      Ok(())
    }

    // Reload from database
    pub fn reload_new(
      &self,
      conn: &crate::db::DbConnection,
    ) -> diesel::prelude::QueryResult<Self> {
      use diesel::prelude::*;

      <Self as diesel::associations::HasTable>::table()
        .find(
          <&Self as diesel::associations::Identifiable>::id(&self)
        )
        .get_result::<Self>(conn.get())
    }

    // Delete item in database
    pub fn delete(
      &self,
      conn: &crate::db::DbConnection,
    ) -> diesel::prelude::QueryResult<Self> {
      use diesel::prelude::*;
      use diesel::delete;

      delete(
        <Self as diesel::associations::HasTable>::table().find(
          <&Self as diesel::associations::Identifiable>::id(&self)
        )
      )
      .get_result::<Self>(conn.get())
    }

    // Delete item in database, but only if it already exists
    pub fn delete_optional(
      &self,
      conn: &crate::db::DbConnection,
    ) -> diesel::prelude::QueryResult<Option<Self>> {
      use diesel::prelude::*;
      use diesel::delete;

      delete(
        <Self as diesel::associations::HasTable>::table().find(
          <&Self as diesel::associations::Identifiable>::id(&self)
        )
      )
      .get_result::<Self>(conn.get())
      .optional()
    }

    // Delete item in database
    pub fn delete_from_id(
      id: <&Self as diesel::associations::Identifiable>::Id,
      conn: &crate::db::DbConnection,
    ) -> diesel::prelude::QueryResult<Self> {
      use diesel::prelude::*;
      use diesel::delete;

      delete(
        <Self as diesel::associations::HasTable>::table().find(id)
      )
      .get_result::<Self>(conn.get())
    }

    // Delete item in database, but only if it already exists
    pub fn delete_from_id_optional(
      id: <&Self as diesel::associations::Identifiable>::Id,
      conn: &crate::db::DbConnection,
    ) -> diesel::prelude::QueryResult<Option<Self>> {
      use diesel::prelude::*;
      use diesel::delete;

      delete(
        <Self as diesel::associations::HasTable>::table().find(id)
      )
      .get_result::<Self>(conn.get())
      .optional()
    }
  };

  (@ order by $order:expr) => {

    // Get all ordered by
    pub fn all_ordered(
      conn: &crate::db::DbConnection,
    ) -> diesel::prelude::QueryResult<Vec<Self>> {
        use diesel::prelude::*;

        <Self as diesel::associations::HasTable>::table().order_by($order)
          .get_results::<Self>(conn.get())
    }
  };
);

// =======================
//      Belongs To
// =======================
#[macro_export]
macro_rules! belongs_to(
  ($parent:ident) => {
    paste::item! {
      belongs_to!($parent, [<$parent:snake>]);
    }
  };

  ($parent:ident, $func_base:ident) => {
    belongs_to!(@ crate::models::$parent, $func_base);
  };

  (@ $parent:path, $func_base:ident) => {
    // Get parent
    paste::item! {
      pub fn [<get_ $func_base>](
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
    }

    // Get parent optional
    paste::item! {
      pub fn [<get_ $func_base _optional>](
        &self,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<Option<$parent>> {
        use diesel::prelude::*;

        match <Self as diesel::associations::BelongsTo<$parent>>::foreign_key(&self) {
          None => Ok(None),
          Some(id) => {
            <$parent as diesel::associations::HasTable>::table()
              .find(id)
              .get_result::<$parent>(conn.get())
              .optional()
          }
        }
      }
    }
  };
);

// =======================
//       Has One
// =======================
#[macro_export]
macro_rules! has_one(
  // Pluralize
  ($child:ident) => {
    paste::item! {
      has_one!($child, [<$child:snake>]);
    }
  };

  ($child:ident, $func_base:ident) => {
    has_one!(@ crate::models::$child, $func_base);
  };

  // Explicit function base
  (@ $child:path, $func_base:ident) => {

    // Get child
    paste::item! {
      pub fn [<get_ $func_base>](
        &self,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<$child> {
        use diesel::prelude::*;

        <$child as diesel::associations::HasTable>::table().filter(
          <$child as diesel::associations::BelongsTo<Self>>::foreign_key_column()
            .eq(<&Self as diesel::associations::Identifiable>::id(&self)),
        ).get_result::<$child>(conn.get())
      }
    }

    // Get child from ID
    paste::item! {
      pub fn [<get_ $func_base _from_id>](
        id: <&Self as diesel::associations::Identifiable>::Id,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<$child> {
        use diesel::prelude::*;

        <$child as diesel::associations::HasTable>::table().filter(
          <$child as diesel::associations::BelongsTo<Self>>::foreign_key_column()
            .eq(id)
        ).get_result::<$child>(conn.get())
      }
    }

    // Delete child
    paste::item! {
      pub fn [<delete_ $func_base>](
        &self,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<Self> {
        use diesel::delete;
        use diesel::prelude::*;

        delete(
          <$child as diesel::associations::HasTable>::table().filter(
            <$child as diesel::associations::BelongsTo<Self>>::foreign_key_column()
              .eq(<&Self as diesel::associations::Identifiable>::id(&self)),
          ),
        )
        .get_result::<Self>(conn.get())
      }
    }

    // Delete child from ID
    paste::item! {
      pub fn [<delete_ $func_base _from_id>](
        id: <&Self as diesel::associations::Identifiable>::Id,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<Self> {
        use diesel::prelude::*;
        use diesel::delete;

        delete(
          <$child as diesel::associations::HasTable>::table().filter(
            <$child as diesel::associations::BelongsTo<Self>>::foreign_key_column()
              .eq(id),
          ),
        )
        .get_result::<Self>(conn.get())
      }
    }
  };
);

// =======================
//   Has Zero or One
// =======================
#[macro_export]
macro_rules! has_zero_or_one(
  // Pluralize
  ($child:ident) => {
    paste::item! {
      has_zero_or_one!($child, [<$child:snake>]);
    }
  };

  ($child:ident, $func_base:ident) => {
    has_zero_or_one!(@ crate::models::$child, $func_base);
  };

  // Explicit function base
  (@ $child:path, $func_base:ident) => {

    // Get child
    paste::item! {
      pub fn [<get_ $func_base>](
        &self,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<Option<$child>> {
        use diesel::prelude::*;

        <$child as diesel::associations::HasTable>::table().filter(
          <$child as diesel::associations::BelongsTo<Self>>::foreign_key_column()
            .eq(<&Self as diesel::associations::Identifiable>::id(&self)),
        )
        .get_result::<$child>(conn.get())
        .optional()
      }
    }

    // Get child from ID
    paste::item! {
      pub fn [<get_ $func_base _from_id>](
        id: <&Self as diesel::associations::Identifiable>::Id,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<Option<$child>> {
        use diesel::prelude::*;

        <$child as diesel::associations::HasTable>::table().filter(
          <$child as diesel::associations::BelongsTo<Self>>::foreign_key_column()
            .eq(id)
        )
        .get_result::<$child>(conn.get())
        .optional()
      }
    }

    // Delete child
    paste::item! {
      pub fn [<delete_ $func_base>](
        &self,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<Option<$child>> {
        use diesel::delete;
        use diesel::prelude::*;

        delete(
          <$child as diesel::associations::HasTable>::table().filter(
            <$child as diesel::associations::BelongsTo<Self>>::foreign_key_column()
              .eq(<&Self as diesel::associations::Identifiable>::id(&self)),
          ),
        )
        .get_result::<$child>(conn.get())
        .optional()
      }
    }

    // Delete child from ID
    paste::item! {
      pub fn [<delete_ $func_base _from_id>](
        id: <&Self as diesel::associations::Identifiable>::Id,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<Option<$child>> {
        use diesel::prelude::*;
        use diesel::delete;

        delete(
          <$child as diesel::associations::HasTable>::table().filter(
            <$child as diesel::associations::BelongsTo<Self>>::foreign_key_column()
              .eq(id),
          ),
        )
        .get_result::<$child>(conn.get())
        .optional()
      }
    }
  };
);

// =======================
//       Has Many
// =======================
#[macro_export]
macro_rules! has_many(
  // Pluralize
  ($child:ident) => {
    paste::item! {
      has_many!($child, [<$child:snake s>]);
    }
  };

  ($child:ident, order by $order:expr) => {
    paste::item! {
      has_many!($child, order by $order, [<$child:snake s>]);
    }
  };

  ($child:ident, $func_base:ident) => {
    has_many!(@ crate::models::$child, $func_base);
  };

  ($child:ident, order by $order:expr, $func_base:ident) => {
    has_many!(@ crate::models::$child, $func_base);
    paste::item! {
      has_many!(@ crate::models::$child, order by crate::schema::$order, $func_base);
    }
  };

  // Explicit function base
  (@ $child:path, $func_base:ident) => {

    // Get children
    paste::item! {
      pub fn [<get_ $func_base>](
        &self,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<Vec<$child>> {
        use diesel::prelude::*;

        $child::belonging_to(self).get_results::<$child>(conn.get())
      }
    }

    // Get children from ID
    paste::item! {
      pub fn [<get_ $func_base _from_id>](
        id: <&Self as diesel::associations::Identifiable>::Id,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<Vec<$child>> {
        use diesel::prelude::*;

        <$child as diesel::associations::HasTable>::table().filter(
          <$child as diesel::associations::BelongsTo<Self>>::foreign_key_column()
            .eq(id)
        ).get_results::<$child>(conn.get())
      }
    }

    // Count children
    paste::item! {
      pub fn [<count_ $func_base>](
        &self,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<i64> {
        use diesel::prelude::*;

        $child::belonging_to(self).count().get_result::<i64>(conn.get())
      }
    }

    // Count children from ID
    paste::item! {
      pub fn [<count_ $func_base _from_id>](
        id: <&Self as diesel::associations::Identifiable>::Id,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<i64> {
        use diesel::prelude::*;

        <$child as diesel::associations::HasTable>::table().filter(
          <$child as diesel::associations::BelongsTo<Self>>::foreign_key_column()
            .eq(id)
        ).count().get_result::<i64>(conn.get())
      }
    }

    // Delete all children
    paste::item! {
      pub fn [<delete_all_ $func_base>](
        &self,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<Vec<$child>> {
        use diesel::delete;
        use diesel::prelude::*;

        delete(
          <$child as diesel::associations::HasTable>::table().filter(
            <$child as diesel::associations::BelongsTo<Self>>::foreign_key_column()
              .eq(<&Self as diesel::associations::Identifiable>::id(&self)),
          ),
        )
        .get_results::<$child>(conn.get())
      }
    }

    // Delete all children from ID
    paste::item! {
      pub fn [<delete_all_ $func_base _from_id>](
        id: <&Self as diesel::associations::Identifiable>::Id,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<Vec<$child>> {
        use diesel::prelude::*;
        use diesel::delete;

        delete(
          <$child as diesel::associations::HasTable>::table().filter(
            <$child as diesel::associations::BelongsTo<Self>>::foreign_key_column()
              .eq(id),
          ),
        )
        .get_results::<$child>(conn.get())
      }
    }
  };

  // Has many ordered
  (@ $child:path, order by $order:expr, $func_base:ident) => {

    // Get children
    paste::item! {
      pub fn [<get_ $func_base _ordered>](
        &self,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<Vec<$child>> {
        use diesel::prelude::*;

        $child::belonging_to(self).order_by($order).get_results::<$child>(conn.get())
      }
    }

    // Get children from ID
    paste::item! {
      pub fn [<get_ $func_base _ordered_from_id>](
        id: <&Self as diesel::associations::Identifiable>::Id,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<Vec<$child>> {
        use diesel::prelude::*;

        <$child as diesel::associations::HasTable>::table().filter(
          <$child as diesel::associations::BelongsTo<Self>>::foreign_key_column()
            .eq(id)
        )
        .order_by($order)
        .get_results::<$child>(conn.get())
      }
    }
  };


  // =======================
  //    Has Many Through
  //     (Special case)
  // =======================
  ($child:ident through $through:ident) => (
    paste::item! {
      has_many!($child through $through, [<$child:snake s>]);
    }
  );

  ($child:ident through $through:ident, order by $order:expr) => (
    paste::item! {
      has_many!($child through $through, order by $order, [<$child:snake s>]);
    }
  );

  ($child:ident through $through:ident, $func_base:ident) => {
    has_many!(@ crate::models::$child => crate::models::$through, $func_base);
  };

  ($child:ident through $through:ident, order by $order:expr, $func_base:ident) => {
    has_many!(@ crate::models::$child => crate::models::$through, $func_base);
    paste::item! {
      has_many!(@ crate::models::$child => crate::models::$through, order by crate::schema::$order, $func_base);
    }
  };

  (@ $child:path => $through:path, $func_base:ident) => {

    // Get children
    paste::item! {
      pub fn [<get_ $func_base>](
        &self,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<Vec<$child>> {
        use diesel::prelude::*;

        <$through as diesel::associations::HasTable>::table()
          .inner_join(<$child as diesel::associations::HasTable>::table())
          .filter(
            <$through as diesel::associations::BelongsTo<Self>>::foreign_key_column()
              .eq(<&Self as diesel::associations::Identifiable>::id(&self)),
          )
          .get_results::<($through, $child)>(conn.get())
          .map(|results| results.into_iter().map(|(_, second)| second).collect())
      }
    }

    // Get children from ID
    paste::item! {
      pub fn [<get_ $func_base _from_id>](
        id: <&Self as diesel::associations::Identifiable>::Id,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<Vec<$child>> {
        use diesel::prelude::*;

        <$through as diesel::associations::HasTable>::table()
          .inner_join(<$child as diesel::associations::HasTable>::table())
          .filter(
            <$through as diesel::associations::BelongsTo<Self>>::foreign_key_column()
              .eq(id),
          )
          .get_results::<($through, $child)>(conn.get())
          .map(|results| results.into_iter().map(|(_, second)| second).collect())
      }
    }

    // Count children
    paste::item! {
      pub fn [<count_ $func_base>](
        &self,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<i64> {
        use diesel::prelude::*;

        <$through as diesel::associations::HasTable>::table()
          .inner_join(<$child as diesel::associations::HasTable>::table())
          .filter(
            <$through as diesel::associations::BelongsTo<Self>>::foreign_key_column()
              .eq(<&Self as diesel::associations::Identifiable>::id(&self)),
          )
          .count().get_result::<i64>(conn.get())
      }
    }

    // Count children from ID
    paste::item! {
      pub fn [<count_ $func_base _from_id>](
        id: <&Self as diesel::associations::Identifiable>::Id,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<i64> {
        use diesel::prelude::*;

        <$through as diesel::associations::HasTable>::table()
          .inner_join(<$child as diesel::associations::HasTable>::table())
          .filter(
            <$through as diesel::associations::BelongsTo<Self>>::foreign_key_column()
              .eq(id),
          )
          .count().get_result::<i64>(conn.get())
      }
    }

    // Delete children from many-to-many the association
    //   Does NOT delete the actual values
    paste::item! {
      pub fn [<remove_all_ $func_base>](
        &self,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<Vec<$through>> {
        use diesel::delete;
        use diesel::prelude::*;

        delete(
          <$through as diesel::associations::HasTable>::table().filter(
            <$through as diesel::associations::BelongsTo<Self>>::foreign_key_column()
              .eq(<&Self as diesel::associations::Identifiable>::id(&self)),
          )
        ).get_results::<$through>(conn.get())
      }
    }

    // Delete children from many-to-many the association by the ID
    //   Does NOT delete the actual values
    paste::item! {
      pub fn [<remove_all_ $func_base _from_id>](
        id: <&Self as diesel::associations::Identifiable>::Id,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<Vec<$through>> {
        use diesel::delete;
        use diesel::prelude::*;

        delete(
          <$through as diesel::associations::HasTable>::table().filter(
            <$through as diesel::associations::BelongsTo<Self>>::foreign_key_column()
              .eq(id),
          )
        ).get_results::<$through>(conn.get())
      }
    }

    // Set children
    paste::item! {
      pub fn [<set_ $func_base>]<'a>(
        &self,
        list: impl std::iter::IntoIterator<Item = &'a $child>,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<Vec<$through>> {
        let ids_list = list.into_iter().map(|item| <&$child as diesel::associations::Identifiable>::id(&item));
        Self::[<set_ $func_base _ids_from_id>](<&Self as diesel::associations::Identifiable>::id(&self), ids_list, conn)
      }
    }

    // Set children Ids
    paste::item! {
      pub fn [<set_ $func_base _ids>]<'a>(
        &self,
        ids_list: impl std::iter::IntoIterator<Item = &'a <$through as diesel::associations::BelongsTo<$child>>::ForeignKey>,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<Vec<$through>> {
        Self::[<set_ $func_base _ids_from_id>](<&Self as diesel::associations::Identifiable>::id(&self), ids_list, conn)
      }
    }

    // Set children from ID
    paste::item! {
      pub fn [<set_ $func_base _from_id>]<'a>(
        id: <&Self as diesel::associations::Identifiable>::Id,
        list: impl std::iter::IntoIterator<Item = &'a $child>,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<Vec<$through>> {
        let ids_list = list.into_iter().map(|item| <&$child as diesel::associations::Identifiable>::id(&item));
        Self::[<set_ $func_base _ids_from_id>](id, ids_list, conn)
      }
    }

    // Set all IDs from ID
    paste::item! {
      pub fn [<set_ $func_base _ids_from_id>]<'a>(
        id: <&Self as diesel::associations::Identifiable>::Id,
        ids_list: impl std::iter::IntoIterator<Item = &'a <$through as diesel::associations::BelongsTo<$child>>::ForeignKey>,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<Vec<$through>> {
        Self::[<remove_all_ $func_base _from_id>](id, conn)?;

        let new_entries: Vec<$through> = ids_list
          .into_iter()
          .map(|item| <$through as crate::db::ManyToManyConstructor<Self, $child>>::new(id, item))
          .collect();

        Ok($through::insert_list(&new_entries, conn)?)
      }
    }
  };

  (@ $child:path => $through:path, order by $order:expr, $func_base:ident) => {
    // Get children ordered
    paste::item! {
      pub fn [<get_ $func_base _ordered>](
        &self,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<Vec<$child>> {
        use diesel::prelude::*;

        <$through as diesel::associations::HasTable>::table()
          .inner_join(<$child as diesel::associations::HasTable>::table())
          .filter(
            <$through as diesel::associations::BelongsTo<Self>>::foreign_key_column()
              .eq(<&Self as diesel::associations::Identifiable>::id(&self)),
          )
          .order_by($order)
          .get_results::<($through, $child)>(conn.get())
          .map(|results| results.into_iter().map(|(_, second)| second).collect())
      }
    }

    // Get ordered children from ID
    paste::item! {
      pub fn [<get_ $func_base _ordered_from_id>](
        id: <&Self as diesel::associations::Identifiable>::Id,
        conn: &crate::db::DbConnection,
      ) -> diesel::prelude::QueryResult<Vec<$child>> {
        use diesel::prelude::*;

        <$through as diesel::associations::HasTable>::table()
          .inner_join(<$child as diesel::associations::HasTable>::table())
          .filter(
            <$through as diesel::associations::BelongsTo<Self>>::foreign_key_column()
              .eq(id),
          )
          .order_by($order)
          .get_results::<($through, $child)>(conn.get())
          .map(|results| results.into_iter().map(|(_, second)| second).collect())
      }
    }
  };
);
