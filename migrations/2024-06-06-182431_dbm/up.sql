-- Your SQL goes here
CREATE TABLE IF NOT EXISTS students (
    s_no SERIAL PRIMARY KEY,
    s_name VARCHAR(100) NOT NULL,
    sex VARCHAR(1) CHECK (sex IN ('M', 'F', 'O')) NOT NULL,
    birthday DATE NOT NULL,
    dept VARCHAR(100) NOT NULL
);
CREATE TABLE IF NOT EXISTS courses (
    c_no SERIAL PRIMARY KEY,
    c_name VARCHAR(100) NOT NULL,
    c_pno SERIAL REFERENCES courses (c_no),
    credit INTEGER CHECK (credit > 0) NOT NULL
);
CREATE TABLE IF NOT EXISTS student_courses (
    s_no SERIAL NOT NULL,
    c_no SERIAL NOT NULL,
    grade INTEGER CHECK (
        grade BETWEEN 0 AND 100
    ),
    semester VARCHAR(100) NOT NULL,
    teaching_class VARCHAR(100) NOT NULL,
    PRIMARY KEY (s_no, c_no),
    FOREIGN KEY (s_no) REFERENCES students (s_no),
    FOREIGN KEY (c_no) REFERENCES courses (c_no)
);