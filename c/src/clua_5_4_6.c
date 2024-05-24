#include "../../lua-5.4.6/src/lua.h"
#include "../../lua-5.4.6/src/lauxlib.h"
#include "../../lua-5.4.6/src/lualib.h"

extern int execute_5_4_6(const char *code);

int execute_5_4_6(const char *code)
{
    lua_State *L = luaL_newstate();
    
    luaL_openlibs(L);

    if (luaL_dostring(L, code))
    {
        const char *error = lua_tostring(L, -1);
    
        printf("Error: %s\n", error);
    
        lua_pop(L, 1);
    
        lua_close(L);
    
        return 1;
    }

    lua_close(L);
    
    return 0;
}