#!/bin/bash

set -e

# -------------------------------------------------------------------
# CONFIGURATION
# -------------------------------------------------------------------
 
DB_HOST="localhost"
DB_PORT="5432"
DB_NAME="movierama_db"
DB_USER="postgres"
DB_PASSWORD="postgres"

export PGPASSWORD="$DB_PASSWORD"

echo "🔎 Checking if mock data already exists..."

MOVIE_COUNT=$(psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -t -c "SELECT COUNT(*) FROM movies;")
MOVIE_COUNT=$(echo "$MOVIE_COUNT" | xargs)

if [ "$MOVIE_COUNT" -gt 0 ]; then
  echo "✅ Mock data already present. Skipping initialization."
  exit 0
fi

echo "🚀 Initializing mock data..."

# -------------------------------------------------------------------
# USERS
# -------------------------------------------------------------------
echo "👤 Creating users..."

for i in $(seq 1 5); do
  USERNAME="user$i"
  EMAIL="user$i@mail.com"
  PASSWORD='$argon2id$v=19$m=19456,t=2,p=1$lv/Keg0ipUKFfrBFbQXn+w$QGaGR+AdQ9e8jF9V9n3+9FOIf2xvmNc0Kr8sIXMpZf0'  # dummy bcrypt hash

  psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -c "
    INSERT INTO users (username, email, password)
    VALUES ('$USERNAME', '$EMAIL', '$PASSWORD')
    ON CONFLICT DO NOTHING;
  "
done


# -------------------------------------------------------------------
# MOVIES
# -------------------------------------------------------------------
echo "🎬 Creating movies..."

for i in $(seq 1 30); do
  TITLE="Movie $i"
  DESCRIPTION="Description for Movie $i"
  RANDOM_DAYS=$((RANDOM % 10))
  DATE_ADDED=$(date -u -Iseconds -d "-$RANDOM_DAYS days")

  # Select a random user ID
  RANDOM_USER=$(psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -t -c "
    SELECT id FROM users ORDER BY random() LIMIT 1;
  " | xargs)

  psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -c "
    INSERT INTO movies (title, description, date_added, user_id)
    VALUES (
      '$TITLE',
      '$DESCRIPTION',
      '$DATE_ADDED',
      $RANDOM_USER
    );
  "
done


# -------------------------------------------------------------------
# VOTES
# -------------------------------------------------------------------
echo "👍👎 Creating votes..."

for i in $(seq 1 100); do

  RANDOM_TYPE=$((RANDOM % 2))
  if [ "$RANDOM_TYPE" -eq 0 ]; then
    VOTE_TYPE="LIKE"
  else
    VOTE_TYPE="HATE"
  fi

  RANDOM_USER=$(psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -t -c "
    SELECT id FROM users ORDER BY random() LIMIT 1;
  " | xargs)

  RANDOM_MOVIE=$(psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -t -c "
    SELECT id FROM movies ORDER BY random() LIMIT 1;
  " | xargs)

  # Insert vote, ignoring duplicates (same user, same movie, same vote)
  psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -c "
    INSERT INTO votes (movie_id, user_id, type)
    VALUES ($RANDOM_MOVIE, $RANDOM_USER, '$VOTE_TYPE')
    ON CONFLICT DO NOTHING;
  "

done

echo "✅ Mock data initialized successfully!"
