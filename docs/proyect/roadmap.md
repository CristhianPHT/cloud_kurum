# 🗺️ Roadmap del Proyecto

## Fase 1: Fundación (✅ COMPLETADO)
- ✅ Diseño de BD
- ✅ Contratos de datos

## Fase 2: Backend Core (⏳ EN PROGRESO)
- ⏳ API Endpoints
- 📋 Lógica de negocio

## Fase 3: Frontend (📋 PENDIENTE)
- 📋 Consumo de API
- 📋 UI/UX

## Fase 4: Testing & Deploy (📋 PENDIENTE)
- 📋 Tests unitarios
- 📋 Tests de integración
- 📋 CI/CD
- 📋 Deploy a producción

## 📅 Timeline Estimado

```mermaid
gantt
    title Cronograma del Proyecto
    dateFormat  YYYY-MM-DD
    section Fundación
    Diseño BD           :done, a1, 2024-01-01, 7d
    Contratos de datos   :done, a2, after a1, 3d
    section Backend
    API Endpoints        :active, b1, after a2, 14d
    Lógica de negocio    :b2, after b1, 10d
    section Frontend
    Consumo API          :c1, after b2, 10d
    UI/UX                :c2, after c1, 10d
    section Testing
    Tests                :d1, after c2, 7d
    Deploy               :d2, after d1, 3d