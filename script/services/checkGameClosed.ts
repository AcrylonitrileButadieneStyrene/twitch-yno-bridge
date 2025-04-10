import { canvas } from "../shared";

setInterval(() => {
    if (canvas.width) return;

    write("Game was closed. Reloading page.");
    setTimeout(() => {
        window.onbeforeunload = undefined;
        document.location.reload();
    }, 2_500);
}, 10_000);