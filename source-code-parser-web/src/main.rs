use actix_web::{middleware::Logger, web, App, FromRequest, HttpServer};
use source_code_parser::{msd::run_msd_parse, parse_project_context, Directory};
use structopt::StructOpt;

mod routes;
use routes::*;

#[derive(StructOpt)]
struct Opt {
    #[structopt(long, short, default_value = "127.0.0.1")]
    host: String,
    #[structopt(long, short, default_value = "8080")]
    port: i32,
}

const input: &'static str = r###"
{
    "project_dir": {
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
  },
    "patterns": [
  {
    "identifier": "Field",
    "pattern": "#{pool_name}",
    "auxiliary_pattern": "ClientPool<ThriftClient<#{service_name}ServiceClient>>",
    "subpatterns": [],
    "callback": "let service_name = ctx.get_variable(\"service_name\").unwrap();let service_tag = ctx.get_variable(\"pool_name\").unwrap();ctx.make_object(service_name);ctx.make_tag(service_tag, service_name);",
    "essential": true
  },
  {
    "identifier": "DeclStmt",
    "pattern": "",
    "subpatterns": [
      {
        "identifier": "VarDecl",
        "pattern": "#{wrapper_name}(.*_client_wrapper)",
        "subpatterns": [],
        "essential": true
      },
      {
        "identifier": "CallExpr",
        "pattern": "Pop",
        "auxiliary_pattern": "#&{pool_name}(.*_client_pool)",
        "subpatterns": [],
        "essential": false
      }
    ],
    "callback": "let wrapper_name = ctx.get_variable(\"wrapper_name\").unwrap();let pool_name = ctx.get_variable(\"pool_name\").unwrap();ctx.make_tag(wrapper_name, pool_name);",
    "essential": true
  },
  {
    "identifier": "DeclStmt",
    "pattern": "",
    "subpatterns": [
      {
        "identifier": "VarDecl",
        "pattern": "#{client_name}(.*_client)",
        "subpatterns": [],
        "essential": true
      },
      {
        "identifier": "CallExpr",
        "pattern": "GetClient",
        "auxiliary_pattern": "#&{wrapper_name}",
        "subpatterns": [],
        "essential": true
      }
    ],
    "callback": "let client_name = ctx.get_variable(\"client_name\").unwrap();let wrapper_name = ctx.get_variable(\"wrapper_name\").unwrap();ctx.make_tag(client_name, wrapper_name);",
    "essential": true
  },
  {
    "identifier": "CallExpr",
    "pattern": "#{endpoint_name}",
    "auxiliary_pattern": "#&{client_name}(.*_client$)",
    "subpatterns": [],
    "callback": "let client_name = ctx.get_variable(\"client_name\").unwrap();let endpoint = ctx.get_variable(\"endpoint_name\").unwrap();ctx.make_attribute(client_name, endpoint, Some(\"\"));ctx.make_transient(endpoint);",
    "essential": true
  },
  {
    "identifier": "ClassOrInterface",
    "pattern": "#{callee_name}",
    "subpatterns": [
      {
        "identifier": "CallExpr",
        "pattern": "#&{endpoint_name}",
        "auxiliary_pattern": "#&{client_name}(.*_client$)",
        "subpatterns": [],
        "essential": true,
        "callback": "let client_name = ctx.get_variable(\"client_name\").unwrap();let endpoint_name = ctx.get_variable(\"endpoint_name\").unwrap();let callee = ctx.get_variable(\"callee_name\").unwrap();let endpoint = ctx.get_object(client_name).unwrap();let new_list = endpoint.get(endpoint_name).unwrap().unwrap().clone();new_list.push_str(callee);new_list.push_str(\", \");ctx.make_attribute(client_name, endpoint_name, Some(new_list));"
      }
    ],
    "essential": true
  },
  {
    "identifier": "ClassOrInterface",
    "pattern": "#{callee_name}",
    "subpatterns": [
      {
        "identifier": "Field",
        "pattern": "#{entity_attribute}",
        "auxiliary_pattern": "#{attribute_type}",
        "subpatterns": [],
        "essential": true,
        "callback": "let client_name = ctx.get_variable(\"client_name\").unwrap();let endpoint_name = ctx.get_variable(\"endpoint_name\").unwrap();let callee = ctx.get_variable(\"callee_name\").unwrap();let endpoint = ctx.get_object(client_name).unwrap();let new_list = endpoint.get(endpoint_name).unwrap().unwrap().clone();new_list.push_str(callee);new_list.push_str(\", \");ctx.make_attribute(client_name, endpoint_name, Some(new_list));"
      }
    ],
    "essential": true
  }
]

}
"###;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let dir = serde_json::from_str::<MsdInput>(input).unwrap();
    for _ in (0..6).enumerate() {
        match parse_project_context(&dir.project_dir) {
            Ok(mut context) => {
                run_msd_parse(&mut context.modules, dir.patterns.clone());
            }
            Err(err) => {}
        }
    }
    Ok(())
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // let opt = Opt::from_args();
    // let addr = format!("{}:{}", opt.host, opt.port);
    // HttpServer::new(|| {
    //     App::new()
    //         .service(ast)
    //         .service(ctx)
    //         .service(msd)
    //         .wrap(Logger::default())
    //         .app_data(web::Json::<Directory>::configure(|cfg| {
    //             cfg.limit(1024 * 1024 * 4)
    //         }))
    // })
    // .bind(addr)?
    // .run()
    // .await
}
