use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rgsl::statistics::*;

extern crate source_code_parser;
use source_code_parser::*;

const directory_json: &'static str = r#"
{
    "instanceType": "DirectoryComponent",
    "path": "",
    "files": [],
    "subDirectories": [
      {
        "instanceType": "DIRECTORYCOMPONENT",
        "path": "/home/jacob/dev/DeathStarBench/mediaMicroservices",
        "instanceName": "/home/jacob/dev/DeathStarBench/mediaMicroservices::DirectoryComponent",
        "files": [
          "/home/jacob/dev/DeathStarBench/mediaMicroservices/docker",
          "/home/jacob/dev/DeathStarBench/mediaMicroservices/third_party",
          "/home/jacob/dev/DeathStarBench/mediaMicroservices/src",
          "/home/jacob/dev/DeathStarBench/mediaMicroservices/datasets",
          "/home/jacob/dev/DeathStarBench/mediaMicroservices/wrk2",
          "/home/jacob/dev/DeathStarBench/mediaMicroservices/docker-compose.yml",
          "/home/jacob/dev/DeathStarBench/mediaMicroservices/README.md",
          "/home/jacob/dev/DeathStarBench/mediaMicroservices/CMakeLists.txt",
          "/home/jacob/dev/DeathStarBench/mediaMicroservices/gen-py",
          "/home/jacob/dev/DeathStarBench/mediaMicroservices/.dockerignore",
          "/home/jacob/dev/DeathStarBench/mediaMicroservices/scripts",
          "/home/jacob/dev/DeathStarBench/mediaMicroservices/cmake",
          "/home/jacob/dev/DeathStarBench/mediaMicroservices/nginx-web-server",
          "/home/jacob/dev/DeathStarBench/mediaMicroservices/config",
          "/home/jacob/dev/DeathStarBench/mediaMicroservices/media_service.thrift",
          "/home/jacob/dev/DeathStarBench/mediaMicroservices/gen-lua",
          "/home/jacob/dev/DeathStarBench/mediaMicroservices/test",
          "/home/jacob/dev/DeathStarBench/mediaMicroservices/Dockerfile",
          "/home/jacob/dev/DeathStarBench/mediaMicroservices/docker-compose-sharding.yml",
          "/home/jacob/dev/DeathStarBench/mediaMicroservices/openshift",
          "/home/jacob/dev/DeathStarBench/mediaMicroservices/gen-cpp"
        ],
        "subDirectories": [
          {
            "instanceType": "DIRECTORYCOMPONENT",
            "path": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src",
            "instanceName": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src::DirectoryComponent",
            "files": [
              "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/UserReviewService",
              "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/tracing.h",
              "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/PlotService",
              "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/RatingService",
              "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/utils_mongodb.h",
              "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/utils.h",
              "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/UniqueIdService",
              "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/PageService",
              "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/logger.h",
              "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/CMakeLists.txt",
              "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/utils_memcached.h",
              "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/UserService",
              "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/GenericClient.h",
              "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/MovieReviewService",
              "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/MovieInfoService",
              "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/ThriftClient.h",
              "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/MovieIdService",
              "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/TextService",
              "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/ReviewStorageService",
              "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/ComposeReviewService",
              "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/RedisClient.h",
              "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/CastInfoService",
              "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/ClientPool.h"
            ],
            "subDirectories": [
              {
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/UserReviewService",
                "instanceName": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/UserReviewService::DirectoryComponent",
                "files": [
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/UserReviewService/CMakeLists.txt",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/UserReviewService/UserReviewHandler.h",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/UserReviewService/UserReviewService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/UserReviewService::PackageName"
              },
              {
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/PlotService",
                "instanceName": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/PlotService::DirectoryComponent",
                "files": [
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/PlotService/PlotHandler.h",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/PlotService/CMakeLists.txt",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/PlotService/PlotService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/PlotService::PackageName"
              },
              {
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/RatingService",
                "instanceName": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/RatingService::DirectoryComponent",
                "files": [
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/RatingService/CMakeLists.txt",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/RatingService/RatingService.cpp",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/RatingService/RatingHandler.h"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/RatingService::PackageName"
              },
              {
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/UniqueIdService",
                "instanceName": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/UniqueIdService::DirectoryComponent",
                "files": [
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/UniqueIdService/CMakeLists.txt",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/UniqueIdService/UniqueIdHandler.h",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/UniqueIdService/UniqueIdService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/UniqueIdService::PackageName"
              },
              {
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/PageService",
                "instanceName": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/PageService::DirectoryComponent",
                "files": [
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/PageService/CMakeLists.txt",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/PageService/PageService.cpp",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/PageService/PageHandler.h"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/PageService::PackageName"
              },
              {
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/UserService",
                "instanceName": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/UserService::DirectoryComponent",
                "files": [
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/UserService/UserHandler.h",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/UserService/CMakeLists.txt",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/UserService/UserService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/UserService::PackageName"
              },
              {
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/MovieReviewService",
                "instanceName": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/MovieReviewService::DirectoryComponent",
                "files": [
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/MovieReviewService/MovieReviewService.cpp",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/MovieReviewService/CMakeLists.txt",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/MovieReviewService/MovieReviewHandler.h"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/MovieReviewService::PackageName"
              },
              {
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/MovieInfoService",
                "instanceName": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/MovieInfoService::DirectoryComponent",
                "files": [
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/MovieInfoService/CMakeLists.txt",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/MovieInfoService/MovieInfoService.cpp",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/MovieInfoService/MovieInfoHandler.h"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/MovieInfoService::PackageName"
              },
              {
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/MovieIdService",
                "instanceName": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/MovieIdService::DirectoryComponent",
                "files": [
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/MovieIdService/CMakeLists.txt",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/MovieIdService/MovieIdHandler.h",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/MovieIdService/MovieIdService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/MovieIdService::PackageName"
              },
              {
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/TextService",
                "instanceName": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/TextService::DirectoryComponent",
                "files": [
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/TextService/TextService.cpp",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/TextService/CMakeLists.txt",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/TextService/TextHandler.h"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/TextService::PackageName"
              },
              {
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/ReviewStorageService",
                "instanceName": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/ReviewStorageService::DirectoryComponent",
                "files": [
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/ReviewStorageService/CMakeLists.txt",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/ReviewStorageService/ReviewStorageHandler.h",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/ReviewStorageService/ReviewStorageService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/ReviewStorageService::PackageName"
              },
              {
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/ComposeReviewService",
                "instanceName": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/ComposeReviewService::DirectoryComponent",
                "files": [
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/ComposeReviewService/CMakeLists.txt",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/ComposeReviewService/ComposeReviewHandler.h",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/ComposeReviewService/ComposeReviewService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/ComposeReviewService::PackageName"
              },
              {
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/CastInfoService",
                "instanceName": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/CastInfoService::DirectoryComponent",
                "files": [
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/CastInfoService/CastInfoService.cpp",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/CastInfoService/CastInfoHandler.h",
                  "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/CastInfoService/CMakeLists.txt"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src/CastInfoService::PackageName"
              }
            ],
            "numFiles": 23,
            "package_name": "/home/jacob/dev/DeathStarBench/mediaMicroservices/src::PackageName"
          }
        ],
        "numFiles": 21,
        "package_name": "/home/jacob/dev/DeathStarBench/mediaMicroservices::PackageName"
      }
    ],
    "numFiles": 0
  }
"#;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

// fn fibonacci(n: u64) -> u64 {
//     match n {
//         0 => 1,
//         1 => 1,
//         n => fibonacci(n - 1) + fibonacci(n - 2),
//     }
// }

fn criterion_benchmark(c: &mut Criterion) {
    let epoch = jemalloc_ctl::epoch::mib().unwrap();
    let allocated = jemalloc_ctl::stats::allocated::mib().unwrap();

    let dir = serde_json::from_str::<Directory>(directory_json).unwrap();
    let mem = vec![];
    c.bench_function("LAAST", move |b| {
        b.iter(|| {
            epoch.advance().unwrap();
            let before = allocated.read().unwrap();
            let _ctx = black_box(parse_project_context(&dir)).unwrap();
            // println!("{}", x);
            // black_box(Box::leak(Box::new(1)));
            epoch.advance().unwrap();
            mem.push(allocated.read().unwrap() - before);
        })
    });
    let len = mem.len();
    println!(
        "{} +/- {} ({}, {})",
        mean(mem, 1, len),
        sd(mem, 1, len),
        min(mem, 1, len),
        max(mem, 1, len)
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
