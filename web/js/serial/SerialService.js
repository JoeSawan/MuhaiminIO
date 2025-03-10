export class SerialService {
  constructor() {
    this.encoder = new TextEncoder();
    this.decoder = new TextDecoder();
    this.serialPort = null;
    this.reader = null;
    this.writer = null;
    this.inputBuffer = "";
  }

  get isConnected() {
    return !!this.serialPort;
  }

  async connect() {
    this.serialPort = await navigator.serial.requestPort();
    await this.serialPort.open({ baudRate: 115200 });
    this.setupListeners();
  }

  async disconnect() {
    if (this.reader) await this.reader.cancel();
    if (this.writer) await this.writer.close();
    this.serialPort = null;
  }

  setupListeners() {
    // إضافة منطق استقبال البيانات هنا
  }

  setupListeners() {
    this.reader = this.serialPort.readable.getReader();
    this.writer = this.serialPort.writable.getWriter();

    this.readLoop();
  }

  async readLoop() {
    try {
      while (true) {
        const { value, done } = await this.reader.read();
        if (done) break;

        this.inputBuffer += this.decoder.decode(value);
        this.processBuffer();
      }
    } catch (error) {
      console.error("Reading error:", error);
    }
  }

  processBuffer() {
    const messages = this.inputBuffer.split("\n");
    this.inputBuffer = messages.pop() || "";

    messages.forEach((msg) => this.parseResponse(msg.trim()));
  }

  parseResponse(response) {
    const pattern = /^1 (\d+) (\d+)$/;
    const match = response.match(pattern);

    if (match) {
      const pinNumber = parseInt(match[1]);
      const value = parseInt(match[2]);
      const pinId = `A${pinNumber}`;

      // إرسال الحدث لتحديث الواجهة
      const event = new CustomEvent("analog-update", {
        detail: { pinId, value },
      });
      document.dispatchEvent(event);
    }
  }

  async readAnalogPin(pinNumber) {
    const command = `1 ${pinNumber}\n`;
    await this.writer.write(this.encoder.encode(command));
  }
}
