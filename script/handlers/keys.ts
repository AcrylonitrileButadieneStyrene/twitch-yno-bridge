import { canvas } from "../shared";

type Keypair = [KeyboardEvent, KeyboardEvent];

export const Keys: Keypair[] = Array(10)
    .fill("Digit")
    .map((x, i) => x + i)
    .concat("WDSAZX".split("").map((x) => "Key" + x))
    .concat("ShiftLeft")
    .map((code) => ({ code }))
    .map((x) => [
        new KeyboardEvent("keydown", x),
        new KeyboardEvent("keyup", x),
    ]);
    
export let queue: {
    keys: Keypair[], 
    looping: boolean,
} = { keys: [], looping: false };

export default function handleKeys(indexes: number[]) {
    queue.keys = indexes.map(index => Keys[index]);
    queue.looping = false;
}

let prev: KeyboardEvent;
setInterval(() => {
    if (!queue?.keys?.length) {
        if (prev) {
            canvas.dispatchEvent(prev);
            prev = undefined;
        }

        return;
    }

    let currentPair = queue.keys.shift();
    if (queue.looping) queue.keys.push(currentPair); // Not efficient. I don't care.

    let [next, nextPrev] = currentPair;
    if (prev && prev != next) {
        canvas.dispatchEvent(prev);
        prev = undefined;
    }

    canvas.dispatchEvent(next);
    prev = nextPrev;
}, 50);
