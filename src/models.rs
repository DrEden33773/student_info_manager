use chrono::NaiveDate;
use diesel::prelude::*;

#[derive(Queryable, Selectable, AsChangeset, Debug /* Identifiable */)]
#[diesel(primary_key(c_no))]
#[diesel(table_name = crate::schema::courses)]
pub struct Course {
  pub c_no: i32,
  pub c_name: String,
  pub c_pno: i32,
  pub credit: i32,
}

#[derive(Queryable, Selectable, AsChangeset, Debug /* Identifiable */)]
#[diesel(primary_key(s_no, c_no))]
#[diesel(table_name = crate::schema::student_courses)]
pub struct StudentCourse {
  pub s_no: i32,
  pub c_no: i32,
  pub grade: Option<i32>,
  pub semester: String,
  pub teaching_class: String,
}

#[derive(Queryable, Selectable, AsChangeset, Debug /* Identifiable */)]
#[diesel(primary_key(s_no))]
#[diesel(table_name = crate::schema::students)]
pub struct Student {
  pub s_no: i32,
  pub s_name: String,
  pub sex: String,
  pub birthday: NaiveDate,
  pub dept: String,
}
