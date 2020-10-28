# for develop
dev:
	(cd develop;sudo docker-compose build;sudo docker-compose up)
dev_down:
	(cd develop;sudo docker-compose down)
db:
	(cd develop/db;sudo docker-compose up)
db_down:
	(cd develop/db;sudo docker-compose down)
clean:
	(sudo docker image prune)