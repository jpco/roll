/*  roll: Roll some dice.
    Copyright (C) 2017  Jack Conger
    Contact: sw@jpco.io

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <sys/random.h>

#define IS_D(x)      (x == 'd' || x == 'D')
#define IS_DIGIT(x)  (x >= 0x30 && x < 0x3a)
#define DIGIT_MIN    0x30
#define BUFSIZE      4096

unsigned int uniform_rand(unsigned const int max) {
    if (max == RAND_MAX) return random();
    unsigned const long end = (RAND_MAX / max) * max;
    unsigned int r;
    while ((r = random()) >= end);
    return 1 + (r % max);
}

size_t roll_one(const char *const in, unsigned int *const out) {
    size_t i = 0;
    unsigned int type = 0, ct = (IS_D(in[0]) ? 1 : 0);

#define NO_D   0
#define D_SEEN 1
#define D_LAST 2
    char c, roll = 0;

    for (; (c = in[i]); i++) {
        if (IS_DIGIT(c)) {
            if (roll) {
                roll &= ~D_LAST;
                type = type * 10 + (c - DIGIT_MIN);
            } else {
                ct = ct * 10 + (c - DIGIT_MIN);
            }
        } else if (IS_D(c) && !roll) {
            roll = D_SEEN | D_LAST;
        } else break;
    }

    if (type == 0) {
        if (!(roll & D_LAST)) return 0;
        i--;
        *out = ct;
    } else {
        unsigned int r = 0;
        for (unsigned int d = 0; d < ct; d++)
            r = r + uniform_rand(type);
        *out = r;
    }
    return i;
}

void roll(char *in, char *out) {
    size_t len = strlen(in), wi = 0;
    for (size_t ri = 0; ri < len; ri++) {
        char c = in[ri];
        if (IS_D(c) || IS_DIGIT(c)) {
            unsigned int roll;
            size_t res = roll_one(in + ri, &roll);
            if (res) {
                ri += res - 1;
                wi += sprintf(out + wi, "%d", roll);
            } else out[wi++] = c;
        } else out[wi++] = c;
    }
    out[wi] = '\0';
}

void seed() {
    unsigned int seed;
    getrandom(&seed, 2, 0);
    srandom(seed);
}

int main(int argc, char **argv) {
    seed();
    if (argc < 2) {
        char buf[BUFSIZE];
        size_t ct;
        while ((ct = read(STDIN_FILENO, &buf, BUFSIZE-1)) > 0) {
            buf[ct] = '\0';
            roll(buf, buf);
            printf("%s", buf);
        }
    } else {
        for (int i = 1; i < argc; i++) {
            roll(argv[i], argv[i]);
            printf((i > 1 ? " %s" : "%s"), argv[i]);
        }
        printf("\n");
    }
}
