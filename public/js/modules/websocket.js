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

import "../../../node_modules/ifvisible.js/src/ifvisible.js";

let websocket = import ("../pkg/index.js").then((wasm) => {
    wasm.run();

    const websockets = {
        "list": {
            session: wasm.List,
            init: function (_) {
            }
        },
        "main": {
            session: wasm.Main,
            init: function (main) {
                ifvisible.onEvery(200, function () {
                    main.update();
                });
            }
        },
        "observe": {
            session: wasm.Observe,
            init: function (_) {
            }
        },
        "user": {
            session: wasm.User,
            init: function (_) {
            }
        },
    };

    return function (config) {
        let res = {
            disconnected: new Set(),
        };

        for (let i in config.connections) {
            res[i] = new websockets[i]();
            websockets[i].init(res[i]);

            res[i].on("disconnect", function () {
                if (res.disconnected.size === 0) {
                    config.onDisconnect();
                }

                res.disconnected.add(i);
            });

            res[i].on("reconnect", function () {
                res.disconnected.delete(i);

                if (res.disconnected.size === 0) {
                    config.onReconnect();
                }
            });
        }

        return res;
    };
});

export default websocket;