# database setup
prepare `dotenv`

    echo DATABASE_URL=postgres://diesel:diesel@localhost/rust_microservice > .env

diesel setup in local

    diesel migration run

diesel migaration in docker

    IP=$(ipconfig getifaddr en0)
    DATABASE_URL=postgres://diesel:diesel@${IP}:5436/rust_microservice
    docker run --rm \
        -v "$PWD:/volume" \
        -w /volume \
        -e DATABASE_URL="${DATABASE_URL}" \
        -it clux/diesel-cli diesel migration run

Now the postgres database is created.

# run service

    cargo run

# run by docker-compose

    cd docker-compose && docker-compose up -d

# send query

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

# run in istio

	kubectl create -f istio/rust-microservice.yaml -f istio/rust-microservice-svc.yaml
	istioctl create -f istio/rust-microservice-virtualservice.yaml -f istio/gateway.yaml

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
