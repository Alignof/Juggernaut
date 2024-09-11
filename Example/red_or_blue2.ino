//=============================================================================
//  START of giver code (copy the below code you wrote into the specification)
//=============================================================================

struct Challenge RedOrBlue2 = {
    .gaming = red_or_blue_2,
    .setup_pin = setup_rob2,
    .time_limit = 300,
};

// giver pin assgin
const uint8_t RED_WIRE2 = 23;
const uint8_t BLUE_WIRE2 = 18;

void setup_rob2(void) {
	pinMode(RED_WIRE2,  INPUT_PULLUP);
	pinMode(BLUE_WIRE2, INPUT_PULLUP);
}

void red_or_blue_2(void *pvParameters) {
	bool flag1 = false;
	bool flag2 = false;

	while(1) {
		flag1 = (digitalRead(BLUE_WIRE2) == HIGH);
		flag2 = (digitalRead(RED_WIRE2)  == HIGH);
		
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
