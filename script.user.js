// ==UserScript==
// @name        Control
// @match       https://ynoproject.net/*
// @grant       none
// ==/UserScript==

let display;
window.addEventListener("load", () => {
  const canvas = document.getElementById("canvas");
  display = document.createElement("div");
  display.id = "display";
  canvas.parentNode.appendChild(display);
  write("Starting...");
  createSocket();
});

function write(message, duration = 3_000) {
  let text = document.createElement("div");
  text.textContent = message;
  display.appendChild(text);
  setTimeout(() => text.parentNode.removeChild(text), duration);
}

const callbacks = {
  onOpen: new Set(),
  onText: new Set(),
  onBinary: new Set(),
};

let socket;
const delay = ms => new Promise(res => setTimeout(res, ms));
function createSocket() {
  if (socket?.readyState < 2) return;

  socket = new WebSocket("ws://localhost:6248");
  socket.binaryType = "arraybuffer";
  socket.onclose = () => (write("Connection lost."), delay(1_000)).then(createSocket);
  socket.onopen = () => callbacks.onOpen.forEach(cb => cb(socket));
  socket.onmessage = event => {
    if (typeof event.data == "string")
      callbacks.onText.forEach(cb => cb(event, JSON.parse(event.data)))
    else callbacks.onBinary.forEach(cb => cb(event, new Uint8Array(event.data)));
  };
}

let firstOpen = true;
callbacks.onOpen.add(() => {
  write("Connected to control socket.");

  if (firstOpen) {
    firstOpen = false;
    socket.send(new Uint8Array([0x7F]));
  }
});

const exec = __script_src => { eval(__script_src); };
function runScript(_, data) {
  if (data[0] != 0x7F) return;

  const script = new TextDecoder().decode(data.slice(1));
  exec(script);

  callbacks.onBinary.delete(runScript);
}
callbacks.onBinary.add(runScript);
