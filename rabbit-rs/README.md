# rabbit-rs

A simple RabbitMQ consumer application.

## How to run

### Run local rabbitmq on docker

docker run -p 15672:15672 -p 5672:5672 -e RABBITMQ_DEFAULT_USER=guest -e RABBITMQ_DEFAULT_PASS=guest  rabbitmq:3.8.4-management
