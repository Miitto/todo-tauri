import "./styles.css";
import App from "./App.svelte";

import '@fortawesome/fontawesome-free/css/all.min.css'

import * as jq from "jquery";

const app = new App({
  target: document.getElementById("app")!,
});

export default app;
