use dotenv::dotenv;
use std::env;
use std::io;
use sqlx::postgres::PgPool;
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}