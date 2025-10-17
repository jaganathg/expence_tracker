# Expense Tracker Application

A full-stack expense tracking application built with Rust, featuring a RESTful API backend using Axum and a reactive frontend using Dioxus.

## 📋 Table of Contents

- [Features](#features)
- [Tech Stack](#tech-stack)
- [Project Structure](#project-structure)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Running the Application](#running-the-application)
- [Testing](#testing)
- [API Documentation](#api-documentation)
- [Docker Deployment](#docker-deployment)
- [CI/CD Pipeline](#cicd-pipeline)
- [Contributing](#contributing)

## ✨ Features

- **Add Expenses**: Track your expenses with amount and category
- **View All Expenses**: Browse through all recorded expenses
- **Highest Expense Tracking**: Automatically identify your largest expense
- **Real-time Updates**: Reactive UI with immediate feedback
- **Data Validation**: Input validation on both frontend and backend
- **Persistent Storage**: SQLite database for reliable data storage

## 🛠 Tech Stack

### Backend
- **Rust** (Edition 2024)
- **Axum** - Web framework
- **SQLx** - Async SQL toolkit with SQLite
- **Tokio** - Async runtime
- **UUID** - Unique identifiers for expenses
- **Chrono** - Date and time handling
- **Validator** - Data validation

### Frontend
- **Rust** with **Dioxus** - Reactive UI framework
- **Dioxus Router** - Client-side routing
- **Reqwest** - HTTP client for API calls
- **Tailwind CSS** - Styling (via Dioxus)

## 📁 Project Structure

```
expense-tracker/
├── src/                    # Backend source code
│   ├── main.rs            # Application entry point
│   ├── database.rs        # Database connection and setup
│   ├── models/            # Data models
│   │   └── expense.rs     # Expense model and validation
│   ├── handlers/          # HTTP request handlers
│   │   └── expenses.rs    # Expense endpoints
│   ├── services/          # Business logic
│   │   └── expense_service.rs
│   └── error.rs           # Error handling
│
├── frontend/              # Frontend application
│   ├── src/
│   │   ├── main.rs       # Frontend entry point
│   │   ├── components/   # Reusable UI components
│   │   ├── pages/        # Application pages
│   │   ├── models/       # Frontend data models
│   │   └── services/     # API communication
│   └── Cargo.toml
│
├── .github/workflows/     # CI/CD configuration
│   └── ci.yml
├── Cargo.toml            # Backend dependencies
├── Dockerfile            # Backend containerization
└── bacon.toml           # Development tooling config
```

## 📋 Prerequisites

- **Rust** 1.90 or later
- **Cargo** (comes with Rust)
- **Dioxus CLI** for frontend development
- **SQLite3** for database
- **Docker** (optional, for containerization)

## 🚀 Installation

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/expense-tracker.git
cd expense-tracker
```

### 2. Install Dioxus CLI

```bash
cargo install dioxus-cli
# Or using cargo-binstall for faster installation
cargo install cargo-binstall
cargo binstall dioxus-cli --no-confirm
```

### 3. Build the Backend

```bash
cargo build --release
```

### 4. Build the Frontend

```bash
cd frontend
dx build --release
cd ..
```

## 🏃 Running the Application

### Start the Backend Server

```bash
# From project root
cargo run

# Or for release mode
cargo run --release
```

The backend API will be available at `http://localhost:3000`

### Start the Frontend Development Server

```bash
# In a new terminal, from the frontend directory
cd frontend
dx serve

# Or for production build
dx serve --release
```

The frontend will be available at `http://localhost:8080` (or the port shown in terminal)

## 🧪 Testing

### Backend Testing

#### Run All Backend Tests

```bash
# From project root
cargo test
```

#### Run Tests with Output

```bash
cargo test -- --nocapture
```

#### Run Specific Test Categories

```bash
# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test '*'

# Run tests for a specific module
cargo test expense_service
cargo test database
cargo test models
```

#### Backend Test Coverage Areas

- **Database Tests** (`src/database.rs`)
  - In-memory SQLite pool creation
  - Table creation and schema validation
  - Database operations

- **Model Tests** (`src/models/expense.rs`)
  - Expense creation validation
  - Input validation (amount must be > 0)
  - Category length validation (1-50 chars)
  - Boundary condition testing

- **Service Tests** (`src/services/expense_service.rs`)
  - Adding expenses
  - Retrieving all expenses
  - Finding highest expense
  - Empty database handling

### Frontend Testing

```bash
cd frontend
cargo test
```

### Code Quality Checks

#### Format Code

```bash
# Backend
cargo fmt

# Frontend
cd frontend && cargo fmt

# Check formatting without applying
cargo fmt -- --check
```

#### Lint with Clippy

```bash
# Backend
cargo clippy -- -D warnings

# Frontend
cd frontend && cargo clippy -- -D warnings
```

### Using Bacon for Continuous Testing

The project includes a `bacon.toml` configuration for continuous testing during development:

```bash
# Install bacon
cargo install bacon

# Run continuous testing
bacon test

# Run clippy checks
bacon clippy

# Run format checks
bacon fmt

# Run all checks
bacon all
```

### Testing API Endpoints Manually

You can test the API endpoints using curl or any HTTP client:

#### Add an Expense

```bash
curl -X POST http://localhost:3000/expenses \
  -H "Content-Type: application/json" \
  -d '{"amount": 25.50, "category": "Groceries"}'
```

#### Get All Expenses

```bash
curl http://localhost:3000/expenses
```

#### Get Highest Expense

```bash
curl http://localhost:3000/expenses/highest
```

### Integration Testing

For full integration testing:

1. Start the backend server
2. Start the frontend server
3. Open the application in a browser
4. Test the following user flows:
   - Add a new expense
   - View the expense list
   - Check the highest expense on the home page
   - Validate error handling for invalid inputs

## 📚 API Documentation

### Endpoints

| Method | Endpoint | Description | Request Body | Response |
|--------|----------|-------------|--------------|----------|
| POST | `/expenses` | Add a new expense | `{"amount": 25.50, "category": "Food"}` | Created expense object |
| GET | `/expenses` | Get all expenses | - | Array of expenses |
| GET | `/expenses/highest` | Get the highest expense | - | Single expense object or null |

### Data Models

#### Expense
```json
{
  "id": "uuid-string",
  "amount": 25.50,
  "category": "Groceries",
  "date": "2025-01-15T12:00:00Z"
}
```

#### CreateExpenseRequest
```json
{
  "amount": 25.50,
  "category": "Groceries"
}
```

### Validation Rules

- **Amount**: Must be greater than 0.01
- **Category**: Required, 1-50 characters

## 🐳 Docker Deployment

### Build and Run Backend with Docker

```bash
# Build the backend image
docker build -t expense-tracker-backend .

# Run the backend container
docker run -p 3000:3000 expense-tracker-backend
```

### Build and Run Frontend with Docker

```bash
# Build the frontend image
cd frontend
docker build -t expense-tracker-frontend .

# Run the frontend container
docker run -p 80:80 expense-tracker-frontend
```

### Using Docker Compose (if you create a compose file)

```yaml
# docker-compose.yml
version: '3.8'

services:
  backend:
    build: .
    ports:
      - "3000:3000"
    environment:
      - DATABASE_URL=sqlite:./expenses.db
    volumes:
      - ./data:/app

  frontend:
    build: ./frontend
    ports:
      - "80:80"
    depends_on:
      - backend
```

Then run:
```bash
docker-compose up
```

## 🔄 CI/CD Pipeline

The project includes GitHub Actions workflow for continuous integration:

### Pipeline Stages

1. **Testing** (runs on Ubuntu, Windows, macOS)
   - Format checking
   - Clippy linting
   - Build verification
   - Unit and integration tests

2. **Frontend Build**
   - Dioxus CLI installation
   - Frontend compilation
   - Frontend tests

3. **Docker Build** (on master branch)
   - Backend image build and push to GitHub Container Registry
   - Frontend image build and push

### Running CI Locally

You can simulate CI checks locally:

```bash
# Run all CI checks
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

## 🛠 Development Workflow

1. **Make Changes**: Edit code in your preferred editor
2. **Run Bacon**: Keep `bacon test` running for immediate feedback
3. **Format Code**: Run `cargo fmt` before committing
4. **Check with Clippy**: Ensure no warnings with `cargo clippy`
5. **Test Thoroughly**: Run all tests with `cargo test`
6. **Manual Testing**: Test the full application flow
7. **Commit**: Make atomic commits with clear messages
8. **Push**: Push to trigger CI pipeline

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Ensure all tests pass
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

## 📝 Environment Variables

- `DATABASE_URL`: SQLite database path (default: `sqlite:./expenses.db`)
- `RUST_LOG`: Logging level (default: `info`)

## 🐛 Troubleshooting

### Common Issues

1. **Port Already in Use**
   - Change the port in `src/main.rs` for backend
   - Use `dx serve --port <PORT>` for frontend

2. **Database Connection Failed**
   - Ensure SQLite is installed
   - Check file permissions for database file

3. **Frontend Can't Connect to Backend**
   - Verify backend is running on port 3000
   - Check CORS settings in backend
   - Update `API_BASE_URL` in `frontend/src/services/expense_service.rs`

4. **Tests Failing**
   - Run `cargo clean` and rebuild
   - Check for uncommitted changes
   - Ensure database migrations are applied

## 📄 License

This project is open source. Please check the repository for license details.

## 🙏 Acknowledgments

- Built with the Rust ecosystem
- Axum web framework by the Tokio team
- Dioxus framework for reactive UIs
- SQLx for async database operations
