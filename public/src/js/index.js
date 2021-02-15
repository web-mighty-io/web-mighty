import "./modules/wasm.js";
import "../scss/index.scss";

window.onload = function () {
    let loginButton = document.getElementById("login-button");
    let registerButton = document.getElementById("register-button");

    loginButton.addEventListener("click", function () {
        window.location.href = "/login";
    }, false);

    registerButton.addEventListener("click", function () {
        window.location.href = "/pre-register";
    }, false);
};
