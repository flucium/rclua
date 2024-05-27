#![feature(test)]

extern crate test;

#[bench]
fn bench_lua_5_4_0(b: &mut test::Bencher) {
    const CODE:&str = r"
local count = 0

function hanoi(n, a, b, c)
    if n > 0 then
        hanoi(n-1, a, c, b)
        
        count = count + 1
        
        hanoi(n-1, c, b, a)
    end
end

hanoi(5, 'a', 'b', 'c')
";

    b.iter(|| {
        rclua::execute(CODE, rclua::Version::V5_4_0);
    });
}


#[bench]
fn bench_lua_5_4_6(b: &mut test::Bencher) {
    const CODE:&str = r"
local count = 0

function hanoi(n, a, b, c)
    if n > 0 then
        hanoi(n-1, a, c, b)
        
        count = count + 1
        
        hanoi(n-1, c, b, a)
    end
end

hanoi(5, 'a', 'b', 'c')
";

    b.iter(|| {
        rclua::execute(CODE, rclua::Version::V5_4_6);
    });
}