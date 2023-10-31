-- Add migration script here
/* Define a specific user for this book's applications */
/*drop user if exists lubex;
create user lubex with password 'lubex6';*/

/* Drop tables if they already exist*/

drop table if exists ezy_course_c6;

/* Create tables. */
/* Note: Don't put a comma after last field */

create table ezy_course_c6
(
    course_id serial primary key,
    tutor_id INT not null,
    course_name varchar(140) not null,
    course_description varchar(2000),
    course_format varchar(30),
    course_structure varchar(200),
    course_duration varchar(30),
    course_price INT,
    course_language varchar(30),
    course_level varchar(30),
    posted_time TIMESTAMP default now()

);

/* Grant privileges to specific user */
grant all privileges on table ezy_course_c6 to lubex;
grant all privileges on all sequences in schema public to lubex;