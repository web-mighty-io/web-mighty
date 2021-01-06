// Login
let login_btn = document.getElementById('login-btn');
let id_login = document.getElementById('id-login');
let pw_login = document.getElementById('pw-login');

pw_login.addEventListener("keyup", (e) => {
    if (e.code === 'Enter') {
        e.preventDefault();
        login_btn.click();
    }
});

login_btn.onclick = function() {
    let req = new XMLHttpRequest();
    req.addEventListener("load", () => {
        if (req.status === 401) { // analyze HTTP status of the response
            id_login.classList.add('is-danger');
            pw_login.classList.add('is-danger');
            document.getElementById('login-false').innerText = 'False username or password';
        } else if (req.status === 302){
            // redirect to a new page
        } else {
            alert(`Error ${req.status}: ${req.statusText}`); // e.g. 404: Not Found
        }
    })
    req.open("POST", "/login");
    req.setRequestHeader("Content-Type", "application/json;charset=UTF-8");
    req.send(JSON.stringify({"username" : id_login.value, "pw" : pw_login.value }));
}

// Signup

