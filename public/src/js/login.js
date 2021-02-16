import "./modules/wasm.js";
import "../scss/login.scss";
import User from "./modules/user.js";

window.onload = function () {
    let form = document.getElementById("login-form");

    let id = document.getElementById("login-id");
    let idError = document.getElementById("login-id-error");
    let isIdError = false;

    let password = document.getElementById("login-password");
    let passwordError = document.getElementById("login-password-error");

    let check = function (isFirst) {
        let value = id.value;
        if (value.includes("@")) {
            if (!User.checkEmail(value)) {
                if (isFirst !== true) {
                    idError.innerText = "잘못된 이메일 형식입니다.";
                }
                isIdError = true;
            } else {
                id.classList.remove("danger");
                idError.innerText = "";
                isIdError = false;
            }
        } else {
            if (!User.checkUserId(value)) {
                if (isFirst !== true) {
                    idError.innerText = "잘못된 아이디 형식입니다.";
                }
                isIdError = true;
            } else {
                id.classList.remove("danger");
                idError.innerText = "";
                isIdError = false;
            }
        }
    };
    check(true);

    let isFormProcessing = false;
    form.onsubmit = function () {
        if (isFormProcessing) {
            return false;
        }
        isFormProcessing = true;

        (async function () {
            check();
            if (isIdError) {
                id.classList.add("danger");
                id.focus();
                isFormProcessing = false;
                return;
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

            await User.login(user, password.value, function () {
                passwordError.innerText = "아이디/이메일이 존재하지 않거나 비밀번호가 일치하지 않습니다.";
            });

            isFormProcessing = false;
        })();

        return false;
    };

    id.oninput = check;
};
