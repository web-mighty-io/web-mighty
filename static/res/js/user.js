export let user = {
    async login(username, password, on_error) {
        let res = await fetch("/login", {
            method: "post",
            headers: {
                "Accept": "application/json, text/plain, */*",
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                user_id: username,
                password: password,
            })
        });
        if (res.ok) {
            let params = new URLSearchParams(window.location.search);
            let redirect_to = "/";
            if (params.has("back")) {
                redirect_to = params.get("back");
            }
            window.location.replace(redirect_to);
        } else {
            on_error(res.text());
        }

    },
};