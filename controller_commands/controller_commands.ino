/*
  هذا الكود يمثل برنامج تحكم عن طريق الاتصال التسلسلي (Serial) للتحكم في دبابيس الأردوينو.
  يدعم الأوامر التالية:
  - قراءة الإشارات التناظرية (Analog Read)
  - قراءة/كتابة المنافذ الرقمية (Digital I/O)
  - التحكم في منافذ PWM
  - ضبط اتجاه المنافذ (Input/Output)
  يتم استقبال الأوامر عبر منفذ Serial بتنسيق: "رقم_الأمر معلمه1 معلمه2"
*/

#define BUFFER_SIZE 16        // حجم المخزن المؤقت لاستقبال البيانات
char inputString[BUFFER_SIZE]; // مخزن تخزين البيانات الواردة
bool stringComplete = false;  // مؤشر لاكتمال استقبال السلسلة
int inputIndex = 0;           // مؤشر الموقع الحالي في المخزن المؤقت
#define ANALOG_READ_DELAY 10  // تأخير بين قراءات الأنالوج

// تعريف دبابيس PWM المتاحة في الأردوينو (UNO/NANO)
const byte pwmPins[] = {3, 5, 6, 9, 10, 11}; 
const int pwmPinsCount = 6;   // عدد دبابيس PWM

void setup() {
  Serial.begin(115200);       // بدء الاتصال التسلسلي بسرعة 115200 باود
}

void loop() {
  receiveSerialData();        // استقبال البيانات في كل دورة

  if (stringComplete) {
    processCommand();         // معالجة الأمر عند اكتماله
    stringComplete = false;   // إعادة تعيين المؤشر
  }
}

// دالة استقبال البيانات من المنفذ التسلسلي
void receiveSerialData() {
  while (Serial.available() > 0 && !stringComplete) {
    char inChar = Serial.read();

    if (inputIndex < BUFFER_SIZE - 1) {
      inputString[inputIndex++] = inChar; // تخزين الحرف في المخزن المؤقت

      // التحقق من نهاية السلسلة (سطر جديد أو إرجاع)
      if (inChar == '\n' || inChar == '\r') { 
        if (inputIndex == 0) return;      // تجاهل السلاسل الفارغة
        inputString[inputIndex] = '\0';   // إنهاء السلسلة بعلامة NULL
        stringComplete = true;            // تأشير لاكتمال السلسلة
        inputIndex = 0;                   // إعادة تعيين المؤشر
        return;
      }
    } else {
      // في حالة امتلاء المخزن، إرسال خطأ وإعادة التعيين
      inputIndex = 0;
      Serial.println("ERROR: Buffer overflow");
    }
  }
}

// دالة معالجة الأوامر مع التحقق من الصحة
void processCommand() {
  int commandID, param1 = -1, param2 = -1;
  
  // تحليل السلسلة إلى أرقام باستخدام sscanf
  int parsed = sscanf(inputString, "%d %d %d", &commandID, &param1, &param2);

  if (parsed < 1) {
    Serial.println("ERROR: Invalid format"); // خطأ في التنسيق
    return;
  }

  switch (commandID) {
    case 1:  // قراءة إشارة تناظرية (Analog Read)
      if (param1 == 9) {
        // قراءة جميع الدبابيس التناظرية A0-A7
        for (int i = 0; i <= 7; i++) {
          ARPins(i);
          delay(ANALOG_READ_DELAY);
        }
      } else if (param1 >= 0 && param1 <= 7) {
        ARPins(param1); // قراءة دبوس تناظري معين
      } else {
        Serial.println("ERROR 1: Invalid analog pin (0-7 or '9' for all)");
      }
      break;

    case 2: // قراءة المنفذ الرقمي (Port Read)
      if (param1 == 5) {
        // قراءة جميع المنافذ B, C, D (الدبابيس الرقمية)
        for (int i = 2; i <= 4; i++) {
          RPINx(i);
          delay(10);
        }
      } else if (param1 >= 2 && param1 <= 4) {
        RPINx(param1); // قراءة منفذ معين
      } else {
        Serial.println("ERROR 2: Invalid port (2-4 or '5' for all port)");
      }
      break;

    case 3: // كتابة قيمة إلى المنفذ الرقمي (Port Write)
      if ((param1 >= 2 && param1 <= 4) && param2 >= 0 && param2 <= 255) {
        WPORT(param1, param2);
        Serial.println("OK 3");
      } else {
        Serial.println("ERROR 3: Invalid port(2- 4) or value(0-255)");
      }
      break;

    case 4: // ضبط اتجاه المنفذ (Port Direction)
      if ((param1 >= 2 && param1 <= 4) && param2 >= 0 && param2 <= 255) {
        WDDRx(param1, param2);
        Serial.println("OK 4");
      } else {
        Serial.println("ERROR 4: Invalid port(2-4) or value(0-255)");
      }
      break;

    case 5: // قراءة دبوس رقمي (Digital Read)
      if (param1 >= 0 && param1 <= 21) { 
        Serial.println(digitalRead(param1));
      } else {
        Serial.println("ERROR 5: Invalid digital pin (0-21)");
      }
      break;

    case 6: // كتابة دبوس رقمي (Digital Write)
      if (param1 >= 0 && param1 <= 19 && (param2 == 0 || param2 == 1)) {
        pinMode(param1, OUTPUT);
        digitalWrite(param1, param2);
        Serial.println("OK 6");
      } else {
        Serial.println("ERROR 6: Invalid pin (0-19) or state (0/1)");
      }
      break;

    case 7: // التحكم في PWM
      if (isPWMPin(param1) && param2 >= 0 && param2 <= 255) {
        analogWrite(param1, param2);
        Serial.println("OK 7");
      } else {
        Serial.println("ERROR 7: Invalid PWM pin or value (0-255)");
      }
      break;

    case 8: // ضبط اتجاه الدبوس (Input/Output)
      if (param1 >= 0 && param1 <= 19 && (param2 == 0 || param2 == 1)) {
        pinMode(param1, param2 ? OUTPUT : INPUT);
        Serial.println("OK 8");
      }
      break;

    default:
      Serial.println("ERROR: Unknown command"); // أمر غير معروف
  }
}

// دالة مساعدة للتحقق من صحة دبوس PWM
bool isPWMPin(int pin) {
  for (int i = 0; i < pwmPinsCount; i++) {
    if (pin == pwmPins[i]) return true;
  }
  return false;
}

// ========== الدوال التنفيذية ==========

// قراءة دبوس تناظري وإرسال النتيجة
void ARPins(int pin) {
  Serial.print("1 ");
  Serial.print(pin);
  Serial.print(" ");
  Serial.println(analogRead(pin));
}

// قراءة المنفذ الرقمي وإرسال القيمة
void RPINx(int port) {
  byte value;
  switch (port) {
    case 2: value = PINB; break; // قراءة المنفذ B (الدبابيس 8-13)
    case 3: value = PINC; break; // قراءة المنفذ C (الدبابيس A0-A5)
    case 4: value = PIND; break; // قراءة المنفذ D (الدبابيس 0-7)
  }
  Serial.print("5 ");
  Serial.print(port);
  Serial.print(" ");
  Serial.println(value);
}

// كتابة قيمة إلى المنفذ الرقمي مع تطبيق قناع للبتات الغير مستخدمة
void WPORT(int port, int value) {
  switch (port) {
    case 2:
      value &= 0b00111111; // قناع للمنفذ B (PB0-PB5 فقط)
      PORTB = value;
      break;
    case 3:
      value &= 0b00111111; // قناع للمنفذ C (PC0-PC5 فقط)
      PORTC = value;
      break;
    case 4:
      value &= 0b11111100; // قناع للمنفذ D (PD2-PD7 فقط)
      PORTD = value;
      break;
  }
}

// ضبط اتجاه منافذ المنفذ الرقمي
void WDDRx(int port, int value) {
  switch (port) {
    case 2:
      value &= 0b00111111; // قناع للمنفذ B
      DDRB = value;
      break;
    case 3:
      value &= 0b00111111; // قناع للمنفذ C
      DDRC = value;
      break;
    case 4:
      value &= 0b11111100; // قناع للمنفذ D
      DDRD = value;
      break;
  }
}