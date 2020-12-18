/*
 * @date		2020 10/31-
 * @development name	Juggernaut
 * @author		Takana Norimasa <j17423@kisarazu.kosen-ac.jp>
 * @brief		Educational bomb disposal game
 * @repository		https://github.com/Takana-Norimasa/Juggernaut
 */ 

#include "freertos/task.h"
#include "freertos/event_groups.h"

#define SER         27
#define RCLK        26
#define SRCLK       25
#define DATASIZE    16

// giver pin assgin
const uint8_t GRAY_BUTTON = 23;

typedef enum{
	RED,
	YELLOW,
	GREEN,
}SIGNAL;

EventGroupHandle_t eg_handle;
bool timer_stop = false;
SIGNAL signal   = YELLOW;

void gaming(void *pvParameters){
	while(1){
		delay(1);
		if(digitalRead(GRAY_BUTTON) == LOW){
			signal     = GREEN;
			timer_stop = true;
			while(1) delay(1e5);
		}
	}
}

void display(void *pvParameters){
	int ms;
	int time_limit = 30;
	long long i,j;
	long long start;
	long long minits,second;

	start  = millis();
	second = time_limit%60;
	minits = time_limit/60;

	while(time_limit > 0){
		if(!(timer_stop)){
			ms = millis()-start;
			if(ms>1000){
				time_limit--;
				minits = time_limit/60;
				second = time_limit%60;
				start  = millis();
			}
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

void loop(){delay(1e5);}
void setup(){
	Serial.begin(115200); 
	eg_handle=xEventGroupCreate();

	pinMode(SER,   OUTPUT);
	pinMode(RCLK,  OUTPUT);
	pinMode(SRCLK, OUTPUT);

	// === declared by giver ===
	pinMode(GRAY_BUTTON, INPUT_PULLUP);
	// =====================

	xTaskCreatePinnedToCore(gaming,  "gaming",  8192, NULL, 1, NULL, 1);
	xTaskCreatePinnedToCore(display, "display", 8192, NULL, 1, NULL, 1);
	delay(100);
}

void data_send(int digit, int num, SIGNAL rgb){
	int i;
	uint16_t data;
	int seg[11] = {0x3f,0x06,0x5b,0x4f,0x66,0x6d,0x7d,0x07,0x7f,0x6f,0x03};

	/*
	 *                QH <--------- QA
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

