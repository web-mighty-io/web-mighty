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
export async function sha256(message) {
    if (crypto.subtle.digest) {
        const msgBuffer = new TextEncoder().encode(message);
        const hashBuffer = await crypto.subtle.digest("SHA-256", msgBuffer);
        const hashArray = Array.from(new Uint8Array(hashBuffer));
        return hashArray.map((b) => ("00" + b.toString(16)).slice(-2)).join("");
    } else {
        if (!this.sha) {
            this.sha = await import ("js-sha256");
        }
        return this.sha.sha256(message);
    }
}

/**
 * Simple SHA-512 function
 *
 * It uses faster browser support hash functions if possible.
 * If not, it uses js version of sha256 (slower).
 *
 * @param {string} message
 * @returns {Promise<string>}
 */
export async function sha512(message) {
    if (crypto.subtle.digest) {
        const msgBuffer = new TextEncoder().encode(message);
        const hashBuffer = await crypto.subtle.digest("SHA-512", msgBuffer);
        const hashArray = Array.from(new Uint8Array(hashBuffer));
        return hashArray.map((b) => ("00" + b.toString(16)).slice(-2)).join("");
    } else {
        if (!this.sha) {
            this.sha = await import ("js-sha512");
        }
        return this.sha.sha512(message);
    }
}
