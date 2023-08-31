-- Add migration script here
/* Define a specific user for this book's applications */
/*drop user if exists lubex;
create user lubex with password 'lubex6';*/

/* Drop table if it already exists*/
drop table if exists ezy_course_c5;
/* Create table. */
/* Note: Don't put a comma after last field */
create table ezy_course_c5
(
    course_id serial primary key,
    tutor_id INT not null,
    course_name varchar(140) not null,
    posted_time TIMESTAMP default now()
);

/* Load seed data for testing */
insert into ezy_course_c5
    (course_id,tutor_id, course_name,posted_time)
values(1, 1, 'First course', '2023-08-31 05:40:00');
insert into ezy_course_c5
    (course_id, tutor_id, course_name,posted_time)
values(2, 1, 'Second course', '2023-08-31 05:45:00');

/* Grant privileges to specific user */
grant all privileges on table ezy_course_c5 to lubex;