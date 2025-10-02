#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: scripts/setup_hh_secrets.sh [--env NAME]

Interactively collects HeadHunter OAuth credentials and resume identifiers,
exchanges an authorization code for a refresh token, and stores everything as
GitHub environment secrets via the `gh` CLI. The script never prints the
provided secrets and writes them directly to GitHub.

Options:
  --env NAME    GitHub environment that should receive the secrets (default: prod)
  -h, --help    Display this help message.

Before running the script ensure that:
  * You have registered a HeadHunter application and know its redirect URI.
  * The GitHub CLI (`gh`) is authenticated with an account that can manage
    secrets in the target repository environment.

The script will prompt you to open an authorization URL in your browser. After
signing in to HeadHunter and approving the application, paste the returned
`code` value back into the terminal so the script can obtain a refresh token.
USAGE
}

environment="prod"
while [[ $# -gt 0 ]]; do
  case "$1" in
    --env)
      if [[ $# -lt 2 ]]; then
        echo "Error: --env requires a value" >&2
        exit 1
      fi
      environment="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "Error: Required command '$1' is not available in PATH" >&2
    exit 1
  fi
}

require_command gh
require_command jq
require_command curl
require_command openssl
require_command python3

if ! gh auth status >/dev/null 2>&1; then
  echo "Error: gh CLI is not authenticated. Run 'gh auth login' first." >&2
  exit 1
fi

read_with_default() {
  local prompt="$1"
  local default_value="${2:-}"
  local value
  if [[ -n "$default_value" ]]; then
    read -r -p "$prompt [$default_value]: " value
    value="${value:-$default_value}"
  else
    read -r -p "$prompt: " value
  fi
  printf '%s' "$value"
}

read_secret() {
  local prompt="$1"
  local value
  read -r -s -p "$prompt: " value
  echo
  printf '%s' "$value"
}

store_secret() {
  local name="$1"
  local value="$2"
  if [[ -z "$value" ]]; then
    echo "Skipping empty $name"
    return
  fi
  printf '%s' "$value" | gh secret set "$name" --env "$environment" >/dev/null
  echo "Stored secret $name in environment $environment"
}

encode_url() {
  python3 - "$1" <<'PY'
import sys
from urllib.parse import quote
if len(sys.argv) != 2:
    sys.exit(1)
print(quote(sys.argv[1], safe=''))
PY
}

validate_redirect_uri() {
  python3 - "$1" <<'PY'
import sys
from urllib.parse import urlparse

uri = sys.argv[1].strip()

if not uri:
    print("Error: Redirect URI is required to request a new refresh token.", file=sys.stderr)
    sys.exit(1)

parsed = urlparse(uri)

if not parsed.scheme:
    print("Error: Redirect URI must include a scheme, for example https://example.com/callback.", file=sys.stderr)
    sys.exit(1)

if parsed.scheme in {"http", "https"} and not parsed.netloc:
    print("Error: HTTP(S) redirect URIs must include a host, for example https://example.com/callback.", file=sys.stderr)
    sys.exit(1)
PY
}

echo "Collecting HeadHunter OAuth application details..."
client_id="$(read_with_default "HeadHunter OAuth client ID")"
if [[ -z "$client_id" ]]; then
  echo "Error: Client ID is required." >&2
  exit 1
fi

client_secret="$(read_secret "HeadHunter OAuth client secret")"
if [[ -z "$client_secret" ]]; then
  echo "Error: Client secret is required." >&2
  exit 1
fi

have_refresh="$(read_with_default "Do you already have a refresh token? (y/N)" "N")"
refresh_token=""
if [[ "$have_refresh" =~ ^[Yy] ]]; then
  refresh_token="$(read_secret "HeadHunter refresh token")"
  if [[ -z "$refresh_token" ]]; then
    echo "Error: Refresh token cannot be empty when requested." >&2
    exit 1
  fi
else
  redirect_uri=""
  while true; do
    redirect_uri="$(read_with_default "HeadHunter redirect URI (must match the value registered in your application)" "$redirect_uri")"
    if validate_redirect_uri "$redirect_uri"; then
      break
    fi
  done
  scope="$(read_with_default "Requested OAuth scope" "resume")"
  state_token=$(openssl rand -hex 16)
  encoded_redirect="$(encode_url "$redirect_uri")"
  encoded_scope="$(encode_url "$scope")"
  auth_url="https://hh.ru/oauth/authorize?response_type=code&client_id=${client_id}&redirect_uri=${encoded_redirect}&scope=${encoded_scope}&state=${state_token}"

  cat <<EOF

1. Open the following URL in your browser:
   ${auth_url}
2. Sign in to HeadHunter and grant access to the application.
3. After the redirect completes, copy the 'code' query parameter from the URL
   and paste it below.
EOF

  authorization_code="$(read_with_default "Authorization code")"
  if [[ -z "$authorization_code" ]]; then
    echo "Error: Authorization code is required to continue." >&2
    exit 1
  fi

  echo "Requesting tokens from HeadHunter..."
  token_response=$(curl -sS -X POST https://hh.ru/oauth/token \
    --data-urlencode grant_type=authorization_code \
    --data-urlencode client_id="${client_id}" \
    --data-urlencode client_secret="${client_secret}" \
    --data-urlencode redirect_uri="${redirect_uri}" \
    --data-urlencode code="${authorization_code}")

  if [[ -z "$token_response" ]]; then
    echo "Error: Empty response from HeadHunter OAuth endpoint." >&2
    exit 1
  fi

  if [[ "$(echo "$token_response" | jq -r '.error // empty')" != "" ]]; then
    echo "HeadHunter returned an error:" >&2
    echo "$token_response" | jq >&2
    exit 1
  fi

  refresh_token="$(echo "$token_response" | jq -r '.refresh_token // empty')"
  if [[ -z "$refresh_token" || "$refresh_token" == "null" ]]; then
    echo "Error: Refresh token is missing in the response:" >&2
    echo "$token_response" | jq >&2
    exit 1
  fi
  echo "Received refresh token from HeadHunter."
fi

echo
res_ru="$(read_with_default "HeadHunter resume ID (Russian)")"
res_en="$(read_with_default "HeadHunter resume ID (English)")"

echo
store_secret HH_CLIENT_ID "$client_id"
store_secret CLIENT_ID "$client_id"
store_secret HH_CLIENT_SECRET "$client_secret"
store_secret CLIENT_SECRET "$client_secret"
store_secret HH_REFRESH_TOKEN "$refresh_token"
store_secret REFRESH_TOKEN "$refresh_token"
store_secret HH_RESUME_ID_RU "$res_ru"
store_secret RESUME_ID_RU "$res_ru"
store_secret HH_RESUME_ID_EN "$res_en"
store_secret RESUME_ID_EN "$res_en"

echo
cat <<EOF
All provided values have been stored as GitHub secrets in environment '$environment'.
You can verify them in the repository settings under Environments → ${environment}.
EOF
