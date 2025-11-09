# 🎬 Movierama

Movierama is a social movie sharing platform where users can share their favorite movies, like or hate movies, and explore submissions from other users.

---

## 🏗 Project Overview

- Users can **sign up** or **log in**.
- Registered users can **add movies** with a title, description, and date.
- Users can **vote** (like or hate) on movies submitted by others.  
  - Votes are limited to one per movie per user.  
  - Users can change or retract their votes.
  - Users **cannot vote** for movies they submitted.
- Movie listings display:
  - Title
  - Description
  - Name of the submitting user (clickable to filter by user)
  - Date of publication
  - Number of likes
  - Number of hates
- Movies can be **sorted** by likes, hates, or publication date.

---

## 🧰 Technology Stack

| Layer        | Technology                        |
| ------------ | --------------------------------- |
| Backend      | Rust, actix, sqlx                 |
| Database     | PostgreSQL                        |
| Frontend     | Vue.js, Bun                       |
| Containerization | Docker, Docker Compose        |

---

## 🏠 Running Movierama

Movierama is fully containerized with Docker. The backend, frontend, and database run in separate containers on a shared network.

### Prerequisites

- Docker
- Docker Compose 

---

### Start the Application

```bash
docker compose up --build
```

### Stop the Application

```bash
docker compose down
```

### Open in Browser

After starting, open http://localhost:5173.
