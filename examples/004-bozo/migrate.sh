#!/bin/sh -e

table="bozo_migrations"

dbmate -d migrations --migrations-table "$table" -u "$1" up
