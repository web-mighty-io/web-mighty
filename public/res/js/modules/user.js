export let user = {
    async login(username, password, onError) {
        let res = await fetch("/login", {
            method: "post",
            headers: {
                "Accept": "application/json, text/plain, */*",
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                "user_id": username,
                password,
            })
        });
        if (res.ok) {
            let params = new URLSearchParams(window.location.search);
            let redirect = "/";
            if (params.has("back")) {
                redirect = params.get("back");
            }
            window.location.replace(redirect);
        } else {
            onError(res.text());
        }
    },
};