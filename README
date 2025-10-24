# Expense Tracker Application

> A full-stack expense tracking application built entirely in **Rust**, demonstrating modern web development with type-safe backend and frontend, comprehensive testing, and automated CI/CD workflows.

## 📋 Table of Contents

- [Overview](#overview)
- [Why Rust for Full-Stack?](#why-rust-for-full-stack)
- [Architecture](#architecture)
- [Project Structure](#project-structure)
- [Tech Stack](#tech-stack)
- [CI/CD Pipeline](#cicd-pipeline)
- [Getting Started](#getting-started)
- [Testing](#testing)
- [Docker Deployment](#docker-deployment)
- [API Documentation](#api-documentation)
- [Contributing](#contributing)

## 🎯 Overview

This project showcases a complete expense tracking application where **both frontend and backend are written in Rust**, leveraging:

- **Backend**: RESTful API with Axum web framework, SQLx for async database operations, and comprehensive testing
- **Frontend**: WebAssembly-powered reactive UI using Dioxus framework with component-based architecture
- **CI/CD**: Multi-platform GitHub Actions workflows with automated testing, linting, and Docker image builds
- **Deployment**: Docker containerization for both backend and frontend services

### Key Features

- Add and track expenses with amount and category
- View all expenses with automatic highest expense identification
- Real-time reactive UI updates compiled to WebAssembly
- Type-safe data validation across frontend and backend
- Persistent SQLite storage with async operations
- Cross-platform support (Linux, macOS, Windows)

## 🦀 Why Rust for Full-Stack?

This project demonstrates the advantages of using Rust across the entire stack:

1. **Shared Type Safety**: Both frontend and backend share the same data models, eliminating type mismatches
2. **Performance**: Backend runs natively with zero-cost abstractions; frontend compiles to WebAssembly for near-native browser performance
3. **Memory Safety**: No null pointer exceptions or data races, guaranteed at compile time
4. **Code Reuse**: Business logic, validation rules, and data models are shared between frontend and backend
5. **Single Toolchain**: One language, one build system (Cargo), consistent testing and linting across the entire codebase

## 🏗 Architecture

### System Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     Client Browser                          │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Dioxus Frontend (Rust → WebAssembly)                 │  │
│  │  - Reactive UI Components                             │  │
│  │  - Client-side Routing                                │  │
│  │  - HTTP Client (Reqwest)                              │  │
│  └────────────────────┬──────────────────────────────────┘  │
└─────────────────────────┼─────────────────────────────────────┘
                          │ HTTP/JSON
                          │
┌─────────────────────────▼─────────────────────────────────────┐
│                  Axum Backend (Rust Native)                   │
│  ┌─────────────────────────────────────────────────────────┐  │
│  │  HTTP Handlers Layer                                    │  │
│  │  - Request validation                                   │  │
│  │  - Response serialization                               │  │
│  │  - CORS middleware                                      │  │
│  └────────────────────┬────────────────────────────────────┘  │
│  ┌────────────────────▼────────────────────────────────────┐  │
│  │  Service Layer                                          │  │
│  │  - Business logic                                       │  │
│  │  - Data validation (validator crate)                   │  │
│  │  - Transaction handling                                │  │
│  └────────────────────┬────────────────────────────────────┘  │
│  ┌────────────────────▼────────────────────────────────────┐  │
│  │  Database Layer (SQLx + SQLite)                         │  │
│  │  - Async database operations                            │  │
│  │  - Connection pooling                                   │  │
│  │  - Compile-time query verification                      │  │
│  └─────────────────────────────────────────────────────────┘  │
└───────────────────────────────────────────────────────────────┘
```

### Backend Architecture (Rust Native)

The backend follows a layered architecture pattern:

- **Handlers** (`src/handlers/`): HTTP endpoint handlers, request/response mapping
- **Services** (`src/services/`): Core business logic, data processing
- **Models** (`src/models/`): Data structures with validation rules
- **Database** (`src/database.rs`): Database connection pooling and table initialization
- **Error Handling** (`src/error.rs`): Centralized error types and HTTP error mapping

### Frontend Architecture (Rust → WebAssembly)

The frontend is a reactive single-page application (SPA):

- **Pages** (`frontend/src/pages/`): Top-level views (Home, Add Expense, Expense List)
- **Components** (`frontend/src/components/`): Reusable UI elements (forms, layouts, lists)
- **Services** (`frontend/src/services/`): API communication layer
- **Models** (`frontend/src/models/`): Frontend data structures (shared types with backend)
- **Router**: Client-side routing with Dioxus Router

## 📁 Project Structure

```
expense-tracker/
├── src/                              # 🦀 Backend (Rust Native)
│   ├── main.rs                       # Application entry, Axum server setup
│   ├── database.rs                   # SQLx connection pool, table creation
│   ├── error.rs                      # Custom error types, HTTP error mapping
│   │
│   ├── models/                       # Data models with validation
│   │   ├── mod.rs
│   │   └── expense.rs                # Expense struct, CreateExpenseRequest
│   │
│   ├── handlers/                     # HTTP request handlers
│   │   ├── mod.rs
│   │   └── expenses.rs               # POST /expenses, GET /expenses, GET /expenses/highest
│   │
│   └── services/                     # Business logic layer
│       ├── mod.rs
│       └── expense_service.rs        # Database operations, expense queries
│
├── frontend/                         # 🌐 Frontend (Rust → WebAssembly)
│   ├── src/
│   │   ├── main.rs                   # Dioxus app entry, router configuration
│   │   │
│   │   ├── components/               # Reusable UI components
│   │   │   ├── mod.rs
│   │   │   ├── layout.rs             # Navigation, page layout
│   │   │   ├── expense_form.rs       # Form for adding expenses
│   │   │   └── expense_list.rs       # Expense list display component
│   │   │
│   │   ├── pages/                    # Top-level page components
│   │   │   ├── mod.rs
│   │   │   ├── home.rs               # Dashboard with highest expense
│   │   │   ├── add_expense.rs        # Add expense page
│   │   │   └── expense_list.rs       # All expenses page
│   │   │
│   │   ├── models/                   # Frontend data structures
│   │   │   ├── mod.rs
│   │   │   └── expense.rs            # Expense types (mirrors backend)
│   │   │
│   │   └── services/                 # API communication
│   │       ├── mod.rs
│   │       └── expense_service.rs    # HTTP client, API calls
│   │
│   ├── Cargo.toml                    # Frontend dependencies
│   └── Dockerfile                    # Frontend Docker build (Rust → WASM → Nginx)
│
├── .github/workflows/                # ⚙️ CI/CD Pipeline
│   └── ci.yml                        # Multi-stage GitHub Actions workflow
│
├── migrations/                       # Database migrations (SQLx)
│
├── tests/                            # Integration tests
│
├── Cargo.toml                        # Backend dependencies & metadata
├── Cargo.lock                        # Dependency lock file
├── Dockerfile                        # Backend Docker build (multi-stage)
├── .dockerignore                     # Docker build exclusions
└── bacon.toml                        # Continuous testing configuration
```

### Key Directories Explained

#### Backend (`src/`)
- **Entry Point**: [`main.rs`](src/main.rs) initializes the Axum web server, sets up routes, and configures middleware (CORS)
- **Database Layer**: [`database.rs`](src/database.rs) manages SQLx connection pooling and schema initialization
- **Models**: Type-safe data structures with `serde` serialization and validation rules
- **Handlers**: Thin layer that receives HTTP requests, calls services, and returns JSON responses
- **Services**: Contains all business logic, database queries, and transaction management

#### Frontend (`frontend/src/`)
- **Entry Point**: [`main.rs`](frontend/src/main.rs) launches the Dioxus application and configures client-side routing
- **Pages**: Full-page components corresponding to routes (`/`, `/add`, `/list`)
- **Components**: Reusable UI building blocks (forms, layouts, lists)
- **Services**: HTTP client using `reqwest` to communicate with the backend API
- **Models**: TypeScript-like type definitions that match backend models for type safety

#### CI/CD (`.github/workflows/`)
- **Automated Workflows**: Multi-platform testing, building, and Docker image publishing
- **Quality Gates**: Code formatting, Clippy linting, and comprehensive test execution

## ⚙️ GitHub Actions CI/CD Integration

This project demonstrates how **GitHub Actions integrates seamlessly with Rust projects**, leveraging Cargo's built-in tooling and the Rust ecosystem to create a robust, automated testing and deployment pipeline.

### How GitHub Actions Works with Rust

GitHub Actions workflows are triggered on every push and pull request to the master branch. The workflow uses Rust's native toolchain and Cargo commands to validate code quality, run tests, and build deployment artifacts across multiple platforms.

### Pipeline Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     GitHub Actions Workflow                     │
│                  Triggers: Push/PR to master                    │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ├─────────────────────────────────┐
                              │                                 │
┌─────────────────────────────▼───────┐      ┌─────────────────▼─────────────┐
│    Job 1: Backend Testing (3x)      │      │  Job 2: Frontend Build (3x)   │
│    Platforms: Ubuntu, macOS, Win    │      │  Platforms: Ubuntu, macOS, Win │
├─────────────────────────────────────┤      ├───────────────────────────────┤
│ 1. Checkout code                    │      │ 1. Checkout code              │
│ 2. Setup Rust toolchain             │      │ 2. Setup Rust + WASM target   │
│ 3. Cache dependencies               │      │ 3. Cache dependencies         │
│ 4. Check code formatting (fmt)      │      │ 4. Install Dioxus CLI         │
│ 5. Run Clippy linting               │      │ 5. Check formatting           │
│ 6. Build backend                    │      │ 6. Run Clippy linting         │
│ 7. Run all tests (verbose)          │      │ 7. Build WASM frontend        │
└─────────────────────────────────────┘      │ 8. Run frontend tests         │
                              │               └───────────────────────────────┘
                              │                                │
                              └────────────┬───────────────────┘
                                           │
                       ┌───────────────────▼──────────────────┐
                       │  Job 3: Docker Build & Push          │
                       │  Condition: master branch only       │
                       ├──────────────────────────────────────┤
                       │ 1. Setup Docker Buildx               │
                       │ 2. Login to GitHub Container Registry│
                       │ 3. Build backend image (multi-stage) │
                       │ 4. Push to ghcr.io                   │
                       │ 5. Build frontend image              │
                       │ 6. Push to ghcr.io                   │
                       └──────────────────────────────────────┘
```

### Rust-Specific GitHub Actions Integration

The workflow leverages Rust's ecosystem tools through GitHub Actions:

#### 1. Rust Toolchain Setup (`dtolnay/rust-toolchain@stable`)
```yaml
- name: Setup Rust
  uses: dtolnay/rust-toolchain@stable
  with:
    toolchain: stable
    components: rustfmt,clippy
```
- Installs the stable Rust compiler
- Adds `rustfmt` (formatter) and `clippy` (linter) components
- Ensures consistent Rust version across all CI environments

#### 2. Cargo Dependency Caching
```yaml
- name: Cache dependencies
  uses: actions/cache@v3
  with:
    path: |
      ~/.cargo/bin/
      ~/.cargo/registry/index/
      ~/.cargo/registry/cache/
      ~/.cargo/git/db/
      target/
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
```
**Benefits:**
- **5-10x faster builds** by caching compiled dependencies
- Cache invalidated only when `Cargo.lock` changes
- Separate caches per OS (Linux, macOS, Windows)
- Dramatically reduces CI run time from ~10 minutes to ~2 minutes

#### 3. Code Quality Checks with Cargo Commands

**Formatting Check** (`cargo fmt --check`):
```yaml
- name: Check formatting
  run: |
    cargo fmt
    cargo fmt -- --check
```
- Ensures code follows Rust style guidelines
- Fails CI if code is not properly formatted
- Zero configuration needed (uses Rust defaults)

**Linting with Clippy** (`cargo clippy`):
```yaml
- name: Run clippy
  run: cargo clippy -- -D warnings
```
- Runs Clippy, Rust's official linter
- `-D warnings`: Treats all warnings as errors
- Catches common mistakes, performance issues, and non-idiomatic code
- Over 500+ lint rules covering safety, correctness, and style

#### 4. Building and Testing

**Backend Build**:
```yaml
- name: Build
  run: cargo build --verbose
```
- Compiles all Rust code
- Verbose output shows dependency compilation
- Validates that code compiles on all platforms

**Backend Tests**:
```yaml
- name: Run tests
  run: cargo test --verbose
```
- Executes all unit and integration tests
- Verbose mode shows each test result
- Tests run in parallel by default (Cargo feature)

**Frontend Build** (Rust to WebAssembly):
```yaml
- name: Install Dioxus CLI
  run: |
    cargo install cargo-binstall
    cargo binstall dioxus-cli --no-confirm

- name: Build frontend
  run: dx build --release
```
- Installs Dioxus CLI using `cargo-binstall` (fast binary installation)
- `dx build` compiles Rust to WebAssembly
- Creates optimized WASM bundle for production

### Pipeline Stages

#### Stage 1: Backend Testing (Parallel across 3 OS)
**Cross-platform validation using Rust's portability:**

| Step | Cargo Command | Purpose |
|------|---------------|---------|
| Format Check | `cargo fmt --check` | Enforce consistent code style |
| Linting | `cargo clippy -- -D warnings` | Catch bugs and non-idiomatic code |
| Build | `cargo build --verbose` | Compile and verify platform compatibility |
| Test | `cargo test --verbose` | Run unit + integration tests |

**What Gets Tested:**
- Database layer (SQLx connection pooling, schema creation)
- Model validation (amount > 0, category length constraints)
- Service layer business logic
- HTTP handler integration tests

#### Stage 2: Frontend Build (Parallel across 3 OS)
**Rust to WebAssembly compilation:**

| Step | Command | Purpose |
|------|---------|---------|
| Install Dioxus | `cargo binstall dioxus-cli` | Fast CLI installation |
| Format Check | `cargo fmt --check` | Enforce code style |
| Linting | `cargo clippy -- -D warnings` | Frontend code quality |
| WASM Build | `dx build --release` | Compile Rust → WebAssembly |
| Test | `cargo test` | Frontend component tests |

#### Stage 3: Docker Image Publishing (master only)
Builds production Docker images with multi-stage builds, leveraging Cargo's `--release` flag for optimized binaries.

### Key CI/CD Features

#### Cross-Platform Matrix Strategy
```yaml
strategy:
  matrix:
    os: [ubuntu-latest, windows-latest, macos-latest]
```
- All jobs run in parallel across 3 operating systems
- Ensures Rust code is truly portable
- Catches platform-specific issues early

#### Fail-Fast Quality Gates
The pipeline enforces quality in order:
1. **First**: Formatting and linting (fast, fail early)
2. **Then**: Build (catches compilation errors)
3. **Finally**: Tests (most time-consuming)

This approach saves CI time by failing fast on style issues before running expensive tests.

#### Separation of Concerns
- **Backend Job**: Validates backend Rust code independently
- **Frontend Job**: Validates frontend Rust/WASM code independently
- **Docker Job**: Only runs after both pass, only on master branch

#### Security Best Practices
- Uses `GITHUB_TOKEN` (automatically provided, minimal permissions)
- Docker images run as non-root user
- No secrets exposed in logs

### Viewing CI Results

Navigate to the **Actions** tab in your repository to see:
- ✅ All tests passing across Ubuntu, macOS, and Windows
- ❌ Specific failures with detailed logs
- 📦 Published Docker images on successful master builds
- ⏱️ Build times and performance metrics

## 🚀 Getting Started

### Prerequisites

- **Rust** 1.90 or later ([rustup.rs](https://rustup.rs))
- **Cargo** (included with Rust)
- **Dioxus CLI** (for frontend development)
- **SQLite3** (usually pre-installed on macOS/Linux)
- **Docker** (optional, for containerized deployment)

### Installation

#### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/expense-tracker.git
cd expense-tracker
```

#### 2. Install Dioxus CLI

```bash
# Fast installation with cargo-binstall (recommended)
cargo install cargo-binstall
cargo binstall dioxus-cli --no-confirm

# Or standard installation
cargo install dioxus-cli
```

#### 3. Build the Backend

```bash
cargo build --release
```

#### 4. Build the Frontend

```bash
cd frontend
dx build --release
cd ..
```

### Running the Application

#### Start the Backend Server

```bash
# Development mode (from project root)
cargo run

# Or production mode
cargo run --release
```

The backend API will be available at **http://localhost:3000**

#### Start the Frontend Development Server

```bash
# In a new terminal, from the frontend directory
cd frontend
dx serve

# Or for production build
dx serve --release
```

The frontend will be available at **http://localhost:8080** (or the port shown in terminal)

## 🧪 Testing

The project includes comprehensive testing at multiple levels: unit tests, integration tests, and code quality checks.

### Test Coverage Overview

```
Backend Testing:
├── Unit Tests (embedded in source files)
│   ├── Database layer (connection, schema, queries)
│   ├── Model validation (data integrity, boundaries)
│   └── Service logic (business rules, edge cases)
│
├── Integration Tests (tests/ directory)
│   └── API endpoint tests (HTTP handlers, error responses)
│
└── Code Quality
    ├── cargo fmt (formatting)
    └── cargo clippy (linting, best practices)

Frontend Testing:
├── Component tests
├── Service layer tests
└── Code quality checks
```

### Running Tests

#### Backend Tests

```bash
# Run all backend tests (from project root)
cargo test

# Run with output visible
cargo test -- --nocapture

# Run specific test categories
cargo test --lib              # Unit tests only
cargo test --test '*'         # Integration tests only
cargo test expense_service    # Specific module
```

**Backend Test Coverage:**
- **Database Layer**: Connection pooling, table creation, query execution
- **Models**: Validation rules (amount > 0, category length 1-50 chars)
- **Services**: Business logic (add expense, get all, find highest, empty state handling)
- **Handlers**: HTTP endpoint integration (status codes, JSON responses)

#### Frontend Tests

```bash
cd frontend
cargo test
```

### Code Quality Tools

#### Format Code

```bash
# Format backend
cargo fmt

# Format frontend
cd frontend && cargo fmt

# Check formatting (CI mode)
cargo fmt -- --check
```

#### Lint with Clippy

Clippy catches common mistakes and enforces Rust best practices:

```bash
# Backend linting (fail on warnings)
cargo clippy -- -D warnings

# Frontend linting
cd frontend && cargo clippy -- -D warnings
```

## 🥓 Bacon: Continuous Integration During Development

While GitHub Actions provides CI/CD in the cloud, **[Bacon](https://github.com/Canop/bacon)** brings continuous integration directly to your local development environment. It's a background job runner for Rust that watches your files and automatically runs checks as you code.

### What is Bacon?

Bacon is a lightweight, terminal-based tool that runs Cargo commands continuously in the background. Think of it as "GitHub Actions for your local machine" – it provides instant feedback by automatically running the same checks (tests, clippy, formatting) that GitHub Actions will run later.

### Why Use Bacon?

**Traditional Development Cycle:**
```
Write code → Save → Switch to terminal → Run cargo test → Wait → See results → Switch back to editor
```

**With Bacon:**
```
Write code → Save → (Bacon automatically runs tests in background) → See results immediately in split terminal
```

**Benefits:**
- **Instant Feedback**: See test results within seconds of saving
- **No Context Switching**: Keep focus on your code while Bacon runs in background
- **Same Checks as CI**: Run the exact same commands that GitHub Actions will run
- **Fail Fast**: Catch errors before committing, before CI even runs
- **Efficient**: Only re-runs checks when files change

### Bacon Configuration for This Project

The project includes a [`bacon.toml`](bacon.toml) configuration file that defines custom jobs:

```toml
# bacon.toml
[jobs]

[jobs.test]
command = ["cargo", "test", "--color", "always"]
need_stdout = true

[jobs.clippy]
command = ["cargo", "clippy", "--color", "always", "--", "-D", "warnings"]
need_stdout = false

[jobs.fmt]
command = ["cargo", "fmt", "--check"]
need_stdout = false

[jobs.all]
command = ["cargo", "test"]
need_stdout = true
watch = ["src/**", "tests/**", "Cargo.toml"]
```

### How to Use Bacon

#### 1. Install Bacon

```bash
cargo install bacon
```

#### 2. Run Bacon in Your Project

**Run continuous testing** (most common during development):
```bash
bacon test
```
- Watches all source files
- Automatically runs `cargo test` on file changes
- Shows test results in real-time

**Run continuous linting**:
```bash
bacon clippy
```
- Automatically runs `cargo clippy` on file changes
- Shows warnings and errors as you code
- Same checks as GitHub Actions

**Run continuous formatting checks**:
```bash
bacon fmt
```
- Checks code formatting without modifying files
- Shows which files need formatting

**Run all checks**:
```bash
bacon all
```
- Runs comprehensive test suite
- Watches source files and Cargo.toml

#### 3. Development Workflow with Bacon

**Recommended Setup:**
1. Open your editor (VS Code, Neovim, etc.) in one pane
2. Open a terminal with `bacon test` in a split pane
3. Edit code in the editor
4. Save file → Bacon instantly shows test results
5. Fix any failures → Save → See results again

**Example Terminal Output:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Testing expense-tracker v0.1.0
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ test database::tests::test_create_pool ... ok
✅ test models::expense::tests::test_expense_validation ... ok
✅ test services::tests::test_add_expense ... ok
✅ test services::tests::test_get_highest_expense ... ok

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  4 tests passed ✓
  Finished in 0.8s
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### Bacon vs GitHub Actions

| Aspect | Bacon (Local CI) | GitHub Actions (Cloud CI) |
|--------|------------------|---------------------------|
| **When** | During development, before commit | After push/PR to master |
| **Speed** | Instant (runs locally) | 1-3 minutes (includes setup time) |
| **Feedback** | Real-time as you type | After pushing code |
| **Platforms** | Your local OS only | Linux, macOS, Windows (matrix) |
| **Purpose** | Catch errors early, rapid iteration | Final validation, deployment |
| **Cost** | Free (uses your CPU) | Free (GitHub provides runners) |

### Integration with CI/CD

Bacon and GitHub Actions work together:

```
Local Development (Bacon)           Cloud CI/CD (GitHub Actions)
┌─────────────────────────┐        ┌────────────────────────────┐
│ 1. Edit code            │        │ 5. Push to GitHub          │
│ 2. Save file            │───────▶│ 6. Trigger workflow        │
│ 3. Bacon runs tests     │        │ 7. Run on 3 platforms      │
│ 4. Fix issues instantly │        │ 8. Build Docker images     │
└─────────────────────────┘        │ 9. Publish to registry     │
         ▲                          └────────────────────────────┘
         │                                      │
         └──────── All checks pass ─────────────┘
```

**Development Workflow:**
1. **Local**: Write code → Bacon catches issues → Fix immediately
2. **Commit**: All Bacon checks pass → Commit with confidence
3. **Push**: GitHub Actions validates on all platforms → Merges successfully

This two-stage approach ensures:
- **Fast local feedback** (seconds with Bacon)
- **Comprehensive validation** (minutes with GitHub Actions)
- **High confidence** when pushing code (already tested locally)

### Bacon Commands Reference

```bash
# Most common commands
bacon test          # Watch and test (recommended default)
bacon clippy        # Watch and lint
bacon check         # Watch and check compilation (fastest)
bacon run           # Watch and run the application

# Job selection
bacon              # Shows job selector menu
bacon <job-name>   # Run specific job from bacon.toml

# Exit Bacon
Ctrl+C             # Stop Bacon and exit
```

### Tips for Using Bacon Effectively

1. **Keep Bacon Running**: Leave `bacon test` running in a terminal throughout your development session
2. **Fix Red First**: When Bacon shows red (failures), fix those before adding new features
3. **Use Multiple Jobs**: Run `bacon test` in one terminal, `bacon clippy` in another
4. **Trust the Process**: If Bacon passes, GitHub Actions will likely pass too
5. **Fast Iteration**: Use Bacon to rapidly test small changes without waiting for CI

### Manual API Testing

Test the API endpoints directly using `curl` or tools like Postman:

```bash
# Add a new expense
curl -X POST http://localhost:3000/expenses \
  -H "Content-Type: application/json" \
  -d '{"amount": 25.50, "category": "Groceries"}'

# Get all expenses
curl http://localhost:3000/expenses

# Get highest expense
curl http://localhost:3000/expenses/highest
```

### Full Integration Testing

1. **Start Backend**: `cargo run` (runs on port 3000)
2. **Start Frontend**: `cd frontend && dx serve` (runs on port 8080)
3. **Test User Flows**:
   - ✅ Navigate to home page, verify highest expense display
   - ✅ Go to "Add Expense", submit valid expense
   - ✅ View "All Expenses" list, verify new entry appears
   - ✅ Test validation (try negative amount, empty category)
   - ✅ Check browser console for errors

## 📚 API Documentation

### REST API Endpoints

| Method | Endpoint | Description | Request Body | Response | Status Codes |
|--------|----------|-------------|--------------|----------|--------------|
| POST | `/expenses` | Add a new expense | `CreateExpenseRequest` | `Expense` | 201, 400 |
| GET | `/expenses` | Get all expenses | - | `Array<Expense>` | 200, 500 |
| GET | `/expenses/highest` | Get the highest expense | - | `Expense \| null` | 200, 500 |

### Data Models

#### Expense
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "amount": 25.50,
  "category": "Groceries",
  "date": "2025-01-15T12:00:00Z"
}
```

**Fields:**
- `id`: UUID v4, auto-generated
- `amount`: Decimal number (f64)
- `category`: String (1-50 characters)
- `date`: ISO 8601 timestamp, auto-generated

#### CreateExpenseRequest
```json
{
  "amount": 25.50,
  "category": "Groceries"
}
```

### Validation Rules

| Field | Validation | Error Response |
|-------|------------|----------------|
| `amount` | Must be > 0.01 | `400 Bad Request` |
| `category` | Required, 1-50 characters | `400 Bad Request` |

### Example Requests

**Add Expense:**
```bash
curl -X POST http://localhost:3000/expenses \
  -H "Content-Type: application/json" \
  -d '{"amount": 42.99, "category": "Entertainment"}'
```

**Response (201 Created):**
```json
{
  "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "amount": 42.99,
  "category": "Entertainment",
  "date": "2025-01-15T14:30:00Z"
}
```

## 🐳 Docker Deployment

The project uses **multi-stage Docker builds** for optimized, production-ready images.

### Backend Docker Image

**Dockerfile structure:**
1. **Builder stage**: Compiles Rust code to optimized binary
2. **Runtime stage**: Minimal Debian Slim image with only the binary

```bash
# Build backend image
docker build -t expense-tracker-backend .

# Run backend container
docker run -p 3000:3000 expense-tracker-backend

# Or with custom database path
docker run -p 3000:3000 \
  -v $(pwd)/data:/app \
  -e DATABASE_URL=sqlite:./data/expenses.db \
  expense-tracker-backend
```

**Image Details:**
- Base: `rust:1.90` (builder) → `debian:bookworm-slim` (runtime)
- Size: ~50MB (runtime only)
- Security: Runs as non-root user

### Frontend Docker Image

**Dockerfile structure:**
1. **Builder stage**: Compiles Rust to WebAssembly using Dioxus CLI
2. **Runtime stage**: Nginx Alpine serving static WASM files

```bash
# Build frontend image (from frontend directory)
cd frontend
docker build -t expense-tracker-frontend .

# Run frontend container
docker run -p 80:80 expense-tracker-frontend
```

**Image Details:**
- Base: `rust:1.90` (builder) → `nginx:alpine` (runtime)
- Size: ~25MB (runtime only)
- Serves: Compiled WASM + HTML/CSS/JS

### Docker Compose (Full Stack)

Create `docker-compose.yml` in project root:

```yaml
version: '3.8'

services:
  backend:
    build: .
    ports:
      - "3000:3000"
    environment:
      - DATABASE_URL=sqlite:./expenses.db
      - RUST_LOG=info
    volumes:
      - ./data:/app
    restart: unless-stopped

  frontend:
    build: ./frontend
    ports:
      - "80:80"
    depends_on:
      - backend
    restart: unless-stopped
```

**Run the full stack:**
```bash
docker-compose up -d
```

### GitHub Container Registry

Pre-built images are automatically published on master branch:

```bash
# Pull and run pre-built backend
docker pull ghcr.io/yourusername/expence_tracker-backend:latest
docker run -p 3000:3000 ghcr.io/yourusername/expence_tracker-backend:latest

# Pull and run pre-built frontend
docker pull ghcr.io/yourusername/expence_tracker-frontend:latest
docker run -p 80:80 ghcr.io/yourusername/expence_tracker-frontend:latest
```

## 🛠 Development Workflow

### Recommended Development Setup

1. **Editor Setup**: Use an editor with Rust language server (rust-analyzer)
   - VS Code with rust-analyzer extension
   - IntelliJ IDEA with Rust plugin
   - Neovim/Vim with CoC or native LSP

2. **Watch Mode**: Keep `bacon test` running in a terminal for instant feedback

3. **Git Workflow**:
   ```bash
   # Create feature branch
   git checkout -b feature/my-feature

   # Make changes, run tests
   cargo test

   # Format and lint before commit
   cargo fmt
   cargo clippy -- -D warnings

   # Commit with clear message
   git commit -m "Add: feature description"

   # Push to trigger CI
   git push origin feature/my-feature
   ```

4. **CI Simulation**: Run the same checks as CI before pushing:
   ```bash
   # Backend checks
   cargo fmt -- --check && \
   cargo clippy -- -D warnings && \
   cargo build --verbose && \
   cargo test --verbose

   # Frontend checks
   cd frontend && \
   cargo fmt -- --check && \
   cargo clippy -- -D warnings && \
   dx build --release && \
   cargo test
   ```

### Development Tips

- **Fast Iteration**: Use `cargo check` for quick compile checks without building binaries
- **Selective Testing**: Run specific tests with `cargo test test_name`
- **Database Reset**: Delete `expenses.db` to start with a fresh database
- **Hot Reload**: Dioxus dev server (`dx serve`) supports hot reloading for frontend changes
- **Debug Logging**: Set `RUST_LOG=debug` to see detailed logs

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes**
4. **Run tests and checks**:
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   cargo test
   cd frontend && cargo test
   ```
5. **Commit with clear messages**: `git commit -m 'Add: amazing feature'`
6. **Push to your fork**: `git push origin feature/amazing-feature`
7. **Open a Pull Request**

### Contribution Guidelines

- Follow Rust naming conventions and idioms
- Add tests for new functionality
- Update documentation (including this README) if needed
- Ensure CI passes before requesting review
- Keep commits atomic and well-described

## 📝 Environment Variables

### Backend Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | `sqlite:./expenses.db` | SQLite database file path |
| `RUST_LOG` | `info` | Logging level (`trace`, `debug`, `info`, `warn`, `error`) |

### Frontend Environment Variables

The frontend is compiled to WebAssembly, so runtime environment variables are not supported. Configuration is done at build time in [`frontend/src/services/expense_service.rs`](frontend/src/services/expense_service.rs).

## 🐛 Troubleshooting

### Common Issues

#### Port Already in Use
```bash
# Backend: Change port in src/main.rs
let addr = SocketAddr::from(([127, 0, 0, 1], 3001));  # Change 3000 → 3001

# Frontend: Use custom port
dx serve --port 8081
```

#### Database Connection Failed
```bash
# Check SQLite installation
sqlite3 --version

# Check database file permissions
ls -la expenses.db

# Create database directory if needed
mkdir -p data
DATABASE_URL=sqlite:./data/expenses.db cargo run
```

#### Frontend Can't Connect to Backend
1. Verify backend is running: `curl http://localhost:3000/expenses`
2. Check browser console for CORS errors
3. Update API base URL in [`frontend/src/services/expense_service.rs`](frontend/src/services/expense_service.rs)

#### Tests Failing
```bash
# Clean and rebuild
cargo clean
cargo build

# Run tests with output
cargo test -- --nocapture

# Check specific test
cargo test test_name -- --nocapture
```

#### Dioxus CLI Installation Fails
```bash
# Try cargo-binstall first (faster)
cargo install cargo-binstall
cargo binstall dioxus-cli --no-confirm

# If that fails, build from source (slower)
cargo install dioxus-cli
```

#### WASM Build Errors
```bash
# Ensure WASM target is installed
rustup target add wasm32-unknown-unknown

# Clear frontend cache
cd frontend
rm -rf target
dx build --release
```

## 📄 License

This project is open source and available for educational purposes. Please check the repository for specific license details.

## 🙏 Acknowledgments

This project is built with amazing Rust ecosystem tools:

- **[Axum](https://github.com/tokio-rs/axum)**: Fast, ergonomic web framework by the Tokio team
- **[Dioxus](https://dioxuslabs.com/)**: Elegant React-like UI framework for Rust
- **[SQLx](https://github.com/launchbadge/sqlx)**: Async SQL toolkit with compile-time query verification
- **[Tokio](https://tokio.rs/)**: Industry-leading async runtime for Rust
- **Rust Community**: For excellent documentation, tools, and ecosystem

## 📖 Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/): Learn Rust fundamentals
- [Axum Documentation](https://docs.rs/axum/latest/axum/): Backend framework guide
- [Dioxus Documentation](https://dioxuslabs.com/learn/0.5/): Frontend framework guide
- [SQLx Guide](https://github.com/launchbadge/sqlx#readme): Database toolkit documentation
- [WebAssembly](https://webassembly.org/): Learn about WASM technology

---

**Built with ❤️ and Rust** | [Report Issues](https://github.com/yourusername/expense-tracker/issues) | [Request Features](https://github.com/yourusername/expense-tracker/issues/new)