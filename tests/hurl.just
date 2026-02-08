

hurl_opts := "--variables-file variables --test --very-verbose"

wait-for-api:
    hurl health.hurl --retry 1 --retry-interval 1000 {{ hurl_opts }}

# run acceptance tests against the running test stack
test *args: wait-for-api
    hurl *.hurl {{ hurl_opts }} {{ args }}
