export default function handleSwitchGame(url: string) {
    window.onbeforeunload = null;
    document.location.href = `${document.location.origin}/${url}`;
}
