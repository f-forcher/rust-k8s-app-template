# rust-k8s-app-template
Template for minimal app in Kubernetes with liveness/readiness probes etc

## Instructions
You can build and run the app in docker with

```
docker build -t k8s-rust-app:1.0 .
docker run --init -ti --rm k8s-rust-app:latest
```

### Kind

Create a named cluster with
```
kind create cluster --name rust-app-cluster
```

Check status with:
```
kubectl cluster-info --context kind-rust-app-cluster
```

Load image with:
```
kind load docker-image k8s-rust-app:latest --name test-cluster --name rust-app-cluster
```

Export logs with:
```
kind export logs --name rust-app-cluster
```

delete cluster with:
```
kind delete cluster --name rust-app-cluster
```
