## Transport protocol
Message transmission will be through axum's websocket rather than custom tcp

## End points
- POST auth/login - (username, password) -> JWT
- POST auth/signup - (username, password)
- GET /chats/me - (user_id) -> chats[]
- POST /chats - (user_id, other_user_id) creates new chat
- DELETE /chats/{id} - (user_id) this checks if user is a member to delete
- GET /chats/{id} - (user_id) -> chat
- GET /chats/{id}/messages/{id} - (user_id) -> message
- DELETE /chats/{id}/messages/{id} - (user_id)
- POST /chats/{id}/messages - (user_id, chat_id, content)
- GET /users/{id} - () -> users[]
- DELETE /users/me - 
- PUT /users/me

