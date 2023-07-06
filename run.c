#include <unistd.h>

int main() {
    char* args[] = {"python", "engine.py"};
    execvp(args[0],args);
    return 0;
}
