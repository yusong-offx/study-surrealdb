```mermaid
erDiagram
    user {
        uuid id PK
        string user_id UK "user id for login"
        string(bcrpyt_hash) password 
    }

    todos {
        string work
    }



```