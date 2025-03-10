import { SerialManager } from "./serial/SerialManager.js";
import { PinManager } from "./pins/PinManager.js";

document.addEventListener("DOMContentLoaded", () => {
  // تهيئة مدير الاتصال التسلسلي
  const serialManager = new SerialManager();
  serialManager.init();

  // تهيئة مدير الـ Pins
  const pinManager = new PinManager("container");
  pinManager.render();
});
