import init, { generate_aes_key } from "./pkg/wasm_sesskey.js";
init().then(() => {
  const rid = document.getElementById("rid").value;
  const sesskey = generate_aes_key(rid);
  sesskey.then(function (result) {
    document.getElementById("tracker-log-1").textContent = result.k;
  });
});
