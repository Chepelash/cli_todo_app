drop table if exists todo_records;

create table if not EXISTS todo_records (
    id bigserial,
    title text,
    description text,
    done boolean DEFAULT FALSE
);