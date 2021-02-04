import websocket from "./modules/websocket.js";

let ws = (await websocket)({connections: ["main"]});

ws.main.on("start", function () {
    console.log("Main connection starting");
})
