# L7 Load Balancer - TODO

## Core Components

- [ ] Implement basic HTTP server listener
- [ ] Create backend server pool management
- [ ] Build HTTP request parser and forwarder
- [ ] Implement load balancing algorithms
  - [ ] Round Robin
  - [ ] Least Connections
  - [ ] Weighted Round Robin (optional)

## Health Checking

- [ ] Add active health checks (periodic pings)
- [ ] Add passive health checks (failure detection)
- [ ] Implement backend server state tracking (healthy/unhealthy)

## Request Handling

- [ ] Forward HTTP requests to backend servers
- [ ] Handle HTTP responses from backends
- [ ] Implement connection pooling for backends
- [ ] Add request/response header manipulation

## Configuration

- [ ] Create config file structure (YAML/TOML)
- [ ] Load backend server list from config
- [ ] Configure health check intervals and timeouts
- [ ] Set load balancing algorithm via config

## Resilience & Error Handling

- [ ] Implement retry logic for failed requests
- [ ] Add circuit breaker pattern
- [ ] Handle backend timeouts gracefully
- [ ] Implement graceful shutdown

## Observability

- [ ] Add structured logging (tracing)
- [ ] Track metrics (requests/sec, latency, errors)
- [ ] Log backend health status changes

## Testing

- [ ] Create mock backend servers for testing
- [ ] Write integration tests for load distribution
- [ ] Test health check functionality
- [ ] Load test with multiple concurrent requests
- [ ] Test failure scenarios (backend down, timeouts)

## Optional Enhancements

- [ ] Add TLS/HTTPS support
- [ ] Implement session persistence (sticky sessions)
- [ ] Add rate limiting
- [ ] WebSocket support
- [ ] Admin API for runtime configuration
