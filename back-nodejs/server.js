const express = require("express");
const app = express();
const path = require("path");
const wasm = require("wasm-sesskey");

app.get("/", (req, res) => {
  res.sendFile(path.join(__dirname + "/index.html"));
});

app.get("/about", (req, res) => {
  const rid = "kFNqugnZ57tHiPgxtmS2dAFAfjsmimLpQB";
  const key = wasm.generate_aes_key(rid).then(console.log).catch(console.error);
  res.sendFile(path.join(__dirname + "/about.html"));
});

app.listen(3000, () => {
  console.log("Listening on port 3000");
});
