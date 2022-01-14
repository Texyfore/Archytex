import express from "express";

express.static.mime.define({ "application/wasm": ["wasm"] });

const PORT = 3000;
const app = express();

app.use(express.static("public"));
app.use("/editor", express.static("../pkg"));

app.listen(PORT, () => {
  console.log(`Listening on port ${PORT}`);
});
