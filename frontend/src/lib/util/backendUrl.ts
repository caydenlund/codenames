export function backendUrl(path: string) {
    return `${import.meta.env.VITE_BACKEND || ""}${path}`;
}
