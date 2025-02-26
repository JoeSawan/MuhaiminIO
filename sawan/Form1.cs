using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;
using System.IO.Ports;

delegate void StringArgReturningVoidDelegate(string text); //used for fixing multithreading problems

namespace sawan
{
    public partial class SAWAN : Form
    {
        String inputString = "";         // a string to hold incoming data
        bool stringComplete = true;  // whether the string is complete
        String commandString = "";
        byte DDRD,PORTD;

        bool D13, D12, D11, D10, D9, D8, D7, D6, D5, D4, D3, D2 = false;
        bool isclickSIO13, isclickSIO12, isclickSIO11, isclickSIO10, isclickSIO9, isclickSIO8, isclickSIO7, isclickSIO6, isclickSIO5, isclickSIO4, isclickSIO3, isclickSIO2 = false;
        bool isclickDRun,isclickD13, isclickD12, isclickD11, isclickD10, isclickD9, isclickD8, isclickD7, isclickD6, isclickD5, isclickD4, isclickD3, isclickD2 = false;
        int[] I = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13 };
        int SIO13, SIO12, SIO11, SIO10, SIO9, SIO8, SIO7, SIO6, SIO5, SIO4, SIO3, SIO2 = 0;
        bool isConnected = false;
        String[] ports;
        //SerialPort port;
        public SAWAN()
        {
            InitializeComponent();
            ports = SerialPort.GetPortNames();
            // port.DataReceived += new SerialDataReceivedEventHandler(DataReceivedHandler);

            foreach (string port in ports)
            {
                comboBox1.Items.Add(port);
                if (ports[0] != null)
                {
                    comboBox1.SelectedItem = ports[0];
                }
            }
        }


        private void connectToArduino()
        {
            try {
                isConnected = true;
                string selectedPort = comboBox1.GetItemText(comboBox1.SelectedItem);
                port = new SerialPort(selectedPort, 115200, Parity.None, 8, StopBits.One);
                port.ReadTimeout = 1000;
                port.Open();
                port.Write("#STAR\n");
                button1.Text = "Disconnect";
                panel1.Enabled = true;
                Run.Enabled = true;
                Run.Visible = true;
                isConnected = true;
                updateDataTimer.Enabled = true;

            }
            catch (Exception ex)
            {
                isConnected = false;
                MessageBox.Show(ex.Message);//displaying the error in a message box
            }
        }


        private void buttonD5_Click(object sender, EventArgs e)
        {
            if (isclickD5)
            {
                buttonD5.BackColor = Color.Black;
                buttonD5.ForeColor = Color.White;
                buttonD5.Text = "O";
                isclickD5 = false;
                D5 = false;
                PORTD=
                

                port.Write("#WRITE005-0\n");
            }
            else
            {
                buttonD5.BackColor = Color.Red;
                buttonD5.ForeColor = Color.Black;
                buttonD5.Text = "I";
                isclickD5 = true;
                D5 = true;
                port.Write("#WRITE005-1\n");
            }
        }

        private void buttonD4_Click(object sender, EventArgs e)
        {
            if (isclickD4)
            {
                buttonD4.BackColor = Color.Black;
                buttonD4.ForeColor = Color.White;
                buttonD4.Text = "O";
                isclickD4 = false;
                D4 = false;
                port.Write("#WRITE004-0\n");
            }
            else
            {
                buttonD4.BackColor = Color.Red;
                buttonD4.ForeColor = Color.Black;
                buttonD4.Text = "I";
                isclickD4 = true;
                D4 = true;
                port.Write("#WRITE004-1\n");
            }
        }

        private void buttonD3_Click(object sender, EventArgs e)
        {
            if (isclickD3)
            {
                buttonD3.BackColor = Color.Black;
                buttonD3.ForeColor = Color.White;
                buttonD3.Text = "O";
                isclickD3 = false;
                D3 = false;
                port.Write("#WRITE003-0\n");
            }
            else
            {
                buttonD3.BackColor = Color.Red;
                buttonD3.ForeColor = Color.Black;
                buttonD3.Text = "I";
                isclickD3 = true;
                D3 = true;
                port.Write("#WRITE003-1\n");
            }
        }

        private void buttonD2_Click(object sender, EventArgs e)
        {
            if (isclickD2)
            {
                buttonD2.BackColor = Color.Black;
                buttonD2.ForeColor = Color.White;
                buttonD2.Text = "O";
                isclickD2 = false;
                D2 = false;
                port.Write("#WRITE002-0\n");
            }
            else
            {
                buttonD2.BackColor = Color.Red;
                buttonD2.ForeColor = Color.Black;
                buttonD2.Text = "I";
                isclickD2 = true;
                D2 = true;
                port.Write("#WRITE002-1\n");

            }
        }

        private void button16_Click(object sender, EventArgs e)
        {

        }

        private void button2_Click(object sender, EventArgs e)
        {
            if (!isConnected)
            {
                comboBox1.Enabled = false;
                connectToArduino();
            }
            else if (isConnected)
            {
                disconnectFromArduino();
                comboBox1.Enabled = true;
            }
        }
        private void disconnectFromArduino()
        {
            isConnected = false;
            //port.Write("#STOP\n");
             
            button1.Text = "Connect";
            updateDataTimer.Enabled = false;
            panel1.Enabled = false;
            Run.Enabled = false;
            Run.Visible = false;
            try { port.Close(); }
            catch { }


            //esetDefaults();
        }

        private void button17_Click(object sender, EventArgs e)
        {
            ports = SerialPort.GetPortNames();

            foreach (string port in ports)
            {
                comboBox1.Items.Add(port);

                if (ports[0] != null)
                {
                    comboBox1.SelectedItem = ports[0];
                }
            }
        }

        private void pictureBox2_Click_1(object sender, EventArgs e)
        {

        }

        private void progressBarA0_Click(object sender, EventArgs e)
        {

        }

        private void timer1_Tick(object sender, EventArgs e)
        {
            if (isConnected) {
                port.Write("#RGPIO/n");
                String S = port.ReadLine();

            }
        }

        private void dateTimePicker1_ValueChanged(object sender, EventArgs e)
        {

        }

        private void button2_Click_1(object sender, EventArgs e)
        {

        }

        private void LED13_Click(object sender, EventArgs e)
        {

        }

        private void GPIO_Click(object sender, EventArgs e)
        {


        }

        private void timer2_Tick(object sender, EventArgs e)
        {

        }

        private void button2_Click_2(object sender, EventArgs e)
        {
            if (isConnected)
            {
                updateDataTimer.Enabled = false;
                port.WriteLine("");
                port.Write(textBox1.Text.ToString());
                textBox1.Clear();
                updateDataTimer.Enabled = true;

            }
        }

        private void button3_Click(object sender, EventArgs e)
        {

            if (isConnected)
            {
                disconnectFromArduino();
                Application.Exit();//dont 

                //MessageBox.Show("Please click Disconnet to disconnect from port First before exit", "Information", MessageBoxButtons.OK, MessageBoxIcon.Exclamation);
            }
            else
            {
                Application.Exit();
               
            }
        }

        private void comboBox1_SelectedIndexChanged(object sender, EventArgs e)
        {

        }

        private void comboBox1_Click(object sender, EventArgs e)
        {
            comboBox1.Items.Clear();
            //comboBox1.Items.Add("");
            int itemsFound = 0;
            foreach (string port in SerialPort.GetPortNames())
            {

                comboBox1.Items.Add(port);
                itemsFound++;
            }
            if (itemsFound < 1)
                MessageBox.Show("NO ITEMS FOUND");//if no serial devices have been found, an error message window will show up


        }


        private void SetTextOfTextBox2(string text)
        {
            // InvokeRequired required compares the thread ID of the
            // calling thread to the thread ID of the creating thread.
            // If these threads are different, it returns true.
            if (this.textBox2.InvokeRequired)
            {
                StringArgReturningVoidDelegate d = new StringArgReturningVoidDelegate(SetTextOfTextBox2);
                this.Invoke(d, new object[] { text });
            }
            else
            {
                this.textBox2.Text = text;
            }
        }
        private void label2_Click(object sender, EventArgs e)
        {

        }

        private void button5_Click(object sender, EventArgs e)
        {
            this.WindowState = FormWindowState.Minimized;
        }

        private void updateDataTimer_Tick(object sender, EventArgs e)
        {
            if (isConnected)
            {
                if (port.ReadTimeout > 1000)
                { textBox2.AppendText(port.ReadLine());
                    textBox2.AppendText("\n");
                    disconnectFromArduino();
                    updateDataTimer.Enabled = false;
                }
                try
                {

                    PINxB();
                    PINxD();
                    // PINSD();
                    PINxC();
                    RPISA();
                    T();
                }
                catch (Exception er)
                {
                    //if an error is detected then the error message is displayed and timer gets disabled

                    //MessageBox.Show(er.Message);
                    disconnectFromArduino();

                    updateDataTimer.Enabled = false;
                    


                }
            }
        }


        private void textBox1_TextChanged(object sender, EventArgs e)
        {

        }

        private void richTextBox1_TextChanged(object sender, EventArgs e)
        {

        }

        private void port_DataReceived(object sender, SerialDataReceivedEventArgs e)
        {
            textBox2.AppendText(port.ReadLine());
            textBox2.AppendText("\n");
        }

        private void button6_Click(object sender, EventArgs e)
        {
            port.WriteLine("#RESET/n");
        }
private void T()
        {
            int val0 = progressBarA0.Value;
            string result = Convert.ToString(val0);
            AR0.Text = (result);
            int val1 = progressBarA1.Value;
            string result1 = Convert.ToString(val1);
            AR1.Text = (result1);
            int val2 = progressBarA2.Value;
            string result2 = Convert.ToString(val2);
            AR2.Text = (result2);
            int val3 = progressBarA3.Value;
            string result3 = Convert.ToString(val3);
            AR3.Text = (result3);
            int val4 = progressBarA4.Value;
            string result4 = Convert.ToString(val4);
            AR4.Text = (result4);
            int val5 = progressBarA5.Value;
            string result5 = Convert.ToString(val5);
            AR5.Text = (result5);
        }
        private void RPISA()
        {
            for (int i = 0; i < 6; i++)
            {
                String tx = ("#RPISA");
                if (i < 10) tx += ("0");
                if (i < 100) tx += ("0");
                tx += (i);
                tx += ("/n");
                port.WriteLine(tx);
                if (stringComplete)
                {
                    inputString = port.ReadLine();
                    // SetTextOfTextBox2(inputString);
                    stringComplete = false;
                    GC();
                    if (commandString.Equals("AREAD"))
                    {
                        String dataString2 = inputString.Substring(6, 3);
                        //SetTextOfTextBox2(dataString2);
                        if (dataString2 == "000")
                        {
                            String Valstr = inputString.Substring(10, 4);
                            int val = Convert.ToInt16(Valstr);
                            progressBarA0.Value = val;
                        }
                        if (dataString2 == "001")
                        {
                            String Valstr = inputString.Substring(10, 4);
                            int val = Convert.ToInt16(Valstr);
                            progressBarA1.Value = val;
                        }
                        if (dataString2 == "002")
                        {
                            String Valstr = inputString.Substring(10, 4);
                            int val = Convert.ToInt16(Valstr);
                            progressBarA2.Value = val;
                        }
                        if (dataString2 == "003")
                        {
                            String Valstr = inputString.Substring(10, 4);
                            int val = Convert.ToInt16(Valstr);
                            progressBarA3.Value = val;
                        }
                        if (dataString2 == "004")
                        {
                            String Valstr = inputString.Substring(10, 4);
                            int val = Convert.ToInt16(Valstr);
                            progressBarA4.Value = val;
                        }
                        if (dataString2 == "005")
                        {
                            String Valstr = inputString.Substring(10, 4);
                            int val = Convert.ToInt16(Valstr);
                            progressBarA5.Value = val;
                        }
                    } 
                } stringComplete = true;
            }
        }
        private void PINxD()// SetTextOfTextBox2(dataString);
        {
            port.WriteLine("#RPINx4/n");
            if (stringComplete)
            {
                    inputString = port.ReadLine();
                    stringComplete = false;
                    GC();
                    if (commandString.Equals("PINx4"))
                    {
                        String dataString = inputString.Substring(6, 3);
                        byte number = Convert.ToByte(dataString);
                        if (getBit(number, 7) == 1) { LED7.BackColor = Color.Red; } else LED7.BackColor = Color.Black;
                        if (getBit(number, 6) == 1) { LED6.BackColor = Color.Red; } else LED6.BackColor = Color.Black;
                        if (getBit(number, 5) == 1) { LED5.BackColor = Color.Red; } else LED5.BackColor = Color.Black;
                        if (getBit(number, 4) == 1) { LED4.BackColor = Color.Red; } else LED4.BackColor = Color.Black;
                        if (getBit(number, 3) == 1) { LED3.BackColor = Color.Red; } else LED3.BackColor = Color.Black;
                        if (getBit(number, 2) == 1) { LED2.BackColor = Color.Red; } else LED2.BackColor = Color.Black;
                        if (getBit(number, 1) == 1) { LED1.BackColor = Color.Red; } else LED1.BackColor = Color.Black;
                        if (getBit(number, 0) == 1) { LED0.BackColor = Color.Red; } else LED0.BackColor = Color.Black;
                    }
            }
            stringComplete = true;
        }
        private void PINSD()// SetTextOfTextBox2(dataString);
        {
            port.WriteLine("#RPINS4/n");
            if (stringComplete)
            {
                inputString = port.ReadLine();
                stringComplete = false;
                GC();
                if (commandString.Equals("PINS4"))
                {
                    String dataString = inputString.Substring(6, 3);
                    byte number = Convert.ToByte(dataString);
                    if (getBit(number, 7) == 1) { LED7.BackColor = Color.Red; } else LED7.BackColor = Color.Black;
                    if (getBit(number, 6) == 1) { LED6.BackColor = Color.Red; } else LED6.BackColor = Color.Black;
                    if (getBit(number, 5) == 1) { LED5.BackColor = Color.Red; } else LED5.BackColor = Color.Black;
                    if (getBit(number, 4) == 1) { LED4.BackColor = Color.Red; } else LED4.BackColor = Color.Black;
                    if (getBit(number, 3) == 1) { LED3.BackColor = Color.Red; } else LED3.BackColor = Color.Black;
                    if (getBit(number, 2) == 1) { LED2.BackColor = Color.Red; } else LED2.BackColor = Color.Black;
                    if (getBit(number, 1) == 1) { LED1.BackColor = Color.Red; } else LED1.BackColor = Color.Black;
                    if (getBit(number, 0) == 1) { LED0.BackColor = Color.Red; } else LED0.BackColor = Color.Black;
                }
            }
            stringComplete = true;
        }
        private void PINxC()// SetTextOfTextBox2(dataString);
        {
            port.WriteLine("#RPINx3/n");
            if (stringComplete)
            {
                inputString = port.ReadLine();
                stringComplete = false;
                GC();
                if (commandString.Equals("PINx3"))
                {
                    String dataString = inputString.Substring(6, 3);
                    byte number = Convert.ToByte(dataString);
                    if (getBit(number, 5) == 1) { LED19.BackColor = Color.Red; } else LED19.BackColor = Color.Black;
                    if (getBit(number, 4) == 1) { LED18.BackColor = Color.Red; } else LED18.BackColor = Color.Black;
                    if (getBit(number, 3) == 1) { LED17.BackColor = Color.Red; } else LED17.BackColor = Color.Black;
                    if (getBit(number, 2) == 1) { LED16.BackColor = Color.Red; } else LED16.BackColor = Color.Black;
                    if (getBit(number, 1) == 1) { LED15.BackColor = Color.Red; } else LED15.BackColor = Color.Black;
                    if (getBit(number, 0) == 1) { LED14.BackColor = Color.Red; } else LED14.BackColor = Color.Black;
                }
            }
            stringComplete = true;
        }
        private void PINxB()
        {
            port.WriteLine("#RPINx2/n");
            if (stringComplete)
            {
                    inputString = port.ReadLine();
                    stringComplete = false;
                    GC();
                    //SetTextOfTextBox2(commandString);
                    if (commandString.Equals("PINx2"))
                    {
                        String dataString = inputString.Substring(6, 3);
                        byte number = Convert.ToByte(dataString);
                        if (getBit(number, 5) == 1) { LED13.BackColor = Color.Red; } else LED13.BackColor = Color.Black;
                        if (getBit(number, 4) == 1) { LED12.BackColor = Color.Red; } else LED12.BackColor = Color.Black;
                        if (getBit(number, 3) == 1) { LED11.BackColor = Color.Red; } else LED11.BackColor = Color.Black;
                        if (getBit(number, 2) == 1) { LED10.BackColor = Color.Red; } else LED10.BackColor = Color.Black;
                        if (getBit(number, 1) == 1) { LED9.BackColor = Color.Red; } else LED9.BackColor = Color.Black;
                        if (getBit(number, 0) == 1) { LED8.BackColor = Color.Red; } else LED8.BackColor = Color.Black;
                    }
            }
            stringComplete = true;
        }
        int getBit(byte b, int bitNumber)
        {
            return ((b >> bitNumber) & 0x01);
        }
        private void GC()
        {   
            int length = inputString.Length;
            if (length > 1)commandString = inputString.Substring(1, 5);  
        }


        int getVal()
        {
            String ValString = inputString.Substring(10, 3);
            return int.Parse(ValString);
        }

        private void textBoxPWM5_TextChanged(object sender, EventArgs e)
        { if (textBoxPWM5.Text!= ("") ) {
                string val = textBoxPWM5.Text;
                int result = Convert.ToInt16(val);
                if (result>255) {
                    textBoxPWM5.Text = ("255");
                    }
                if (result > -1 & result < 256) {
                    hSBarPWM5.Value = (result);
                } } }

        private void AR5_TextChanged(object sender, EventArgs e)
        {
      
        }

        private void progressBarA5_Click(object sender, EventArgs e)
        {

        }

        private void hSBarPWM5_ValueChanged(object sender, EventArgs e)
        {
            int val = hSBarPWM5.Value;
            string result = Convert.ToString(val);
            textBoxPWM5.Text=(result);
            String TX = "#PWMWD";
            TX += ("005");
            TX += ("-"+result);
            TX += ("/n");
            port.WriteLine(TX);
        }

        private void textBoxPWM6_TextChanged(object sender, EventArgs e)
        {
            if (textBoxPWM6.Text != (""))
            {
                string val = textBoxPWM6.Text;
                int result = Convert.ToInt16(val);
                if (result > 255)
                {
                    textBoxPWM6.Text = ("255");
                }
                if (result > -1 & result < 256)
                {
                    hSBarPWM6.Value = (result);
                }
            }
        }

        private void textBoxPWM3_TextChanged(object sender, EventArgs e)
        {
            if (textBoxPWM3.Text != (""))
            {
                string val = textBoxPWM3.Text;
                int result = Convert.ToInt16(val);
                if (result > 255)
                {
                    textBoxPWM3.Text = ("255");
                }
                if (result > -1 & result < 256)
                {
                    hSBarPWM3.Value = (result);
                }
            }
        }

 

        private void textBoxPWM9_TextChanged(object sender, EventArgs e)
        {
            if (textBoxPWM9.Text != (""))
            {
                string val = textBoxPWM9.Text;
                int result = Convert.ToInt16(val);
                if (result > 255)
                {
                    textBoxPWM9.Text = ("255");
                }
                if (result > -1 & result < 256)
                {
                    hSBarPWM9.Value = (result);
                }
            }
        }

        private void textBoxPWM10_TextChanged(object sender, EventArgs e)
        {
            if (textBoxPWM10.Text != (""))
            {
                string val = textBoxPWM10.Text;
                int result = Convert.ToInt16(val);
                if (result > 255)
                {
                    textBoxPWM10.Text = ("255");
                }
                if (result > -1 & result < 256)
                {
                    hSBarPWM10.Value = (result);
                }
            }
        }

        private void hSBarPWM3_ValueChanged(object sender, EventArgs e)
        {
            int val = hSBarPWM3.Value;
            string result = Convert.ToString(val);
            textBoxPWM3.Text = (result);
            String TX = "#PWMWD";
            TX += ("003");
            TX += ("-"+result);
            TX += ("/n");
            port.WriteLine(TX);
        }

        private void button4_Click(object sender, EventArgs e)
        {
            if (isConnected)
            {
                if (SIO13 == 1) {

                }

            }
        }

        private void hSBarPWM5_Scroll(object sender, ScrollEventArgs e)
        {

        }

        private void panel3_MouseClick(object sender, MouseEventArgs e)
        {

        }

        private void checkBox1_CheckedChanged_1(object sender, EventArgs e)
        {

        }

        private void textBox1_TextChanged_1(object sender, EventArgs e)
        {

        }

        private void Run_Click(object sender, EventArgs e)
        {
            if (isclickDRun)
            {
                Run.BackColor = Color.Green;
                Run.ForeColor = Color.White;
                Run.Text = "Run Code";
                isclickDRun = false;
                port.Write("#SoRUN\n");
            }
            else
            {
                Run.BackColor = Color.Black;
                Run.ForeColor = Color.White;
                Run.Text = "Stop Code";
                isclickDRun = true;
                port.Write("#SrRUN\n");
            }
        }

        private void hSBarPWM6_ValueChanged(object sender, EventArgs e)
        {
            int val = hSBarPWM6.Value;
            string result = Convert.ToString(val);
            textBoxPWM6.Text = (result);
            String TX = "#PWMWD";
            TX += ("006");
            TX += ("-"+result);
            TX += ("/n");
            port.WriteLine(TX);
        }

        private void hSBarPWM9_ValueChanged(object sender, EventArgs e)
        {
            int val = hSBarPWM9.Value;
            string result = Convert.ToString(val);
            textBoxPWM9.Text = (result);
            String TX = "#PWMWD";
            TX += ("009");
            TX += ("-"+result);
            TX += ("/n");
            port.WriteLine(TX);
        }

        private void hSBarPWM10_ValueChanged(object sender, EventArgs e)
        {
            int val = hSBarPWM10.Value;
            string result = Convert.ToString(val);
            textBoxPWM10.Text = (result);
            String TX = "#PWMWD";
            TX += ("010");
            TX += ("-"+result);
            TX += ("/n");
            port.WriteLine(TX);
        }

        private void hSBarPWM11_ValueChanged(object sender, EventArgs e)
        {
            int val = hSBarPWM11.Value;
            string result = Convert.ToString(val);
            textBoxPWM11.Text = (result);
            String TX = "#PWMWD";
            TX += ("011");
            TX += ("-"+result);
            TX += ("/n");
            port.WriteLine(TX);

        }

        private void pictureBox4_Click(object sender, EventArgs e)
        {

        }

        private void buttonD6_Click(object sender, EventArgs e)
        {
            if (isclickD6)
            {
                buttonD6.BackColor = Color.Black;
                buttonD6.ForeColor = Color.White;
                buttonD6.Text = "O";
                isclickD6 = false;
                D6 = false;
                port.Write("#WRITE006-0\n");

            }
            else
            {
                buttonD6.BackColor = Color.Red;
                buttonD6.ForeColor = Color.Black;
                buttonD6.Text = "I";
                isclickD6 = true;
                D6 = true;
                port.Write("#WRITE006-1\n");

            }
        }

        private void buttonD7_Click(object sender, EventArgs e)
        {
            if (isclickD7)
            { 
           
                buttonD7.BackColor = Color.Black;
                buttonD7.ForeColor = Color.White;
                buttonD7.Text = "O";
                isclickD7 = false;
                D7 = false;
                port.Write("#WRITE007-0\n");
            }
            else
            {
                buttonD7.BackColor = Color.Red;
                buttonD7.ForeColor = Color.Black;
                buttonD7.Text = "I";
                isclickD7 = true;
                D7 = true;
                port.Write("#WRITE007-1\n");

            }
        }

        private void buttonD8_Click(object sender, EventArgs e)
        {

            if (isclickD8)
            {
                buttonD8.BackColor = Color.Black;
                buttonD8.ForeColor = Color.White;
                buttonD8.Text = "O";
                isclickD8 = false;
                D8 = false;
                port.Write("#WRITE008-0\n");
            }
            else
            {
                buttonD8.BackColor = Color.Red;
                buttonD8.ForeColor = Color.Black;
                buttonD8.Text = "I";
                isclickD8 = true;
                D8 = true;
                port.Write("#WRITE008-1\n");
            }
        }
        private void pictureBox1_Click(object sender, EventArgs e)
        {

        }

        private void pictureBox1_Click_1(object sender, EventArgs e)
        {

        }

        private void Form1_Load(object sender, EventArgs e)
        {

        }

        private void pictureBox1_Click_2(object sender, EventArgs e)
        {

        }

        private void pictureBox2_Click(object sender, EventArgs e)
        {

        }

        private void GND0_Click(object sender, EventArgs e)
        {

        }

        private void pictureBox1_Click_3(object sender, EventArgs e)
        {

        }

        private void panel1_Paint(object sender, PaintEventArgs e)
        {
        
        }

        private void pictureBox1_Click_4(object sender, EventArgs e)
        {

        }

        private void pictureBox3_Click(object sender, EventArgs e)
        {

        }

        private void checkBox1_CheckedChanged(object sender, EventArgs e)
        {

        }

        private void progressBar1_Click(object sender, EventArgs e)
        {

        }

        private void label1_Click(object sender, EventArgs e)
        {

        }

     

        private void pictureBox22_Click(object sender, EventArgs e)
        {

        }

        private void textBoxPWM11_TextChanged(object sender, EventArgs e)
        {
            if (textBoxPWM11.Text != (""))
            {
                string val = textBoxPWM11.Text;
                int result = Convert.ToInt16(val);
                if (result > 255)
                {
                    textBoxPWM11.Text = ("255");
                }
                if (result > -1 & result < 256)
                {
                    hSBarPWM11.Value = (result);
                }
            }
        }

        private void buttonD11_Click(object sender, EventArgs e)
        {
            if (isclickD11)
            {
                buttonD11.BackColor = Color.Black;
                buttonD11.ForeColor = Color.White;
                buttonD11.Text = "O";
                isclickD11 = false;
                D11 = false;
                port.Write("#WRITE011-0\n");
            }
            else
            {
                buttonD11.BackColor = Color.Red;
                buttonD11.ForeColor = Color.Black;
                buttonD11.Text = "I";
                isclickD11 = true;
                D11 = true;
                port.Write("#WRITE011-1\n");
            }
        }

        private void buttonD10_Click(object sender, EventArgs e)
        {
            if (isclickD10)
            {
                buttonD10.BackColor = Color.Black;
                buttonD10.ForeColor = Color.White;
                buttonD10.Text = "O";
                isclickD10 = false;
                D10 = false;
                port.Write("#WRITE010-0\n");
            }
            else
            {
                buttonD10.BackColor = Color.Red;
                buttonD10.ForeColor = Color.Black;
                buttonD10.Text = "I";
                isclickD10 = true;
                D10 = true;
                port.Write("#WRITE010-1\n");
            }
        }

 


        private void buttonSIO9_Click(object sender, EventArgs e)
        {
            if (!isclickSIO9)
            {

                SIO9 = ++SIO9 % 4;
                switch (SIO9)
                {
                    case 0:
                        buttonSIO9.BackColor = Color.PaleGreen;
                        buttonSIO9.Text = "INPUT";
                        buttonD9.Enabled = false;
                        buttonD9.Visible = false;
                        textBoxPWM9.Enabled = false;
                        textBoxPWM9.Visible = false;
                        hSBarPWM9.Enabled = false;
                        hSBarPWM9.Visible = false;
                        break;
                    case 1:
                        buttonSIO9.BackColor = Color.DarkOrange;
                        buttonSIO9.Text = "INPUT👆";
                        buttonD9.Enabled = false;
                        buttonD9.Visible = false;
                        textBoxPWM9.Enabled = false;
                        textBoxPWM9.Visible = false;
                        hSBarPWM9.Enabled = false;
                        hSBarPWM9.Visible = false;
                        break;
                    case 2:
                        buttonSIO9.BackColor = Color.LightSeaGreen;
                        buttonSIO9.Text = "OUTPUT";
                        buttonD9.Enabled = true;
                        buttonD9.Visible = true;
                        textBoxPWM9.Enabled = false;
                        textBoxPWM9.Visible = false;
                        hSBarPWM9.Enabled = false;
                        hSBarPWM9.Visible = false;
                        break;
                    case 3:
                        buttonSIO9.BackColor = Color.RoyalBlue;
                        buttonSIO9.Text = "PWM";
                        buttonD9.Enabled = false;
                        buttonD9.Visible = false;
                        textBoxPWM9.Enabled = true;
                        textBoxPWM9.Visible = true;
                        hSBarPWM9.Enabled = true;
                        hSBarPWM9.Visible = true;
                        break;
                    default:
                        break;
                }

            }
        }

        private void buttonSIO6_Click(object sender, EventArgs e)
        {
            if (!isclickSIO6)
            {
                SIO6 = ++SIO6 % 4;
                switch (SIO6)
                {
                    case 0:
                        buttonSIO6.BackColor = Color.PaleGreen;
                        buttonSIO6.Text = "INPUT";
                        buttonD6.Enabled = false;
                        buttonD6.Visible = false;
                        textBoxPWM6.Enabled = false;
                        textBoxPWM6.Visible = false;
                        hSBarPWM6.Enabled = false;
                        hSBarPWM6.Visible = false;
                        break;
                    case 1:
                        buttonSIO6.BackColor = Color.DarkOrange;
                        buttonSIO6.Text = "INPUT👆";
                        buttonD6.Enabled = false;
                        buttonD6.Visible = false;
                        textBoxPWM6.Enabled = false;
                        textBoxPWM6.Visible = false;
                        hSBarPWM6.Enabled = false;
                        hSBarPWM6.Visible = false;
                        break;
                    case 2:
                        buttonSIO6.BackColor = Color.LightSeaGreen;
                        buttonSIO6.Text = "OUTPUT";
                        buttonD6.Enabled = true;
                        buttonD6.Visible = true;
                        textBoxPWM6.Enabled = false;
                        textBoxPWM6.Visible = false;
                        hSBarPWM6.Enabled = false;
                        hSBarPWM6.Visible = false;
                        break;
                    case 3:
                        buttonSIO6.BackColor = Color.RoyalBlue;
                        buttonSIO6.Text = "PWM";
                        buttonD6.Enabled = false;
                        buttonD6.Visible = false;
                        textBoxPWM6.Enabled = true;
                        textBoxPWM6.Visible = true;
                        hSBarPWM6.Enabled = true;
                        hSBarPWM6.Visible = true;
                        break;
                    default:
                        break;
                }

            }
        }

        private void buttonSIO5_Click(object sender, EventArgs e)
        {
            if (!isclickSIO5)
            {
                SIO5 = ++SIO5 % 4;
                switch (SIO5)
                {
                    case 0:
                        buttonSIO5.BackColor = Color.PaleGreen;
                        buttonSIO5.Text = "INPUT";
                        buttonD5.Enabled = false;
                        buttonD5.Visible = false;
                        textBoxPWM5.Enabled = false;
                        textBoxPWM5.Visible = false;
                        hSBarPWM5.Enabled = false;
                        hSBarPWM5.Visible = false;
                        break;
                    case 1:
                        buttonSIO5.BackColor = Color.DarkOrange;
                        buttonSIO5.Text = "INPUT👆";
                        buttonD5.Enabled = false;
                        buttonD5.Visible = false;
                        textBoxPWM5.Enabled = false;
                        textBoxPWM5.Visible = false;
                        hSBarPWM5.Enabled = false;
                        hSBarPWM5.Visible = false;
                        break;
                    case 2:
                        buttonSIO5.BackColor = Color.LightSeaGreen;
                        buttonSIO5.Text = "OUTPUT";
                        buttonD5.Enabled = true;
                        buttonD5.Visible = true;
                        textBoxPWM5.Enabled = false;
                        textBoxPWM5.Visible = false;
                        hSBarPWM5.Enabled = false;
                        hSBarPWM5.Visible = false;
                        break;
                    case 3:
                        buttonSIO5.BackColor = Color.RoyalBlue;
                        buttonSIO5.Text = "PWM";
                        buttonD5.Enabled = false;
                        buttonD5.Visible = false;
                        textBoxPWM5.Enabled = true;
                        textBoxPWM5.Visible = true;
                        hSBarPWM5.Enabled = true;
                        hSBarPWM5.Visible = true;
                        break;
                    default:
                        break;
                }

            }
        }

        private void buttonSIO3_Click(object sender, EventArgs e)
        {
            if (!isclickSIO3)
            {
                SIO3 = ++SIO3 % 4;
                switch (SIO3)
                {
                    case 0:
                        buttonSIO3.BackColor = Color.PaleGreen;
                        buttonSIO3.Text = "INPUT";
                        buttonD3.Enabled = false;
                        buttonD3.Visible = false;
                        textBoxPWM3.Enabled = false;
                        textBoxPWM3.Visible = false;
                        hSBarPWM3.Enabled = false;
                        hSBarPWM3.Visible = false;
                        break;
                    case 1:
                        buttonSIO3.BackColor = Color.DarkOrange;
                        buttonSIO3.Text = "INPUT👆";
                        buttonD3.Enabled = false;
                        buttonD3.Visible = false;
                        textBoxPWM3.Enabled = false;
                        textBoxPWM3.Visible = false;
                        hSBarPWM3.Enabled = false;
                        hSBarPWM3.Visible = false;
                        break;
                    case 2:
                        buttonSIO3.BackColor = Color.LightSeaGreen;
                        buttonSIO3.Text = "OUTPUT";
                        buttonD3.Enabled = true;
                        buttonD3.Visible = true;
                        textBoxPWM3.Enabled = false;
                        textBoxPWM3.Visible = false;
                        hSBarPWM3.Enabled = false;
                        hSBarPWM3.Visible = false;
                        break;
                    case 3:
                        buttonSIO3.BackColor = Color.RoyalBlue;
                        buttonSIO3.Text = "PWM";
                        buttonD3.Enabled = false;
                        buttonD3.Visible = false;
                        textBoxPWM3.Enabled = true;
                        textBoxPWM3.Visible = true;
                        hSBarPWM3.Enabled = true;
                        hSBarPWM3.Visible = true;
                        break;
                    default:
                        break;
                }

            }
        }

        private void buttonD12_Click(object sender, EventArgs e)
        {
            if (isclickD12)
            {
                buttonD12.BackColor = Color.Black;
                buttonD12.ForeColor = Color.White;
                buttonD12.Text = "O";
                isclickD12 = false;
                D12 = false;
                port.Write("#WRITE012-0\n");

            }
            else
            {
                buttonD12.BackColor = Color.Red;
                buttonD12.ForeColor = Color.Black;
                buttonD12.Text = "I";
                isclickD12 = true;
                D12 = true;
                port.Write("#WRITE012-1\n");
            }
        }

        private void buttonSIO8_Click(object sender, EventArgs e)
        {
            if (!isclickSIO8)
            {

                SIO8 = ++SIO8 % 3;
                switch (SIO8)
                {
                    case 0:
                        buttonSIO8.BackColor = Color.PaleGreen;
                        buttonSIO8.Text = "INPUT";
                        buttonD8.Enabled = false;
                        buttonD8.Visible = false;
                        break;
                    case 1:
                        buttonSIO8.BackColor = Color.DarkOrange;
                        buttonSIO8.Text = "INPUT👆";
                        buttonD8.Enabled = false;
                        buttonD8.Visible = false;
                        break;
                    case 2:
                        buttonSIO8.BackColor = Color.LightSeaGreen;
                        buttonSIO8.Text = "OUTPUT";
                        buttonD8.Enabled = true;
                        buttonD8.Visible = true;
                        break;
                    default:
                        break;
                }

            }
        }

        private void buttonSIO7_Click(object sender, EventArgs e)
        {
            if (!isclickSIO7)
            {

                SIO7 = ++SIO7 % 3;
                switch (SIO7)
                {
                    case 0:
                        buttonSIO7.BackColor = Color.PaleGreen;
                        buttonSIO7.Text = "INPUT";
                        buttonD7.Enabled = false;
                        buttonD7.Visible = false;
                        break;
                    case 1:
                        buttonSIO7.BackColor = Color.DarkOrange;
                        buttonSIO7.Text = "INPUT👆";
                        buttonD7.Enabled = false;
                        buttonD7.Visible = false;
                        break;
                    case 2:
                        buttonSIO7.BackColor = Color.LightSeaGreen;
                        buttonSIO7.Text = "OUTPUT";
                        buttonD7.Enabled = true;
                        buttonD7.Visible = true;
                        break;
                    default:
                        break;
                }

            }
        }

        private void buttonSIO4_Click(object sender, EventArgs e)
        {
            if (!isclickSIO4)
            {

                SIO4 = ++SIO4 % 3;
                switch (SIO4)
                {
                    case 0:
                        buttonSIO4.BackColor = Color.PaleGreen;
                        buttonSIO4.Text = "INPUT";
                        buttonD4.Enabled = false;
                        buttonD4.Visible = false;
                        break;
                    case 1:
                        buttonSIO4.BackColor = Color.DarkOrange;
                        buttonSIO4.Text = "INPUT👆";
                        buttonD4.Enabled = false;
                        buttonD4.Visible = false;
                        break;
                    case 2:
                        buttonSIO4.BackColor = Color.LightSeaGreen;
                        buttonSIO4.Text = "OUTPUT";
                        buttonD4.Enabled = true;
                        buttonD4.Visible = true;
                        break;
                    default:
                        break;
                }

            }
        }

        private void buttonSIO2_Click(object sender, EventArgs e)
        {
            if (!isclickSIO2)
            {

                SIO2 = ++SIO2 % 3;
                switch (SIO2)
                {
                    case 0:
                        buttonSIO2.BackColor = Color.PaleGreen;
                        buttonSIO2.Text = "INPUT";
                        buttonD2.Enabled = false;
                        buttonD2.Visible = false;
                        break;
                    case 1:
                        buttonSIO2.BackColor = Color.DarkOrange;
                        buttonSIO2.Text = "INPUT👆";
                        buttonD2.Enabled = false;
                        buttonD2.Visible = false;
                        break;
                    case 2:
                        buttonSIO2.BackColor = Color.LightSeaGreen;
                        buttonSIO2.Text = "OUTPUT";
                        buttonD2.Enabled = true;
                        buttonD2.Visible = true;
                        break;
                    default:
                        break;
                }

            }
        }

        private void pictureBox21_Click(object sender, EventArgs e)
        {

        }

        private void progressBar4_Click(object sender, EventArgs e)
        {

        }

        private void LOGO_Click(object sender, EventArgs e)
        {

        }



        private void textBox17_TextChanged(object sender, EventArgs e)
        {

        }

        private void button26_Click(object sender, EventArgs e)
        {

            if (!isclickSIO10)
            {
                SIO10 = ++SIO10 % 4;
                switch (SIO10)
                {
                    case 0:
                        buttonSIO10.BackColor = Color.PaleGreen;
                        buttonSIO10.Text = "INPUT";
                        buttonD10.Enabled = false;
                        buttonD10.Visible = false;
                        textBoxPWM10.Enabled = false;
                        textBoxPWM10.Visible = false;
                        hSBarPWM10.Enabled = false;
                        hSBarPWM10.Visible = false;
                        break;
                    case 1:
                        buttonSIO10.BackColor = Color.DarkOrange;
                        buttonSIO10.Text = "INPUT👆";
                        buttonD10.Enabled = false;
                        buttonD10.Visible = false;
                        textBoxPWM10.Enabled = false;
                        textBoxPWM10.Visible = false;
                        hSBarPWM10.Enabled = false;
                        hSBarPWM10.Visible = false;
                        break;
                    case 2:
                        buttonSIO10.BackColor = Color.LightSeaGreen;
                        buttonSIO10.Text = "OUTPUT";
                        buttonD10.Enabled = true;
                        buttonD10.Visible = true;
                        textBoxPWM10.Enabled = false;
                        textBoxPWM10.Visible = false;
                        hSBarPWM10.Enabled = false;
                        hSBarPWM10.Visible = false;
                        break;
                    case 3:
                        buttonSIO10.BackColor = Color.RoyalBlue;
                        buttonSIO10.Text = "PWM";
                        buttonD10.Enabled = false;
                        buttonD10.Visible = false;
                        textBoxPWM10.Enabled = true;
                        textBoxPWM10.Visible = true;
                        hSBarPWM10.Enabled = true;
                        hSBarPWM10.Visible = true;
                        break;
                    default:
                        break;
                }

            }
        }

        private void button31_Click(object sender, EventArgs e)
        {

            if (isclickD13)
            {
                buttonD13.BackColor = Color.Black;
                buttonD13.ForeColor = Color.White;
                buttonD13.Text = "O";
                isclickD13 = false;
                D13 = false;
                port.Write("#WRITE013-0\n");
            }
            else
            {
                buttonD13.BackColor = Color.Red;
                buttonD13.ForeColor = Color.Black;
                buttonD13.Text = "I";
                isclickD13 = true;
                D13 = true;
                port.Write("#WRITE013-1\n");
            }
        }

        private void button23_Click(object sender, EventArgs e)
        {
            if (isclickD9)
            {
                buttonD9.BackColor = Color.Black;
                buttonD9.ForeColor = Color.White;
                buttonD9.Text = "O";
                isclickD9 = false;
                D9 = false;
                port.Write("#WRITE009-0\n");
            }
            else
            {
                buttonD9.BackColor = Color.Red;
                buttonD9.ForeColor = Color.Black;
                buttonD9.Text = "I";
                isclickD9 = true;
                D9 = true;
                port.Write("#WRITE009-1\n");
            }
        }

        private void buttonSIO13_Click_1(object sender, EventArgs e)
        {
            if (!isclickSIO13)
            {
                SIO13 = ++SIO13 % 3;
                switch (SIO13)
                {
                    case 0:
                        buttonSIO13.BackColor = Color.PaleGreen;
                        buttonSIO13.Text = "INPUT";
                        buttonD13.Enabled = false;
                        buttonD13.Visible = false;
                        break;
                    case 1:
                        buttonSIO13.BackColor = Color.DarkOrange;
                        buttonSIO13.Text = "INPUT👆";
                        buttonD13.Enabled = false;
                        buttonD13.Visible = false;
                        break;
                    case 2:
                        buttonSIO13.BackColor = Color.LightSeaGreen;
                        buttonSIO13.Text = "OUTPUT";
                        buttonD13.Enabled = true;
                        buttonD13.Visible = true;
                        break;
                    default:
                        break;
                }

            }
        }
        private void buttonSIO12_Click(object sender, EventArgs e)
        {
            {
                if (!isclickSIO12)
                {
                    SIO12 = ++SIO12 % 3;
                    switch (SIO12)
                    {
                        case 0:
                            buttonSIO12.BackColor = Color.PaleGreen;
                            buttonSIO12.Text = "INPUT";
                            buttonD12.Enabled = false;
                            buttonD12.Visible = false;
                            break;
                        case 1:
                            buttonSIO12.BackColor = Color.DarkOrange;
                            buttonSIO12.Text = "INPUT👆";
                            buttonD12.Enabled = false;
                            buttonD12.Visible = false;
                            break;
                        case 2:
                            buttonSIO12.BackColor = Color.LightSeaGreen;
                            buttonSIO12.Text = "OUTPUT";
                            buttonD12.Enabled = true;
                            buttonD12.Visible = true;
                            break;
                        default:
                            break;
                    }

                }
            }
        }

        private void buttonSIO11_Click(object sender, EventArgs e)
        {
            
            {
                if (!isclickSIO11)
                {
                    SIO11 = ++SIO11 % 4;
                    switch (SIO11)
                    {
                        case 0:
                            buttonSIO11.BackColor = Color.PaleGreen;
                            buttonSIO11.Text = "INPUT";
                            buttonD11.Enabled = false;
                            buttonD11.Visible = false;
                            textBoxPWM11.Enabled = false;
                            textBoxPWM11.Visible = false;
                            hSBarPWM11.Enabled = false;
                            hSBarPWM11.Visible = false;
                            break;
                        case 1:
                            buttonSIO11.BackColor = Color.DarkOrange;
                            buttonSIO11.Text = "INPUT👆";
                            buttonD11.Enabled = false;
                            buttonD11.Visible = false;
                            textBoxPWM11.Enabled = false;
                            textBoxPWM11.Visible = false;
                            hSBarPWM11.Enabled = false;
                            hSBarPWM11.Visible = false;
                            break;
                        case 2:
                            buttonSIO11.BackColor = Color.LightSeaGreen;
                            buttonSIO11.Text = "OUTPUT";
                            buttonD11.Enabled = true;
                            buttonD11.Visible = true;
                            textBoxPWM11.Enabled = false;
                            textBoxPWM11.Visible = false;
                            hSBarPWM11.Enabled = false;
                            hSBarPWM11.Visible = false;
                            break;
                        case 3:
                            buttonSIO11.BackColor = Color.RoyalBlue;
                            buttonSIO11.Text = "PWM";
                            buttonD11.Enabled = false;
                            buttonD11.Visible = false;
                            textBoxPWM11.Enabled = true;
                            textBoxPWM11.Visible = true;
                            hSBarPWM11.Enabled = true;
                            hSBarPWM11.Visible = true;
                            break;
                        default:
                            break;
                    }

                }
            }
        }
    }
}
