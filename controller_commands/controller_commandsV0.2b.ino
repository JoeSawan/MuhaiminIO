#define BUFFER_SIZE 64  // زيادة حجم المخزن لاستيعاب الأوامر الطويلة
char inputString[BUFFER_SIZE];
bool stringComplete = false;
int inputIndex = 0;

// تعريف دبابيس PWM المتاحة
const byte pwmPins[] = { 3, 5, 6, 9, 10, 11 };
const int pwmPinsCount = 6;

void setup() {
  Serial.begin(115200);  // بدء الاتصال التسلسلي
}

void loop() {
  receiveSerialData();  // استقبال البيانات في كل iteration

  if (stringComplete) {
    processCommand();  // معالجة الأمر عند اكتماله
    stringComplete = false;
  }
}

// دالة لاستقبال البيانات من السيريال
void receiveSerialData() {
  while (Serial.available() > 0) {
    char inChar = Serial.read();

    if (inputIndex < BUFFER_SIZE - 1) {
      inputString[inputIndex++] = inChar;

      if (inChar == '\n') {
        inputString[inputIndex] = '\0';  // إنهاء السلسلة
        stringComplete = true;
        inputIndex = 0;  // إعادة المؤشر
        return;
      }
    } else {
      // تجاوز سعة المخزن - إعادة التعيين
      inputIndex = 0;
      Serial.println("ERROR: Buffer overflow");
    }
  }
}

// دالة معالجة الأوامر مع التحقق من الصحة
void processCommand() {
  int commandID, param1 = -1, param2 = -1;
  int parsed = sscanf(inputString, "%d %d %d", &commandID, &param1, &param2);

  if (parsed < 1) {
    Serial.println("ERROR: Invalid format");
    return;
  }

  switch (commandID) {
    case 1:  // قراءة أنالوج
      if (param1 >= 0 && param1 <= 7) {
        ARPins(param1);
      } else {
        Serial.println("ERROR 1: Invalid analog pin (0-7)");
      }
      break;
    case 2:  // قراءة port
      if (param1 >= 2 && param1 <= 4) {
        RPINx(param1);
      } else {
        Serial.println("ERROR 2: Invalid port (2-4)");
      }
      break;
    case 3:  // كتابة منفذ
      if ((param1 == 2 || param1 == 4) && param2 >= 0 && param2 <= 255) {
        WPORT(param1, param2);
        Serial.println("OK 3");
      } else {
        Serial.println("ERROR 3: Invalid port(2,4) or value(0-255)");
      }
      break;
    case 4:  // تعين منفذ
      if ((param1 >= 2 && param1 <= 4) && param2 >= 0 && param2 <= 255) {
        WDDRx(param1, param2);
        Serial.println("OK 4");
      } else {
        Serial.println("ERROR 4: Invalid port(2-4) or value(0-255)");
      }
      break;
    case 5:                               // قراءة دبوس رقمي
      if (param1 >= 0 && param1 <= 21) {  // دعم الدبابيس 0-13 + A0-A7
        RPINx(param1);
      } else {
        Serial.println("ERROR 5: Invalid digital pin (0-21)");
      }
      break;
    case 6:  // كتابة دبوس رقمي
      if (param1 >= 0 && param1 <= 13 && (param2 == 0 || param2 == 1)) {
        pinMode(param1, OUTPUT);
        digitalWrite(param1, param2);
        Serial.println("OK 6");
      } else {
        Serial.println("ERROR 6: Invalid pin (0-13) or state (0/1)");
      }
      break;
    case 7:  // PWM
      if (isPWMPin(param1) && param2 >= 0 && param2 <= 255) {
        analogWrite(param1, param2);
        Serial.println("OK 7");
      } else {
        Serial.println("ERROR 7: Invalid PWM pin or value (0-255)");
      }
      break;

    default:
      Serial.println("ERROR: Unknown command");
  }
}
// التحقق من دبوس PWM
bool isPWMPin(int pin) {
  for (int i = 0; i < pwmPinsCount; i++) {
    if (pin == pwmPins[i]) return true;
  }
  return false;
}
// ========== دوال التنفيذ ==========
void ARPins(int pin) {
  Serial.print("1 ");
  Serial.print(pin);
  Serial.print(" ");
  Serial.println(analogRead(pin));
}
void RPINx(int port) {
  byte value;
  switch (port) {
    case 2: value = PINB; break;
    case 3: value = PINC; break;
    case 4: value = PIND; break;
  }
  Serial.print("5 ");
  Serial.print(port);
  Serial.print(" ");
  Serial.println(value);
} 
void WPORT(int port, int value) {
  switch (port) {
    case 2:
      PORTB = value;
      break;
    case 4:
      PORTD = value;
      break;
  }
}
void WDDRx(int port, int value) {
  switch (port) {
    case 2:
      DDRB = value;
      break;
    case 3:
      DDRC = value;
      break;
    case 4:
      DDRD = value;
      break;
  }
}