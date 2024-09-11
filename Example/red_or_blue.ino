//=============================================================================
//  START of giver code (copy the below code you wrote into the specification)
//=============================================================================

struct Challenge RedOrBlue = {
    .gaming = red_or_blue,
    .setup_pin = setup_rob,
    .time_limit = 300,
};

// giver pin assgin
const uint8_t RED_WIRE = 23;
const uint8_t BLUE_WIRE = 18;

void setup_rob(void) {
	pinMode(RED_WIRE,  INPUT_PULLUP);
	pinMode(BLUE_WIRE, INPUT_PULLUP);
}

void red_or_blue(void *pvParameters) {
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
