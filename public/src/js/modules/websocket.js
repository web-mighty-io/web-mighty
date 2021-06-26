/**
 * @fileOverview Websocket wasm binding to js
 * @author Jaeyong Sung
 *
 * This js code helps you to connect to server by websocket
 * in webassembly written in rust language. It uses ifvisible.js
 * for catching user action and check if user is active so
 * server can decide the user's status.
 *
 * **Warning**: do not override "reconnect" and "disconnect" events.
 *
 * @see ifvisible
 * @example
 * import websocket from "<this module path>";
 *
 * let ws = (await websocket)({
 *     // choose connection you want to open
 *     connection: ["list", "main", "observe", "user"],
 *
 *     // callback function called when disconnected
 *     onDisconnect: function() {
 *         alert("websocket is disconnected");
 *     },
 *
 *     // callback function called when reconnected
 *     onReconnect: function() {
 *         alert("websocket is reconnected");
 *     }
 * });
 *
 * ws.main.on("start", function() {
 *     alert("main connection started");
 * });
 *
 * ws.main.start();
 */

import ifvisible from "ifvisible.js";

let websocket = import ("../pkg").then((wasm) => {
    wasm.run();

    const websockets = {
        "list": {
            construct() {
                return new wasm.List();
            },
            init() {
            }
        },
        "main": {
            construct() {
                return new wasm.Main();
            },
            init(main) {
                // todo: fix this
                ifvisible.onEvery(200, function () {
                    main.update();
                });
            }
        },
        "observe": {
            construct() {
                return new wasm.Observe();
            },
            init() {
            }
        },
        "user": {
            construct() {
                return new wasm.User();
            },
            init() {
            }
        },
    };

    return function (config) {
        let res = {
            disconnected: new Set(),
        };

        for (let i in config.connections) {
            if (Object.prototype.hasOwnProperty.call(config.connections, i)) {
                let v = config.connections[i];

                if (websockets.hasOwnProperty(v)) {
                    res[v] = websockets[v].construct();
                    websockets[v].init(res[v]);

                    res[v].on("disconnect", function () {
                        if (res.disconnected.size === 0) {
                            config.onDisconnect();
                        }

                        res.disconnected.add(v);
                    });

                    res[v].on("reconnect", function () {
                        res.disconnected.delete(v);

                        if (res.disconnected.size === 0) {
                            config.onReconnect();
                        }
                    });
                }
            }
        }

        return res;
    };
});

export default websocket;