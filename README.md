# rust-k8s-app-template
Template for minimal app in Kubernetes with liveness/readiness probes etc

## Instructions
You can build and run the app in docker with

```
docker build -t k8s-rust-app .
docker run --init -ti --rm k8s-rust-app
```
