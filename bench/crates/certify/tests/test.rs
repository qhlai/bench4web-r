use clap::Command;

use shadow_rs::shadow;

shadow!(build);

extern crate chrono; // 时间库
use chrono::prelude::*;
use std::time::{SystemTime,Duration};
use chrono::Duration as Durationx; //这样就可以编译通过了
use chrono::ParseResult;
#[test]
fn maincd() {
    Command::new("example_shadow")
        .version(build::CLAP_LONG_VERSION)
        .get_matches(); //USAGE: ./example_shadow -V

    // shadow-rs built in function
    println!("is_debug:{}", shadow_rs::is_debug());
    println!("branch:{}", shadow_rs::branch());
    println!("tag:{}", shadow_rs::tag());
    println!("git_clean:{}", shadow_rs::git_clean());
    println!("git_status_file:{}", shadow_rs::git_status_file());

    // print_build()

    build::print_build_in();
}

#[test]
fn local_utc_match() {
    println!("version:{}", build::VERSION);
    println!("version:{}", build::CLAP_LONG_VERSION);
    println!("pkg_version:{}", build::PKG_VERSION);
    println!("pkg_version_major:{}", build::PKG_VERSION_MAJOR);
    println!("pkg_version_minor:{}", build::PKG_VERSION_MINOR);
    println!("pkg_version_patch:{}", build::PKG_VERSION_PATCH);
    println!("pkg_version_pre:{}", build::PKG_VERSION_PRE);

    println!("tag:{}", build::TAG);
    println!("branch:{}", build::BRANCH);
    println!("commit_id:{}", build::COMMIT_HASH);
    println!("short_commit:{}", build::SHORT_COMMIT);
    println!("commit_date:{}", build::COMMIT_DATE);
    println!("commit_date_2822:{}", build::COMMIT_DATE_2822);
    println!("commit_date_3339:{}", build::COMMIT_DATE_3339);
    println!("commit_author:{}", build::COMMIT_AUTHOR);
    println!("commit_email:{}", build::COMMIT_EMAIL);

    println!("build_os:{}", build::BUILD_OS);
    println!("rust_version:{}", build::RUST_VERSION);
    println!("rust_channel:{}", build::RUST_CHANNEL);
    println!("cargo_version:{}", build::CARGO_VERSION);
    println!("cargo_tree:{}", build::CARGO_TREE);

    println!("project_name:{}", build::PROJECT_NAME);
    println!("build_time:{}", build::BUILD_TIME);
    println!("build_time_2822:{}", build::BUILD_TIME_2822);
    println!("build_time_3339:{}", build::BUILD_TIME_3339);
    println!("build_rust_channel:{}", build::BUILD_RUST_CHANNEL);
}

#[test]
fn expire() {
    // 字符串转换时间类型2022-07-05T18:00:38+08:00
    let compiled_time: DateTime<FixedOffset> =
        DateTime::parse_from_str(build::BUILD_TIME_3339, "%Y-%m-%dT%H:%M:%S%z").unwrap();
    println!("compiled_time({})", compiled_time);
    let valid_time=compiled_time+Durationx::days(7);
    println!("valid_time({})", valid_time);

    let local: DateTime<Local> = Local::now(); // 本地时间
    let now: DateTime<FixedOffset>=local.try_into().unwrap();
    let expire=valid_time>now;
    let remain=valid_time-now;
    println!("{}  {}",expire,remain);
}
#[test]
fn chrono_test() {
    let sys_time = SystemTime::now();
    println!("now time 显示的是一个长整数 {:?}", sys_time);
    let local: DateTime<Local> = Local::now(); // 本地时间
    println!(
        " date 精确到秒 ({:?})",
        local.format("%Y-%m-%d %H:%M:%S").to_string()
    );
    println!(
        " date 精确到毫秒 ({:?})",
        local.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
    );
    println!(
        " date 精确到微秒 ({:?})",
        local.format("%Y-%m-%d %H:%M:%S%.6f").to_string()
    );
    println!(
        " date 精确到纳秒 ({:?})",
        local.format("%Y-%m-%d %H:%M:%S%.9f").to_string()
    );
    let _t0u0 = Utc.ymd(2014, 7, 8).and_hms_micro(9, 10, 11, 12_000); // Utc带时区 全球时间
    let _t0u1 = Utc.ymd(2014, 7, 8).and_hms_nano(9, 10, 11, 12_000_000);
    let _t0l0 = Local.ymd(2014, 7, 8).and_hms_micro(9, 10, 11, 12_000); // 本地时间
    let _t0l1 = Local.ymd(2014, 7, 8).and_hms_nano(9, 10, 11, 12_000_000);
    // 时间加减
    let _t0_as_0 = Utc.ymd(1970, 1, 1).and_hms(0, 0, 0) + Durationx::seconds(1_000_000_000);
    let _t0_as_1 = Utc.ymd(2020, 2, 1).and_hms(0, 0, 0)
        - Durationx::from_std(Duration::from_secs(1_000_000_258)).unwrap();
    println!(
        " 本地时间 20200201 + 1_000_000_000 10亿秒 把标准库10亿秒转换成time10亿秒 再加时间计算 {:?}",
        _t0_as_1.format("%y-%m-%d %H:%M:%S.%3f").to_string()
    );

    let _t0_as_2 = Local.ymd(2020, 1, 1).and_hms(0, 0, 0)
        + Durationx::from_std(Duration::from_millis(1_258_101_957)).unwrap(); // Durationx::from_std 方法从标准库的时间戳转换

    println!(
        " 本地时间 20200101 + 2_000_000_000 20亿秒 转换标准库 12亿 毫秒 {:?}",
        _t0_as_2.format("%y-%m-%d %H:%M:%S.%3f").to_string()
    );

    println!(
        " 本地时间 20200101 + 2_000_000_000 20亿秒 {:?}",
        _t0_as_2.format("%Y-%m-%d %H:%M:%S.%6f").to_string()
    );
    println!(
        " 本地时间 20200101 + 2_000_000_000 20亿秒 {:?}",
        _t0_as_2.format("%Y-%m-%d %H:%M:%S.%9f").to_string()
    );
    let t0_fmt_01 =
        DateTime::parse_from_str("1983 Apr 13 12:09:14.274 +0000", "%Y %b %d %H:%M:%S%.3f %z"); // 注意 %.3f 这个写法
    println!(
        r#" 从字符串 "1983 Apr 13 12:09:14.274 +0000", 格式字符： "%Y %b %d %H:%M:%S%.3f %z" 格式化而来的时间 显示 {:?}"#,
        t0_fmt_01.unwrap()
    );
    // 字符串转换时间类型
    let t1: ParseResult<DateTime<FixedOffset>> =
        DateTime::parse_from_str("2020-03-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z");
    println!("t1: ParseResult<DateTime<FixedOffset>>({:?})", t1);
    // 常见转换输入 写法 .ok().unwrap() 很重要
    let t2: ParseResult<DateTime<Utc>> =
        Utc.datetime_from_str("2020-03-06 12:00:09", "%Y-%m-%d %H:%M:%S");
    println!(
        "t2: ParseResult<DateTime<Utc>>({:?})",
        t2.ok().unwrap().format("%y-%m-%d %H:%M:%S").to_string()
    );
    //
    let t3: DateTime<Local> = Local
        .datetime_from_str("2020-03-28 12:00:09", "%Y-%m-%d %H:%M:%S")
        .ok()
        .unwrap();
    println!("t3: DateTime<Local>({:?})", t3);
    
}
