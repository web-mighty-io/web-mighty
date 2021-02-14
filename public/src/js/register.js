import "./modules/wasm.js";
import "../scss/register.scss";
import User from "./modules/user.js";

window.onload = function () {
    let form = document.getElementById("register-form");

    let name = document.getElementById("register-name");
    let nameError = document.getElementById("register-name-error");
    let isNameError = false;

    let password = document.getElementById("register-password");
    let passwordError = document.getElementById("register-password-error");
    let isPasswordError = false;

    let passwordCheck = document.getElementById("register-password-check");
    let passwordCheckError = document.getElementById("register-password-check-error");
    let isPasswordCheckError = false;

    let token = document.getElementById("register-token").innerText;
    let userId = document.getElementById("register-id").innerText;

    let checkName = function (isFirst) {
        if (User.checkUserName(name.value)) {
            name.classList.remove("danger");
            nameError.innerText = "";
            isNameError = false;
        } else {
            if (isFirst !== true) {
                nameError.innerText = "이름은 특수문자를 포함하지 않아야 합니다.";
            }
            isNameError = true;
        }
    };

    let checkPassword = function (isFirst) {
        if (User.checkPassword(password.value)) {
            password.classList.remove("danger");
            passwordError.innerText = "";
            isPasswordError = false;
        } else {
            if (isFirst !== true) {
                passwordError.innerText = "비밀번호는 소문자, 대문자, 숫자, 특수문자중 3가지 이상을 포함해야 합니다.";
            }
            isPasswordError = true;
        }
    };

    let checkPasswordCheck = function (isFirst) {
        if (password.value === passwordCheck.value) {
            passwordCheck.classList.remove("danger");
            passwordCheckError.innerText = "";
            isPasswordCheckError = false;
        } else {
            if (isFirst !== true) {
                passwordCheckError.innerText = "비밀번호가 일치하지 않습니다.";
            }
            isPasswordCheckError = true;
        }
    };

    form.onsubmit = function () {
        checkName();
        if (isNameError) {
            name.classList.add("danger");
            name.focus();
            return false;
        }

        checkPassword();
        if (isPasswordError) {
            password.classList.add("danger");
            password.focus();
            return false;
        }

        checkPasswordCheck();
        if (isPasswordCheckError) {
            passwordCheck.classList.add("danger");
            passwordCheck.focus();
            return false;
        }

        let user = new User({
            info: {
                id: userId,
                name: name.value,
            },
            token
        });

        User.register(new User({
            info: {
                id: userId,
                name: name.value,
            },
            token
        }), password.value);

        return false;
    };

    name.onchange = checkName;
    password.onchange = function () {
        checkPassword();
        checkPasswordCheck();
    };
    passwordCheck.onchange = checkPasswordCheck;
};
