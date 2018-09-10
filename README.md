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

