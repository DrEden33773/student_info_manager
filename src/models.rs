use crate::schema::*;
use chrono::NaiveDate;
use diesel::prelude::*;

#[derive(Queryable, Selectable, AsChangeset, Debug /* Identifiable */)]
#[diesel(primary_key(c_no))]
#[diesel(table_name = courses)]
pub struct Course {
  pub c_no: i32,
  pub c_name: String,
  pub c_pno: i32,
  pub credit: i32,
}

#[derive(Queryable, Selectable, AsChangeset, Debug /* Identifiable */)]
#[diesel(primary_key(s_no, c_no))]
#[diesel(table_name = student_courses)]
pub struct StudentCourse {
  pub s_no: i32,
  pub c_no: i32,
  pub grade: Option<i32>,
  pub semester: String,
  pub teaching_class: String,
}

#[derive(Queryable, Selectable, AsChangeset, Debug /* Identifiable */)]
#[diesel(primary_key(s_no))]
#[diesel(table_name = students)]
pub struct Student {
  pub s_no: i32,
  pub s_name: String,
  pub sex: String,
  pub birthday: NaiveDate,
  pub dept: String,
}

#[derive(Insertable)]
#[diesel(table_name = courses)]
pub struct NewCourse<'a> {
  pub c_no: i32,
  pub c_name: &'a str,
  pub c_pno: i32,
  pub credit: i32,
}

#[derive(Insertable)]
#[diesel(table_name = student_courses)]
pub struct NewStudentCourse<'a> {
  pub s_no: i32,
  pub c_no: i32,
  pub grade: Option<i32>,
  pub semester: &'a str,
  pub teaching_class: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = students)]
pub struct NewStudent<'a> {
  pub s_no: i32,
  pub s_name: &'a str,
  pub sex: &'a str,
  pub birthday: NaiveDate,
  pub dept: &'a str,
}
