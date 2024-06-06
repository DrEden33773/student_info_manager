# Requirements

现在假设你是一个资深 rust 开发者, 也是一个数据科学专家.

假设我已经有了一个 `PostgreSQL` 数据库, 现在请你使用 rust 和 iced, diesel 框架, 开发一个具有良好用户交互界面 的 `数据管理系统`.

在实现的过程中, 你可以使用任意主流的框架.

接下来, 我会一步步的告诉你, 这个工程的前置条件或者前置实现. 在我回复 `开始实现` 之前, 每次告诉你的信息只需要你记住即可, 并回复 `我了解了`.

## Part I

构造表结构:

```sql
CREATE TABLE IF NOT EXISTS students (
    s_no SERIAL PRIMARY KEY,
    s_name VARCHAR(100) NOT NULL,
    sex CHAR(1) CHECK (sex IN ('M', 'F', 'O')) NOT NULL,
    birthday DATE NOT NULL,
    dept VARCHAR(100) NOT NULL
);
CREATE TABLE IF NOT EXISTS courses (
    c_no SERIAL PRIMARY KEY,
    c_name VARCHAR(100) NOT NULL,
    c_pno VARCHAR(100) REFERENCES courses (c_no),
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
```

你可以根据需要实现的/我要求实现的功能, 实时拓展表/列

## Part II

提供 `数据库` 的关键信息:

- host: 120.46.70.140
- port: 26000
- username: dbm
- password: Password_123
- database: db

dbm 这个 `用户` 已获得所有权限, 数据库中也建立了 `dbm` schema (授权于用户 `dbm` 上), 且 search_path 被设置为此

## Part III

你需要保证你的实现具有相对较高的性能, 能够支撑 `百万级数据查询`, 请在必要的逻辑中, 使用 `高并发` 的实现方式

## Part IV

项目显然是可以 `分文件` 的, 这一点请你随意

rust 项目, 已经利用 `rustfmt.toml` 指定为 `2 空格缩进`, 生成代码时请注意

最初, 你可以做一个界面相对简单的实现

但是请你一定要在每一步都详细的为我说明 (尤其是代码段中, 如果出现了 `use <module>`, 请一定要列出)

## Part V

你可以分步给出 `解释` 和 `实现`, 如果未完, 在最后指出 `(未完待续)`

## Answer
