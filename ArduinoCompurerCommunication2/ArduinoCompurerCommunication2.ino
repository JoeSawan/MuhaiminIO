String inputString = "";         // a string to hold incoming data
bool stringComplete = false;  // whether the string is complete
String commandString = "";
bool isConnected = false;
void setup() {
  Serial.begin(115200);
}
void loop() {
  if (stringComplete)
  {
    stringComplete = false;
    getCommand();
    if (commandString.equals("RPISA"))
    { int i = getdata();
      ARPins(i);
    }
    else if (commandString.equals("RPINx"))
    {
      String I = inputString.substring(6, 7);
      RPINx (I.toInt());
    }else if (commandString.equals("RPORT"))
    {
      String I = inputString.substring(6, 7);
      RPORT (I.toInt());
    }
    else if (commandString.equals("WPORT"))
    { String I = inputString.substring(6, 7);
      String val = inputString.substring(7, 10);
      WPORTD (I.toInt(),val.toInt());
    }
    else if (commandString.equals("RDDRS"))
    {
      //char i[1];String I = inputString.substring(6, 7);I.toCharArray(i, 2);
      String I = inputString.substring(6, 7);
      RDDRS (I.toInt());
    }
    /*else if (commandString.equals("RPINS"))
      {
      //char i[1];String I = inputString.substring(6, 7);I.toCharArray(i, 2);
      String I = inputString.substring(6, 7);
      RPINS (I.toInt());
      }*/
    else if (commandString.equals("WRITE"))
    {
      int Dxxx = getdata();
      bool DState = getDState();
      // Serial.println(Dxxx);
      //Serial.println(DState);
      pinMode(Dxxx, OUTPUT);
      digitalWrite(Dxxx, DState);
    }  else if (commandString.equals("PWMWD"))
    {
      int Dxxx = getdata();
      int Val = getVal();
      pinMode(Dxxx, OUTPUT);
      analogWrite(Dxxx, Val);
    }
    inputString = "";
  }
}
void getCommand()
{
  if (inputString.length() > 4)
  {
    commandString = inputString.substring(1, 6);
  }
}
int getdata()
{
  String dataString = inputString.substring(6, 9);
  return dataString.toInt();
}
int getVal()
{
  String ValString = inputString.substring(10, 13);
  return ValString.toInt();
}
bool getDState()
{
  bool state = false;
  if (inputString.substring(10, 11).equals("1"))
  {
    state = true;
  } else {
    state = false;
  }
  return state;
}
/*String getTextToPrint()
  {
  String value = inputString.substring(5, inputString.length() - 2);
  return value;
  }*/
void serialEvent() {
  while (Serial.available()) {
    char inChar = (char)Serial.read();
    inputString += inChar;
    if (inChar == '\n') {
      stringComplete = true;
    }
  }
}
void RPINx (int I) {
  String tx = ("#PINx");
  tx += (I);
  String readi;
  switch (I) {
    case 1:
      //  readi = String(PINA);
      break;
    case 2:
      readi = String(PINB);
      break;
    case 3:
      readi = String(PINC);
      break;
    case 4:
      readi = String(PIND);
      break;
    default:
      break;
  }
  int myInt = readi.toInt();
  if (myInt < 10) tx += String("0");
  if (myInt < 100) tx += String("0");
  tx += (readi);
  tx += String("/n");
  Serial.println(tx.c_str());
}
void RPORT (int I) {
  String tx = ("#PORT");
  tx += (I);
  String readi;
  switch (I) {
    case 1:
      //  readi = String(PORTA);
      break;
    case 2:
      readi = String(PORTB);
      break;
    case 3:
      readi = String(PORTC);
      break;
    case 4:
      readi = String(PORTD);
      break;
    default:
      break;
  }
  int myInt = readi.toInt();
  if (myInt < 10) tx += String("0");
  if (myInt < 100) tx += String("0");
  tx += (readi);
  tx += String("/n");
  Serial.println(tx.c_str());
}
void WPORTD (int I, int val) {
  switch (I) {
    case 1:
      //  PORTB =val;
      break;
    case 2:
      PORTB =val;
      break;
      case 3:
     // PORTC =val;
      break;
      case 4:
      PORTD =val;
      break;
  }}
  void RDDRS (int I) {
    String tx = ("#DDRS");
    tx += (I);
    String readi;
    switch (I) {
      case 1:
        //  readi = String((*(&DDRA - 2)));
        break;
      case 2:
        readi = String(DDRB);
        break;
      case 3:
        readi = String(DDRC);
        break;
      case 4:
        readi = String(DDRD);
        break;
      default:
        break;
    }
    int myInt = readi.toInt();
    if (myInt < 10) tx += String("0");
    if (myInt < 100) tx += String("0");
    tx += (readi);
    tx += String("/n");
    Serial.println(tx.c_str());
  }
  void ARPins(int i ) {
    int A;
    A = analogRead(i);
    String tx = ("#AREAD");
    if (i < 10) tx += String("0");
    if (i < 100) tx += String("0");
    tx += String(i);
    tx += String("-");
    if (A < 10) tx += String("0");
    if (A < 100) tx += String("0");
    if (A < 1000) tx += String("0");
    tx += String(A);
    tx += String("/n");
    Serial.println(tx.c_str());
  }
