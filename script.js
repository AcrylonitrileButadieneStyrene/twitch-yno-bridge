(() => {
  if (window.__remote_script_loaded) return;
  window.__remote_script_loaded = true;

  write("Loaded.");

  const commands = {
    KeyPresses: handleKeypress,
    Bang: handleBang,
  };

  callbacks.onText.add(function (_, data) {
    const handler = commands[data.t];
    if (handler) handler(data.c);
    else write("Unhandled event: " + data.t);
  });

  const keys = Array(10)
    .fill("Digit")
    .map((x, i) => x + i)
    .concat("WDSAZX".split("").map((x) => "Key" + x))
    .concat("ShiftLeft")
    .map((code) => ({ code }))
    .map((x) => [
      new KeyboardEvent("keydown", x),
      new KeyboardEvent("keyup", x),
    ]);

  const canvas = document.getElementById("canvas");
  let nexts;
  let prev;
  function handleKeypress(indexes) {
    console.log(indexes);
    nexts = indexes.map((index) => keys[index]);
  }

  function handleBang(event) {
    if (event.t == "SwitchGame") {
      window.onbeforeunload = undefined;
      document.location.href = `${document.location.origin}/${event.c}`;
    }
  }

  setInterval(() => {
    let nextPair = nexts?.length ? nexts.shift() : undefined;
    if (!nextPair) {
      if (prev) {
        canvas.dispatchEvent(prev);
        prev = undefined;
      }

      return;
    }

    let [next, nextPrev] = nextPair;
    if (prev && prev != next) {
      canvas.dispatchEvent(prev);
      prev = undefined;
    }
    
    canvas.dispatchEvent(next);
    prev = nextPrev;
  }, 50);
})();
