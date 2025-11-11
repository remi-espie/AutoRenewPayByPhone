# Makefile for AutoRenewPayByPhone Project
# Manages back (Axum) and front (Dioxus) modules

.PHONY: help all build build-back build-front run run-back run-front dev dev-back dev-front clean clean-back clean-front check install-deps

# Default target
.DEFAULT_GOAL := help

help: ## Display this help
	@echo "AutoRenewPayByPhone - Makefile"
	@echo.
	@echo "Available targets:"
	@echo   build           - Build the entire project (back + front)
	@echo   build-back      - Build the backend (release)
	@echo   build-front     - Build the frontend (release)
	@echo   build-dev       - Build in debug mode
	@echo.
	@echo   run-back        - Run the backend
	@echo   run-front       - Run the frontend with Dioxus
	@echo   dev-back        - Run the backend with auto-reload
	@echo   dev-front       - Run the frontend with hot-reload
	@echo.
	@echo   check           - Check code (clippy + fmt)
	@echo   fmt             - Format code
	@echo   clippy          - Run clippy
	@echo.
	@echo   clean           - Clean all generated files
	@echo   clean-all       - Clean the entire project
	@echo.
	@echo   install-deps    - Install necessary dependencies
	@echo   config          - Copy example configuration file

all: build

# ============================================
# Build targets
# ============================================

build: build-back build-front

build-back:
	@echo [BUILD] Building backend...
	cargo build --release --package back

build-front:
	@echo [BUILD] Building frontend...
	cargo build --release --package front

build-dev: build-dev-back build-dev-front

build-dev-back:
	cargo build --package back

build-dev-front:
	cargo build --package front

# ============================================
# Run targets
# ============================================

run-back:
	cargo run --package back

run-front:
	cd front && dx run

# ============================================
# Development targets
# ============================================

dev-back:
	@echo [DEV] Running backend in development mode...
	cargo watch -x "run --package back"

dev-front:
	@echo [DEV] Running frontend in development mode...
	cd front && dx serve

# ============================================
# Quality checks
# ============================================

check:
	@echo [CHECK] Checking code...
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	@echo [FMT] Formatting code...
	cargo fmt --all

clippy:
	@echo [CLIPPY] Running clippy analysis...
	cargo clippy --all-targets --all-features -- -D warnings

# ============================================
# Clean targets
# ============================================

clean: clean-back clean-front

clean-back:
	@echo [CLEAN] Cleaning backend...
	cargo clean --package back

clean-front:
	@echo [CLEAN] Cleaning frontend...
	cargo clean --package front

clean-all:
	@echo [CLEAN] Complete cleanup...
	cargo clean

# ============================================
# Installation targets
# ============================================

install-deps:
	@echo [INSTALL] Installing dependencies...
	@echo Installing cargo-watch for hot-reload...
	cargo install cargo-watch
	@echo Installing dioxus-cli...
	cargo install dioxus-cli
	@echo Dependencies installed!

check-deps:
	@echo [CHECK] Checking dependencies...
	@where cargo >nul 2>&1 || (echo Cargo is not installed! && exit 1)
	@where dx >nul 2>&1 || echo dioxus-cli is not installed. Run 'make install-deps'
	@where cargo-watch >nul 2>&1 || echo cargo-watch is not installed. Run 'make install-deps'
	@echo Dependencies OK!

# ============================================
# Configuration targets
# ============================================

config:
	@if not exist config.yaml ( \
		echo [CONFIG] Copying config.example.yaml to config.yaml... && \
		copy config.example.yaml config.yaml \
	) else ( \
		echo [CONFIG] config.yaml already exists \
	)

# ============================================
# Info targets
# ============================================

info:
	@echo === Informations du projet ===
	@echo.
	@echo Rust version:
	@rustc --version
	@echo.
	@echo Cargo version:
	@cargo --version
	@echo.
	@echo Workspace members:
	@cargo metadata --no-deps --format-version 1 | findstr "\"name\""

