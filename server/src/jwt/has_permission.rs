use uuid_b64::UuidB64 as Uuid;

use crate::db::DbConnection;
use crate::errors::{ResourceAction, ResourceType, ServiceError};
use crate::jwt::{Audience, JWTToken, Permission};
use crate::models::User;

/// Helper trait that exposes methods for checking specific permissions
pub trait HasPermission {
  /// Get the user ID associated with the JWT token
  fn get_user_uuid(&self) -> Uuid;

  /// Test if the user has permission to do something
  fn has_permission(&self, p: Permission) -> bool;

  // ==================================================
  // Make sure the user ID is acutally in the database
  // ==================================================
  fn validate_user_id(&self, conn: &DbConnection) -> Result<User, ServiceError> {
    let user_id = self.get_user_uuid();
    User::find_optional(&user_id, conn)?.ok_or_else(|| ServiceError::JWTNoSuchUser { user_id })
  }

  // ===================================
  //   Methods to test user permissions
  // ===================================
  fn test_can_manage_account(&self) -> Result<(), ServiceError> {
    if self.has_permission(Permission::CanLogin) {
      Ok(())
    } else {
      Err(ServiceError::ForbiddenResourceAction(
        ResourceType::User,
        ResourceAction::Update,
      ))
    }
  }

  fn test_can_view_elections(&self) -> Result<(), ServiceError> {
    if self.has_permission(Permission::CanLogin) {
      Ok(())
    } else {
      Err(ServiceError::ForbiddenResourceAction(
        ResourceType::Election,
        ResourceAction::ReadPrivate,
      ))
    }
  }

  fn test_can_create_election(&self) -> Result<(), ServiceError> {
    if self.has_permission(Permission::CanLogin) && self.has_permission(Permission::CreateElection) {
      Ok(())
    } else {
      Err(ServiceError::ForbiddenResourceAction(
        ResourceType::Election,
        ResourceAction::Create,
      ))
    }
  }

  fn test_can_register_for_election(&self) -> Result<(), ServiceError> {
    if self.has_permission(Permission::CanLogin) && self.has_permission(Permission::Register) {
      Ok(())
    } else {
      Err(ServiceError::ForbiddenResourceAction(
        ResourceType::Election,
        ResourceAction::Register,
      ))
    }
  }

  fn test_can_vote(&self) -> Result<(), ServiceError> {
    if self.has_permission(Permission::CanLogin) && self.has_permission(Permission::Vote) {
      Ok(())
    } else {
      Err(ServiceError::ForbiddenResourceAction(
        ResourceType::Election,
        ResourceAction::Vote,
      ))
    }
  }
}

impl<A: Audience, T> HasPermission for JWTToken<A, T> {
  fn get_user_uuid(&self) -> Uuid {
    self.get_user_id()
  }

  fn has_permission(&self, p: Permission) -> bool {
    self.test_has_permission(p)
  }
}
