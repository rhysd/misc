const std = @import("std");

pub fn main() void {
    var i: u32 = 1;
    while (i <= 100) : (i += 1) {
        if (i % 15 == 0) {
            std.debug.print("fizzbuzz\n", .{});
        } else if (i % 3 == 0) {
            std.debug.print("fizz\n", .{});
        } else if (i % 5 == 0) {
            std.debug.print("buzz\n", .{});
        } else {
            std.debug.print("{}\n", .{i});
        }
    }
}
