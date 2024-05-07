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