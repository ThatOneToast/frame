import { mount } from "@frame/runtime-dom";
import appIr from "./generated/app.ir";
import { handlers } from "./handlers.js";

const app = mount(appIr, {
    component: "IdeApp",
    target: document.getElementById("app")!,
    handlers,
});

(window as any).frameApp = app;
