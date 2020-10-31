//#define LED_BUILTIN 2
#define SER 27
#define RCLK 26
#define SRCLK 25
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
}

void loop(){
        int ms;
        int limit = 300;
        long long i,j;
        long long start;
        long long minits,second;

        start=millis();
        second = limit%60;
        minits = limit/60;

        while(limit>=0){
                ms = millis()-start;
                if(ms>1000){
                        limit--;
                        minits = limit/60;
                        second = limit%60;
                        start  = millis();
                }

                data_send(3, (minits/10)%10, YELLOW);
                data_send(2, minits%10,	     YELLOW);
                data_send(1, (second/10)%10, YELLOW);
                data_send(0, second%10,      YELLOW);
        }
}

void data_send(int digit, int num, RGB rgb){
        int i;
        uint16_t data;
        int seg[10] = {0x3f,0x06,0x5b,0x4f,0x66,0x6d,0x7d,0x07,0x7f,0x6f};

        data = (1<<(digit+11))+(1<<(rgb+8))+(seg[num]);

        Serial.println(data,BIN);
        digitalWrite(RCLK,LOW);
        for(i = 0;i < DATASIZE;i++){
                digitalWrite(SER,(data>>i)&1);
                digitalWrite(SRCLK,LOW);
                digitalWrite(SRCLK,HIGH);
        }
        digitalWrite(RCLK,HIGH);
}

