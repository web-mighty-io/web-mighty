# Web Mighty Server

## Server Configuration

This server uses postgresql as database.

**Example**

```dotenv
PG__HOST=pg.example.com
PG__USER=jaeyong_sung
PG__PASSWORD=topsecret
PG__DBNAME=example
PG__POOL__MAX_SIZE=16
PG__POOL__TIMEOUTS__WAIT__SECS=5
PG__POOL__TIMEOUTS__WAIT__NANOS=0
```

## URL

### GET

1. `/`: login x: main page,  login o: dashboard page
1. `/list`: list of rooms
1. `/room/{room-id}`: game playing page
1. `/join/{room-id}`: enters room -> if succeeded: `/room/{room-id}`, failed: `/list`
1. `/observe/{room-id}`: observe room
1. `/ranking`: ranking page
1. `/user/{user-id}`: user profile page
1. `/setting`: profile/setting edit page
1. `/mail/{token}`: mail verification

---

1. `/api/user/{user-id}`: user data
1. `/api/ranking`: ranking data

---

1. `/ws/room`: WebSocket connection in `/room/*`
1. `/ws/list`: WebSocket connection in `/list`
1. `/ws/observe`: WebSocket connection in `/observe/*`
1. `/ws/chat`: WebSocket connection in chatting
1. `/ws/main`: WebSocket connection in all pages
1. `/res/{file-path}`: resource file

### POST

1. `/login`
1. `/logout`
1. `/register`
1. `/setting`: profile/setting edit

### DELETE

1. `/delete_user`

---
## WebSocket

### `/ws/room`

1. game state
1. game chatting

### `/ws/list`

1. user count
1. room rule
1. room name

### `/ws/observe`

1. observer chatting
1. game chatting

### `/ws/chat`

1. chatting between users

### `/ws/main`

1. online state
1. chatting notification
