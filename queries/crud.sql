
--! select_all
SELECT * FROM todo_records;


--! update
update todo_records set title = (:title),
 						description = (:description),
 						done = (:done)
 					where id = (:id);
 
--! incert
insert into todo_records (title, description, done) values (:title, :description, :done);

--! delete_with_id
delete from todo_records where id = (:id); 
