//! Handlers da API para operações de usuários

#[cfg(feature = "postgres")]
pub use postgres_handlers::*;

#[cfg(feature = "postgres")]
mod postgres_handlers {
    use crate::api::{ApiError, ApiResponse, AppState};
    use crate::db::DbUser;
    use axum::{
        extract::{Path, State},
        Json,
    };
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    #[derive(Debug, Deserialize, Validate)]
    pub struct CreateUserRequest {
        #[validate(length(min = 1, max = 255))]
        pub name: String,
        
        #[validate(email)]
        pub email: String,
    }

    #[derive(Debug, Serialize)]
    pub struct UserResponse {
        pub id: i32,
        pub name: String,
        pub email: String,
        pub active: bool,
    }

    impl From<DbUser> for UserResponse {
        fn from(user: DbUser) -> Self {
            Self {
                id: user.id,
                name: user.name,
                email: user.email,
                active: user.active,
            }
        }
    }

    /// Lista todos os usuários
    pub async fn list_users(
        State(state): State<AppState>,
    ) -> Result<Json<ApiResponse<Vec<UserResponse>>>, ApiError> {
        let users = DbUser::list_all(state.db.pool())
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;
        let response: Vec<UserResponse> = users.into_iter().map(Into::into).collect();
        
        Ok(Json(ApiResponse::success(response)))
    }

    /// Cria um novo usuário
    pub async fn create_user(
        State(state): State<AppState>,
        Json(payload): Json<CreateUserRequest>,
    ) -> Result<Json<ApiResponse<UserResponse>>, ApiError> {
        // Validar dados
        payload.validate()
            .map_err(|e| ApiError::BadRequest(e.to_string()))?;

        // Criar usuário
        let user = DbUser::create(state.db.pool(), &payload.name, &payload.email)
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;
        
        Ok(Json(ApiResponse::success(user.into())))
    }

    /// Busca um usuário por ID
    pub async fn get_user(
        State(state): State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<ApiResponse<UserResponse>>, ApiError> {
        let user = DbUser::find_by_id(state.db.pool(), id)
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?
            .ok_or_else(|| ApiError::NotFound(format!("User with id {} not found", id)))?;
        
        Ok(Json(ApiResponse::success(user.into())))
    }

    /// Deleta um usuário
    pub async fn delete_user(
        State(state): State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<ApiResponse<()>>, ApiError> {
        DbUser::delete(state.db.pool(), id)
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;
        
        Ok(Json(ApiResponse::success(())))
    }
}
