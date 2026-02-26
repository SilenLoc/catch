<div align="center">
  <img src="/assets/catch_logo.png" alt="Catch Logo" />
</div>

An extension to [Hurl](https://hurl.dev)

- Run script files in the context of hurl - [example](/tests/script_js.hurl)
- Save results of your tests over multiple hurl files - [example](/tests/kv_plaintext_string.hurl) - and extract them out with curl later
- Run catch as a proxy from any service, record the calls and then copy them out as hurl files - [example](/tests/proxy.hurl)

### How to use

If you use docker compose you can run catch as a service beside your service you are testing.
If you use docker same idea but you handle the lifetime of the service.
There is no tutorial yet on how to install it with cargo. You will need to be able to install V8 somehow, depending on your environment this can be challenging.

For how to use it with Hurl, of course you can go and read the hurl tests [here](/tests)

### Stability

I am quite happy where the current HTTP API is. 
The internal code is unstable and may change without notice.
The goal is that all languages supported by /script support the same runtime features, otherwise they are in Experimental state.

### Roadmap

- Special javascript functions
  - input from /script?name=foo for usage in js
  - http
  - direct access to the key value store
  - default js libraries
  - import your own scripts as modules
- Python support
- Search in the ui
- Build docker image in CI for ease of use
- Better Dev setup documentation

### Dev setup

- cargo
- just - optional - task runner
- hurl - optional - e2e tests
- docker - optional - e2e tests
