// @generated automatically by Diesel CLI.

diesel::table! {
    courses (c_no) {
        c_no -> Int4,
        #[max_length = 100]
        c_name -> Varchar,
        c_pno -> Int4,
        credit -> Int4,
    }
}

diesel::table! {
    student_courses (s_no, c_no) {
        s_no -> Int4,
        c_no -> Int4,
        grade -> Nullable<Int4>,
        #[max_length = 100]
        semester -> Varchar,
        #[max_length = 100]
        teaching_class -> Varchar,
    }
}

diesel::table! {
    students (s_no) {
        s_no -> Int4,
        #[max_length = 100]
        s_name -> Varchar,
        #[max_length = 1]
        sex -> Varchar,
        birthday -> Date,
        #[max_length = 100]
        dept -> Varchar,
    }
}

diesel::joinable!(student_courses -> courses (c_no));
diesel::joinable!(student_courses -> students (s_no));

diesel::allow_tables_to_appear_in_same_query!(courses, student_courses, students,);
