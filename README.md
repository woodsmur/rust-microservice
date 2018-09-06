# database setup
prepare `dotenv`

    echo DATABASE_URL=postgres://diesel:diesel@localhost/rust_microservice > .env

diesel setup

    diesel setup

Now the postgres database is created.

# run service

    cargo run

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

