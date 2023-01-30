# axum-readme

## How to Use

### GET

`Hello, World!`を表示する。

```
$ curl -X GET 0.0.0.0:3000
```

### POST

ユーザ登録を行う。

```
$ curl -X POST \
    -H 'Content-Type: application/json' \
    --data '{"id": 12, "username": "powell"}' \
    0.0.0.0:3000/users
```
