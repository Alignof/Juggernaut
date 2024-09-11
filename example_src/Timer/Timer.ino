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

volatile int time_remain = 300;

// giver pin assgin
const uint8_t RED_WIRE = 23;
const uint8_t BLUE_WIRE = 18;

void setup_pin(void) {
	pinMode(RED_WIRE,  INPUT_PULLUP);
	pinMode(BLUE_WIRE, INPUT_PULLUP);
}

void gaming(void *pvParameters) {
	bool flag1 = false;
	bool flag2 = false;

	while(1) {
		flag1 = (digitalRead(RED_WIRE)  == HIGH);
		flag2 = (digitalRead(BLUE_WIRE) == HIGH);
		
		// succeeded
		if(flag1) {
            succeeded();
		}

		// failed
		if(flag2) {
            failed();
		}
	}
}

//=============================================================================
//  END of giver code
//=============================================================================
