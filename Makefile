docker-build:
	sudo docker build -t smitemotd .
	sudo rm -f smitemotd.tar
	sudo docker save smitemotd -o smitemotd.tar