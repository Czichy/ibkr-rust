# IBKR Rust API — Test Guide

## Test Categories

### 1. Unit Tests (Offline)

No TWS/IB Gateway required. Run on every build.

**Encoding Tests** (`src/orders/order/tests/encoding_tests.rs`):
- Verify PlaceOrder wire protocol encoding is correct
- Regression tests for Error 320 (`Option<bool>` encoding bug)
- Field position verification for critical fields (`what_if`, `order_misc_options`, `solicited`)
- Known-good reference encoding comparison

```bash
just test-encoding    # Only encoding tests
just test-unit        # All unit tests
```

### 2. Integration Tests (Online, Feature-Gated)

Require a running TWS or IB Gateway (Paper Trading account).
Gated behind `--features ibkr_client_test` — ignored by default.

**Connection Tests** (`tests/all/connection.rs`):
- `connect_and_disconnect` — Basic TCP handshake
- `request_server_time` — Server time within 60s of local
- `request_next_order_id` — Valid order ID > 0
- `request_contract_details_by_symbol` — AMD lookup → con_id 4391
- `request_contract_details_by_con_id` — con_id 265598 → AAPL

**Order Lifecycle** (`tests/all/order_lifecycle.rs`):
- `place_limit_order_far_from_market` — Limit $1.00 AAPL, verify Submitted status
- `place_order_no_error_320` — Regression: no Error 320 on order placement
- `place_market_order_and_receive_execution` — 1 share AAPL market order, verify execution

**Market Data** (`tests/all/market_data_integration.rs`):
- `historical_bars_amd` — Download 1-day 1-min bars
- `realtime_bars_subscribe_unsubscribe` — Subscribe/cancel realtime bars
- `historical_head_timestamp` — Head timestamp for AAPL

```bash
just test-integration   # All integration tests
just test-connection    # Only connection tests
just test-orders        # Only order lifecycle tests
```

## Prerequisites for Integration Tests

1. **TWS or IB Gateway** must be running
2. **Paper Trading Account** (never run against live!)
3. Default address: `127.0.0.1:4002` (IB Gateway Paper)

### Override Connection Settings

```bash
TWS_HOST=10.15.10.25 TWS_PORT=4002 just test-integration
```

### TWS API Settings

In TWS/Gateway → Configuration → API → Settings:
- Enable ActiveX and Socket Clients
- Socket port: 4002 (Paper) / 4001 (Live)
- Allow connections from localhost
- Master API client ID: leave empty or set to 0

## Test Client IDs

Each test uses a unique client_id to avoid conflicts:
- 70-79: Market data tests
- 80-89: Order lifecycle tests
- 90-99: Connection tests

## Running Tests

```bash
# Fast: offline encoding tests only (~0.1s)
just test-encoding

# Medium: all offline unit tests (~1s)
just test-unit

# Full: all integration tests against Paper-TWS (~60s)
just test-integration

# Specific: only order tests
just test-orders
```
