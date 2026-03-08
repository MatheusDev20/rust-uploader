Reset Public schema 

docker exec -it mini-tube_db_1 psql -U postgres -d minitube -c "DROP SCHEMA public CASCADE; CREATE SCHEMA public;"