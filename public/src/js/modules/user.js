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
        if (!this.sjcl) {
            this.sjcl = await import ("sjcl");
        }
        const bitArray = this.sjcl.hash.sha512.hash(message);
        return this.sjcl.codec.hex.fromBits(bitArray);
    }
}

/**
 * User class
 *
 * This doesn't save user passwords.
 */
class User {
    constructor(conf) {
        this.info = conf.info;
        this.token = conf.token;
        this.status = conf.status;
    }

    /**
     * Validates if user id doesn't exist in server
     *
     * @param {string} userId
     * @param {function} onError
     * @returns {Promise<void>}
     */
    static async validateUserId(userId, onError) {
        let res = await fetch("/validate-user-id", {
            method: "post",
            headers: {
                "Accept": "application/json, text/plain, */*",
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                "user_id": userId
            })
        });
        if (res.ok) {
            let json = await res.json();
            if (json["user_id"] === userId) {
                return json.exists;
            } else {
                onError("User id doesn't match");
            }
        } else {
            onError(await res.text());
        }
    }

    /**
     * Validates if email doesn't exist in server
     *
     * @param {string} email
     * @param {function} onError
     * @returns {Promise<void>}
     */
    static async validateEmail(email, onError) {
        let res = await fetch("/validate-email", {
            method: "post",
            headers: {
                "Accept": "application/json, text/plain, */*",
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                email,
            })
        });
        if (res.ok) {
            let json = await res.json();
            if (json["email"] === email) {
                return json.exists;
            } else {
                onError("Email doesn't match");
            }
        } else {
            onError(await res.text());
        }
    }

    /**
     * Check if user id is in right format
     *
     * User id can contain alphabets, numbers, `.`, `_` and `-`
     *
     * @param {string} userId
     * @returns {boolean}
     */
    static checkUserId(userId) {
        return /^[a-zA-z0-9._\-]{4,31}$/.test(userId);
    }

    /**
     * Check if email is in right format
     *
     * @param {string} email
     * @returns {boolean}
     */
    static checkEmail(email) {
        return /^[a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/.test(email) && email.length <= 63;
    }

    /**
     * Check if password is in right format
     *
     * Password must have 3 or more following:
     * - contains number
     * - contains lowercase alphabet
     * - contains uppercase alphabet
     * - contains special character
     *
     * @param {string} password
     * @returns {boolean}
     */
    static checkPassword(password) {
        const hasNumber = /[0-9]/.test(password);
        const hasLowercase = /[a-z]/.test(password);
        const hasUppercase = /[A-Z]/.test(password);
        const hasSpecialCharacter = /[!@#$%^&*()_+-=:;'\[\]{}\\|<>?,./]/.test(password);

        return hasNumber + hasLowercase + hasUppercase + hasSpecialCharacter >= 3 && password.length >= 8 && password.length <= 100;
    }


    /**
     * Check if username is in right format
     *
     * Username can have all characters except special characters
     *
     * @param {string} name
     * @returns {boolean}
     */
    static checkUserName(name) {
        return /^[^!@#$%^&*()_+-=:;'\[\]{}\\|<>?,./]{4,63}$/.test(name);
    }

    /**
     * Logins to the server
     *
     * @param {User} user
     * @param {string} password
     * @param {function} onError
     * @returns {Promise<void>}
     */
    static async login(user, password, onError) {
        let hashedPassword = await sha256(password);
        let res = await fetch("/login", {
            method: "post",
            headers: {
                "Accept": "application/json, text/plain, */*",
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                "user_id": user.info.id,
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
            onError(await res.text());
        }
    }

    /**
     * Pre-registers to the server
     *
     * @param {User} user
     * @param {function} onError
     * @returns {Promise<void>}
     */
    static async preRegister(user, onError) {
        let res = await fetch("/pre-register", {
            method: "post",
            headers: {
                "Accept": "application/json, text/plain, */*",
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                "user_id": user.info.id,
                "email": user.info.email,
            })
        });
        if (res.ok) {
            window.location.replace("/");
        } else {
            onError(await res.text());
        }
    }

    /**
     * Registers to server
     *
     * @param {User} user
     * @param {string} password
     * @param {function} onError
     * @returns {Promise<void>}
     */
    static async register(user, password, onError) {
        let hashedPassword = await sha256(password);
        let res = await fetch("/register", {
            method: "post",
            headers: {
                "Accept": "application/json, text/plain, */*",
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                "user_id": user.info.id,
                "name": user.info.name,
                "password": hashedPassword,
                "token": user.token,
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
            onError(await res.text());
        }
    }

    /**
     * Logout from server
     *
     * @param {function} onError
     * @returns {Promise<void>}
     */
    static async logout(onError) {
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
     * @param {User} user
     * @param {string} password
     * @param {function} onError
     * @returns {Promise<void>}
     */
    static async delete(user, password, onError) {
        let hashedPassword = await sha256(password);
        let res = await fetch("/delete-user", {
            method: "delete",
            headers: {
                "Accept": "application/json, text/plain, */*",
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                "user_id": user.info.id,
                "password": hashedPassword,
            })
        });
        if (res.ok) {
            window.location.replace("/");
        } else {
            onError(await res.text());
        }
    }
}

export default User;