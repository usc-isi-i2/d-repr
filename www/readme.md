## Development

1. Start postgres, msgqueue, and api


    docker-compose up -f
    

* Run Test


    pipenv run pytest -s tests 


## Start Swagger API docs

```bash
docker run -p 80:8080 -e SWAGGER_JSON=/docs/api.yml -v $(pwd)/docs:/docs swaggerapi/swagger-ui
```