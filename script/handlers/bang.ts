import handleSwitchGame from "./bang/switchGame";
import handleLoop from "./bang/loop";

const commands = {
    "SwitchGame": handleSwitchGame,
    "Loop": handleLoop,
};

export default function handleBang(data: any) {
    const handler = commands[data.t];
    if (handler) handler(data.c);
    else write("Unhandled bang event: " + data.t);
}
