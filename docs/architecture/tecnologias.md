# 🔧 Stack Tecnológico

## Backend

### Lenguaje y Framework
| Tecnología | Versión | Propósito |
|------------|---------|-----------|
| **Rust** | 1.75+ | Lenguaje principal |
| **Axum** | 0.7+ | Framework web |
| **Tokio** | 1.0+ | Runtime asíncrono |
| **Serde** | 1.0+ | Serialización JSON |

### Base de Datos
| Tecnología | Versión | Propósito |
|------------|---------|-----------|
| **PostgreSQL** | 15+ | Base de datos principal |
| **Diesel** | 2.1+ | ORM y migraciones |
| **R2D2** | 0.8+ | Pool de conexiones |

### Almacenamiento
| Tecnología | Propósito |
|------------|-----------|
| **Cloudflare R2** | Almacenamiento de imágenes |
| **AWS SDK for Rust** | Cliente S3-compatible |

### Procesamiento de Imágenes
| Tecnología | Propósito |
|------------|-----------|
| **libvips / sharp-rs** | Redimensionar y optimizar imágenes |

### Desarrollo
| Herramienta | Propósito |
|-------------|-----------|
| **cargo** | Gestor de paquetes |
| **cargo-watch** | Recarga automática en desarrollo |
| **docker / docker-compose** | Contenerización |

## 📦 Dependencias Principales

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
diesel = { version = "2.1", features = ["postgres", "uuid", "chrono"] }
diesel-async = "0.4"
diesel-migrations = "2.1"
dotenvy = "0.15"
chrono = "0.4"
uuid = { version = "1.0", features = ["v4"] }