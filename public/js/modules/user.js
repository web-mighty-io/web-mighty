let sjcl = null;

/**
 * Simple SHA-256 function
 *
 * It uses faster browser support hash functions if possible.
 * If not, it uses js version of sha256 (slower).
 *
 * @link https://stackoverflow.com/a/48161723
 * @param {string} message
 * @returns {Promise<string>}
 */
async function sha256(message) {
    if (crypto.subtle.digest) {
        const msgBuffer = new TextEncoder().encode(message);
        const hashBuffer = await crypto.subtle.digest("SHA-256", msgBuffer);
        const hashArray = Array.from(new Uint8Array(hashBuffer));
        return hashArray.map(b => ("00" + b.toString(16)).slice(-2)).join("");
    } else {
        if (sjcl == null) {
            sjcl = await import ("../../../node_modules/sjcl/sjcl.js");
        }
        const bitArray = sjcl.hash.sha512.hash(message);
        return sjcl.codec.hex.fromBits(bitArray);
    }
}

/**
 * User class
 *
 * This doesn't save user passwords.
 */
class User {
    constructor(info) {
        this.id = info.id;
        this.email = info.email;
        this.name = info.name;
        this.token = info.token; // for register
    }

    get_email() {
        return this.email;
    }

    get_id() {
        return this.id;
    }

    get_name() {
        return this.name;
    }

    /**
     * Logins to the server
     *
     * `onError` is called
     *
     * @param {string} password
     * @param {function} onError
     * @returns {Promise<void>}
     */
    async login(password, onError) {
        let hashedPassword = await sha256(password);
        let res = await fetch("/login", {
            method: "post",
            headers: {
                "Accept": "application/json, text/plain, */*",
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                "user_id": this.id,
                "password": hashedPassword,
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
    }

    async preRegister(onError) {
        let res = await fetch("/pre-register", {
            method: "post",
            headers: {
                "Accept": "application/json, text/plain, */*",
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                "user_id": this.id,
                "email": this.email,
            })
        })
        if (res.ok) {
            window.location.replace("/");
        } else {
            onError(res.text());
        }
    }

    async register(password, onError) {
        let hashedPassword = await sha256(password);
        let res = await fetch("/register", {
            method: "post",
            headers: {
                "Accept": "application/json, text/plain, */*",
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                "user_id": this.id,
                "name": this.name,
                "password": hashedPassword,
                "token": this.token,
            })
        })
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
    }

    async logout(onError) {
        let res = await fetch("/logout", {
            method: "get",
            headers: {
                "Accept": "application/json, text/plain, */*",
            },
        })
        if (res.ok) {
            window.location.replace("/");
        } else {
            onError(res.text());
        }
    }

    async delete(password, onError) {
        let hashedPassword = await sha256(password);
        let res = await fetch("/delete-user", {
            method: "delete",
            headers: {
                "Accept": "application/json, text/plain, */*",
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                "user_id": this.id,
                "password": hashedPassword,
            })
        })
        if (res.ok) {
            window.location.replace("/");
        } else {
            onError(res.text());
        }
    }
}

export default User;