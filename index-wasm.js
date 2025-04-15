import init, { generate_aes_key, store_key } from "./pkg/wasm_sesskey.js";
init().then(() => {
  const sesskey = generate_aes_key();
  sesskey.then(function(result) {
    document.getElementById("tracker-log-1").textContent = result.k;
  });
});
