use super::errors::EzyTutorError;
use super::models::Course;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_tutor(
    pool: &PgPool,
    tutor_id: i32,
) -> Result<Vec<Course>, EzyTutorError> {
    // Prepare SQL statement
    let course_rows = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course_c5 where tutor_id = $1"
        tutor_id
    )
    .fetch_all(pool)
    .await?;
    // Extract result

    let course: Vec<Course> = course_rows
        .iter()
        .map(|course_row| Course {
            course_id: course_row.course_id,
            
        })
}