export function random64(): bigint {
    const bytes = new BigUint64Array(1);
    crypto.getRandomValues(bytes);
    return bytes[0];
}