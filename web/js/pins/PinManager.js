import { pwmPins, analogPins } from "../config.js";
import { PinComponent } from "./PinComponent.js";

export class PinManager {
  constructor(containerId) {
    this.container = document.getElementById(containerId);
    this.pins = this.createPins();
  }

  createPins() {
    return Array.from({ length: 21 }, (_, i) => {
      const isAnalog = i >= 14;
      const id = isAnalog ? `A${i - 14}` : `D${i}`;

      return new PinComponent({
        id,
        mode: 0,
        modes: this.getModesForPin(id),
        value: 0,
      });
    });
  }

  getModesForPin(id) {
    if (pwmPins.has(id)) return ["💡", "⚡", "🔌", "🌓"];
    if (analogPins.has(id)) return ["💡", "⚡", "📊"];
    return ["💡", "⚡", "🔌"];
  }

  render() {
    this.pins.forEach((pin) => {
      this.container.appendChild(pin.element);
    });
  }
}
