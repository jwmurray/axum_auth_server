#!/bin/bash

# Local Development Docker Script
# This script is designed for local development with port exposure

echo "🚀 Starting local development environment..."

# Define the location of the .env file (change if needed)
ENV_FILE="./auth-service/.env"

# Check if the .env file exists
if ! [[ -f "$ENV_FILE" ]]; then
  echo "❌ Error: .env file not found at $ENV_FILE!"
  exit 1
fi

echo "📄 Loading environment variables from $ENV_FILE"

# Read each line in the .env file (ignoring comments)
while IFS= read -r line; do
  # Skip blank lines and lines starting with #
  if [[ -n "$line" ]] && [[ "$line" != \#* ]]; then
    # Split the line into key and value
    key=$(echo "$line" | cut -d '=' -f1)
    value=$(echo "$line" | cut -d '=' -f2-)
    # Export the variable
    export "$key=$value"
    echo "  ✓ Exported $key"
  fi
done < <(grep -v '^#' "$ENV_FILE")

echo ""
echo "🔧 Building and starting services with local development configuration..."
echo "   - Auth service will be available at: http://localhost:3000"
echo "   - App service will be available at: http://localhost:8000"
echo "   - Traefik dashboard available at: http://localhost:8080"
echo ""

# Use compose.yml + compose.override.yml (default behavior)
# The override file exposes ports for local development
docker compose build
docker compose up

echo ""
echo "🛑 Services stopped"
