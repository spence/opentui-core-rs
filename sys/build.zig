const std = @import("std");

pub fn build(b: *std.Build) void {
    const optimize = b.standardOptimizeOption(.{});
    const target_option = b.option(
        []const u8,
        "target",
        "Build for specific target",
    );

    const target = if (target_option) |t| blk: {
        const q = std.Target.Query.parse(.{ .arch_os_abi = t }) catch
            @panic("invalid target");
        break :blk b.resolveTargetQuery(q);
    } else b.resolveTargetQuery(.{});

    const build_options = b.addOptions();
    build_options.addOption(bool, "gpa_safe_stats", false);

    const upstream = b.path("vendor/opentui/packages/core/src/zig");

    const module = b.createModule(.{
        .root_source_file = upstream.path(b, "lib.zig"),
        .target = target,
        .optimize = optimize,
    });

    module.addOptions("build_options", build_options);

    if (b.lazyDependency("uucode", .{
        .target = target,
        .optimize = optimize,
        .fields = @as([]const []const u8, &.{
            "grapheme_break",
            "east_asian_width",
            "general_category",
            "is_emoji_presentation",
        }),
    })) |uucode_dep| {
        module.addImport("uucode", uucode_dep.module("uucode"));
    }

    const lib = b.addLibrary(.{
        .name = "opentui",
        .root_module = module,
        .linkage = .static,
    });

    b.installArtifact(lib);
}
