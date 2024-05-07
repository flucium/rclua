release()
{
  cd lua-5.4.6/src
  make
  ar rcs liblua.a ./*.o
  mv ./liblua.a ../liblua.a
  cd ../../c/src
  gcc -c ./clua.c -o clua.o
  ar rcs libclua.a clua.o
  mv ./libclua.a ../libclua.a
  cd ../..
  cargo build --release
}

debug()
{
  cd lua-5.4.6/src
  make
  ar rcs liblua.a ./*.o
  mv ./liblua.a ../liblua.a
  cd ../../c/src
  gcc -c ./clua.c -o clua.o
  ar rcs libclua.a clua.o
  mv ./libclua.a ../libclua.a
  cd ../..
  cargo build
}

test()
{
  cd lua-5.4.6/src
  make
  ar rcs liblua.a ./*.o
  mv ./liblua.a ../liblua.a
  cd ../../c/src
  gcc -c ./clua.c -o clua.o
  ar rcs libclua.a clua.o
  mv ./libclua.a ../libclua.a
  cd ../..
  cargo test
}

clean()
{
  cargo clean
  rm ./c/*.a ./c/src/*.o
  rm ./lua-5.4.6/src/*.o ./lua-5.4.6/liblua.a
}

for entry in "$@" ; do
  r=$?
  if [ "x$r" = "x0" ]; then
    $entry
  fi
done