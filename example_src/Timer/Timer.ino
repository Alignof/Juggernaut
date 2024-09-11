/*
 * @date		2020 10/31-
 * @code name	Juggernaut
 * @author		Takana Norimasa <Alignof@outlook.com>
 * @brief		Educational bomb disposal game
 * @repository	https://github.com/Alignof/Juggernaut
 */

#include "control.h"
#include "freertos/event_groups.h"
#include "freertos/task.h"

//=============================================================================
//  START of giver code (copy the below code you wrote into the specification)
//=============================================================================

int time_remain = 300;

// giver pin assgin
const uint8_t NAVY_BUTTON  = 22;
const uint8_t WHITE_BUTTON = 18;
const uint8_t RED_BUTTON   = 19;
const uint8_t BLUE_BUTTON  = 23;

void setup_pin(void) {
	pinMode(NAVY_BUTTON, INPUT_PULLUP);
	pinMode(WHITE_BUTTON, INPUT_PULLUP);
	pinMode(RED_BUTTON, INPUT_PULLUP);
	pinMode(BLUE_BUTTON, INPUT_PULLUP);
}

void gaming(void *pvParameters) {
	bool flag1 = false;
	bool flag2 = false;
	bool flag3 = false;
	bool flag4 = false;

	while(1) {
		delay(1);
		flag1 = (digitalRead(NAVY_BUTTON) == LOW);
		flag2 = (digitalRead(WHITE_BUTTON) == HIGH);
		flag3 = (digitalRead(BLUE_BUTTON) == LOW);
		flag4 = (digitalRead(RED_BUTTON) == HIGH);

		// succeeded
		if(flag1 && flag2 && flag3) {
			succeeded();
		}

		// failed
		if(!flag4) {
			failed();
		}
	}
}

//=============================================================================
//  END of giver code
//=============================================================================
