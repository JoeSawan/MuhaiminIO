#define BUFFER_SIZE 30  // حجم المخزن المؤقت لتخزين البيانات المستقبلة
char inputString[BUFFER_SIZE];  // مصفوفة لتخزين البيانات المستلمة من السيريال
bool stringComplete = false;  // متغير يشير إلى اكتمال استقبال الأمر
int inputLength = 0;  // متغير لتتبع طول البيانات المستقبلة

void setup() {
  Serial.begin(115200);  // بدء الاتصال التسلسلي بسرعة 115200 بت في الثانية
}

void loop() {
  // عند اكتمال استقبال أمر من السيريال، يتم معالجته
  if (stringComplete) {
    stringComplete = false;  // إعادة ضبط حالة استقبال البيانات
    processCommand();  // استدعاء دالة معالجة الأوامر
    inputLength = 0;  // إعادة تعيين طول البيانات المستلمة
  }
}

// دالة استقبال البيانات من السيريال
void serialEvent() {
  while (Serial.available()) {  // طالما يوجد بيانات في السيريال
    char inChar = (char)Serial.read();  // قراءة الحرف المستلم
    if (inputLength < BUFFER_SIZE - 1) {  // التأكد من عدم تجاوز حجم المخزن
      inputString[inputLength++] = inChar;  // تخزين الحرف في المصفوفة
      if (inChar == '\n') {  // التحقق من اكتمال الأمر (وصول حرف السطر الجديد)
        inputString[inputLength] = '\0';  // إنهاء النص بـ NULL
        stringComplete = true;  // تعيين حالة الإكمال
      }
    } else {
      inputLength = 0;  // إعادة تعيين الطول عند تجاوز الحجم
    }
  }
}

// دالة معالجة الأوامر بعد استقبالها من السيريال
void processCommand() {
  int commandID = inputString[0] - '0';  // استخراج رقم الأمر الأول
  int param1 = getdata(1);  // استخراج أول بارامتر (إن وجد)
  int param2 = getdata(2);  // استخراج ثاني بارامتر (إن وجد)

  // تنفيذ الأمر بناءً على رقمه
  switch (commandID) {
    case 1: ARPins(param1); break;  // قراءة قيمة أنالوج
    case 2: RPINx(param1); break;  // قراءة قيمة رقمية
    case 3: RPORT(param1); break;  // قراءة قيمة منفذ
    case 4: WPORTD(param1, param2); break;  // كتابة قيمة إلى منفذ
    case 5: RDDRS(param1); break;  // قراءة سجل DDR
    case 6:  // ضبط خرج رقمي
      pinMode(param1, OUTPUT);
      digitalWrite(param1, param2);
      break;
    case 7:  // ضبط PWM على دبوس معين
      pinMode(param1, OUTPUT);
      analogWrite(param1, param2);
      break;
  }
}

// دالة استخراج البيانات من النص المستلم
int getdata(int index) {
  char *token = strtok(inputString, " ");  // تقسيم النص باستخدام المسافات
  for (int i = 0; i < index; i++) {  // التنقل للوصول إلى البارامتر المطلوب
    token = strtok(NULL, " ");
    if (token == NULL) return -1;  // في حال عدم وجود بيانات كافية
  }
  return atoi(token);  // تحويل النص إلى رقم وإرجاعه
}

// **دوال الإدخال والإخراج المحسنة**

// دالة قراءة قيمة دبوس رقمي وإرسال النتيجة عبر السيريال
void RPINx(int I) {
  Serial.print("2 ");
  Serial.print(I);
  Serial.print(" ");
  Serial.println(digitalRead(I));
}

// دالة قراءة قيمة منفذ معين وإرسال النتيجة عبر السيريال
void RPORT(int I) {
  Serial.print("3 ");
  Serial.print(I);
  Serial.print(" ");
  switch (I) {
    case 2: Serial.println(PORTB, DEC); break;  // قراءة PORTB
    case 3: Serial.println(PORTC, DEC); break;  // قراءة PORTC
    case 4: Serial.println(PORTD, DEC); break;  // قراءة PORTD
    default: Serial.println("-1");  // في حال عدم وجود منفذ مطابق
  }
}

// دالة كتابة قيمة إلى منفذ معين
void WPORTD(int I, int val) {
  switch (I) {
    case 2: PORTB = val; break;  // كتابة إلى PORTB
    case 4: PORTD = val; break;  // كتابة إلى PORTD
  }
}

// دالة قراءة سجل DDR وإرسال النتيجة عبر السيريال
void RDDRS(int I) {
  Serial.print("5 ");
  Serial.print(I);
  Serial.print(" ");
  switch (I) {
    case 2: Serial.println(DDRB, DEC); break;  // قراءة DDRB
    case 3: Serial.println(DDRC, DEC); break;  // قراءة DDRC
    case 4: Serial.println(DDRD, DEC); break;  // قراءة DDRD
    default: Serial.println("-1");  // في حال عدم وجود سجل مطابق
  }
}

// دالة قراءة قيمة أنالوج من دبوس معين وإرسال النتيجة عبر السيريال
void ARPins(int i) {
  Serial.print("1 ");
  Serial.print(i);
  Serial.print(" ");
  Serial.println(analogRead(i));
}
