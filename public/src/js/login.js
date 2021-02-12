import "./modules/wasm.js";
import "../scss/login.scss";
import User from "./modules/user.js";

window.onload = function () {
    let form = document.getElementById("login-form");
    let id = document.getElementById("login-id");
    let idError = document.getElementById("login-id-error");
    let password = document.getElementById("login-password");
    let passwordError = document.getElementById("login-password-error");
    let isError = false;

    form.onsubmit = function () {
        if (isError) {
            id.classList.add("danger");
            id.focus();
            return false;
        }

        let user;
        let value = id.value;

        if (value.includes("@")) {
            user = new User({
                info: {
                    email: value,
                }
            });
        } else {
            user = new User({
                info: {
                    id: value,
                }
            });
        }

        User.login(user, password.value, function () {
            passwordError.innerText = "아이디/이메일이 존재하지 않거나 비밀번호가 일치하지 않습니다.";
        });

        return false;
    };

    id.oninput = function () {
        let value = id.value;
        if (value.includes("@")) {
            if (!User.checkEmail(value)) {
                idError.innerText = "잘못된 이메일 형식입니다.";
                isError = true;
            } else {
                id.classList.remove("danger");
                idError.innerText = "";
                isError = false;
            }
        } else {
            if (!User.checkUserId(value)) {
                idError.innerText = "잘못된 아이디 형식입니다.";
                isError = true;
            } else {
                id.classList.remove("danger");
                idError.innerText = "";
                isError = false;
            }
        }
    };
};
