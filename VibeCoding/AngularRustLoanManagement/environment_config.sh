# .env (for Rust API)
DATABASE_URL=postgresql://username:password@localhost:5432/loan_db
BIND_ADDRESS=127.0.0.1:8080
RUST_LOG=info

# docker-compose.yml
version: '3.8'

services:
  postgres:
    image: postgres:15
    environment:
      POSTGRES_DB: loan_db
      POSTGRES_USER: loan_user
      POSTGRES_PASSWORD: loan_password
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./init.sql:/docker-entrypoint-initdb.d/init.sql
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U loan_user -d loan_db"]
      interval: 10s
      timeout: 5s
      retries: 5

  loan-api:
    build: .
    environment:
      DATABASE_URL: postgresql://loan_user:loan_password@postgres:5432/loan_db
      BIND_ADDRESS: 0.0.0.0:8080
      RUST_LOG: info
    ports:
      - "8080:8080"
    depends_on:
      postgres:
        condition: service_healthy
    restart: unless-stopped

volumes:
  postgres_data:

# Dockerfile (for Rust API)
FROM rust:1.75 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release
RUN rm src/main.rs

# Copy source code
COPY src ./src
COPY migrations ./migrations

# Build application
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/loan-api /app/loan-api
COPY --from=builder /app/migrations /app/migrations

EXPOSE 8080

CMD ["./loan-api"]

# package.json (for Angular frontend)
{
  "name": "loan-search-frontend",
  "version": "0.0.0",
  "scripts": {
    "ng": "ng",
    "start": "ng serve",
    "build": "ng build",
    "watch": "ng build --watch --configuration development",
    "test": "ng test",
    "lint": "ng lint",
    "e2e": "ng e2e"
  },
  "private": true,
  "dependencies": {
    "@angular/animations": "^17.0.0",
    "@angular/common": "^17.0.0",
    "@angular/compiler": "^17.0.0",
    "@angular/core": "^17.0.0",
    "@angular/forms": "^17.0.0",
    "@angular/platform-browser": "^17.0.0",
    "@angular/platform-browser-dynamic": "^17.0.0",
    "@angular/router": "^17.0.0",
    "rxjs": "~7.8.0",
    "tslib": "^2.3.0",
    "zone.js": "~0.14.0"
  },
  "devDependencies": {
    "@angular-devkit/build-angular": "^17.0.0",
    "@angular/cli": "^17.0.0",
    "@angular/compiler-cli": "^17.0.0",
    "@types/jasmine": "~5.1.0",
    "@types/node": "^18.18.0",
    "jasmine-core": "~5.1.0",
    "karma": "~6.4.0",
    "karma-chrome-headless": "~3.1.0",
    "karma-coverage": "~2.2.0",
    "karma-jasmine": "~5.1.0",
    "karma-jasmine-html-reporter": "~2.1.0",
    "typescript": "~5.2.0"
  }
}

# angular.json (key parts)
{
  "$schema": "./node_modules/@angular/cli/lib/config/schema.json",
  "version": 1,
  "newProjectRoot": "projects",
  "projects": {
    "loan-search-frontend": {
      "projectType": "application",
      "schematics": {},
      "root": "",
      "sourceRoot": "src",
      "prefix": "app",
      "architect": {
        "build": {
          "builder": "@angular-devkit/build-angular:browser",
          "options": {
            "outputPath": "dist/loan-search-frontend",
            "index": "src/index.html",
            "main": "src/main.ts",
            "polyfills": [
              "zone.js"
            ],
            "tsConfig": "tsconfig.app.json",
            "assets": [
              "src/favicon.ico",
              "src/assets"
            ],
            "styles": [
              "src/styles.css"
            ],
            "scripts": []
          }
        },
        "serve": {
          "builder": "@angular-devkit/build-angular:dev-server",
          "configurations": {
            "production": {
              "buildTarget": "loan-search-frontend:build:production"
            },
            "development": {
              "buildTarget": "loan-search-frontend:build:development"
            }
          },
          "defaultConfiguration": "development"
        }
      }
    }
  }
}

# Setup scripts
# setup.sh
#!/bin/bash
echo "Setting up Loan Search Application..."

# Create directories
mkdir -p loan-search-app/{frontend,backend,database}
cd loan-search-app

# Backend setup
echo "Setting up Rust backend..."
cd backend
cargo init --name loan-api
# Copy Rust files here

# Install sqlx-cli for migrations
cargo install sqlx-cli --features postgres

# Frontend setup
echo "Setting up Angular frontend..."
cd ../frontend
npm install -g @angular/cli
ng new loan-search-frontend --routing --style=css
cd loan-search-frontend
# Copy Angular files here

# Database setup
echo "Setting up database..."
cd ../../database
# Copy migration files here

echo "Setup complete! Run 'docker-compose up' to start the application."