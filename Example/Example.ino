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

#define CHALLENGES_NUM 1
void (* challenges[CHALLENGES_NUM]) (void *) = {
    &red_or_blue,
};
