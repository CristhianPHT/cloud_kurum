# 📋 Decisiones de Arquitectura (ADR)

## ADR-001: Stack Tecnológico

### Estado
✅ Aceptado

### Contexto
Decidir qué tecnologías usar para el backend.

### Decisión
- **Lenguaje:** Rust (por seguridad, rendimiento y concurrencia)
- **ORM:** Diesel (por type-safe queries y migraciones)
- **Base de Datos:** PostgreSQL (por JSON support y robustez)
- **Framework web:** Axum (por ser moderno y rápido)

### Consecuencias
- ✅ Mayor seguridad en memoria
- ✅ Mejor rendimiento
- ⚠️ Curva de aprendizaje más pronunciada

---

## ADR-002: Soft Delete

### Estado
✅ Aceptado

### Contexto
Necesitar eliminar registros sin perder datos históricos.

### Decisión
- Usar `is_deleted` BOOLEAN DEFAULT FALSE
- Usar `deleted_at` TIMESTAMP
- Query siempre filtran `is_deleted = false`

### Consecuencias
- ✅ Recuperación de datos posible
- ✅ Auditoría de eliminaciones
- ⚠️ Necesidad de índices específicos

---

## ADR-003: Almacenamiento de Imágenes

### Estado
✅ Aceptado

### Contexto
Necesitar almacenar imágenes de libros.

### Decisión
- Usar **Cloudflare R2** (S3-compatible, más barato)
- Guardar solo URL en la BD
- Procesar imágenes en el backend (Sharp)
- Generar slugs para URLs amigables

### Consecuencias
- ✅ Bajo costo
- ✅ CDN integrado
- ✅ Escalable
- ⚠️ Dependencia externa

---

## ADR-004: Estructura de API

### Estado
✅ Aceptado

### Contexto
Diseñar endpoints RESTful para el sistema.

### Decisión
- Usar `/api/libros` en plural
- Usar **slug** en lugar de ID para URLs públicas
- Usar **ID** para operaciones internas (PUT, DELETE)
- Soft delete con `DELETE` lógico

### Consecuencias
- ✅ URLs amigables para SEO
- ✅ Consistencia REST
- ✅ Fácil de entender

---

## ADR-005: Migraciones

### Estado
✅ Aceptado

### Contexto
Manejar cambios en el esquema de BD.

### Decisión
- Usar **Diesel migrations**
- Migraciones versionadas
- SQL puro en `up.sql` y `down.sql`
- Seeds separados en `seeds/`

### Consecuencias
- ✅ Versionado de esquema
- ✅ Rollback posible
- ✅ Control total de SQL

- ------------------------------------------------------------------------

# ¿Por qué Rust?
✅ Memory safe sin GC

✅ Rendimiento comparable a C++

✅ Excelente soporte para concurrencia

✅ Type system robusto

✅ Crecimiento en el ecosistema web

# ¿Por qué Diesel?

✅ Type-safe queries

✅ Migraciones integradas

✅ Soporte para PostgreSQL

✅ Buen rendimiento

# ¿Por qué PostgreSQL?

✅ Soporte JSON nativo

✅ ACID compliance

✅ Indexes avanzados

✅ Comunidad grande

✅ Escalable