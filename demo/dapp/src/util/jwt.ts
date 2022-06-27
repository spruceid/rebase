
const encode = (c): string => {
    return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);
};

export const parseJWT = (jwt_str: string): any => {
    const v = jwt_str.split('.');

    if (v.length !== 3) {
        throw new Error("Invalid JWT format");
    }

    const u = v[1];
    const b64 = u.replace(/-/g, '+').replace(/_/g, '/');
    const encoded = atob(b64).split('').map(encode).join('')
    const json_str = decodeURIComponent(encoded);

    return JSON.parse(json_str);
}