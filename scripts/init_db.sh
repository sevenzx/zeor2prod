#!/usr/bin/env bash
set -x
set -eo pipefail

# 检查是否安装了 psql
if ! command -v psql &> /dev/null; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

# 检查是否安装了 sqlx
if ! command -v sqlx &> /dev/null; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 " cargo install sqlx-cli --no-default-features --features native-tls,postgres"
  exit 1
fi

# 设置默认的数据库环境变量
DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_HOST="${POSTGRES_HOST:=localhost}"

# 如果没有设置 SKIP_DOCKER 环境变量，就运行 Docker 容器
if [[ -z "${SKIP_DOCKER}" ]]; then
  docker run \
    -e POSTGRES_USER="${DB_USER}" \
    -e POSTGRES_PASSWORD="${DB_PASSWORD}" \
    -e POSTGRES_DB="${DB_NAME}" \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000
fi

# 等待 PostgresSQL 准备好接受连接
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q' &> /dev/null; do
  echo >&2 "Postgres is still unavailable - sleeping"
  sleep 1
done

echo >&2 "Postgres is up and running on port ${DB_PORT} - running migrations now!"

# 设置数据库连接地址
export DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"

# 执行数据库创建和迁移
sqlx database create
sqlx migrate run

echo >&2 "Postgres has been migrated, ready to go!"
