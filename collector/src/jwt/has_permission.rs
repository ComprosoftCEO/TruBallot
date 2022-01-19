use crate::errors::{ResourceAction, ResourceType, ServiceError};
use crate::jwt::{Audience, JWTToken, Permission};

/// Helper trait that exposes methods for checking specific permissions
pub trait HasPermission {
  /// Test if the user has permission to do something
  fn has_permission(&self, p: Permission) -> bool;

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
        ResourceAction::Register,
      ))
    }
  }
}

impl<A: Audience, T> HasPermission for JWTToken<A, T> {
  fn has_permission(&self, p: Permission) -> bool {
    self.test_has_permission(p)
  }
}
