// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod prisma;
use prisma::user;

use prisma_client_rust::QueryError;
use std::cmp;

use chrono::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    title: Option<String>,
    surname: String,
    first_name: String,
    other_names: Option<String>,
    gender: String,
    date_of_birth: DateTime<Local>,
}

#[tauri::command]
async fn upload_users_in_bulk(users_data: Vec<User>) -> Result<(), QueryError> {
    let batch_size = 999;
    let step = cmp::min(batch_size, users_data.len());
    for i in (0..users_data.len()).step_by(step) {
        let first = i;
        let mut last = i + batch_size;
        if last > users_data.len() {
            last = users_data.len();
        }
        let batch = &users_data[first..last];
        let client = prisma::new_client().await.unwrap();
        let user_creates = batch.into_iter().map(|user_data| {
            client.user().create(
                user_data.surname.to_string(),
                user_data.first_name.to_string(),
                user_data.gender.to_string(),
                user_data.date_of_birth.into(),
                vec![
                    user::title::set(Some(user_data.clone().title.unwrap_or_default())),
                    user::title::set(Some(user_data.clone().other_names.unwrap_or_default())),
                ],
            )
        });

        let new_users = client._batch(user_creates).await?;

        assert_eq!(new_users.len(), last - first)
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![upload_users_in_bulk])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
