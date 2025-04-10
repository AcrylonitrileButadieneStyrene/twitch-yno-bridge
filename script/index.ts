import handleBang from "./handlers/bang";
import handleKeys from "./handlers/keys";
import "./services";

declare global {
    var __remote_script_loaded: boolean | undefined;
    function write(message: string, duration?: number): void;
    const callbacks: {
        onText: Set<((event: MessageEvent, data: any) => void)>,
        onBinary: Set<((event: MessageEvent, data: Uint8Array) => void)>,
        onOpen: Set<(() => void)>,
    };
}

if (window.__remote_script_loaded) throw new Error("Tried to run twice.");
window.__remote_script_loaded = true;

write("Loaded.");

const commands = {
    Keys: handleKeys,
    Bang: handleBang,
};

callbacks.onText.add(function (_, data) {
    const handler = commands[data.t];
    if (handler) handler(data.c);
    else write("Unhandled event: " + data.t);
});
