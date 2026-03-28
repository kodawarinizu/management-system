# Buenas Prácticas Profesionales con Git

> Guía de referencia para equipos de desarrollo que quieren usar Git de forma ordenada, segura y colaborativa.

---

## 1. Configuración Inicial

Antes de cualquier cosa, configura tu identidad. Cada commit lleva tu nombre y correo.

```bash
git config --global user.name "Tu Nombre"
git config --global user.email "tu@email.com"
git config --global core.editor "code --wait"   # o vim, nano, etc.
git config --global init.defaultBranch main
```

**Verifica tu configuración:**
```bash
git config --list
```

---

## 2. Estructura de Ramas (Branching Strategy)

### Modelo recomendado: Git Flow simplificado

```
main         → código en producción (siempre estable)
develop      → integración de features (base de trabajo)
feature/xxx  → nuevas funcionalidades
fix/xxx      → corrección de bugs
hotfix/xxx   → correcciones urgentes en producción
release/xxx  → preparación de versiones
```

### Reglas de oro para ramas

- **Nunca trabajes directamente en `main`.**
- Crea una rama por cada tarea, feature o bug.
- Nombres de rama en minúsculas con guiones: `feature/login-oauth`, `fix/null-pointer-error`.
- Elimina ramas después de mergearlas.

```bash
# Crear y cambiar a una rama nueva
git checkout -b feature/mi-funcionalidad

# Eliminar rama local ya mergeada
git branch -d feature/mi-funcionalidad

# Eliminar rama remota
git push origin --delete feature/mi-funcionalidad
```

---

## 3. Commits: el corazón del historial

### Formato estándar: Conventional Commits

```
<tipo>(<alcance opcional>): <descripción corta>

[cuerpo opcional]

[pie opcional: breaking changes, issues]
```

**Tipos principales:**

| Tipo       | Uso                                              |
|------------|--------------------------------------------------|
| `feat`     | Nueva funcionalidad                              |
| `fix`      | Corrección de bug                                |
| `docs`     | Solo documentación                               |
| `style`    | Formato, sin cambio de lógica                    |
| `refactor` | Reestructuración sin nuevas features ni bugs     |
| `test`     | Agregar o corregir tests                         |
| `chore`    | Tareas de mantenimiento (build, dependencias)    |
| `perf`     | Mejora de rendimiento                            |

**Ejemplos:**

```
feat(auth): agregar login con Google OAuth

fix(api): corregir respuesta 500 en endpoint /users

docs: actualizar instrucciones de instalación en README

refactor(cart): separar lógica de cálculo en servicio independiente
```

### Reglas para buenos commits

- **Un commit = un propósito.** No mezcles features con fixes.
- Descripción en **imperativo presente**: "Agrega", "Corrige", "Elimina" (no "Agregué" ni "Agregando").
- Máximo ~72 caracteres en la primera línea.
- El cuerpo explica el **por qué**, no el qué (el diff ya muestra el qué).
- Haz commits frecuentes, no al final del día con todo junto.

---

## 4. El archivo `.gitignore`

Nunca subas archivos que no deben estar en el repositorio.

**Lo que siempre debes ignorar:**

```gitignore
# Dependencias
node_modules/
vendor/
__pycache__/

# Variables de entorno y secretos
.env
.env.local
*.pem
*.key

# Builds y compilados
dist/
build/
*.pyc
*.class

# Editores e IDEs
.vscode/
.idea/
*.swp

# Sistema operativo
.DS_Store
Thumbs.db
```

> 💡 Usa [gitignore.io](https://www.toptal.com/developers/gitignore) para generar `.gitignore` según tu stack.

---

## 5. Flujo de Trabajo Diario

### Antes de empezar

```bash
git checkout develop
git pull origin develop          # sincroniza antes de crear tu rama
git checkout -b feature/mi-tarea
```

### Durante el desarrollo

```bash
git status                       # revisa qué cambió
git diff                         # revisa los cambios en detalle
git add archivo.js               # agrega solo lo relevante (evita git add .)
git commit -m "feat: descripción clara"
```

### Al terminar

```bash
git push origin feature/mi-tarea
# Abre un Pull Request / Merge Request en GitHub / GitLab
```

---

## 6. Rebase vs Merge

| Situación                                 | Usar         |
|-------------------------------------------|--------------|
| Integrar tu rama con develop/main         | `merge`      |
| Limpiar historial local antes de PR       | `rebase`     |
| Actualizar tu rama con cambios de develop | `rebase`     |
| Historial en rama compartida              | **nunca rebase** |

```bash
# Actualizar tu rama con los últimos cambios de develop (limpio)
git fetch origin
git rebase origin/develop

# Merge estándar (preserva contexto de la integración)
git merge develop
```

> ⚠️ **Regla crítica:** Nunca hagas `rebase` sobre ramas que otros también estén usando. Solo en ramas propias y locales.

---

## 7. Pull Requests / Merge Requests

Un PR es una conversación, no solo código.

### Como autor

- Abre el PR con una descripción clara: qué hace, por qué, cómo probarlo.
- Vincula el issue o ticket relacionado (`Closes #42`).
- Mantén los PRs pequeños y enfocados (idealmente < 400 líneas).
- Responde los comentarios con respeto y en tiempo razonable.
- No merges tú mismo sin revisión (salvo proyectos personales).

### Como reviewer

- Revisa el propósito antes de los detalles de código.
- Distingue entre bloqueante (`BLOQUEANTE: esto rompe X`) y sugerencia (`Sugerencia: podrías simplificarlo así`).
- Aprueba cuando el código es correcto, no cuando es perfecto.
- Sé constructivo: critica el código, no a la persona.

---

## 8. Tags y Versiones

Usa tags para marcar versiones de producción. Sigue [Semantic Versioning](https://semver.org/lang/es/):

```
MAJOR.MINOR.PATCH
  │      │     └─ Bug fixes sin breaking changes
  │      └─────── Nueva funcionalidad compatible
  └────────────── Cambios que rompen compatibilidad
```

```bash
# Crear tag anotado (recomendado para releases)
git tag -a v1.2.0 -m "Release v1.2.0: soporte multi-idioma"

# Publicar el tag
git push origin v1.2.0

# Ver todos los tags
git tag -l
```

---

## 9. Comandos para Situaciones Comunes

### Deshacer el último commit (sin perder cambios)
```bash
git reset --soft HEAD~1
```

### Quitar un archivo del staging sin borrar cambios
```bash
git restore --staged archivo.js
```

### Guardar trabajo en progreso temporalmente
```bash
git stash                        # guarda
git stash pop                    # recupera
git stash list                   # ver todos
```

### Ver quién escribió cada línea
```bash
git blame archivo.js
```

### Buscar en qué commit se introdujo un bug
```bash
git bisect start
git bisect bad                   # el commit actual tiene el bug
git bisect good v1.0.0           # este tag funcionaba bien
# Git hace checkout automático, tú marcas good/bad hasta encontrar el commit culpable
```

### Aplicar un commit específico a otra rama
```bash
git cherry-pick <hash-del-commit>
```

---

## 10. Seguridad y Secretos

- **Nunca subas credenciales, tokens ni contraseñas al repositorio.**
- Si accidentalmente lo hiciste, no basta con borrarlo en otro commit: el historial lo conserva.
- Usa herramientas como `git-secrets` o `gitleaks` para prevenir esto automáticamente.
- Rota inmediatamente cualquier credencial expuesta en el historial.
- Usa variables de entorno y archivos `.env` (ignorados con `.gitignore`).

---

## 11. Resumen Rápido: Lo que SÍ y lo que NO

| ✅ Hacer                                        | ❌ Evitar                                      |
|------------------------------------------------|------------------------------------------------|
| Commits pequeños y frecuentes                  | Un commit enorme al final del día              |
| Nombres de rama descriptivos                   | Ramas llamadas `prueba`, `fix2`, `final`       |
| Pull antes de empezar a trabajar               | Trabajar días sin sincronizar                  |
| Mensajes de commit claros                      | `git commit -m "wip"` o `"arreglos"`           |
| Revisar con `git diff` antes de commitear      | `git add .` sin revisar qué se agrega          |
| `.gitignore` bien configurado desde el inicio  | Subir `node_modules`, `.env`, binarios         |
| PRs pequeños y enfocados                       | PRs de 2000+ líneas con múltiples features     |
| Tags para cada versión de producción           | Deployar sin marcar la versión en el historial |

---

*Basado en Conventional Commits, Git Flow, y prácticas comunes en equipos de desarrollo profesional.*
