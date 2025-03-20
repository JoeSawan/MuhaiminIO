#include <Arduino.h>

// ------ ثوابت البروتوكول ------
#define STX 0x02
#define ETX 0x03
#define BUFFER_SIZE 32
#define MAX_PARAMS 2

enum CommandCodes {
  CMD_ANALOG_READ  = 0xAE, // A: Analog  E:read
  CMD_PORT_WRITE   = 0xBF, // B: PORT F:Write
  CMD_DDR_SET      = 0xDD, // D: DDR
  CMD_PWM_WRITE    = 0xE4, // E: PWM
  CMD_PIN_READ     = 0xFE  // F: PIN  E:read
};

// ------ أكواد الأخطاء ------
enum ErrorCodes {
  ERR_INVALID_CMD   = 0x01,
  ERR_BAD_PARAM     = 0x02,
  ERR_PIN_RANGE     = 0x03,
  ERR_PORT_RANGE    = 0x04,
  ERR_PWM_NOT_SUPP  = 0x05
};

// ------ المتغيرات العامة ------
byte inputBuffer[BUFFER_SIZE];
byte bufferIndex = 0;
bool packetStarted = false;

const byte pwmPins[] = {3, 5, 6, 9, 10, 11};

void setup() {
  Serial.begin(115200);
}

void loop() {
  receivePacket();
}

// ------ نظام استقبال الحزم ------
void receivePacket() {
  while (Serial.available() > 0) {
    byte inByte = Serial.read();
    
    if (inByte == STX) {
      bufferIndex = 0;
      packetStarted = true;
    } 
    else if (inByte == ETX && packetStarted) {
      processPacket(inputBuffer, bufferIndex);
      packetStarted = false;
    } 
    else if (packetStarted && bufferIndex < BUFFER_SIZE) {
      inputBuffer[bufferIndex++] = inByte;
    }
  }
}

// ------ نظام معالجة الأوامر ------
void processPacket(byte* data, byte length) {
  if (length < 1) {
    sendError(ERR_INVALID_CMD, 0x00);
    return;
  }

  byte command = data[0];
  byte params[MAX_PARAMS] = {0};
  
  for(byte i=1; i<length; i++){
    if(i-1 < MAX_PARAMS) params[i-1] = data[i];
  }

  switch(command){
    case CMD_ANALOG_READ:
      if(length != 2) sendError(ERR_BAD_PARAM, command);
      else handleAnalogRead(params[0]);
      break;
      
    case CMD_PORT_WRITE:
      if(length != 3) sendError(ERR_BAD_PARAM, command);
      else handlePortWrite(params[0], params[1]);
      break;
      
    case CMD_DDR_SET:
      if(length != 3) sendError(ERR_BAD_PARAM, command);
      else handleDDRSet(params[0], params[1]);
      break;
      
    case CMD_PWM_WRITE:
      if(length != 3) sendError(ERR_BAD_PARAM, command);
      else handlePWMWrite(params[0], params[1]);
      break;
      
    case CMD_PIN_READ:
      if(length != 2) sendError(ERR_BAD_PARAM, command);
      else handlePinRead(params[0]);
      break;
      
    default:
      sendError(ERR_INVALID_CMD, command);
  }
}

// ------ معالجة الأوامر ------
void handleAnalogRead(byte pin) {
  if(pin > A7) {
    sendError(ERR_PIN_RANGE, pin);
    return;
  }
  int value = analogRead(pin);
  byte response[] = {CMD_ANALOG_READ, pin, highByte(value), lowByte(value)};
  sendPacket(response, sizeof(response));
}

void handlePortWrite(byte port, byte value) {
  switch(port){
    case 0xBB: PORTB = value; break;
    case 0xCC: PORTC = value; break;
    case 0xDD: PORTD = value; break;
  }
  sendAck(CMD_PORT_WRITE);
}

void handleDDRSet(byte port, byte mode) {
  switch(port){
    case 0xBB: DDRB = mode; break;
    case 0xCC: DDRC = mode; break;
    case 0xDD: DDRD = mode; break;
  }
  sendAck(CMD_DDR_SET);
}

void handlePWMWrite(byte pin, byte value) {
  bool valid = false;
  for(byte p : pwmPins){
    if(pin == p) {
      valid = true;
      break;
    }
  }
  if(!valid){
    sendError(ERR_PWM_NOT_SUPP, pin);
    return;
  }
  analogWrite(pin, value);
  sendAck(CMD_PWM_WRITE);
}

void handlePinRead(byte port) {
  byte value;
  switch(port){
    case 0xBB: value = PINB; break;
    case 0xCC: value = PINC; break;
    case 0xDD: value = PIND; break;
  }
  byte response[] = {CMD_PIN_READ, port, value};
  sendPacket(response, sizeof(response));
}

// ------ نظام الإرسال ------
void sendPacket(byte* data, byte length) {
  Serial.write(STX);
  Serial.write(data, length);
  Serial.write(ETX);
  Serial.flush();
}

void sendAck(byte command) {
  byte ack[] = {command, 0x01};
  sendPacket(ack, sizeof(ack));
}

void sendError(byte errorCode, byte details) {
  byte error[] = {0xEE, errorCode, details};
  sendPacket(error, sizeof(error));
}