// use std::sync::Arc;

// use anyhow::Context as _;
// use async_trait::async_trait;
// use shaku::{Component, Interface};

// use crate::domain::{
//     models::{User, ValidationError},
//     repositories::users::UsersRepository,
// };

// #[async_trait]
// pub trait UpdateAuthnUser: Interface {
//     async fn update_authenticated_user(
//         &self,
//         input: UpdateAuthnUserInput,
//     ) -> Result<UpdateAuthnUserOutput, UpdateAuthnUserUseCaseError>;
// }

// #[derive(Component)]
// #[shaku(interface = UpdateAuthnUser)]
// pub struct UpdateAuthnUserUseCase {
//     #[shaku(inject)]
//     repository: Arc<dyn UsersRepository>,
// }

// #[async_trait]
// impl UpdateAuthnUser for UpdateAuthnUserUseCase {
//     async fn update_authenticated_user(
//         &self,
//         input: UpdateAuthnUserInput,
//     ) -> Result<UpdateAuthnUserOutput, UpdateAuthnUserUseCaseError> {
//         Ok(self
//             .repository
//             .update_user(input.username.try_into()?)
//             .await?
//             .context)
//     }
// }
