# Test proxy functionality

# Test that unmatched routes without proxy target return 404
GET {{target}}/completely-unmatched-route
HTTP 404

[Asserts]
body contains "No route found and no proxy target configured"

# Test that the proxy UI page loads correctly
GET {{target}}/ui/proxy
HTTP 200

[Asserts]
body contains "Proxy"
body contains "HTTP proxy and recording"

# Test proxy with target URL including path
# Request an unmatched route, proxy to /test-target (registered route)
# Normally this would be the actual route but here we are testing that the proxy will call the catch server itself
GET {{target}}/test-target?test=123
X-Proxy-Target: {{target}}/_catch/test-target
HTTP 200

[Asserts]
body contains "Test endpoint response"

# Verify the proxy call was recorded in the UI
GET {{target}}/ui/proxy
HTTP 200

[Asserts]
body contains "GET"
body contains "/test-target"
body contains "test=123"
body contains ".hurl File"
