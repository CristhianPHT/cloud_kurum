flow = flujo o camino de las cosas

{
  "flujo": {
    "registro": {
      "1": "POST /api/usuarios/registrar (sin autenticación)",
      "2": "Backend hashea password, genera nickname si no se envía",
      "3": "Retorna usuario creado"
    },
    "login": {
      "1": "POST /api/usuarios/login (sin autenticación)",
      "2": "Verifica email y password",
      "3": "Genera JWT token (expiración: 24h)",
      "4": "Retorna token + datos del usuario"
    },
    "protegidas": {
      "1": "Todas las demás rutas requieren JWT",
      "2": "Frontend envía: Authorization: Bearer {token}",
      "3": "Backend valida token y extrae user_id"
    }
  }
}
