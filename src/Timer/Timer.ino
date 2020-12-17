#define SER         27
#define RCLK        26
#define SRCLK       25
#define LED_BUILTIN 2
#define DATASIZE    16

typedef enum{
	RED,
	YELLOW,
	GREEN,
}SIGNAL;

SIGNAL signal = YELLOW;

void data_send(int digit, int num, SIGNAL signal);

void setup(){
	Serial.begin(115200); 
	pinMode(SER, OUTPUT);
	pinMode(RCLK, OUTPUT);
	pinMode(SRCLK, OUTPUT);
	pinMode(LED_BUILTIN, OUTPUT);
}

void loop(){
	int ms;
	int time_limit = 30;
	long long i,j;
	long long start;
	long long minits,second;

	start  = millis();
	second = time_limit%60;
	minits = time_limit/60;

	while(time_limit > 0){
		ms = millis()-start;
		if(ms>1000){
			time_limit--;
			minits = time_limit/60;
			second = time_limit%60;
			start  = millis();
		}

		data_send(5, 10,	     signal);
		data_send(4, (minits/10)%10, signal);
		data_send(3, minits%10,	     signal);
		data_send(2, (second/10)%10, signal);
		data_send(1, second%10,      signal);
		
	}

	// time over
	signal = RED;
	while(1){
		data_send(5, 10, signal);
		for(int i=1;i<=4;i++){
			data_send(i, 0, signal);
		}
	}
}

void data_send(int digit, int num, SIGNAL rgb){
	int i;
	uint16_t data;
	int seg[11] = {0x3f,0x06,0x5b,0x4f,0x66,0x6d,0x7d,0x07,0x7f,0x6f,0x03};

	/*
	 *                QH <---------- QA
	 * register Left  :a,b,c,d,e,f,g,#,
	 * register Right :                R,Y,G,4,3,2,1,DP 
	 */
	data = (1<<(digit+10))+(1<<(rgb+8))+(seg[num]);

	Serial.println(data,BIN);
	digitalWrite(RCLK,LOW);
	for(i = 0;i < DATASIZE;i++){
		digitalWrite(SER,(data>>i)&1);
		digitalWrite(SRCLK,LOW);
		digitalWrite(SRCLK,HIGH);
	}
	digitalWrite(RCLK,HIGH);
}

