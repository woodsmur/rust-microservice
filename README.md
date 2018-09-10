# 1. database setup

## local database

prepare `dotenv`

    echo DATABASE_URL=postgres://diesel:diesel@localhost/rust_microservice > .env

## managed database

You can launch a postgres database in https://www.elephantsql.com/, and replace `.env` with managed database URL:

    echo DATABASE_URL=postgres://managedb:MANAGEDDATABASEPASSWORD@stampy.db.elephantsql.com:5432/managedb > .env

You can also launch a postgres database in https://data.heroku.com, and replace `.env` with heroku managed database URL:

    echo DATABASE_URL=postgres://blackmirror:c2118146e07blackmirrormirrorblackblackmirrormirrorblackf82190758@ec2-111-111-111-111.compute-1.amazonaws.com:5432/blackmirror
 > .env

# 2. database migration

diesel setup in local or in managed database

    diesel migration run

if you use run database instance in docker, you can do database migration using `clux/diesel-cli`

    IP=$(ipconfig getifaddr en0)
    DATABASE_URL=postgres://diesel:diesel@${IP}:5436/rust_microservice
    docker run --rm \
        -v "$PWD:/volume" \
        -w /volume \
        -e DATABASE_URL="${DATABASE_URL}" \
        -it clux/diesel-cli diesel migration run

Now the postgres database is created.

# 3. run service

run service in local

    cargo run

run by docker-compose

    cd docker-compose && docker-compose up -d

# 4. send query

## new a book

    curl -X POST -H "Content-Type: application/json" \
        localhost:8000/api/v1/books \
        -d '{"title":"a book", "author":"a author", "published":false}'

## get book

    curl -H "Content-Type: application/json" localhost:8000/api/v1/books/1

## get all books

    curl -H "Content-Type: application/json" localhost:8000/api/v1/books

## update a book

    curl -X PUT -H "Content-Type: application/json" \
        localhost:8000/api/v1/books/1 \
        -d '{"title":"another book", "author":"another author", "published":false}'

## delete book

    curl -X DELETE -H "Content-Type: application/json" localhost:8000/api/v1/books/1 

# 5. run in istio
First, replace `DATABASE_URL` in `rust-microservice.yaml`

    kubectl create -f istio/rust-microservice.yaml \
                   -f istio/rust-microservice-svc.yaml
    istioctl create -f istio/rust-microservice-virtualservice.yaml \
                    -f istio/gateway.yaml \
                    -f istio/serviceentry.yaml

check running status:

kuernetes pods

    $ kubectl get pod
    NAME                                 READY     STATUS    RESTARTS   AGE
    rust-microservice-6d565944d9-hjzpj   2/2       Running   0          18m
    rust-microservice-6d565944d9-q6chj   2/2       Running   0          18m

kubernetes service

    $ kubectl get svc
    NAME                TYPE        CLUSTER-IP      EXTERNAL-IP   PORT(S)    AGE
    kubernetes          ClusterIP   10.233.0.1      <none>        443/TCP    96d
    rust-microservice   ClusterIP   10.233.12.109   <none>        8000/TCP   1h

istio service entry

    $ istioctl get serviceentry
    SERVICE-ENTRY NAME             HOSTS                                                   PORTS      NAMESPACE   AGE
    postgres-access-microservice   *.stampy.db.elephantsql.com,*.compute-1.amazonaws.com   TCP/5432   default     26s

istio virtual service

    $ istioctl get virtualservice
    VIRTUAL-SERVICE NAME               GATEWAYS                    HOSTS                     #HTTP     #TCP      NAMESPACE   AGE
    rust-microservice-virtualservice   rust-microservice-gateway   rust-microservice.istio       1        0      default     1h

istio gateway

    $ istioctl get gateway
    GATEWAY NAME                HOSTS                     NAMESPACE   AGE
    rust-microservice-gateway   rust-microservice.istio   default     1h

in order to send query to istio service mesh, you need to get the gateway.

    export INGRESS_PORT=$(kubectl -n istio-system get service istio-ingressgateway -o jsonpath='{.spec.ports[?(@.name=="http2")].nodePort}')
    export INGRESS_HOST=$(kubectl get po -l istio=ingressgateway -n istio-system -o 'jsonpath={.items[0].status.hostIP}')
    export GATEWAY_URL=$INGRESS_HOST:$INGRESS_PORT

then send query by adding `-H "Host: rust-microservice.istio"`

    curl -X POST \
        -H "Host: rust-microservice.istio" \
        -H "Content-Type: application/json" \
        ${GATEWAY_URL}:8000/api/v1/books \
        -d '{"title":"a book", "author":"a author", "published":false}'

