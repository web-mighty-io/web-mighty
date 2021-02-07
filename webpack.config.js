const glob = require("glob");
const path = require("path");
const FileManagerPlugin = require("filemanager-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = (env) => {
    let plugins = [
        new FileManagerPlugin({
            events: {
                onStart: {
                    delete: [
                        path.posix.resolve(__dirname, "public/res/js")
                    ]
                },
            }
        })
    ];

    if (!env.docker) {
        plugins.push(
            new WasmPackPlugin({
                crateDirectory: path.posix.resolve(__dirname, "client"),
                outDir: path.posix.resolve(__dirname, "public/src/js/pkg"),
            })
        );
    }

    return {
        mode: "production",
        entry: glob.sync(path.posix.resolve(__dirname, "public/src/js/*.js")).reduce((acc, item) => {
            const path = item.split("/");
            const name = path[path.length - 1].split(".").slice(0, -1).join(".");
            acc[name] = item;
            return acc;
        }, {}),
        output: {
            path: path.posix.resolve(__dirname, "public/res/js"),
            filename: "[name].js"
        },
        plugins,
        experiments: {
            syncWebAssembly: true,
            topLevelAwait: true,
        },
        module: {
            rules: [{
                test: /\.s[ac]ss$/i,
                use: ["style-loader", "css-loader", "sass-loader"],
            }],
        },
        watchOptions: {
            ignored: [
                path.posix.resolve(__dirname, "node_modules"),
                path.posix.resolve(__dirname, "public/res")
            ]
        }
    };
};