import { Keys, queue } from "../keys";

export default function handleLoop(indexes: number[]) {
    queue.keys = indexes.map(index => Keys[index]);
    queue.looping = true;
}