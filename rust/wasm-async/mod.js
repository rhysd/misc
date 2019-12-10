export function hello(name) {
    const i = Math.random();
    if (i < 0.3) {
        return `Hello, ${name}`;
    } else if (i < 0.6) {
        return `こんにちは ${name}`;
    } else if (i < 0.9) {
        return `您好${name}`;
    } else {
        return `Bowwow ${name}!`;
    }
}

export async function fetchBytes(url) {
    const res = await fetch(url);
    if (!res.ok) {
        throw new Error(`Request failed with status ${res.status} (${res.statusText})`);
    }
    const buf = await res.arrayBuffer();
    return new Uint8Array(buf);
}
