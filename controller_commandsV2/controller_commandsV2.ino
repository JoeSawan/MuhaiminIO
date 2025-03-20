#include <Arduino.h>

#define STX 0x02
#define ETX 0x03
#define OK 0x11
#define BUFFER_SIZE 16
#define MAX_PARAMS 2

byte inputBuffer[BUFFER_SIZE];
byte bufferIndex = 0;
bool packetStarted = false;

const byte pwmPins[] = { 3, 5, 6, 9, 10, 11 };
const byte pwmPinsCount = 6;

void setup() {
  Serial.begin(115200);
}

void loop() {
  receivePacket();
}

void receivePacket() {
  while (Serial.available() > 0) {
    byte inByte = Serial.read();
    if (inByte == STX) {
      bufferIndex = 0;
      packetStarted = true;
    } else if (inByte == ETX && packetStarted) {
      if (bufferIndex > 0) {
        processPacket(inputBuffer, bufferIndex);
      }
      packetStarted = false;
    } else if (packetStarted && bufferIndex < BUFFER_SIZE) {
      inputBuffer[bufferIndex++] = inByte;
    }
  }
}

void processPacket(byte* data, byte length) {
  if (length < 2) {      // commandID + 1 param
    sendError(0x01, 0);  // خطأ في البارامترات
    return;
  }  // commandID + 1 param
  byte commandID = data[0];
  byte params[MAX_PARAMS] = { 0 };
  for (byte i = 1; i < length && i - 1 < MAX_PARAMS; i++) {
    params[i - 1] = data[i];
  }
  switch (commandID) {
    case 0x01:
      handleAnalogRead(params[0]);
      break;
    case 0x02:
      handlePortRead(params[0]);
      break;
    case 0x03:
      handlePortWrite(params[0], params[1]);
      break;
    case 0x04:
      handlePortDirection(params[0], params[1]);
      break;
    case 0x05:
      handleDigitalRead(params[0]);
      break;
    case 0x06:
      handleDigitalWrite(params[0], params[1]);
      break;
    case 0x07:
      handlePWMWrite(params[0], params[1]);
      break;
    case 0x08:
      handlePinMode(params[0], params[1]);
      break;
    default:
      sendError(0x00, commandID);
  }
}

void handleAnalogRead(byte pin) {
  if (pin <= 7) {
    sendAnalogResponse(pin, analogRead(pin));
  } else {
    sendError(0x01, pin);
  }
}

void handleDigitalWrite(byte pin, byte state) {
  if (digitalPinToPort(pin) == NOT_A_PIN || (state != 0 && state != 1)) {
    sendError(0x06, pin);
    return;
  }
  pinMode(pin, OUTPUT);
  digitalWrite(pin, state);
  sendAck(0x06);
}

void handlePortRead(byte port) {
  byte value = 0;
  switch (port) {
    case 0x02: value = PINB; break;
    case 0x03: value = PINC; break;
    case 0x04: value = PIND; break;
    default: sendError(0x02, port); return;
  }
  byte response[1] = { value };
  sendPacket(response, 1);
}

void handlePortWrite(byte port, byte value) {
  switch (port) {
    case 0x02: PORTB = value; break;
    case 0x03: PORTC = value; break;
    case 0x04: PORTD = value; break;
    default: sendError(0x03, port); return;
  }
  sendAck(0x03);
}

void handlePortDirection(byte port, byte value) {
  switch (port) {
    case 0x02: DDRB = value; break;
    case 0x03: DDRC = value; break;
    case 0x04: DDRD = value; break;
    default: sendError(0x04, port); return;
  }
  sendAck(0x04);
}

void handleDigitalRead(byte pin) {
  if (digitalPinToPort(pin) == NOT_A_PIN) {
    sendError(0x05, pin);
    return;
  }
  byte response[2] = { 0x05, digitalRead(pin) };
  sendPacket(response, 2);
}

void handlePWMWrite(byte pin, byte value) {
  if (!isPWMPin(pin)) {
    sendError(0x07, pin);
    return;
  }
  analogWrite(pin, value);
  sendAck(0x07);
}

void handlePinMode(byte pin, byte mode) {
  if (digitalPinToPort(pin) == NOT_A_PIN || mode > 1) {
    sendError(0x08, pin);
    return;
  }
  pinMode(pin, mode ? OUTPUT : INPUT);
  sendAck(0x08);
}

void sendPacket(byte* data, byte length) {
  Serial.write(STX);
  Serial.write(data, length);
  Serial.write(ETX);
  Serial.flush();  // تأكد من إرسال جميع البيانات قبل الانتقال
}

void sendAnalogResponse(byte pin, int value) {
  byte response[4] = { 0x01, pin, highByte(value), lowByte(value) };
  sendPacket(response, 4);
}

void sendAck(byte commandID) {
  byte ack[1] = { commandID };  // تأكد من أن الرد يحتوي على 3 بايتات كاملة
  sendPacket(ack, 1);
}


void sendError(byte errorCode, byte detail) {
  byte error[2] = { 0x80 | errorCode, detail };
  sendPacket(error, 2);
}

bool isPWMPin(byte pin) {
  switch (pin) {
    case 3:
    case 5:
    case 6:
    case 9:
    case 10:
    case 11:
      return true;
    default:
      return false;
  }
}
