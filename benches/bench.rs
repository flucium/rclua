#![feature(test)]
extern crate test;

use test::Bencher;

const CODE: &str = r"
local count = 0

function hanoi(n, a, b, c)
    if n > 0 then
        hanoi(n-1, a, c, b)
        
        count = count + 1
        
        hanoi(n-1, c, b, a)
    end
end

hanoi(3, 'a', 'b', 'c')
";

#[bench]
fn bench_execute_5_4_6(b: &mut Bencher) {
    b.iter(|| {
        rclua::execute(CODE, rclua::Version::V5_4_6);
    });
}

#[bench]
fn bench_execute_5_4_0(b: &mut Bencher) {
    b.iter(|| {
        rclua::execute(CODE, rclua::Version::V5_4_0);
    });
}


#[bench]
fn bench_lua_obj_5_4_0_execute(b: &mut Bencher) {
    
    b.iter(|| {
        let mut lua = rclua::Lua::new(rclua::Version::V5_4_0);
        
        lua.load(CODE).unwrap();
        
        lua.execute();    
    });
}

#[bench]
fn bench_lua_obj_5_4_6_execute(b: &mut Bencher) {
    
    b.iter(|| {
        let mut lua = rclua::Lua::new(rclua::Version::V5_4_6);
        
        lua.load(CODE).unwrap();
        
        lua.execute();
    });
}