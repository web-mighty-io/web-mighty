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
        return hashArray.map((b) => ("00" + b.toString(16)).slice(-2)).join("");
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

    getEmail() {
        return this.email;
    }

    getId() {
        return this.id;
    }

    getName() {
        return this.name;
    }

    /**
     * Validates if user id doesn't exist in server
     *
     * @param {string} userId
     * @returns {Promise<void>}
     */
    static async validateUserId(userId) {
        console.log(userId);
        // todo
    }

    /**
     * Validates if email doesn't exist in server
     *
     * @param {string} email
     * @returns {Promise<void>}
     */
    static async validateEmail(email) {
        console.log(email);
        // todo
    }

    /**
     * Check if user id is in right format
     *
     * @param {string} userId
     * @returns {boolean}
     */
    static checkUserId(userId) {
        console.log(userId);
        // todo
    }

    /**
     * Check if email is in right format
     *
     * @param {string} email
     * @returns {boolean}
     */
    static checkEmail(email) {
        console.log(email);
        // todo
    }

    /**
     * Check if password is in right format
     *
     * @param {string} password
     * @returns {boolean}
     */
    static checkPassword(password) {
        console.log(password);
        // todo
    }

    /**
     * Check if username is in right format
     *
     * @param {string} name
     * @returns {boolean}
     */
    static checkUserName(name) {
        console.log(name);
        // todo
    }

    /**
     * Logins to the server
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

    /**
     * Pre-registers to the server
     *
     * @param {function} onError
     * @returns {Promise<void>}
     */
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
        });
        if (res.ok) {
            window.location.replace("/");
        } else {
            onError(res.text());
        }
    }

    /**
     * Registers to server
     *
     * @param {string} password
     * @param {function} onError
     * @returns {Promise<void>}
     */
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

    /**
     * Logout from server
     *
     * @param {function} onError
     * @returns {Promise<void>}
     */
    async logout(onError) {
        let res = await fetch("/logout", {
            method: "get",
            headers: {
                "Accept": "application/json, text/plain, */*",
            },
        });
        if (res.ok) {
            window.location.replace("/");
        } else {
            onError(res.text());
        }
    }

    /**
     * Deletes user from server
     *
     * @param {string} password
     * @param {function} onError
     * @returns {Promise<void>}
     */
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
        });
        if (res.ok) {
            window.location.replace("/");
        } else {
            onError(res.text());
        }
    }
}

export default User;