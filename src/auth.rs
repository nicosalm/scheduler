
// auth.rs

use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use serde::{Serialize, Deserialize};
use rocket::request::{self, Request, FromRequest, Outcome};
use rocket::http::Status;
use crate::models::UserRole;
use chrono::{Utc, Duration};
use std::fmt::Debug;
use std::env;

// Struct for JWT Claims (Payload)
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32, // Subject (User ID)
    role: UserRole,
    exp: usize, // Expiration (UNIX timestamp)
}

// Function to generate JWT
pub fn generate_jwt(user_id: i32, role: UserRole) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(60))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims { 
        sub: user_id, 
        role, 
        exp: expiration 
    };
    let secret_key = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set"); // Use a real secret key from a configuration file or environment variable
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref()))
}

// Function to decode and validate JWT
fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret_key = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
    decode::<Claims>(token, &DecodingKey::from_secret(secret_key.as_ref()), &Validation::new(Algorithm::HS256))
        .map(|data| data.claims)
}

// Define AuthenticatedUser struct
pub struct AuthenticatedUser {
    pub user_id: i32,
    pub role: UserRole,
}

// Implement the request guard
#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("Authorization") {
            Some(header) if header.starts_with("Bearer ") => {
                let token = &header["Bearer ".len()..];
                match validate_jwt(token) {
                    Ok(claims) => Outcome::Success(AuthenticatedUser {
                        user_id: claims.sub,
                        role: claims.role,
                    }),
                    Err(_) => Outcome::Error((Status::Unauthorized, ())),
                }
            },
            _ => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}

