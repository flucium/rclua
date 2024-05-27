lua_5_4_6()
{
  # Lua 5.4.6
  cd lua-5.4.6/src
  make
  ar rcs liblua.a ./*.o
  mv ./liblua.a ../liblua.a

  # Back to root
  cd ../..
}

lua_5_4_0()
{
  # Lua 5.4.0
  cd lua-5.4.0/src
  make
  ar rcs liblua.a ./*.o
  mv ./liblua.a ../liblua.a

  # Back to root
  cd ../..
}

clua_5_4_6()
{
  # To CLua
  cd c/src
  
  # CLua 5.4.6
  gcc -c ./clua_5_4_6.c -o clua_5_4_6.o
  ar rcs libclua_5_4_6.a clua_5_4_6.o
  mv ./libclua_5_4_6.a ../libclua_5_4_6.a

  # Back to root
  cd ../..
}

clua_5_4_0()
{
  # To CLua
  cd c/src

  # CLua 5.4.0
  gcc -c ./clua_5_4_0.c -o clua_5_4_0.o
  ar rcs libclua_5_4_0.a clua_5_4_0.o
  mv ./libclua_5_4_0.a ../libclua_5_4_0.a

  # Back to root
  cd ../..
}

doc()
{
  cargo doc
}

doc_open()
{
  cargo doc --open
}

release()
{
  lua_5_4_6
  lua_5_4_0
  clua_5_4_6
  clua_5_4_0
  cargo build --release
}

debug()
{
  lua_5_4_6
  lua_5_4_0
  clua_5_4_6
  clua_5_4_0
  cargo build
}

package()
{ 
  # debug
  release
  cargo package --allow-dirty
}

clean()
{
  # Remove ./lua-*/src/*.o
  rm ./lua-*/src/*.o
  
  # Remove ./lua-*/*.a
  rm ./lua-*/*.a

  # Remove ./c/src/*.o
  rm ./c/src/*.o

  # Remove ./c/*.a
  rm ./c/*.a

  cargo clean
}

for entry in "$@" ; do
  r=$?
  if [ "x$r" = "x0" ]; then
    $entry
  fi
done