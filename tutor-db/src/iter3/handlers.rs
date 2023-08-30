use super::db_access::*;
use super::models::Course;
use super::state::AppState;

use actix_web::{web, HttpResponse};
use std::convert::TryFrom;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> HttpResponse {
    let tuple = params.0;
    let tutor_id: i32 = tuple;
    let courses = get_courses_for_tutor_db(&app_state.db, tutor_id).await;
    HttpResponse::Ok().json(courses)
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> HttpResponse { /*
    let tuple = params;
    let tutor_id: i32 = tuple.0;
    let course_id: i32 = tuple.1; */
    let (tutor_id, course_id) = (params.0,params.1);
    let course = get_course_details_db(&app_state.db, tutor_id, course_id).await;
    HttpResponse::Ok().json(course)
}