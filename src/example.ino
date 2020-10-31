#define LED_BUILTIN 2
#define SER 27
#define RCLK 26
#define SRCLK 25
#define DIGIT1 15
#define DIGIT2 4
#define DIGIT3 18
#define DIGIT4 19
#define DIGIT5 23
#define DIGIT6 21
#define CORON  2
#define DATASIZE 16

typedef enum{
	GREEN,
	YELLOW,
	RED,
}RGB;

void data_send(int digit, int num, RGB rgb);

void setup(){
	Serial.begin(115200); 
	pinMode(SER,OUTPUT);
	pinMode(RCLK,OUTPUT);
	pinMode(SRCLK,OUTPUT);
	pinMode(DIGIT1,OUTPUT);
	pinMode(DIGIT2,OUTPUT);
	pinMode(DIGIT3,OUTPUT);
	pinMode(DIGIT4,OUTPUT);
	pinMode(DIGIT5,OUTPUT);
	pinMode(DIGIT6,OUTPUT);
	pinMode(CORON,OUTPUT);
	//pinMode(LED_BUILTIN,OUTPUT);

	digitalWrite(DIGIT1,HIGH);
	digitalWrite(DIGIT2,HIGH);
	digitalWrite(DIGIT3,HIGH);
	digitalWrite(DIGIT4,HIGH);
	digitalWrite(DIGIT5,LOW);
	digitalWrite(DIGIT6,LOW);
	digitalWrite(CORON,HIGH);
}

void loop(){
	int ms;
	int limit=300;
	long long i,j;
	long long start;
	long long minits,second;

/*
	start=millis();
	second=limit%60;
	minits=limit/60;
	
	while(limit>=0){
		ms=millis()-start;
		if(ms>1000){
			limit--;
			minits=limit/60;
			second=limit%60;
			start=millis();
		}

		data_send(0,(minits/10)%10);
		delay(500);
		data_send(1,minits%10);
		delay(500);
		data_send(2,(second/10)%10);
		delay(500);
		data_send(3,second%10);
		delay(500);
		data_send(4,2);
		delay(500);
		data_send(5,ms/100);
		delay(500);
		data_send(6,(ms/10)%10);
		delay(500);
		Serial.printf("\n");
	}
*/
	while(1){
		data_send(0, 1, YELLOW);
		data_send(1, 2, YELLOW);
		data_send(2, 4, YELLOW);
		data_send(3, 2, YELLOW);
		data_send(4, 2, YELLOW);
	}
}

void data_send(int digit, int num, RGB rgb){
	int i;
	uint16_t data;
	static int prev=DIGIT4;
	int digit_pin[7]={DIGIT1,DIGIT2,DIGIT3,DIGIT4,GREEN,YELLOW,RED};
	int seg[10]={0x3f,0x06,0x5b,0x4f,0x66,0x6d,0x7d,0x07,0x7f,0x6f};

	data=(1<<(rgb+13))+(1<<(digit+9))+(seg[num]);
	
	//Serial.printf("digit: %d\t num: %d\n",digit,num);
	Serial.println(data,BIN);
	digitalWrite(RCLK,LOW);
	for(i=0;i<DATASIZE;i++){
		digitalWrite(SER,(data>>i)&1);
		digitalWrite(SRCLK,LOW);
		digitalWrite(SRCLK,HIGH);
	}
	digitalWrite(RCLK,HIGH);

	digitalWrite(prev,!(digitalRead(prev)));
	
	//digitalWrite(digit_pin[digit],HIGH);
	prev=digit_pin[digit];
}
