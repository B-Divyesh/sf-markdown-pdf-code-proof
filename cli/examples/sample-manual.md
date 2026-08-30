# API deployment guide

[Review the retry policy](#retry-policy) before release.

## Request endpoint

```javascript
const endpoint = "https://api.example.test/v1/releases"; return fetch(endpoint);
```

## Retry policy

Retry failed release requests twice, then surface the response to the operator.
