use crate::errors::ServiceError;
use crate::jwt::{Audience, JWTToken, Permission};

/// Helper trait that exposes methods for checking specific permissions
pub trait HasPermission {
  /// Test if the user has permission to do something
  fn has_permission(&self, p: Permission) -> bool;

  // ===================================
  //   Methods to test user permissions
  // ===================================
  fn test_can_send_notification(&self) -> Result<(), ServiceError> {
    if self.has_permission(Permission::CanLogin) {
      Ok(())
    } else {
      Err(ServiceError::NoNotifyPermission)
    }
  }

  fn test_can_subscribe_to_notifications(&self) -> Result<(), ServiceError> {
    if self.has_permission(Permission::CanLogin) {
      Ok(())
    } else {
      Err(ServiceError::NoSubscribePermission)
    }
  }
}

impl<A: Audience, T> HasPermission for JWTToken<A, T> {
  fn has_permission(&self, p: Permission) -> bool {
    self.test_has_permission(p)
  }
}
