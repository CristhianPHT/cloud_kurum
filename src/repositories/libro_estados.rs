use crate::model::libro_estados::{LibroTipo, NewLibroTipo, LibroEstado, NewLibroEstado};
use crate::schema::libro_tipo::dsl::{libro_tipo, id as tip_id, nombre as lib_tip_name};
use crate::schema::libro_estado::dsl::{libro_estado, id as est_id, nombre as lib_est_name};
use diesel::{update, delete};
use diesel::dsl::insert_into;
use diesel::prelude::*;

// ------------------------------------------- Libro - Tipo -------------------------------------------
pub fn insert_lib_tip_new(conn: &mut PgConnection, new_lib_tip: NewLibroTipo) -> QueryResult<i32> { // usize o i32?
  insert_into(libro_tipo)
    .values(new_lib_tip)
    .returning(tip_id)
    .get_result(conn)
    // .execute(conn)  // filas afectadas (usize)
}

pub fn select_lib_tip(conn: &mut PgConnection, identificacion: i32) -> QueryResult<LibroTipo> {
  libro_tipo
    .filter(tip_id.eq(identificacion))
    .first::<LibroTipo>(conn)
}

pub fn select_lib_tip_all( conn: &mut PgConnection, pagina: i64 ) -> QueryResult<Vec<LibroTipo>> {
  let por_pagina: i64 = 10;
  let pagina = pagina.max(1); // Dame el mayor entre pagina y 1
  let offset = (pagina - 1) * por_pagina;
  libro_tipo
    .order(tip_id.desc())
    .limit(por_pagina)
    .offset(offset)
    .load::<LibroTipo>(conn)
}

// pub fn select_lib_tip_allv1( conn: &mut PgConnection ) -> QueryResult<Vec<LibroTipo>> {
//   libro_tipo
//     .load::<LibroTipo>(conn)
// }

pub fn update_lib_tip( conn: &mut PgConnection, identificacion: i32, cambios: LibroTipo ) -> QueryResult<String> {
  update(libro_tipo.filter(tip_id.eq(identificacion)))
    .set(cambios)
    .returning(lib_tip_name)
    .get_result(conn)
}

pub fn delete_lib_tip( conn: &mut PgConnection, identificacion: i32 ) -> QueryResult<usize> {
  delete(libro_tipo.filter(tip_id.eq(identificacion)))
    .execute(conn)
}

// ------------------------------------------- Libro - Estado -------------------------------------------

pub fn insert_lib_est_new( conn: &mut PgConnection, new_lib_est: NewLibroEstado) -> QueryResult<i32> {
  insert_into(libro_estado)
    .values(new_lib_est)
    .returning(est_id)
    .get_result(conn)
}

pub fn select_lib_est( conn: &mut PgConnection, identificacion: i32) -> QueryResult<LibroEstado> {
  libro_estado
    .filter(est_id.eq(identificacion))
    .first::<LibroEstado>(conn)
}

pub fn select_lib_est_all( conn: &mut PgConnection, pagina: i64 ) -> QueryResult<Vec<LibroEstado>> {
  let por_pagina: i64 = 10;
  let pagina = pagina.max(1); // Dame el mayor entre pagina y 1
  let offset = (pagina - 1) * por_pagina;
  libro_estado
    .order(est_id.desc())
    .limit(por_pagina)
    .offset(offset)
    .load::<LibroEstado>(conn)
}
// pub fn select_lib_est_all( conn: &mut PgConnection) -> QueryResult<Vec<LibroEstado>> {
//   libro_estado
//     .load::<LibroEstado>(conn)
// }

pub fn update_lib_est( conn: &mut PgConnection, identificacion: i32, cambios: LibroEstado) -> QueryResult<String> {
  update( libro_estado.filter(est_id.eq(identificacion)) )
    .set(cambios)
    .returning(lib_est_name)
    .get_result(conn)
}

pub fn delete_lib_est( conn: &mut PgConnection, identificacion: i32 ) -> QueryResult<usize> {
  delete(libro_estado.filter(est_id.eq(identificacion)))
    .execute(conn)
}