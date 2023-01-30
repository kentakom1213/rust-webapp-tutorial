# Todos

Provides a RESTful web server managing some Todos.

API will be:

- `GET /todos`: return a JSON list of Todos.
- `POST /todos`: create a new Todo.
- `PUT /todos/:id`: update a specific Todo.
- `DELETE /todos/:id`: delete a specific Todo.

Run with

```not_rust
cd examples && cargo run -p example-todos
```

## 参考
- https://github.com/tokio-rs/axum/tree/main/examples/todos

