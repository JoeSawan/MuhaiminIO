let serialPort;
let reader;
let writer;

const encoder = new TextEncoder();
const decoder = new TextDecoder();

document.getElementById("connectBtn").addEventListener("click", async () => {
  try {
    if (!serialPort) {
      serialPort = await navigator.serial.requestPort();
      await serialPort.open({ baudRate: 115200 });
      document.getElementById("connectBtn").textContent = "قطع الاتصال";
      setupSerialListeners();
    } else {
      await reader.cancel();
      await writer.close();
      serialPort = null;
      document.getElementById("connectBtn").textContent = "الاتصال";
    }
  } catch (error) {
    console.error("خطأ في الاتصال:", error);
  }
});
let analogValues = new Array(8).fill(0); // تخزين قيم 8 دبابيس أنالوج
let isReading = false; // حالة القراءة المستمرة
let readInterval; // مؤقت القراءة

// بدء/إيقاف القراءة المستمرة (تعديل دالة setupSerialListeners)
async function setupSerialListeners() {
  writer = serialPort.writable.getWriter();
  startContinuousReading(); // بدء القراءة عند الاتصال
  readData();
}

// دالة القراءة المستمرة للأنالوج
function startContinuousReading() {
  if (!isReading) {
    isReading = true;
    let currentPin = 0;

    readInterval = setInterval(() => {
      if (serialPort && writer) {
        sendCommand(`1 ${currentPin}`); // إرسال أمر القراءة للدبوس الحالي
        currentPin = (currentPin + 1) % 8; // الانتقال للدبوس التالي
      }
    }, 300); // فاصل زمني بين الطلبات (ملي ثانية)
  }
}

// تعديل دالة معالجة البيانات
function processData(data) {
  const parts = data.split(" ");

  // معالجة ردود قراءة الأنالوج (الأمر 1)
  if (parts[0] === "1" && parts.length === 3) {
    const pin = parseInt(parts[1]);
    const value = parseInt(parts[2]);

    if (!isNaN(pin) && pin >= 0 && pin <= 7) {
      analogValues[pin] = value; // تحديث القيمة في المصفوفة
      console.log(`A${pin}: ${value}`); // عرض القيمة في الكونسول
    }
  }
  // معالجة الأخطاء (مثال)
  else if (data.startsWith("ERROR")) {
    console.error(`Error: ${data}`);
  }
}

// تعديل دالة قطع الاتصال
document.getElementById("connectBtn").addEventListener("click", async () => {
  try {
    if (!serialPort) {
      // ... الكود الأصلي ...
    } else {
      stopContinuousReading(); // إيقاف القراءة عند قطع الاتصال
      // ... الكود الأصلي ...
    }
  } catch (error) {
    console.error("خطأ في الاتصال:", error);
  }
});

// إيقاف القراءة المستمرة
function stopContinuousReading() {
  clearInterval(readInterval);
  isReading = false;
  analogValues.fill(0); // إعادة تعيين القيم
}
