import serial
import time
from textwrap import dedent

# ------ Protocol Constants ------
STX = 0x02
ETX = 0x03
CMD_ANALOG_READ = 0xAE
CMD_PORT_WRITE = 0xBF
CMD_DDR_SET = 0xDD
CMD_PWM_WRITE = 0xE4
CMD_PIN_READ = 0xFE

ERR_INVALID_CMD = 0x01
ERR_BAD_PARAM = 0x02
ERR_PIN_RANGE = 0x03
ERR_PORT_RANGE = 0x04
ERR_PWM_NOT_SUPP = 0x05

class ArduinoCommander:
    def __init__(self, port, baudrate=115200):
        self.ser = serial.Serial(port, baudrate, timeout=1)
        time.sleep(2)
        
    def __del__(self):
        self.ser.close()

    def send_command(self, cmd, params=[]):
        """إرسال أمر مع عرض البيانات الخام"""
        packet = bytes([STX, cmd] + params + [ETX])
        raw_sent = ' '.join(f'{b:02X}' for b in packet)
        print(f"\n[TX] {raw_sent}")
        
        self.ser.write(packet)
        response = self._read_response()
        
        if response:
            raw_recv = ' '.join(f'{b:02X}' for b in response)
            print(f"[RX] {raw_recv}")
        return response

    def _read_response(self):
        """قراءة الرد مع التعامل مع المهلة الزمنية"""
        start_time = time.time()
        response = bytearray()
        
        while time.time() - start_time < 2:  # مهلة 2 ثانية
            b = self.ser.read(1)
            if not b:
                continue
                
            if b[0] == STX:
                response = bytearray()
            elif b[0] == ETX:
                return bytes(response)
            else:
                response.append(b[0])
        
        print("Timeout waiting for response!")
        return None

    def _test_command(self, name, cases):
        """دالة مساعدة لتنفيذ حالات اختبارية"""
        print(f"\n{' ' + name + ' ':-^40}")
        for desc, (cmd, params, expected) in cases.items():
            print(f"\nTest: {desc}")
            res = self.send_command(cmd, params)
            
            if not res:
                print("No response received!")
                continue
                
            if res[0] == 0xEE:  # في حالة الخطأ
                if len(res) >= 3:
                    print(f"Error: code={res[1]:02X}, details={res[2]:02X}")
                else:
                    print("Malformed error response")
            else:
                print("Success:", self._parse_response(res, cmd))

    def _parse_response(self, res, cmd):
        """تحليل الردود بناء على نوع الأمر"""
        try:
            if cmd == CMD_ANALOG_READ:
                return f"Analog A{res[1]} = {(res[2] << 8) | res[3]}"
            elif cmd == CMD_PIN_READ:
                return f"PORT{res[1]} = 0x{res[2]:02X}"
            return "Acknowledged"
        except IndexError:
            return "Malformed response"

    def full_test_sequence(self):
        """سلسلة اختبارات شاملة لجميع الأوامر"""
        test_cases = {
            # اختبارات القراءة
            'Analog Read (Valid)': (
                CMD_ANALOG_READ, 
                [0x00], 
                {'type': CMD_ANALOG_READ}
            ),
            'Analog Read (Invalid Pin)': (
                CMD_ANALOG_READ, 
                [0x08], 
                {'error': ERR_PIN_RANGE}
            ),
            
            # اختبارات المنافذ الرقمية
            'DDR Set (Valid)': (
                CMD_DDR_SET, 
                [0xBB, 0xFF], 
                {'type': CMD_DDR_SET}
            ),
            'Port Write (Valid)': (
                CMD_PORT_WRITE, 
                [0xBB, 0xAA], 
                {'type': CMD_PORT_WRITE}
            ),
            'Pin Read (Valid)': (
                CMD_PIN_READ, 
                [0xCC], 
                {'type': CMD_PIN_READ}
            ),
            
            # اختبارات PWM
            'PWM Write (Valid)': (
                CMD_PWM_WRITE, 
                [0x09, 0x80], 
                {'type': CMD_PWM_WRITE}
            ),
            'PWM Write (Invalid Pin)': (
                CMD_PWM_WRITE, 
                [0x02, 0x80], 
                {'error': ERR_PWM_NOT_SUPP}
            ),
            
            # اختبارات الأخطاء
            'Invalid Command': (
                0xFF, 
                [], 
                {'error': ERR_INVALID_CMD}
            ),
            'Bad Parameters': (
                CMD_ANALOG_READ, 
                [0x00, 0x00], 
                {'error': ERR_BAD_PARAM}
            )
        }
        
        for case_name, (cmd, params, expected) in test_cases.items():
            self._test_command(case_name, {
                case_name: (cmd, params, expected)
            })

if __name__ == "__main__":
    comm = ArduinoCommander('COM9')
    
    print(dedent("""
    *******************************
    * Arduino Communication Tester *
    *******************************
    """))
    
    comm.full_test_sequence()
    print("\nAll tests completed successfully!")