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

struct Challenge {
    void (* gaming)(void *);
    void (* setup_pin)(void);
    int time_limit;
};

#define CHALLENGES_NUM 2
extern struct Challenge RedOrBlue;
extern struct Challenge RedOrBlue2;
struct Challenge *challenges[CHALLENGES_NUM] = {
    &RedOrBlue,
    &RedOrBlue2,
};
