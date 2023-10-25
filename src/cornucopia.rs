// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod crud
{ use postgres::{{fallible_iterator::FallibleIterator,GenericClient}};#[derive( Debug)] pub struct UpdateParams < T1 : cornucopia_sync::StringSql,T2 : cornucopia_sync::StringSql,> { pub title : T1,pub description : T2,pub done : bool,pub id : i64,}#[derive( Debug)] pub struct IncertParams < T1 : cornucopia_sync::StringSql,T2 : cornucopia_sync::StringSql,> { pub title : T1,pub description : T2,pub done : bool,}#[derive( Debug, Clone, PartialEq, )] pub struct SelectAll
{ pub id : i64,pub title : String,pub description : String,pub done : bool,}pub struct SelectAllBorrowed < 'a >
{ pub id : i64,pub title : &'a str,pub description : &'a str,pub done : bool,} impl < 'a > From < SelectAllBorrowed <
'a >> for SelectAll
{
    fn
    from(SelectAllBorrowed { id,title,description,done,} : SelectAllBorrowed < 'a >)
    -> Self { Self { id,title: title.into(),description: description.into(),done,} }
}pub struct SelectAllQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a mut C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_sync
    :: private :: Stmt, extractor : fn(& postgres :: Row) -> SelectAllBorrowed,
    mapper : fn(SelectAllBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > SelectAllQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(SelectAllBorrowed) -> R) -> SelectAllQuery
    < 'a, C, R, N >
    {
        SelectAllQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub  fn one(self) -> Result < T, postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client)  ? ; let row =
        self.client.query_one(stmt, & self.params)  ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub  fn all(self) -> Result < Vec < T >, postgres :: Error >
    { self.iter()  ?.collect() } pub  fn opt(self) -> Result
    < Option < T >, postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client)  ? ;
        Ok(self.client.query_opt(stmt, & self.params) 
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub  fn iter(self,) -> Result < impl Iterator < Item = Result
    < T, postgres :: Error >> + 'a, postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client)  ? ; let it =
        self.client.query_raw(stmt, cornucopia_sync :: private ::
        slice_iter(& self.params))  ?
        .iterator().map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row))))  ;
        Ok(it)
    }
}pub fn select_all() -> SelectAllStmt
{ SelectAllStmt(cornucopia_sync :: private :: Stmt :: new("SELECT * FROM todo_records")) } pub
struct SelectAllStmt(cornucopia_sync :: private :: Stmt) ; impl
SelectAllStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a mut C,
) -> SelectAllQuery < 'a, C,
SelectAll, 0 >
{
    SelectAllQuery
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { SelectAllBorrowed { id : row.get(0),title : row.get(1),description : row.get(2),done : row.get(3),} }, mapper : | it | { <SelectAll>::from(it) },
    }
} }pub fn update() -> UpdateStmt
{ UpdateStmt(cornucopia_sync :: private :: Stmt :: new("update todo_records set title = ($1),
 						description = ($2),
 						done = ($3)
 					where id = ($4)")) } pub
struct UpdateStmt(cornucopia_sync :: private :: Stmt) ; impl
UpdateStmt { pub  fn bind < 'a, C : GenericClient, T1 : cornucopia_sync::StringSql,T2 : cornucopia_sync::StringSql,>
(& 'a mut self, client : & 'a mut C,
title : & 'a T1,description : & 'a T2,done : & 'a bool,id : & 'a i64,) -> Result < u64, postgres :: Error >
{
    let stmt = self.0.prepare(client)  ? ;
    client.execute(stmt, & [title,description,done,id,]) 
} }impl < 'a, C : GenericClient , T1 : cornucopia_sync::StringSql,T2 : cornucopia_sync::StringSql,>
cornucopia_sync :: Params < 'a, UpdateParams < T1,T2,>, Result <
u64, postgres :: Error > , C > for UpdateStmt
{
    fn
    params(& 'a mut self, client : & 'a mut C, params : & 'a
    UpdateParams < T1,T2,>) -> Result < u64, postgres ::
    Error >  { self.bind(client, & params.title,& params.description,& params.done,& params.id,)  }
}pub fn incert() -> IncertStmt
{ IncertStmt(cornucopia_sync :: private :: Stmt :: new("insert into todo_records (title, description, done) values ($1, $2, $3)")) } pub
struct IncertStmt(cornucopia_sync :: private :: Stmt) ; impl
IncertStmt { pub  fn bind < 'a, C : GenericClient, T1 : cornucopia_sync::StringSql,T2 : cornucopia_sync::StringSql,>
(& 'a mut self, client : & 'a mut C,
title : & 'a T1,description : & 'a T2,done : & 'a bool,) -> Result < u64, postgres :: Error >
{
    let stmt = self.0.prepare(client)  ? ;
    client.execute(stmt, & [title,description,done,]) 
} }impl < 'a, C : GenericClient , T1 : cornucopia_sync::StringSql,T2 : cornucopia_sync::StringSql,>
cornucopia_sync :: Params < 'a, IncertParams < T1,T2,>, Result <
u64, postgres :: Error > , C > for IncertStmt
{
    fn
    params(& 'a mut self, client : & 'a mut C, params : & 'a
    IncertParams < T1,T2,>) -> Result < u64, postgres ::
    Error >  { self.bind(client, & params.title,& params.description,& params.done,)  }
}pub fn delete_with_id() -> DeleteWithIdStmt
{ DeleteWithIdStmt(cornucopia_sync :: private :: Stmt :: new("delete from todo_records where id = ($1)")) } pub
struct DeleteWithIdStmt(cornucopia_sync :: private :: Stmt) ; impl
DeleteWithIdStmt { pub  fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a mut C,
id : & 'a i64,) -> Result < u64, postgres :: Error >
{
    let stmt = self.0.prepare(client)  ? ;
    client.execute(stmt, & [id,]) 
} }}}