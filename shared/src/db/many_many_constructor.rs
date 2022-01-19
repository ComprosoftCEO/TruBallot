use diesel::associations::BelongsTo;

/// Trait that must be implemented to construct a many-to-many relationship
pub trait ManyToManyConstructor<Left, Right>: BelongsTo<Left> + BelongsTo<Right> {
  fn new(left: &<Self as BelongsTo<Left>>::ForeignKey, right: &<Self as BelongsTo<Right>>::ForeignKey) -> Self;
}
