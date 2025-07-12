# Deployment Guide

This document covers production deployment strategies, infrastructure setup, and operational considerations for the task management system.

## Production Architecture

### High-Level Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Load Balancer │    │   Web Server    │    │   Database      │
│   (nginx/ALB)   │◄──►│   (Frontend)    │    │   (PostgreSQL)  │
│                 │    │                 │    │                 │
│ - SSL/TLS       │    │ - Next.js       │    │ - Primary DB    │
│ - Rate Limiting │    │ - Static Assets │    │ - Read Replicas │
│ - DDoS Protection   │ - CDN           │    │ - Backups       │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                │
                                ▼
                       ┌─────────────────┐    ┌─────────────────┐
                       │   API Server    │    │     Cache       │
                       │   (Rust/axum)   │◄──►│    (Redis)      │
                       │                 │    │                 │
                       │ - REST API      │    │ - Session Store │
                       │ - WebSocket     │    │ - Query Cache   │
                       │ - File Upload   │    │ - Rate Limiting │
                       └─────────────────┘    └─────────────────┘
```

## Container Strategy

### Docker Images

**Backend Dockerfile:**
```dockerfile
# Multi-stage build for Rust backend
FROM rust:1.70-slim as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build dependencies first (for better caching)
RUN cargo build --release

FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/taskmanager /app/taskmanager

# Create non-root user
RUN useradd -m -u 1000 taskmanager
USER taskmanager

EXPOSE 8000

CMD ["./taskmanager"]
```

**Frontend Dockerfile:**
```dockerfile
# Multi-stage build for Next.js frontend
FROM node:18-alpine as builder

WORKDIR /app
COPY package.json pnpm-lock.yaml ./
RUN npm install -g pnpm
RUN pnpm install --frozen-lockfile

COPY . .
RUN pnpm build

FROM node:18-alpine as runner

WORKDIR /app

# Create non-root user
RUN addgroup --system --gid 1001 nodejs
RUN adduser --system --uid 1001 nextjs

# Copy built application
COPY --from=builder /app/public ./public
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static

USER nextjs

EXPOSE 3000

ENV PORT 3000
ENV HOSTNAME "0.0.0.0"

CMD ["node", "server.js"]
```

### Docker Compose Production

**docker-compose.prod.yml:**
```yaml
version: '3.8'

services:
  frontend:
    build:
      context: ./frontend
      dockerfile: Dockerfile
    ports:
      - "3000:3000"
    environment:
      - NODE_ENV=production
      - NEXT_PUBLIC_API_URL=https://api.yourdomain.com
      - NEXT_PUBLIC_WS_URL=wss://api.yourdomain.com/ws
    restart: unless-stopped
    depends_on:
      - backend

  backend:
    build:
      context: ./backend
      dockerfile: Dockerfile
    ports:
      - "8000:8000"
    environment:
      - RUST_LOG=info
      - DATABASE_URL=postgres://taskmanager:${DB_PASSWORD}@postgres:5432/taskmanager
      - REDIS_URL=redis://redis:6379
      - JWT_SECRET=${JWT_SECRET}
      - CORS_ORIGIN=https://yourdomain.com
    restart: unless-stopped
    depends_on:
      - postgres
      - redis
    volumes:
      - ./uploads:/app/uploads

  postgres:
    image: postgres:15
    environment:
      - POSTGRES_DB=taskmanager
      - POSTGRES_USER=taskmanager
      - POSTGRES_PASSWORD=${DB_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./backups:/backups
    restart: unless-stopped
    ports:
      - "5432:5432"

  redis:
    image: redis:7-alpine
    command: redis-server --appendonly yes --requirepass ${REDIS_PASSWORD}
    volumes:
      - redis_data:/data
    restart: unless-stopped
    ports:
      - "6379:6379"

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./ssl:/etc/nginx/ssl
      - ./uploads:/var/www/uploads
    restart: unless-stopped
    depends_on:
      - frontend
      - backend

volumes:
  postgres_data:
  redis_data:
```

## Kubernetes Deployment

### Namespace and ConfigMaps

```yaml
# namespace.yaml
apiVersion: v1
kind: Namespace
metadata:
  name: taskmanager
  labels:
    app: taskmanager

---
# configmap.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: taskmanager-config
  namespace: taskmanager
data:
  RUST_LOG: "info"
  CORS_ORIGIN: "https://yourdomain.com"
  UPLOAD_DIR: "/app/uploads"
  MAX_FILE_SIZE: "10485760"
```

### Secrets

```yaml
# secrets.yaml
apiVersion: v1
kind: Secret
metadata:
  name: taskmanager-secrets
  namespace: taskmanager
type: Opaque
data:
  DATABASE_URL: <base64-encoded-db-url>
  JWT_SECRET: <base64-encoded-jwt-secret>
  REDIS_PASSWORD: <base64-encoded-redis-password>
```

### Database Deployment

```yaml
# postgres-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: postgres
  namespace: taskmanager
spec:
  replicas: 1
  selector:
    matchLabels:
      app: postgres
  template:
    metadata:
      labels:
        app: postgres
    spec:
      containers:
      - name: postgres
        image: postgres:15
        env:
        - name: POSTGRES_DB
          value: taskmanager
        - name: POSTGRES_USER
          value: taskmanager
        - name: POSTGRES_PASSWORD
          valueFrom:
            secretKeyRef:
              name: taskmanager-secrets
              key: DB_PASSWORD
        ports:
        - containerPort: 5432
        volumeMounts:
        - name: postgres-storage
          mountPath: /var/lib/postgresql/data
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
      volumes:
      - name: postgres-storage
        persistentVolumeClaim:
          claimName: postgres-pvc

---
apiVersion: v1
kind: Service
metadata:
  name: postgres
  namespace: taskmanager
spec:
  selector:
    app: postgres
  ports:
  - port: 5432
    targetPort: 5432

---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: postgres-pvc
  namespace: taskmanager
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 10Gi
```

### Redis Deployment

```yaml
# redis-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: redis
  namespace: taskmanager
spec:
  replicas: 1
  selector:
    matchLabels:
      app: redis
  template:
    metadata:
      labels:
        app: redis
    spec:
      containers:
      - name: redis
        image: redis:7-alpine
        command: ["redis-server", "--appendonly", "yes", "--requirepass", "$(REDIS_PASSWORD)"]
        env:
        - name: REDIS_PASSWORD
          valueFrom:
            secretKeyRef:
              name: taskmanager-secrets
              key: REDIS_PASSWORD
        ports:
        - containerPort: 6379
        volumeMounts:
        - name: redis-storage
          mountPath: /data
        resources:
          requests:
            memory: "128Mi"
            cpu: "100m"
          limits:
            memory: "256Mi"
            cpu: "200m"
      volumes:
      - name: redis-storage
        persistentVolumeClaim:
          claimName: redis-pvc

---
apiVersion: v1
kind: Service
metadata:
  name: redis
  namespace: taskmanager
spec:
  selector:
    app: redis
  ports:
  - port: 6379
    targetPort: 6379

---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: redis-pvc
  namespace: taskmanager
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 1Gi
```

### Backend Deployment

```yaml
# backend-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: backend
  namespace: taskmanager
spec:
  replicas: 3
  selector:
    matchLabels:
      app: backend
  template:
    metadata:
      labels:
        app: backend
    spec:
      containers:
      - name: backend
        image: your-registry/taskmanager-backend:latest
        envFrom:
        - configMapRef:
            name: taskmanager-config
        - secretRef:
            name: taskmanager-secrets
        ports:
        - containerPort: 8000
        volumeMounts:
        - name: uploads
          mountPath: /app/uploads
        livenessProbe:
          httpGet:
            path: /health
            port: 8000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 8000
          initialDelaySeconds: 5
          periodSeconds: 5
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
      volumes:
      - name: uploads
        persistentVolumeClaim:
          claimName: uploads-pvc

---
apiVersion: v1
kind: Service
metadata:
  name: backend
  namespace: taskmanager
spec:
  selector:
    app: backend
  ports:
  - port: 8000
    targetPort: 8000

---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: uploads-pvc
  namespace: taskmanager
spec:
  accessModes:
    - ReadWriteMany
  resources:
    requests:
      storage: 5Gi
```

### Frontend Deployment

```yaml
# frontend-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: frontend
  namespace: taskmanager
spec:
  replicas: 2
  selector:
    matchLabels:
      app: frontend
  template:
    metadata:
      labels:
        app: frontend
    spec:
      containers:
      - name: frontend
        image: your-registry/taskmanager-frontend:latest
        env:
        - name: NODE_ENV
          value: production
        - name: NEXT_PUBLIC_API_URL
          value: https://api.yourdomain.com
        - name: NEXT_PUBLIC_WS_URL
          value: wss://api.yourdomain.com/ws
        ports:
        - containerPort: 3000
        livenessProbe:
          httpGet:
            path: /
            port: 3000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 5
        resources:
          requests:
            memory: "128Mi"
            cpu: "100m"
          limits:
            memory: "256Mi"
            cpu: "200m"

---
apiVersion: v1
kind: Service
metadata:
  name: frontend
  namespace: taskmanager
spec:
  selector:
    app: frontend
  ports:
  - port: 3000
    targetPort: 3000
```

### Ingress Configuration

```yaml
# ingress.yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: taskmanager-ingress
  namespace: taskmanager
  annotations:
    kubernetes.io/ingress.class: nginx
    cert-manager.io/cluster-issuer: letsencrypt-prod
    nginx.ingress.kubernetes.io/rate-limit: "100"
    nginx.ingress.kubernetes.io/rate-limit-window: "1m"
spec:
  tls:
  - hosts:
    - yourdomain.com
    - api.yourdomain.com
    secretName: taskmanager-tls
  rules:
  - host: yourdomain.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: frontend
            port:
              number: 3000
  - host: api.yourdomain.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: backend
            port:
              number: 8000
```

## CI/CD Pipeline

### GitHub Actions Workflow

**.github/workflows/deploy.yml:**
```yaml
name: Deploy to Production

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  REGISTRY: ghcr.io
  IMAGE_NAME: ${{ github.repository }}

jobs:
  test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_PASSWORD: postgres
          POSTGRES_DB: taskmanager_test
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
      redis:
        image: redis:7
        options: >-
          --health-cmd "redis-cli ping"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    steps:
    - name: Checkout
      uses: actions/checkout@v3

    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true

    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: 18
        cache: 'npm'

    - name: Install pnpm
      run: npm install -g pnpm

    - name: Test Backend
      run: |
        cd backend
        cargo test
      env:
        DATABASE_URL: postgres://postgres:postgres@localhost/taskmanager_test

    - name: Test Frontend
      run: |
        cd frontend
        pnpm install
        pnpm test
        pnpm build

  build-and-push:
    needs: test
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    
    steps:
    - name: Checkout
      uses: actions/checkout@v3

    - name: Log in to Container Registry
      uses: docker/login-action@v2
      with:
        registry: ${{ env.REGISTRY }}
        username: ${{ github.actor }}
        password: ${{ secrets.GITHUB_TOKEN }}

    - name: Build and push Backend
      uses: docker/build-push-action@v4
      with:
        context: ./backend
        push: true
        tags: |
          ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}-backend:latest
          ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}-backend:${{ github.sha }}

    - name: Build and push Frontend
      uses: docker/build-push-action@v4
      with:
        context: ./frontend
        push: true
        tags: |
          ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}-frontend:latest
          ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}-frontend:${{ github.sha }}

  deploy:
    needs: build-and-push
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    
    steps:
    - name: Checkout
      uses: actions/checkout@v3

    - name: Setup kubectl
      uses: azure/setup-kubectl@v1

    - name: Configure kubectl
      run: |
        echo "${{ secrets.KUBE_CONFIG }}" | base64 -d > kubeconfig
        export KUBECONFIG=kubeconfig

    - name: Deploy to Kubernetes
      run: |
        kubectl set image deployment/backend backend=${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}-backend:${{ github.sha }} -n taskmanager
        kubectl set image deployment/frontend frontend=${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}-frontend:${{ github.sha }} -n taskmanager
        kubectl rollout status deployment/backend -n taskmanager
        kubectl rollout status deployment/frontend -n taskmanager
```

## Monitoring and Observability

### Prometheus Monitoring

```yaml
# monitoring.yaml
apiVersion: v1
kind: ServiceMonitor
metadata:
  name: taskmanager-backend
  namespace: taskmanager
spec:
  selector:
    matchLabels:
      app: backend
  endpoints:
  - port: metrics
    path: /metrics

---
apiVersion: monitoring.coreos.com/v1
kind: PrometheusRule
metadata:
  name: taskmanager-alerts
  namespace: taskmanager
spec:
  groups:
  - name: taskmanager
    rules:
    - alert: HighErrorRate
      expr: rate(http_requests_total{status=~"5.."}[5m]) > 0.1
      for: 5m
      labels:
        severity: warning
      annotations:
        summary: High error rate detected
    
    - alert: DatabaseDown
      expr: up{job="postgres"} == 0
      for: 1m
      labels:
        severity: critical
      annotations:
        summary: PostgreSQL database is down
```

### Logging with ELK Stack

```yaml
# logging.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: filebeat-config
  namespace: taskmanager
data:
  filebeat.yml: |
    filebeat.inputs:
    - type: container
      paths:
        - /var/log/containers/*.log
      processors:
        - add_kubernetes_metadata:
            host: ${NODE_NAME}
            matchers:
            - logs_path:
                logs_path: "/var/log/containers/"
    output.elasticsearch:
      hosts: ["${ELASTICSEARCH_HOST}:${ELASTICSEARCH_PORT}"]
```

## Security Hardening

### Network Policies

```yaml
# network-policy.yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: taskmanager-network-policy
  namespace: taskmanager
spec:
  podSelector: {}
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - namespaceSelector:
        matchLabels:
          name: ingress-nginx
    ports:
    - protocol: TCP
      port: 8000
    - protocol: TCP
      port: 3000
  egress:
  - to:
    - namespaceSelector:
        matchLabels:
          name: kube-system
    ports:
    - protocol: TCP
      port: 53
    - protocol: UDP
      port: 53
```

### Pod Security Policy

```yaml
# pod-security-policy.yaml
apiVersion: policy/v1beta1
kind: PodSecurityPolicy
metadata:
  name: taskmanager-psp
spec:
  privileged: false
  allowPrivilegeEscalation: false
  requiredDropCapabilities:
    - ALL
  volumes:
    - 'configMap'
    - 'emptyDir'
    - 'projected'
    - 'secret'
    - 'downwardAPI'
    - 'persistentVolumeClaim'
  runAsUser:
    rule: 'MustRunAsNonRoot'
  seLinux:
    rule: 'RunAsAny'
  fsGroup:
    rule: 'RunAsAny'
```

## Backup and Disaster Recovery

### Database Backup

```bash
#!/bin/bash
# backup-script.sh

DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR="/backups"
DB_NAME="taskmanager"

# Create backup
kubectl exec -n taskmanager $(kubectl get pods -n taskmanager -l app=postgres -o jsonpath='{.items[0].metadata.name}') -- \
  pg_dump -U taskmanager $DB_NAME > $BACKUP_DIR/taskmanager_$DATE.sql

# Compress backup
gzip $BACKUP_DIR/taskmanager_$DATE.sql

# Upload to S3 (optional)
aws s3 cp $BACKUP_DIR/taskmanager_$DATE.sql.gz s3://your-backup-bucket/database/

# Clean old backups (keep last 7 days)
find $BACKUP_DIR -name "taskmanager_*.sql.gz" -mtime +7 -delete
```

### Automated Backup CronJob

```yaml
# backup-cronjob.yaml
apiVersion: batch/v1
kind: CronJob
metadata:
  name: postgres-backup
  namespace: taskmanager
spec:
  schedule: "0 2 * * *"  # Daily at 2 AM
  jobTemplate:
    spec:
      template:
        spec:
          containers:
          - name: postgres-backup
            image: postgres:15
            command:
            - /bin/bash
            - -c
            - |
              pg_dump -h postgres -U taskmanager taskmanager > /backups/backup-$(date +%Y%m%d-%H%M%S).sql
              gzip /backups/backup-$(date +%Y%m%d-%H%M%S).sql
            env:
            - name: PGPASSWORD
              valueFrom:
                secretKeyRef:
                  name: taskmanager-secrets
                  key: DB_PASSWORD
            volumeMounts:
            - name: backup-storage
              mountPath: /backups
          volumes:
          - name: backup-storage
            persistentVolumeClaim:
              claimName: backup-pvc
          restartPolicy: OnFailure
```

## Performance Optimization

### Resource Allocation

```yaml
# Resource requests and limits for production
resources:
  requests:
    memory: "512Mi"
    cpu: "250m"
  limits:
    memory: "1Gi"
    cpu: "500m"
```

### Horizontal Pod Autoscaling

```yaml
# hpa.yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: backend-hpa
  namespace: taskmanager
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: backend
  minReplicas: 3
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

This deployment guide provides a comprehensive production-ready setup with security, monitoring, backup, and scaling considerations for the task management system.