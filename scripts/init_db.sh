#!/usr/bin/env bash

export DATABASE_URL=postgres://maxim:canada@localhost:5432/newsletter
sqlx database create
sqlx migrate run