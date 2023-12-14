# how to run

```bash
docker compose build
docker compose up
```


# access register api

POST -> http://localhost:3353/register
        BODY : application/json
            ```json
            {
                "username": "foo",
                "email": "foo@bar.com"
            }
            ```

> email must be unique


# access mailhog ui

http://localhost:3354
