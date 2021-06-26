import "./modules/wasm.js";
import "../scss/pre-register.scss";
import User from "./modules/user.js";

window.onload = function () {
    let form = document.getElementById("pre-register-form");

    let email = document.getElementById("pre-register-email");
    let emailError = document.getElementById("pre-register-email-error");
    let isEmailError = false;

    let id = document.getElementById("pre-register-id");
    let idError = document.getElementById("pre-register-id-error");
    let isIdError = false;

    let checkId = function (isFirst) {
        if (User.checkUserId(id.value)) {
            id.classList.remove("danger");
            idError.innerText = "";
            isIdError = false;
        } else {
            if (isFirst !== true) {
                idError.innerText = "아이디는 영문, 숫자, ., _, -만 포함해야 하며 4자 이상이어야 합니다.";
            }
            isIdError = true;
        }
    };
    checkId(true);

    let checkEmail = function (isFirst) {
        if (User.checkEmail(email.value)) {
            email.classList.remove("danger");
            emailError.innerText = "";
            isEmailError = false;
        } else {
            if (isFirst !== true) {
                emailError.innerText = "잘못된 이메일 형식입니다.";
            }
            isEmailError = true;
        }
    };
    checkEmail(true);

    let isFormProcessing = false;
    form.onsubmit = function () {
        if (isFormProcessing) {
            return false;
        }
        isFormProcessing = true;

        (async function () {
            checkEmail();
            if (!isEmailError && await User.validateEmail(email.value)) {
                isEmailError = true;
                emailError.innerText = "사용중인 이메일입니다.";
            }
            if (isEmailError) {
                email.classList.add("danger");
                email.focus();
                isFormProcessing = false;
                return;
            }

            checkId();
            if (!isIdError && await User.validateUserId(id.value)) {
                isIdError = true;
                idError.innerText = "사용중인 아이디 입니다.";
            }
            if (isIdError) {
                id.classList.add("danger");
                id.focus();
                isFormProcessing = false;
                return;
            }

            User.preRegister(new User({
                info: {
                    id: id.value,
                    email: email.value,
                }
            }));
            isFormProcessing = false;
        })();

        return false;
    };

    id.oninput = checkId;
    email.oninput = checkEmail;
};
