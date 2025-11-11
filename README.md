# 🚗 AutoRenewPayByPhone

AutoRenewPayByPhone is a Rust-based project that automates the process of renewing parking sessions using the PayByPhone API. 

The project consists of two main components: `front` and `back`.

## 📁 Project Structure

- `front`: The frontend application built with [Dioxus](https://dioxuslabs.com/).
- `back`: The backend service that interacts with the PayByPhone API.

## 🛠 Prerequisites

- Rust (latest stable version)
  - Cargo
  - WASM target for the frontend
  - Whatever other target you need to build the backend
- Docker (for building Docker images)

## 🚀 Getting Started

### 📥 Cloning the Repository

```sh
git clone https://github.com/remi-espie/AutoRenewPayByPhone.git
cd AutoRenewPayByPhone
```

### 🏗 Building the Frontend


Navigate to the `front` directory and build the frontend:

```sh
cd front
cargo build
```

### 🏗Building the Backend

Navigate to the `back` directory and build the backend:

```sh
cd back
cargo build
```

### 🏗 Building the entire project

```sh
cargo build
```

## ▶️ Running the Application

### Frontend

```sh
cd front
dx run
```

### Backend

```sh
cargo run -p back
```

## 🐳 Docker

You can also build and run the project using Docker.

### 🏗 Building Docker Images

```sh
docker build -t autopbf/front ./front
docker build -t autopbf/back ./back
```

### ▶️ Running Docker Containers

```sh
docker run -d -p 8080:8080 autopbf/front
docker run -d -p 8081:8081 autopbf/back
```

### ▶️ Using Docker Compose

You can also use Docker Compose to run the project from already built image available in [this repo](https://github.com/remi-espie?tab=packages&repo_name=AutoRenewPayByPhone).

```sh
docker compose up
```

## ⚙️Configuration

### 📝 Env file

The project uses environment variables for configuration. You can set the following environment variables:

- `BEARER`: Bearer token used to authenticate to the backend.
- `API_URL`: The URL of the backend API.
- `PORT`: The port on which the backend will listen.

> [!NOTE]
> Because the frontend is a WASM app, environment variable are not available.
> `API_URL` is hardcoded in the frontend code, but is recreated from the environment or the `.env` file at build time, so you can set it in the `.env` file, rebuild and it will be taken into account.

### 📝 Configuration file

The project also uses a configuration file located at `config.yaml`. You can set the following configuration options:

```yaml
accounts: # A list of car "accounts", each account represents a car
  - name: # The display name of a car account
    plate: # The plate of the car
    lot: # The ID of the parking lot, find it in the URL of the parking lot page
    pay_by_phone: # PayByPhone related information
      login: # PayByPhone login information, typically your phone number
      password: # PayByPhone password
      payment_account_id:  # PayByPhone payment card ID, can be empty if you park on a free lot
```

## 🤝 Contributing
Contributions are welcome! Please open an issue or submit a pull request.

## 📜 License
This project is licensed under the MIT License. See the `LICENSE` file for details.

