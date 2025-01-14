use crate::app::{
    entities::notification::Notification,
    errors::BaseError,
    traits::repositories::{notification::TNotificationRepositories, user::TUserRepositories},
};

use super::user::UserService;

pub struct NotificationService<'a> {
    user_service: UserService<'a>,
    notification_rep: &'a (dyn TNotificationRepositories + Send + Sync),
}

impl<'a> NotificationService<'a> {
    pub fn new(
        user_rep: &'a (dyn TUserRepositories + Send + Sync),
        notification_rep: &'a (dyn TNotificationRepositories + Send + Sync),
        token: &'a str
    ) -> Self {
        Self {
            notification_rep,
            user_service: UserService::new(user_rep, token),
        }
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Notification, BaseError> {
        let user = match self.user_service.get_current_user().await {
            Ok(user) => user,
            Err(err) => return Err(err),
        };

        match self.notification_rep.find_by_id(id, &user.id).await {
            Some(notification) => Ok(notification),
            None => Err(BaseError::new("Notification not found ".to_string())),
        }
    }

    pub async fn read_by_id(&self, id: i32) -> Result<(), BaseError> {
        let user = match self.user_service.get_current_user().await {
            Ok(user) => user,
            Err(err) => return Err(err),
        };

        match self.notification_rep.find_by_id(id, &user.id).await {
            Some(data) => data,
            None => return Err(BaseError::new("Notification not found ".to_string())),
        };

        match self.notification_rep.set_read_by_id(id, &user.id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(BaseError::new(err)),
        }
    }

    pub async fn delete_by_id(&self, id: i32) -> Result<(), BaseError> {
        let user = match self.user_service.get_current_user().await {
            Ok(user) => user,
            Err(err) => return Err(err),
        };

        match self.notification_rep.find_by_id(id, &user.id).await {
            Some(data) => data,
            None => return Err(BaseError::new("Notification not found ".to_string())),
        };

        match self.notification_rep.set_delete_by_id(id, &user.id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(BaseError::new(err)),
        }
    }

    pub async fn get_all_for_current_user(
        &self,
    ) -> Result<Vec<Notification>, BaseError> {
        let user = match self.user_service.get_current_user().await {
            Ok(user) => user,
            Err(err) => return Err(err),
        };
        Ok(self
            .notification_rep
            .find_by_receiver(&user.id, false, None)
            .await)
    }

    pub async fn delete_all_for_current_user(&self) -> Result<(), BaseError> {
        let user = match self.user_service.get_current_user().await {
            Ok(user) => user,
            Err(err) => return Err(err),
        };

        match self.notification_rep.set_delete_by_receiver(&user.id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(BaseError::new(err)),
        }
    }

    pub async fn read_all_for_current_user(&self) -> Result<(), BaseError> {
        let user = match self.user_service.get_current_user().await {
            Ok(user) => user,
            Err(err) => return Err(err),
        };

        match self.notification_rep.set_read_by_receiver(&user.id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(BaseError::new(err)),
        }
    }
}
