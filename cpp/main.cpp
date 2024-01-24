#include "day9.h"
#include <iostream>
#include <stdlib.h>

using namespace std;

int main(int argc, char *argv[])
{
    if (argc != 2)
    {
        cout << "Specify a day number" << endl;
        return 1;
    }

    int day = strtol(argv[1], NULL, 10);
    if (day == 9)
    {
        return day9::main();
    }
    return 2;
}
