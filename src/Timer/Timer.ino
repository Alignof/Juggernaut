/*
 * @date		2020 10/31-
 * @code name	Juggernaut
 * @author		Takana Norimasa <Alignof@outlook.com>
 * @brief		Educational bomb disposal game
 * @repository	https://github.com/Takana-Norimasa/Juggernaut
 */ 

#include "freertos/task.h"
#include "freertos/event_groups.h"

#define SER      27
#define RCLK     26
#define SRCLK    25
#define BUZZER   12
#define DATASIZE 16

// giver pin assgin
const uint8_t NAVY_BUTTON  = 22;
const uint8_t WHITE_BUTTON = 18;
const uint8_t RED_BUTTON   = 19;
const uint8_t BLUE_BUTTON  = 23;

typedef enum {
	RED,
	YELLOW,
	GREEN,
} SIGNAL;

int time_limit = 300;
bool timer_stop = false;
SIGNAL signal = YELLOW;
EventGroupHandle_t eg_handle;

void gaming(void *pvParameters) {
	bool flag1 = false;
	bool flag2 = false;
	bool flag3 = false;
	bool flag4 = false;
	while(1) {
		delay(1);
		flag1 = (digitalRead(NAVY_BUTTON)  == LOW);
		flag2 = (digitalRead(WHITE_BUTTON) == HIGH);
		flag3 = (digitalRead(BLUE_BUTTON)  == LOW);
		flag4 = (digitalRead(RED_BUTTON)   == HIGH);
		
		// succeeded
		if(flag1 && flag2 && flag3) {
			signal     = GREEN;
			timer_stop = true;
			while(1) delay(1e5);
		}

		// failed
		if(!flag4) {
			signal = RED;
			timer_stop = true;
			digitalWrite(BUZZER, HIGH);
			while(1) delay(1e5);
		}
	}
}

void display(void *pvParameters) {
	int ms;
	long long i,j;
	long long start;
	long long minits,second;

	start  = millis();
	second = time_limit%60;
	minits = time_limit/60;

	while(time_limit > 0) {
		if(!(timer_stop)) {
			ms = millis()-start;
			if(ms>1000) {
				time_limit--;
				minits = time_limit/60;
				second = time_limit%60;
				start  = millis();
				digitalWrite(BUZZER, HIGH);
			}else{
				digitalWrite(BUZZER, LOW);
			}
		}

		data_send(4, (minits/10)%10, signal);
		data_send(3, minits%10,	     signal);
		data_send(5, 10, signal);
		data_send(2, (second/10)%10, signal);
		data_send(1, second%10,      signal);
	}

	// time over
	signal = RED;
	digitalWrite(BUZZER, HIGH);
	while(1) {
		data_send(5, 10, signal);
		for(int i=1;i<=4;i++) {
			data_send(i, 0, signal);
		}
	}
}

void loop() { delay(1e5); }

void setup() {
	eg_handle=xEventGroupCreate();

	pinMode(SER,    OUTPUT);
	pinMode(RCLK,   OUTPUT);
	pinMode(SRCLK,  OUTPUT);
	pinMode(BUZZER, OUTPUT);

	// === declared by giver ===
	pinMode(NAVY_BUTTON,  INPUT_PULLUP);
	pinMode(WHITE_BUTTON, INPUT_PULLUP);
	pinMode(RED_BUTTON,   INPUT_PULLUP);
	pinMode(BLUE_BUTTON,  INPUT_PULLUP);
	// =====================

	xTaskCreatePinnedToCore(gaming,  "gaming",  8192, NULL, 1, NULL, 1);
	xTaskCreatePinnedToCore(display, "display", 8192, NULL, 1, NULL, 1);
	delay(100);
}

void data_send(int digit, int num, SIGNAL rgb) {
	int i;
	uint16_t data;
	int seg[11] = {0x3f, 0x06, 0x5b, 0x4f, 0x66, 0x6d, 0x7d, 0x07, 0x7f, 0x6f, 0x03};

	/*
	 *                QH <--------- QA
	 * register Left  :a,b,c,d,e,f,g,#,
	 * register Right :                R,Y,G,4,3,2,1,DP 
	 */
	data = (1<<(digit+10))+(1<<(rgb+8))+(seg[num]);

	Serial.println(data, BIN);
	digitalWrite(RCLK, LOW);
	for(i = 0;i < DATASIZE;i++) {
		digitalWrite(SER, (data>>i)&1);
		digitalWrite(SRCLK, LOW);
		digitalWrite(SRCLK, HIGH);
	}
	digitalWrite(RCLK, HIGH);
    delay(1);
}

