use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate source_code_parser;
use source_code_parser::*;

const directory_json: &'static str = r#"
{
    "path": "",
    "files": [],
    "subDirectories": [
      {
        "path": "C:/dev/DeathStarBench/socialNetwork",
        "instanceName": "C:/dev/DeathStarBench/socialNetwork::DirectoryComponent",
        "instanceType": "DIRECTORYCOMPONENT",
        "files": [
          "C:/dev/DeathStarBench/socialNetwork/docker",
          "C:/dev/DeathStarBench/socialNetwork/third_party",
          "C:/dev/DeathStarBench/socialNetwork/src",
          "C:/dev/DeathStarBench/socialNetwork/datasets",
          "C:/dev/DeathStarBench/socialNetwork/socialNet_jaeger.png",
          "C:/dev/DeathStarBench/socialNetwork/wrk2",
          "C:/dev/DeathStarBench/socialNetwork/docker-compose.yml",
          "C:/dev/DeathStarBench/socialNetwork/README.md",
          "C:/dev/DeathStarBench/socialNetwork/figures",
          "C:/dev/DeathStarBench/socialNetwork/CMakeLists.txt",
          "C:/dev/DeathStarBench/socialNetwork/gen-py",
          "C:/dev/DeathStarBench/socialNetwork/scripts",
          "C:/dev/DeathStarBench/socialNetwork/cmake",
          "C:/dev/DeathStarBench/socialNetwork/media-frontend",
          "C:/dev/DeathStarBench/socialNetwork/nginx-web-server",
          "C:/dev/DeathStarBench/socialNetwork/config",
          "C:/dev/DeathStarBench/socialNetwork/gen-lua",
          "C:/dev/DeathStarBench/socialNetwork/test",
          "C:/dev/DeathStarBench/socialNetwork/social_network.thrift",
          "C:/dev/DeathStarBench/socialNetwork/Dockerfile",
          "C:/dev/DeathStarBench/socialNetwork/openshift",
          "C:/dev/DeathStarBench/socialNetwork/gen-cpp",
          "C:/dev/DeathStarBench/socialNetwork/socialNet_arch.png"
        ],
        "subDirectories": [
          {
            "path": "C:/dev/DeathStarBench/socialNetwork/docker",
            "instanceName": "C:/dev/DeathStarBench/socialNetwork/docker::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/socialNetwork/docker/media-frontend",
              "C:/dev/DeathStarBench/socialNetwork/docker/mcrouter",
              "C:/dev/DeathStarBench/socialNetwork/docker/thrift-microservice-deps",
              "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift"
            ],
            "subDirectories": [
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/docker/media-frontend",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/docker/media-frontend::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/docker/media-frontend/travis.yml.etlua",
                  "C:/dev/DeathStarBench/socialNetwork/docker/media-frontend/README.md",
                  "C:/dev/DeathStarBench/socialNetwork/docker/media-frontend/COPYRIGHT",
                  "C:/dev/DeathStarBench/socialNetwork/docker/media-frontend/nginx.conf",
                  "C:/dev/DeathStarBench/socialNetwork/docker/media-frontend/gen_travis.lua",
                  "C:/dev/DeathStarBench/socialNetwork/docker/media-frontend/AUTHORS.md",
                  "C:/dev/DeathStarBench/socialNetwork/docker/media-frontend/nginx.vh.default.conf",
                  "C:/dev/DeathStarBench/socialNetwork/docker/media-frontend/lualongnumber",
                  "C:/dev/DeathStarBench/socialNetwork/docker/media-frontend/xenial",
                  "C:/dev/DeathStarBench/socialNetwork/docker/media-frontend/CHANGELOG.md"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/socialNetwork/docker/media-frontend/lualongnumber",
                    "instanceName": "C:/dev/DeathStarBench/socialNetwork/docker/media-frontend/lualongnumber::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/socialNetwork/docker/media-frontend/lualongnumber/longnumberutils.c",
                      "C:/dev/DeathStarBench/socialNetwork/docker/media-frontend/lualongnumber/lualongnumber.c",
                      "C:/dev/DeathStarBench/socialNetwork/docker/media-frontend/lualongnumber/Makefile"
                    ],
                    "subDirectories": [],
                    "numFiles": 3,
                    "package_name": "C:/dev/DeathStarBench/socialNetwork/docker/media-frontend/lualongnumber::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/socialNetwork/docker/media-frontend/xenial",
                    "instanceName": "C:/dev/DeathStarBench/socialNetwork/docker/media-frontend/xenial::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/socialNetwork/docker/media-frontend/xenial/Dockerfile"
                    ],
                    "subDirectories": [],
                    "numFiles": 1,
                    "package_name": "C:/dev/DeathStarBench/socialNetwork/docker/media-frontend/xenial::PackageName"
                  }
                ],
                "numFiles": 10,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/docker/media-frontend::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/docker/mcrouter",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/docker/mcrouter::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/docker/mcrouter/Dockerfile"
                ],
                "subDirectories": [],
                "numFiles": 1,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/docker/mcrouter::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/docker/thrift-microservice-deps",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/docker/thrift-microservice-deps::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/docker/thrift-microservice-deps/cpp",
                  "C:/dev/DeathStarBench/socialNetwork/docker/thrift-microservice-deps/python"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/socialNetwork/docker/thrift-microservice-deps/cpp",
                    "instanceName": "C:/dev/DeathStarBench/socialNetwork/docker/thrift-microservice-deps/cpp::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/socialNetwork/docker/thrift-microservice-deps/cpp/README.md",
                      "C:/dev/DeathStarBench/socialNetwork/docker/thrift-microservice-deps/cpp/Dockerfile"
                    ],
                    "subDirectories": [],
                    "numFiles": 2,
                    "package_name": "C:/dev/DeathStarBench/socialNetwork/docker/thrift-microservice-deps/cpp::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/socialNetwork/docker/thrift-microservice-deps/python",
                    "instanceName": "C:/dev/DeathStarBench/socialNetwork/docker/thrift-microservice-deps/python::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/socialNetwork/docker/thrift-microservice-deps/python/Dockerfile-py"
                    ],
                    "subDirectories": [],
                    "numFiles": 1,
                    "package_name": "C:/dev/DeathStarBench/socialNetwork/docker/thrift-microservice-deps/python::PackageName"
                  }
                ],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/docker/thrift-microservice-deps::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer",
                  "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/travis.yml.etlua",
                  "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/README.md",
                  "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/appveyor.yml",
                  "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/.dockerignore",
                  "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/COPYRIGHT",
                  "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-json",
                  "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/nginx.conf",
                  "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/gen_travis.lua",
                  "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/AUTHORS.md",
                  "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/nginx.vh.default.conf",
                  "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/.travis.yml",
                  "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/xenial",
                  "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/CHANGELOG.md",
                  "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer",
                    "instanceName": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/example",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/src",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/.circleci",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/LICENSE",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/README.md",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/CMakeLists.txt",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/scripts",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/.clang-format",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/test",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/ci"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/example",
                        "instanceName": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/example::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/example/hello_server",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/example/tutorial"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/example/hello_server",
                            "instanceName": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/example/hello_server::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/example/hello_server/jaeger",
                              "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/example/hello_server/README.md"
                            ],
                            "subDirectories": [
                              {
                                "path": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/example/hello_server/jaeger",
                                "instanceName": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/example/hello_server/jaeger::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/example/hello_server/jaeger/server_hello.lua",
                                  "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/example/hello_server/jaeger/docker-compose.yaml",
                                  "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/example/hello_server/jaeger/jaeger-config.json",
                                  "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/example/hello_server/jaeger/server.lua",
                                  "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/example/hello_server/jaeger/Dockerfile"
                                ],
                                "subDirectories": [],
                                "numFiles": 5,
                                "package_name": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/example/hello_server/jaeger::PackageName"
                              }
                            ],
                            "numFiles": 2,
                            "package_name": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/example/hello_server::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/example/tutorial",
                            "instanceName": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/example/tutorial::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/example/tutorial/README.md",
                              "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/example/tutorial/tutorial.lua"
                            ],
                            "subDirectories": [],
                            "numFiles": 2,
                            "package_name": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/example/tutorial::PackageName"
                          }
                        ],
                        "numFiles": 2,
                        "package_name": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/example::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/src",
                        "instanceName": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/src::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/src/lua_tracer.cpp",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/src/lua_tracer.h",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/src/carrier.cpp",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/src/lua_span.h",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/src/dynamic_tracer.cpp",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/src/lua_span_context.h",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/src/lua_span.cpp",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/src/dynamic_tracer.h",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/src/lua_class_description.h",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/src/lua_span_context.cpp",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/src/module.cpp",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/src/carrier.h",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/src/utility.h",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/src/utility.cpp"
                        ],
                        "subDirectories": [],
                        "numFiles": 14,
                        "package_name": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/src::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/.circleci",
                        "instanceName": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/.circleci::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/.circleci/config.yml"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/.circleci::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/scripts",
                        "instanceName": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/scripts::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/scripts/run_clang_format.sh"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/scripts::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/test",
                        "instanceName": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/test::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/test/tracer.lua"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/test::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/ci",
                        "instanceName": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/ci::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/ci/setup_build_environment.sh",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/ci/install_lua.sh",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/ci/install_opentracing.sh",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/ci/install_rocks.sh",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/ci/Dockerfile",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/ci/do_ci.sh",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/ci/run_docker.sh"
                        ],
                        "subDirectories": [],
                        "numFiles": 7,
                        "package_name": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer/ci::PackageName"
                      }
                    ],
                    "numFiles": 10,
                    "package_name": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-bridge-tracer::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-json",
                    "instanceName": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-json::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-json/json.lua"
                    ],
                    "subDirectories": [],
                    "numFiles": 1,
                    "package_name": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-json::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/xenial",
                    "instanceName": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/xenial::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/xenial/Dockerfile"
                    ],
                    "subDirectories": [],
                    "numFiles": 1,
                    "package_name": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/xenial::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift",
                    "instanceName": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/src",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/TBufferedTransport.lua",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/RpcClientFactory.lua",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/TFramedTransport.lua",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/TJsonProtocol.lua",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/TCompactProtocol.lua",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/GenericObjectPool.lua",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/Thrift.lua",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/TSocket.lua",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/RpcClient.lua",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/Object.lua",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/THttpTransport.lua",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/TTransport.lua",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/Makefile.am",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/TMemoryBuffer.lua",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/TProtocol.lua",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/coding_standards.md",
                      "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/TBinaryProtocol.lua"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/src",
                        "instanceName": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/src::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/src/longnumberutils.c",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/src/usocket.c",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/src/luasocket.c",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/src/lualongnumber.c",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/src/luabitwise.c",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/src/Makefile",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/src/socket.h",
                          "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/src/luabpack.c"
                        ],
                        "subDirectories": [],
                        "numFiles": 8,
                        "package_name": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift/src::PackageName"
                      }
                    ],
                    "numFiles": 18,
                    "package_name": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift/lua-thrift::PackageName"
                  }
                ],
                "numFiles": 15,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/docker/openresty-thrift::PackageName"
              }
            ],
            "numFiles": 4,
            "package_name": "C:/dev/DeathStarBench/socialNetwork/docker::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/socialNetwork/third_party",
            "instanceName": "C:/dev/DeathStarBench/socialNetwork/third_party::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/socialNetwork/third_party/PicoSHA2"
            ],
            "subDirectories": [
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/third_party/PicoSHA2",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/third_party/PicoSHA2::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/third_party/PicoSHA2/picosha2.h",
                  "C:/dev/DeathStarBench/socialNetwork/third_party/PicoSHA2/LICENSE",
                  "C:/dev/DeathStarBench/socialNetwork/third_party/PicoSHA2/README.md"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/third_party/PicoSHA2::PackageName"
              }
            ],
            "numFiles": 1,
            "package_name": "C:/dev/DeathStarBench/socialNetwork/third_party::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/socialNetwork/src",
            "instanceName": "C:/dev/DeathStarBench/socialNetwork/src::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/socialNetwork/src/tracing.h",
              "C:/dev/DeathStarBench/socialNetwork/src/UserMentionService",
              "C:/dev/DeathStarBench/socialNetwork/src/utils_mongodb.h",
              "C:/dev/DeathStarBench/socialNetwork/src/AmqpLibeventHandler.h",
              "C:/dev/DeathStarBench/socialNetwork/src/utils.h",
              "C:/dev/DeathStarBench/socialNetwork/src/UrlShortenService",
              "C:/dev/DeathStarBench/socialNetwork/src/UniqueIdService",
              "C:/dev/DeathStarBench/socialNetwork/src/logger.h",
              "C:/dev/DeathStarBench/socialNetwork/src/CMakeLists.txt",
              "C:/dev/DeathStarBench/socialNetwork/src/utils_memcached.h",
              "C:/dev/DeathStarBench/socialNetwork/src/UserService",
              "C:/dev/DeathStarBench/socialNetwork/src/GenericClient.h",
              "C:/dev/DeathStarBench/socialNetwork/src/UserTimelineService",
              "C:/dev/DeathStarBench/socialNetwork/src/ComposePostService",
              "C:/dev/DeathStarBench/socialNetwork/src/MediaService",
              "C:/dev/DeathStarBench/socialNetwork/src/ThriftClient.h",
              "C:/dev/DeathStarBench/socialNetwork/src/SocialGraphService",
              "C:/dev/DeathStarBench/socialNetwork/src/WriteHomeTimelineService",
              "C:/dev/DeathStarBench/socialNetwork/src/PostStorageSerivce",
              "C:/dev/DeathStarBench/socialNetwork/src/TextService",
              "C:/dev/DeathStarBench/socialNetwork/src/HomeTimelineService",
              "C:/dev/DeathStarBench/socialNetwork/src/RedisClient.h",
              "C:/dev/DeathStarBench/socialNetwork/src/ClientPool.h"
            ],
            "subDirectories": [
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/src/UserMentionService",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/src/UserMentionService::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/src/UserMentionService/CMakeLists.txt",
                  "C:/dev/DeathStarBench/socialNetwork/src/UserMentionService/UserMentionService.cpp",
                  "C:/dev/DeathStarBench/socialNetwork/src/UserMentionService/UserMentionHandler.h"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/src/UserMentionService::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/src/UrlShortenService",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/src/UrlShortenService::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/src/UrlShortenService/CMakeLists.txt",
                  "C:/dev/DeathStarBench/socialNetwork/src/UrlShortenService/UrlShortenService.cpp",
                  "C:/dev/DeathStarBench/socialNetwork/src/UrlShortenService/UrlShortenHandler.h"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/src/UrlShortenService::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/src/UniqueIdService",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/src/UniqueIdService::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/src/UniqueIdService/CMakeLists.txt",
                  "C:/dev/DeathStarBench/socialNetwork/src/UniqueIdService/UniqueIdHandler.h",
                  "C:/dev/DeathStarBench/socialNetwork/src/UniqueIdService/UniqueIdService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/src/UniqueIdService::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/src/UserService",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/src/UserService::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/src/UserService/UserHandler.h",
                  "C:/dev/DeathStarBench/socialNetwork/src/UserService/CMakeLists.txt",
                  "C:/dev/DeathStarBench/socialNetwork/src/UserService/UserService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/src/UserService::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/src/UserTimelineService",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/src/UserTimelineService::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/src/UserTimelineService/CMakeLists.txt",
                  "C:/dev/DeathStarBench/socialNetwork/src/UserTimelineService/UserTimelineHandler.h",
                  "C:/dev/DeathStarBench/socialNetwork/src/UserTimelineService/UserTimelineService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/src/UserTimelineService::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/src/ComposePostService",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/src/ComposePostService::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/src/ComposePostService/CMakeLists.txt",
                  "C:/dev/DeathStarBench/socialNetwork/src/ComposePostService/RabbitmqClient.h",
                  "C:/dev/DeathStarBench/socialNetwork/src/ComposePostService/ComposePostService.cpp",
                  "C:/dev/DeathStarBench/socialNetwork/src/ComposePostService/ComposePostHandler.h"
                ],
                "subDirectories": [],
                "numFiles": 4,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/src/ComposePostService::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/src/MediaService",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/src/MediaService::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/src/MediaService/CMakeLists.txt",
                  "C:/dev/DeathStarBench/socialNetwork/src/MediaService/MediaHandler.h",
                  "C:/dev/DeathStarBench/socialNetwork/src/MediaService/MediaService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/src/MediaService::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/src/SocialGraphService",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/src/SocialGraphService::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/src/SocialGraphService/SocialGraphService.cpp",
                  "C:/dev/DeathStarBench/socialNetwork/src/SocialGraphService/CMakeLists.txt",
                  "C:/dev/DeathStarBench/socialNetwork/src/SocialGraphService/SocialGraphHandler.h"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/src/SocialGraphService::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/src/WriteHomeTimelineService",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/src/WriteHomeTimelineService::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/src/WriteHomeTimelineService/WriteHomeTimelineService.cpp",
                  "C:/dev/DeathStarBench/socialNetwork/src/WriteHomeTimelineService/CMakeLists.txt"
                ],
                "subDirectories": [],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/src/WriteHomeTimelineService::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/src/PostStorageSerivce",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/src/PostStorageSerivce::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/src/PostStorageSerivce/PostStorageService.cpp",
                  "C:/dev/DeathStarBench/socialNetwork/src/PostStorageSerivce/PostStorageHandler.h",
                  "C:/dev/DeathStarBench/socialNetwork/src/PostStorageSerivce/CMakeLists.txt"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/src/PostStorageSerivce::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/src/TextService",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/src/TextService::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/src/TextService/TextService.cpp",
                  "C:/dev/DeathStarBench/socialNetwork/src/TextService/CMakeLists.txt",
                  "C:/dev/DeathStarBench/socialNetwork/src/TextService/TextHandler.h"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/src/TextService::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/src/HomeTimelineService",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/src/HomeTimelineService::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/src/HomeTimelineService/HomeTimelineHandler.h",
                  "C:/dev/DeathStarBench/socialNetwork/src/HomeTimelineService/CMakeLists.txt",
                  "C:/dev/DeathStarBench/socialNetwork/src/HomeTimelineService/HomeTimelineService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/src/HomeTimelineService::PackageName"
              }
            ],
            "numFiles": 23,
            "package_name": "C:/dev/DeathStarBench/socialNetwork/src::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/socialNetwork/datasets",
            "instanceName": "C:/dev/DeathStarBench/socialNetwork/datasets::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/socialNetwork/datasets/social-graph"
            ],
            "subDirectories": [
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/datasets/social-graph",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/datasets/social-graph::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/datasets/social-graph/socfb-Reed98"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/socialNetwork/datasets/social-graph/socfb-Reed98",
                    "instanceName": "C:/dev/DeathStarBench/socialNetwork/datasets/social-graph/socfb-Reed98::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/socialNetwork/datasets/social-graph/socfb-Reed98/readme.html",
                      "C:/dev/DeathStarBench/socialNetwork/datasets/social-graph/socfb-Reed98/socfb-Reed98.mtx"
                    ],
                    "subDirectories": [],
                    "numFiles": 2,
                    "package_name": "C:/dev/DeathStarBench/socialNetwork/datasets/social-graph/socfb-Reed98::PackageName"
                  }
                ],
                "numFiles": 1,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/datasets/social-graph::PackageName"
              }
            ],
            "numFiles": 1,
            "package_name": "C:/dev/DeathStarBench/socialNetwork/datasets::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/socialNetwork/wrk2",
            "instanceName": "C:/dev/DeathStarBench/socialNetwork/wrk2::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/socialNetwork/wrk2/SCRIPTING",
              "C:/dev/DeathStarBench/socialNetwork/wrk2/src",
              "C:/dev/DeathStarBench/socialNetwork/wrk2/LICENSE",
              "C:/dev/DeathStarBench/socialNetwork/wrk2/README.md",
              "C:/dev/DeathStarBench/socialNetwork/wrk2/CoordinatedOmission",
              "C:/dev/DeathStarBench/socialNetwork/wrk2/gen_path.py",
              "C:/dev/DeathStarBench/socialNetwork/wrk2/scripts",
              "C:/dev/DeathStarBench/socialNetwork/wrk2/deps",
              "C:/dev/DeathStarBench/socialNetwork/wrk2/Makefile",
              "C:/dev/DeathStarBench/socialNetwork/wrk2/NOTICE"
            ],
            "subDirectories": [
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/wrk2/src",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/wrk2/src::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/wrk.c",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/zmalloc.h",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/aprintf.c",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/tinymt64.c",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/main.h",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/config.h",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/hdr_histogram.h",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/ae_kqueue.c",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/ssl.c",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/zmalloc.c",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/script.h",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/wrk.h",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/stats.c",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/units.c",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/ae_evport.c",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/http_parser.c",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/hdr_histogram.c",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/ae.h",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/http_parser.h",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/units.h",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/wrk.lua",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/tinymt64.h",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/ae_epoll.c",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/script.c",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/aprintf.h",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/ssl.h",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/ae.c",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/net.c",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/stats.h",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/ae_select.c",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/src/net.h"
                ],
                "subDirectories": [],
                "numFiles": 31,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/wrk2/src::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/wrk2/CoordinatedOmission",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/wrk2/CoordinatedOmission::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/CoordinatedOmission/wrk2_CleanVsCO.png"
                ],
                "subDirectories": [],
                "numFiles": 1,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/wrk2/CoordinatedOmission::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/wrk2/scripts",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/wrk2/scripts::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/scripts/social-network",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/scripts/pipeline.lua",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/scripts/setup.lua",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/scripts/addr.lua",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/scripts/post.lua",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/scripts/stop.lua",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/scripts/report.lua",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/scripts/auth.lua",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/scripts/multiplepaths.lua",
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/scripts/counter.lua"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/socialNetwork/wrk2/scripts/social-network",
                    "instanceName": "C:/dev/DeathStarBench/socialNetwork/wrk2/scripts/social-network::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/socialNetwork/wrk2/scripts/social-network/compose-post.lua",
                      "C:/dev/DeathStarBench/socialNetwork/wrk2/scripts/social-network/read-home-timeline.lua",
                      "C:/dev/DeathStarBench/socialNetwork/wrk2/scripts/social-network/mixed-workload.lua",
                      "C:/dev/DeathStarBench/socialNetwork/wrk2/scripts/social-network/read-user-timeline.lua"
                    ],
                    "subDirectories": [],
                    "numFiles": 4,
                    "package_name": "C:/dev/DeathStarBench/socialNetwork/wrk2/scripts/social-network::PackageName"
                  }
                ],
                "numFiles": 10,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/wrk2/scripts::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit",
                    "instanceName": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src",
                      "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc",
                      "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/dynasm",
                      "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/etc",
                      "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/COPYRIGHT",
                      "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/Makefile",
                      "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/README"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src",
                        "instanceName": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj.supp",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_jit.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_trace.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_ccallback.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_opt_mem.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/luaconf.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_asm_x86.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_traceerr.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_target_mips.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_ir.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_asm.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_lex.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_udata.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_obj.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_def.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_alloc.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_opt_fold.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_errmsg.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lib_ffi.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_clib.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_state.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lib_table.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_record.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_err.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lib_string.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/vm_ppc.dasc",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/host",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/msvcbuild.bat",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_err.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lib_base.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_dispatch.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_ff.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_vmevent.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lib_debug.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_emit_x86.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_emit_arm.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_cparse.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_strscan.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_trace.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/ljamalg.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_char.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/xedkbuild.bat",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_ircall.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_obj.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_parse.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_target.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_ffrecord.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_meta.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_char.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_gdbjit.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_clib.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_ffrecord.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_load.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_bcread.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_lex.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lib_io.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_record.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_emit_mips.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_alloc.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_iropt.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_vmmath.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_func.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_gdbjit.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_api.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_parse.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lib_os.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_debug.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_lib.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_state.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/Makefile.dep",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_meta.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/vm_mips.dasc",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_target_ppc.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_crecord.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_lib.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_target_x86.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_ccall.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_opt_split.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_cconv.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_vm.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_bc.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_bc.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_cdata.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lib_jit.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lua.hpp",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/vm_ppcspe.dasc",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_cparse.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_bcdump.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_asm.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_gc.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/luajit.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_vmevent.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_func.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lib_bit.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_target_arm.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lauxlib.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_frame.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_cconv.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_gc.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_bcwrite.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_asm_mips.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_mcode.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lib_package.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_asm_ppc.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_opt_dce.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_str.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_str.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_ctype.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_snap.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_ccallback.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lualib.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_asm_arm.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_ir.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_cdata.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_debug.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_udata.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/Makefile",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_tab.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/luajit.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_tab.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_snap.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_opt_loop.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_arch.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_opt_sink.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_strscan.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lib_aux.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_dispatch.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_ctype.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lua.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/vm_arm.dasc",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_emit_ppc.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_carith.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_ccall.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/vm_x86.dasc",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/ps4build.bat",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/jit",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_carith.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_mcode.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lib_init.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lib_math.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_opt_narrow.c",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/lj_crecord.h"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/host",
                            "instanceName": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/host::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/host/buildvm.c",
                              "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/host/minilua.c",
                              "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/host/buildvm_fold.c",
                              "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/host/buildvm_asm.c",
                              "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/host/buildvm_lib.c",
                              "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/host/genminilua.lua",
                              "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/host/buildvm.h",
                              "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/host/buildvm_peobj.c",
                              "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/host/README"
                            ],
                            "subDirectories": [],
                            "numFiles": 9,
                            "package_name": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/host::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/jit",
                            "instanceName": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/jit::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/jit/dis_ppc.lua",
                              "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/jit/v.lua",
                              "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/jit/dis_mips.lua",
                              "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/jit/bc.lua",
                              "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/jit/dis_arm.lua",
                              "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/jit/bcsave.lua",
                              "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/jit/dis_x86.lua",
                              "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/jit/dis_mipsel.lua",
                              "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/jit/dump.lua",
                              "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/jit/dis_x64.lua"
                            ],
                            "subDirectories": [],
                            "numFiles": 10,
                            "package_name": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src/jit::PackageName"
                          }
                        ],
                        "numFiles": 142,
                        "package_name": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/src::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc",
                        "instanceName": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc/bluequad-print.css",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc/extensions.html",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc/ext_ffi.html",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc/changes.html",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc/ext_c_api.html",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc/install.html",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc/ext_jit.html",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc/running.html",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc/status.html",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc/faq.html",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc/contact.html",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc/ext_ffi_tutorial.html",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc/luajit.html",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc/ext_ffi_api.html",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc/ext_ffi_semantics.html",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc/img",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc/bluequad.css"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc/img",
                            "instanceName": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc/img::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc/img/contact.png"
                            ],
                            "subDirectories": [],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc/img::PackageName"
                          }
                        ],
                        "numFiles": 17,
                        "package_name": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/doc::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/dynasm",
                        "instanceName": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/dynasm::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/dynasm/dasm_mips.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/dynasm/dasm_x86.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/dynasm/dasm_x86.lua",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/dynasm/dynasm.lua",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/dynasm/dasm_x64.lua",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/dynasm/dasm_ppc.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/dynasm/dasm_ppc.lua",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/dynasm/dasm_proto.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/dynasm/dasm_arm.h",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/dynasm/dasm_mips.lua",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/dynasm/dasm_arm.lua"
                        ],
                        "subDirectories": [],
                        "numFiles": 11,
                        "package_name": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/dynasm::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/etc",
                        "instanceName": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/etc::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/etc/luajit.1",
                          "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/etc/luajit.pc"
                        ],
                        "subDirectories": [],
                        "numFiles": 2,
                        "package_name": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit/etc::PackageName"
                      }
                    ],
                    "numFiles": 7,
                    "package_name": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps/luajit::PackageName"
                  }
                ],
                "numFiles": 1,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/wrk2/deps::PackageName"
              }
            ],
            "numFiles": 10,
            "package_name": "C:/dev/DeathStarBench/socialNetwork/wrk2::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/socialNetwork/figures",
            "instanceName": "C:/dev/DeathStarBench/socialNetwork/figures::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/socialNetwork/figures/socialNet_jaeger.png",
              "C:/dev/DeathStarBench/socialNetwork/figures/follow.png",
              "C:/dev/DeathStarBench/socialNetwork/figures/user_timeline.png",
              "C:/dev/DeathStarBench/socialNetwork/figures/home_timeline.png",
              "C:/dev/DeathStarBench/socialNetwork/figures/signup.png",
              "C:/dev/DeathStarBench/socialNetwork/figures/login.png",
              "C:/dev/DeathStarBench/socialNetwork/figures/socialNet_arch.png"
            ],
            "subDirectories": [],
            "numFiles": 7,
            "package_name": "C:/dev/DeathStarBench/socialNetwork/figures::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/socialNetwork/gen-py",
            "instanceName": "C:/dev/DeathStarBench/socialNetwork/gen-py::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network",
              "C:/dev/DeathStarBench/socialNetwork/gen-py/__init__.py"
            ],
            "subDirectories": [
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/TextService-remote",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/MediaService-remote",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/HomeTimelineService-remote",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/MediaService.py",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/ComposePostService.py",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/UserService-remote",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/UniqueIdService.py",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/WriteHomeTimelineService.py",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/TextService.py",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/UserService.py",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/HomeTimelineService.py",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/SocialGraphService.py",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/UrlShortenService-remote",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/PostStorageService.py",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/constants.py",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/WriteHomeTimelineService-remote",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/UniqueIdService-remote",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/UserMentionService.py",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/ttypes.py",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/SocialGraphService-remote",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/UserTimelineService-remote",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/PostStorageService-remote",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/UserTimelineService.py",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/__init__.py",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/UrlShortenService.py",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/UserMentionService-remote",
                  "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network/ComposePostService-remote"
                ],
                "subDirectories": [],
                "numFiles": 27,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/gen-py/social_network::PackageName"
              }
            ],
            "numFiles": 2,
            "package_name": "C:/dev/DeathStarBench/socialNetwork/gen-py::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/socialNetwork/scripts",
            "instanceName": "C:/dev/DeathStarBench/socialNetwork/scripts::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/socialNetwork/scripts/init_social_graph.py"
            ],
            "subDirectories": [],
            "numFiles": 1,
            "package_name": "C:/dev/DeathStarBench/socialNetwork/scripts::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/socialNetwork/cmake",
            "instanceName": "C:/dev/DeathStarBench/socialNetwork/cmake::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/socialNetwork/cmake/Findlibmemcached.cmake",
              "C:/dev/DeathStarBench/socialNetwork/cmake/Findthrift.cmake",
              "C:/dev/DeathStarBench/socialNetwork/cmake/FindLibevent.cmake"
            ],
            "subDirectories": [],
            "numFiles": 3,
            "package_name": "C:/dev/DeathStarBench/socialNetwork/cmake::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/socialNetwork/media-frontend",
            "instanceName": "C:/dev/DeathStarBench/socialNetwork/media-frontend::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/socialNetwork/media-frontend/conf",
              "C:/dev/DeathStarBench/socialNetwork/media-frontend/lua-scripts"
            ],
            "subDirectories": [
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/media-frontend/conf",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/media-frontend/conf::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/media-frontend/conf/nginx.conf"
                ],
                "subDirectories": [],
                "numFiles": 1,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/media-frontend/conf::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/media-frontend/lua-scripts",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/media-frontend/lua-scripts::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/media-frontend/lua-scripts/upload-media.lua",
                  "C:/dev/DeathStarBench/socialNetwork/media-frontend/lua-scripts/get-media.lua"
                ],
                "subDirectories": [],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/media-frontend/lua-scripts::PackageName"
              }
            ],
            "numFiles": 2,
            "package_name": "C:/dev/DeathStarBench/socialNetwork/media-frontend::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server",
            "instanceName": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/jaeger-config.json",
              "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/conf",
              "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts",
              "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages"
            ],
            "subDirectories": [
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/conf",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/conf::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/conf/nginx.conf"
                ],
                "subDirectories": [],
                "numFiles": 1,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/conf::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api",
                  "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api",
                    "instanceName": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api/home-timeline",
                      "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api/user-timeline",
                      "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api/post",
                      "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api/user"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api/home-timeline",
                        "instanceName": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api/home-timeline::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api/home-timeline/read.lua"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api/home-timeline::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api/user-timeline",
                        "instanceName": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api/user-timeline::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api/user-timeline/read.lua"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api/user-timeline::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api/post",
                        "instanceName": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api/post::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api/post/compose.lua"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api/post::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api/user",
                        "instanceName": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api/user::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api/user/register.lua",
                          "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api/user/unfollow.lua",
                          "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api/user/follow.lua"
                        ],
                        "subDirectories": [],
                        "numFiles": 3,
                        "package_name": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api/user::PackageName"
                      }
                    ],
                    "numFiles": 4,
                    "package_name": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/wrk2-api::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api",
                    "instanceName": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/home-timeline",
                      "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/user-timeline",
                      "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/post",
                      "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/user"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/home-timeline",
                        "instanceName": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/home-timeline::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/home-timeline/read.lua"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/home-timeline::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/user-timeline",
                        "instanceName": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/user-timeline::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/user-timeline/read.lua"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/user-timeline::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/post",
                        "instanceName": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/post::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/post/compose.lua"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/post::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/user",
                        "instanceName": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/user::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/user/get_followee.lua",
                          "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/user/login.lua",
                          "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/user/register.lua",
                          "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/user/get_follower.lua",
                          "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/user/unfollow.lua",
                          "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/user/follow.lua"
                        ],
                        "subDirectories": [],
                        "numFiles": 6,
                        "package_name": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api/user::PackageName"
                      }
                    ],
                    "numFiles": 4,
                    "package_name": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts/api::PackageName"
                  }
                ],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/lua-scripts::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/style",
                  "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/compose.html",
                  "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/jaeger-config.json",
                  "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/signup.html",
                  "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/contact.html",
                  "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/main.html",
                  "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/javascript",
                  "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/index.html",
                  "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/profile.html"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/style",
                    "instanceName": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/style::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/style/main.css",
                      "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/style/login.css",
                      "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/style/signup.css"
                    ],
                    "subDirectories": [],
                    "numFiles": 3,
                    "package_name": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/style::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/javascript",
                    "instanceName": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/javascript::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/javascript/get-follower.js",
                      "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/javascript/get-followee.js",
                      "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/javascript/button-click.js",
                      "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/javascript/create-post.js",
                      "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/javascript/timeline.js",
                      "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/javascript/contact.js",
                      "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/javascript/default_user.js"
                    ],
                    "subDirectories": [],
                    "numFiles": 7,
                    "package_name": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages/javascript::PackageName"
                  }
                ],
                "numFiles": 9,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server/pages::PackageName"
              }
            ],
            "numFiles": 4,
            "package_name": "C:/dev/DeathStarBench/socialNetwork/nginx-web-server::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/socialNetwork/config",
            "instanceName": "C:/dev/DeathStarBench/socialNetwork/config::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/socialNetwork/config/jaeger-config.yml",
              "C:/dev/DeathStarBench/socialNetwork/config/service-config.json"
            ],
            "subDirectories": [],
            "numFiles": 2,
            "package_name": "C:/dev/DeathStarBench/socialNetwork/config::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/socialNetwork/gen-lua",
            "instanceName": "C:/dev/DeathStarBench/socialNetwork/gen-lua::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/socialNetwork/gen-lua/social_network_UserMentionService.lua",
              "C:/dev/DeathStarBench/socialNetwork/gen-lua/social_network_UserService.lua",
              "C:/dev/DeathStarBench/socialNetwork/gen-lua/social_network_SocialGraphService.lua",
              "C:/dev/DeathStarBench/socialNetwork/gen-lua/social_network_ComposePostService.lua",
              "C:/dev/DeathStarBench/socialNetwork/gen-lua/social_network_TextService.lua",
              "C:/dev/DeathStarBench/socialNetwork/gen-lua/social_network_UserTimelineService.lua",
              "C:/dev/DeathStarBench/socialNetwork/gen-lua/social_network_UrlShortenService.lua",
              "C:/dev/DeathStarBench/socialNetwork/gen-lua/social_network_UniqueIdService.lua",
              "C:/dev/DeathStarBench/socialNetwork/gen-lua/social_network_HomeTimelineService.lua",
              "C:/dev/DeathStarBench/socialNetwork/gen-lua/social_network_constants.lua",
              "C:/dev/DeathStarBench/socialNetwork/gen-lua/social_network_ttypes.lua",
              "C:/dev/DeathStarBench/socialNetwork/gen-lua/social_network_MediaService.lua",
              "C:/dev/DeathStarBench/socialNetwork/gen-lua/social_network_PostStorageService.lua"
            ],
            "subDirectories": [],
            "numFiles": 13,
            "package_name": "C:/dev/DeathStarBench/socialNetwork/gen-lua::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/socialNetwork/test",
            "instanceName": "C:/dev/DeathStarBench/socialNetwork/test::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/socialNetwork/test/TestComposePostE2E.py",
              "C:/dev/DeathStarBench/socialNetwork/test/TestTextService.py",
              "C:/dev/DeathStarBench/socialNetwork/test/TestUniqueIdService.py",
              "C:/dev/DeathStarBench/socialNetwork/test/TestReadHomeTimelineService.py",
              "C:/dev/DeathStarBench/socialNetwork/test/TestUserService.py",
              "C:/dev/DeathStarBench/socialNetwork/test/TestWriteHomeTimelineService.py",
              "C:/dev/DeathStarBench/socialNetwork/test/TestPostStorage.py",
              "C:/dev/DeathStarBench/socialNetwork/test/TestUserMentionService.py",
              "C:/dev/DeathStarBench/socialNetwork/test/TestUserTimelineService.py",
              "C:/dev/DeathStarBench/socialNetwork/test/TestSocialGraph.py",
              "C:/dev/DeathStarBench/socialNetwork/test/testComposePost.py",
              "C:/dev/DeathStarBench/socialNetwork/test/TestUrlShortenService.py"
            ],
            "subDirectories": [],
            "numFiles": 12,
            "package_name": "C:/dev/DeathStarBench/socialNetwork/test::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/socialNetwork/openshift",
            "instanceName": "C:/dev/DeathStarBench/socialNetwork/openshift::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/socialNetwork/openshift/social-graph-service.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/social-graph-mongodb.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/social-network-ns.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/unique-id-service.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/user-mention-service.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/media-memcached.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/user-mongodb.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/README.md",
              "C:/dev/DeathStarBench/socialNetwork/openshift/ubuntu-client.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/social-graph-redis.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/scripts",
              "C:/dev/DeathStarBench/socialNetwork/openshift/post-storage-mongodb.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/url-shorten-mongodb.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/jaeger.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config",
              "C:/dev/DeathStarBench/socialNetwork/openshift/write-home-timeline-rabbitmq.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/config",
              "C:/dev/DeathStarBench/socialNetwork/openshift/url-shorten-memcached.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/media-service.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/post-storage-service.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/write-home-timeline-service.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/media-frontend.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/compose-post-redis.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/home-timeline-service.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/user-timeline-redis.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/user-timeline-mongodb.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/media-mongodb.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/user-service.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/compose-post-service.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/user-memcached.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/media-frontend-config",
              "C:/dev/DeathStarBench/socialNetwork/openshift/url-shorten-service.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/text-service.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/user-timeline-service.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/home-timeline-redis.yaml",
              "C:/dev/DeathStarBench/socialNetwork/openshift/post-storage-memcached.yaml"
            ],
            "subDirectories": [
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/openshift/scripts",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/openshift/scripts::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/openshift/scripts/deploy-all-services-and-configurations.sh",
                  "C:/dev/DeathStarBench/socialNetwork/openshift/scripts/zap.sh",
                  "C:/dev/DeathStarBench/socialNetwork/openshift/scripts/create-all-micro-services.sh",
                  "C:/dev/DeathStarBench/socialNetwork/openshift/scripts/update-micro-service.sh",
                  "C:/dev/DeathStarBench/socialNetwork/openshift/scripts/restartall.sh",
                  "C:/dev/DeathStarBench/socialNetwork/openshift/scripts/configmaps",
                  "C:/dev/DeathStarBench/socialNetwork/openshift/scripts/build-docker-img.sh",
                  "C:/dev/DeathStarBench/socialNetwork/openshift/scripts/update-all-micro-services.sh",
                  "C:/dev/DeathStarBench/socialNetwork/openshift/scripts/streamlogs.sh",
                  "C:/dev/DeathStarBench/socialNetwork/openshift/scripts/create-all-configmap.sh",
                  "C:/dev/DeathStarBench/socialNetwork/openshift/scripts/update-all-configmap.sh",
                  "C:/dev/DeathStarBench/socialNetwork/openshift/scripts/dumplogs.sh"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/socialNetwork/openshift/scripts/configmaps",
                    "instanceName": "C:/dev/DeathStarBench/socialNetwork/openshift/scripts/configmaps::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/socialNetwork/openshift/scripts/configmaps/create-jaeger-configmap.sh",
                      "C:/dev/DeathStarBench/socialNetwork/openshift/scripts/configmaps/update-media-frontend-configmap.sh",
                      "C:/dev/DeathStarBench/socialNetwork/openshift/scripts/configmaps/create-nginx-thrift-configmap.sh",
                      "C:/dev/DeathStarBench/socialNetwork/openshift/scripts/configmaps/update-jaeger-configmap.sh",
                      "C:/dev/DeathStarBench/socialNetwork/openshift/scripts/configmaps/update-nginx-thrift-configmap.sh",
                      "C:/dev/DeathStarBench/socialNetwork/openshift/scripts/configmaps/create-media-frontend-configmap.sh"
                    ],
                    "subDirectories": [],
                    "numFiles": 6,
                    "package_name": "C:/dev/DeathStarBench/socialNetwork/openshift/scripts/configmaps::PackageName"
                  }
                ],
                "numFiles": 12,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/openshift/scripts::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/jaeger-config.json",
                  "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/nginx.conf",
                  "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts",
                  "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/gen-lua",
                  "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/pages"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts",
                    "instanceName": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api",
                      "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api",
                        "instanceName": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api/home-timeline",
                          "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api/user-timeline",
                          "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api/post",
                          "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api/user"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api/home-timeline",
                            "instanceName": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api/home-timeline::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api/home-timeline/read.lua"
                            ],
                            "subDirectories": [],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api/home-timeline::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api/user-timeline",
                            "instanceName": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api/user-timeline::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api/user-timeline/read.lua"
                            ],
                            "subDirectories": [],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api/user-timeline::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api/post",
                            "instanceName": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api/post::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api/post/compose.lua"
                            ],
                            "subDirectories": [],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api/post::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api/user",
                            "instanceName": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api/user::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api/user/register.lua",
                              "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api/user/unfollow.lua",
                              "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api/user/follow.lua"
                            ],
                            "subDirectories": [],
                            "numFiles": 3,
                            "package_name": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api/user::PackageName"
                          }
                        ],
                        "numFiles": 4,
                        "package_name": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/wrk2-api::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api",
                        "instanceName": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api/home-timeline",
                          "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api/user-timeline",
                          "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api/post",
                          "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api/user"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api/home-timeline",
                            "instanceName": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api/home-timeline::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api/home-timeline/read.lua"
                            ],
                            "subDirectories": [],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api/home-timeline::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api/user-timeline",
                            "instanceName": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api/user-timeline::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api/user-timeline/read.lua"
                            ],
                            "subDirectories": [],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api/user-timeline::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api/post",
                            "instanceName": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api/post::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api/post/compose.lua"
                            ],
                            "subDirectories": [],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api/post::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api/user",
                            "instanceName": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api/user::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api/user/login.lua",
                              "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api/user/register.lua",
                              "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api/user/unfollow.lua",
                              "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api/user/follow.lua"
                            ],
                            "subDirectories": [],
                            "numFiles": 4,
                            "package_name": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api/user::PackageName"
                          }
                        ],
                        "numFiles": 4,
                        "package_name": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts/api::PackageName"
                      }
                    ],
                    "numFiles": 2,
                    "package_name": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/lua-scripts::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/gen-lua",
                    "instanceName": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/gen-lua::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/gen-lua/social_network_UserMentionService.lua",
                      "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/gen-lua/social_network_UserService.lua",
                      "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/gen-lua/social_network_SocialGraphService.lua",
                      "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/gen-lua/social_network_ComposePostService.lua",
                      "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/gen-lua/social_network_TextService.lua",
                      "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/gen-lua/social_network_UserTimelineService.lua",
                      "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/gen-lua/social_network_UrlShortenService.lua",
                      "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/gen-lua/social_network_UniqueIdService.lua",
                      "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/gen-lua/social_network_HomeTimelineService.lua",
                      "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/gen-lua/social_network_constants.lua",
                      "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/gen-lua/social_network_ttypes.lua",
                      "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/gen-lua/social_network_MediaService.lua",
                      "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/gen-lua/social_network_PostStorageService.lua"
                    ],
                    "subDirectories": [],
                    "numFiles": 13,
                    "package_name": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/gen-lua::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/pages",
                    "instanceName": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/pages::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/pages/compose.html",
                      "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/pages/login.html",
                      "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/pages/favicon.ico",
                      "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/pages/user-timeline.html",
                      "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/pages/index.html",
                      "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/pages/home-timeline.html"
                    ],
                    "subDirectories": [],
                    "numFiles": 6,
                    "package_name": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config/pages::PackageName"
                  }
                ],
                "numFiles": 5,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/openshift/nginx-thrift-config::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/openshift/config",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/openshift/config::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/openshift/config/jaeger-config.yml",
                  "C:/dev/DeathStarBench/socialNetwork/openshift/config/service-config.json"
                ],
                "subDirectories": [],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/openshift/config::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/socialNetwork/openshift/media-frontend-config",
                "instanceName": "C:/dev/DeathStarBench/socialNetwork/openshift/media-frontend-config::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/socialNetwork/openshift/media-frontend-config/nginx.conf",
                  "C:/dev/DeathStarBench/socialNetwork/openshift/media-frontend-config/lua-scripts"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/socialNetwork/openshift/media-frontend-config/lua-scripts",
                    "instanceName": "C:/dev/DeathStarBench/socialNetwork/openshift/media-frontend-config/lua-scripts::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/socialNetwork/openshift/media-frontend-config/lua-scripts/upload-media.lua",
                      "C:/dev/DeathStarBench/socialNetwork/openshift/media-frontend-config/lua-scripts/get-media.lua"
                    ],
                    "subDirectories": [],
                    "numFiles": 2,
                    "package_name": "C:/dev/DeathStarBench/socialNetwork/openshift/media-frontend-config/lua-scripts::PackageName"
                  }
                ],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/socialNetwork/openshift/media-frontend-config::PackageName"
              }
            ],
            "numFiles": 37,
            "package_name": "C:/dev/DeathStarBench/socialNetwork/openshift::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/socialNetwork/gen-cpp",
            "instanceName": "C:/dev/DeathStarBench/socialNetwork/gen-cpp::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/PostStorageService.cpp",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/UniqueIdService.h",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/SocialGraphService.cpp",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/MediaService.h",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/UniqueIdService_server.skeleton.cpp",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/SocialGraphService_server.skeleton.cpp",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/UserMentionService_server.skeleton.cpp",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/social_network_types.cpp",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/TextService.cpp",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/social_network_constants.cpp",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/social_network_types.h",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/HomeTimelineService_server.skeleton.cpp",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/PostStorageService.h",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/UrlShortenService.h",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/social_network_constants.h",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/UrlShortenService.cpp",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/MediaService.cpp",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/UserMentionService.h",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/MediaService_server.skeleton.cpp",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/UserTimelineService.cpp",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/HomeTimelineService.h",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/ComposePostService.cpp",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/UserService_server.skeleton.cpp",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/ComposePostService_server.skeleton.cpp",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/UserService.h",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/ComposePostService.h",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/UrlShortenService_server.skeleton.cpp",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/UserService.cpp",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/UserTimelineService.h",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/TextService_server.skeleton.cpp",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/HomeTimelineService.cpp",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/UserMentionService.cpp",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/UniqueIdService.cpp",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/TextService.h",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/PostStorageService_server.skeleton.cpp",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/SocialGraphService.h",
              "C:/dev/DeathStarBench/socialNetwork/gen-cpp/UserTimelineService_server.skeleton.cpp"
            ],
            "subDirectories": [],
            "numFiles": 37,
            "package_name": "C:/dev/DeathStarBench/socialNetwork/gen-cpp::PackageName"
          }
        ],
        "numFiles": 23,
        "package_name": "C:/dev/DeathStarBench/socialNetwork::PackageName"
      },
      {
        "path": "C:/dev/DeathStarBench/hotelReservation",
        "instanceName": "C:/dev/DeathStarBench/hotelReservation::DirectoryComponent",
        "instanceType": "DIRECTORYCOMPONENT",
        "files": [
          "C:/dev/DeathStarBench/hotelReservation/data",
          "C:/dev/DeathStarBench/hotelReservation/services",
          "C:/dev/DeathStarBench/hotelReservation/docker-compose.yml",
          "C:/dev/DeathStarBench/hotelReservation/README.md",
          "C:/dev/DeathStarBench/hotelReservation/Docker",
          "C:/dev/DeathStarBench/hotelReservation/config.json",
          "C:/dev/DeathStarBench/hotelReservation/Gopkg.toml",
          "C:/dev/DeathStarBench/hotelReservation/tracing",
          "C:/dev/DeathStarBench/hotelReservation/registry",
          "C:/dev/DeathStarBench/hotelReservation/cmd",
          "C:/dev/DeathStarBench/hotelReservation/Dockerfile",
          "C:/dev/DeathStarBench/hotelReservation/dialer",
          "C:/dev/DeathStarBench/hotelReservation/Makefile",
          "C:/dev/DeathStarBench/hotelReservation/openshift",
          "C:/dev/DeathStarBench/hotelReservation/Gopkg.lock",
          "C:/dev/DeathStarBench/hotelReservation/wrk2_lua_scripts",
          "C:/dev/DeathStarBench/hotelReservation/vendor"
        ],
        "subDirectories": [
          {
            "path": "C:/dev/DeathStarBench/hotelReservation/data",
            "instanceName": "C:/dev/DeathStarBench/hotelReservation/data::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/hotelReservation/data/inventory.json",
              "C:/dev/DeathStarBench/hotelReservation/data/hotels.json",
              "C:/dev/DeathStarBench/hotelReservation/data/bindata.go",
              "C:/dev/DeathStarBench/hotelReservation/data/geo.json",
              "C:/dev/DeathStarBench/hotelReservation/data/locales.json"
            ],
            "subDirectories": [],
            "numFiles": 5,
            "package_name": "C:/dev/DeathStarBench/hotelReservation/data::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/hotelReservation/services",
            "instanceName": "C:/dev/DeathStarBench/hotelReservation/services::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/hotelReservation/services/geo",
              "C:/dev/DeathStarBench/hotelReservation/services/frontend",
              "C:/dev/DeathStarBench/hotelReservation/services/reservation",
              "C:/dev/DeathStarBench/hotelReservation/services/profile",
              "C:/dev/DeathStarBench/hotelReservation/services/rate",
              "C:/dev/DeathStarBench/hotelReservation/services/recommendation",
              "C:/dev/DeathStarBench/hotelReservation/services/user",
              "C:/dev/DeathStarBench/hotelReservation/services/search"
            ],
            "subDirectories": [
              {
                "path": "C:/dev/DeathStarBench/hotelReservation/services/geo",
                "instanceName": "C:/dev/DeathStarBench/hotelReservation/services/geo::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/hotelReservation/services/geo/server.go",
                  "C:/dev/DeathStarBench/hotelReservation/services/geo/proto"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/hotelReservation/services/geo/proto",
                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/services/geo/proto::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/hotelReservation/services/geo/proto/geo.proto",
                      "C:/dev/DeathStarBench/hotelReservation/services/geo/proto/geo.pb.go"
                    ],
                    "subDirectories": [],
                    "numFiles": 2,
                    "package_name": "C:/dev/DeathStarBench/hotelReservation/services/geo/proto::PackageName"
                  }
                ],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/hotelReservation/services/geo::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/hotelReservation/services/frontend",
                "instanceName": "C:/dev/DeathStarBench/hotelReservation/services/frontend::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/hotelReservation/services/frontend/server.go",
                  "C:/dev/DeathStarBench/hotelReservation/services/frontend/static"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/hotelReservation/services/frontend/static",
                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/services/frontend/static::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/hotelReservation/services/frontend/static/index.html",
                      "C:/dev/DeathStarBench/hotelReservation/services/frontend/static/stylesheets"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/services/frontend/static/stylesheets",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/services/frontend/static/stylesheets::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/services/frontend/static/stylesheets/main.css"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/services/frontend/static/stylesheets::PackageName"
                      }
                    ],
                    "numFiles": 2,
                    "package_name": "C:/dev/DeathStarBench/hotelReservation/services/frontend/static::PackageName"
                  }
                ],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/hotelReservation/services/frontend::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/hotelReservation/services/reservation",
                "instanceName": "C:/dev/DeathStarBench/hotelReservation/services/reservation::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/hotelReservation/services/reservation/server.go",
                  "C:/dev/DeathStarBench/hotelReservation/services/reservation/proto"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/hotelReservation/services/reservation/proto",
                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/services/reservation/proto::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/hotelReservation/services/reservation/proto/reservation.proto",
                      "C:/dev/DeathStarBench/hotelReservation/services/reservation/proto/reservation.pb.go"
                    ],
                    "subDirectories": [],
                    "numFiles": 2,
                    "package_name": "C:/dev/DeathStarBench/hotelReservation/services/reservation/proto::PackageName"
                  }
                ],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/hotelReservation/services/reservation::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/hotelReservation/services/profile",
                "instanceName": "C:/dev/DeathStarBench/hotelReservation/services/profile::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/hotelReservation/services/profile/server.go",
                  "C:/dev/DeathStarBench/hotelReservation/services/profile/proto"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/hotelReservation/services/profile/proto",
                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/services/profile/proto::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/hotelReservation/services/profile/proto/profile.proto",
                      "C:/dev/DeathStarBench/hotelReservation/services/profile/proto/profile.pb.go"
                    ],
                    "subDirectories": [],
                    "numFiles": 2,
                    "package_name": "C:/dev/DeathStarBench/hotelReservation/services/profile/proto::PackageName"
                  }
                ],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/hotelReservation/services/profile::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/hotelReservation/services/rate",
                "instanceName": "C:/dev/DeathStarBench/hotelReservation/services/rate::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/hotelReservation/services/rate/server.go",
                  "C:/dev/DeathStarBench/hotelReservation/services/rate/proto"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/hotelReservation/services/rate/proto",
                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/services/rate/proto::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/hotelReservation/services/rate/proto/rate.proto",
                      "C:/dev/DeathStarBench/hotelReservation/services/rate/proto/rate.pb.go"
                    ],
                    "subDirectories": [],
                    "numFiles": 2,
                    "package_name": "C:/dev/DeathStarBench/hotelReservation/services/rate/proto::PackageName"
                  }
                ],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/hotelReservation/services/rate::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/hotelReservation/services/recommendation",
                "instanceName": "C:/dev/DeathStarBench/hotelReservation/services/recommendation::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/hotelReservation/services/recommendation/server.go",
                  "C:/dev/DeathStarBench/hotelReservation/services/recommendation/proto"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/hotelReservation/services/recommendation/proto",
                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/services/recommendation/proto::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/hotelReservation/services/recommendation/proto/recommendation.pb.go",
                      "C:/dev/DeathStarBench/hotelReservation/services/recommendation/proto/recommendation.proto"
                    ],
                    "subDirectories": [],
                    "numFiles": 2,
                    "package_name": "C:/dev/DeathStarBench/hotelReservation/services/recommendation/proto::PackageName"
                  }
                ],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/hotelReservation/services/recommendation::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/hotelReservation/services/user",
                "instanceName": "C:/dev/DeathStarBench/hotelReservation/services/user::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/hotelReservation/services/user/server.go",
                  "C:/dev/DeathStarBench/hotelReservation/services/user/proto"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/hotelReservation/services/user/proto",
                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/services/user/proto::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/hotelReservation/services/user/proto/user.proto",
                      "C:/dev/DeathStarBench/hotelReservation/services/user/proto/user.pb.go"
                    ],
                    "subDirectories": [],
                    "numFiles": 2,
                    "package_name": "C:/dev/DeathStarBench/hotelReservation/services/user/proto::PackageName"
                  }
                ],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/hotelReservation/services/user::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/hotelReservation/services/search",
                "instanceName": "C:/dev/DeathStarBench/hotelReservation/services/search::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/hotelReservation/services/search/server.go",
                  "C:/dev/DeathStarBench/hotelReservation/services/search/proto"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/hotelReservation/services/search/proto",
                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/services/search/proto::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/hotelReservation/services/search/proto/search.proto",
                      "C:/dev/DeathStarBench/hotelReservation/services/search/proto/search.pb.go"
                    ],
                    "subDirectories": [],
                    "numFiles": 2,
                    "package_name": "C:/dev/DeathStarBench/hotelReservation/services/search/proto::PackageName"
                  }
                ],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/hotelReservation/services/search::PackageName"
              }
            ],
            "numFiles": 8,
            "package_name": "C:/dev/DeathStarBench/hotelReservation/services::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/hotelReservation/Docker",
            "instanceName": "C:/dev/DeathStarBench/hotelReservation/Docker::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/hotelReservation/Docker/.gitignore",
              "C:/dev/DeathStarBench/hotelReservation/Docker/._.DS_Store"
            ],
            "subDirectories": [],
            "numFiles": 2,
            "package_name": "C:/dev/DeathStarBench/hotelReservation/Docker::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/hotelReservation/tracing",
            "instanceName": "C:/dev/DeathStarBench/hotelReservation/tracing::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/hotelReservation/tracing/mux.go",
              "C:/dev/DeathStarBench/hotelReservation/tracing/tracer.go"
            ],
            "subDirectories": [],
            "numFiles": 2,
            "package_name": "C:/dev/DeathStarBench/hotelReservation/tracing::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/hotelReservation/registry",
            "instanceName": "C:/dev/DeathStarBench/hotelReservation/registry::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/hotelReservation/registry/registry.go"
            ],
            "subDirectories": [],
            "numFiles": 1,
            "package_name": "C:/dev/DeathStarBench/hotelReservation/registry::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/hotelReservation/cmd",
            "instanceName": "C:/dev/DeathStarBench/hotelReservation/cmd::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/hotelReservation/cmd/geo",
              "C:/dev/DeathStarBench/hotelReservation/cmd/frontend",
              "C:/dev/DeathStarBench/hotelReservation/cmd/reservation",
              "C:/dev/DeathStarBench/hotelReservation/cmd/profile",
              "C:/dev/DeathStarBench/hotelReservation/cmd/rate",
              "C:/dev/DeathStarBench/hotelReservation/cmd/recommendation",
              "C:/dev/DeathStarBench/hotelReservation/cmd/user",
              "C:/dev/DeathStarBench/hotelReservation/cmd/search"
            ],
            "subDirectories": [
              {
                "path": "C:/dev/DeathStarBench/hotelReservation/cmd/geo",
                "instanceName": "C:/dev/DeathStarBench/hotelReservation/cmd/geo::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/hotelReservation/cmd/geo/db.go",
                  "C:/dev/DeathStarBench/hotelReservation/cmd/geo/main.go"
                ],
                "subDirectories": [],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/hotelReservation/cmd/geo::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/hotelReservation/cmd/frontend",
                "instanceName": "C:/dev/DeathStarBench/hotelReservation/cmd/frontend::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/hotelReservation/cmd/frontend/main.go"
                ],
                "subDirectories": [],
                "numFiles": 1,
                "package_name": "C:/dev/DeathStarBench/hotelReservation/cmd/frontend::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/hotelReservation/cmd/reservation",
                "instanceName": "C:/dev/DeathStarBench/hotelReservation/cmd/reservation::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/hotelReservation/cmd/reservation/db.go",
                  "C:/dev/DeathStarBench/hotelReservation/cmd/reservation/main.go"
                ],
                "subDirectories": [],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/hotelReservation/cmd/reservation::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/hotelReservation/cmd/profile",
                "instanceName": "C:/dev/DeathStarBench/hotelReservation/cmd/profile::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/hotelReservation/cmd/profile/db.go",
                  "C:/dev/DeathStarBench/hotelReservation/cmd/profile/main.go"
                ],
                "subDirectories": [],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/hotelReservation/cmd/profile::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/hotelReservation/cmd/rate",
                "instanceName": "C:/dev/DeathStarBench/hotelReservation/cmd/rate::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/hotelReservation/cmd/rate/db.go",
                  "C:/dev/DeathStarBench/hotelReservation/cmd/rate/main.go"
                ],
                "subDirectories": [],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/hotelReservation/cmd/rate::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/hotelReservation/cmd/recommendation",
                "instanceName": "C:/dev/DeathStarBench/hotelReservation/cmd/recommendation::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/hotelReservation/cmd/recommendation/db.go",
                  "C:/dev/DeathStarBench/hotelReservation/cmd/recommendation/main.go"
                ],
                "subDirectories": [],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/hotelReservation/cmd/recommendation::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/hotelReservation/cmd/user",
                "instanceName": "C:/dev/DeathStarBench/hotelReservation/cmd/user::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/hotelReservation/cmd/user/db.go",
                  "C:/dev/DeathStarBench/hotelReservation/cmd/user/main.go"
                ],
                "subDirectories": [],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/hotelReservation/cmd/user::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/hotelReservation/cmd/search",
                "instanceName": "C:/dev/DeathStarBench/hotelReservation/cmd/search::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/hotelReservation/cmd/search/main.go"
                ],
                "subDirectories": [],
                "numFiles": 1,
                "package_name": "C:/dev/DeathStarBench/hotelReservation/cmd/search::PackageName"
              }
            ],
            "numFiles": 8,
            "package_name": "C:/dev/DeathStarBench/hotelReservation/cmd::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/hotelReservation/dialer",
            "instanceName": "C:/dev/DeathStarBench/hotelReservation/dialer::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/hotelReservation/dialer/dialer.go"
            ],
            "subDirectories": [],
            "numFiles": 1,
            "package_name": "C:/dev/DeathStarBench/hotelReservation/dialer::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/hotelReservation/openshift",
            "instanceName": "C:/dev/DeathStarBench/hotelReservation/openshift::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/hotelReservation/openshift/recommendation-persistentvolumeclaim.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/rate-persistentvolumeclaim.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/mongodb-profile-service.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/mongodb-profile-deployment.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/mongodb-user-deployment.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/hr-client.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/mongodb-recommendation-service.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/mongodb-reservation-deployment.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/jaeger-service.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/mongodb-rate-deployment.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/user-persistentvolumeclaim.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/reservation-service.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/mongodb-reservation-service.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/memcached-profile-deployment.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/consul-deployment.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/search-service.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/README.md",
              "C:/dev/DeathStarBench/hotelReservation/openshift/jaeger-deployment.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/user-deployment.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/scripts",
              "C:/dev/DeathStarBench/hotelReservation/openshift/profile-persistentvolumeclaim.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/reservation-deployment.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/memcached-rate-deployment.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/geo-deployment.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/mongodb-user-service.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/geo-service.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/memcached-reserve-deployment.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/memcached-profile-service.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/rate-deployment.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/geo-persistentvolumeclaim.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/consul-service.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/profile-service.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/mongodb-geo-service.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/profile-deployment.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/memcached-rate-service.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/rate-service.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/recommendation-deployment.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/recommendation-service.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/configmaps",
              "C:/dev/DeathStarBench/hotelReservation/openshift/search-deployment.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/memcached-reserve-service.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/frontend-deployment.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/user-service.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/reservation-persistentvolumeclaim.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/mongodb-rate-service.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/mongodb-recommendation-deployment.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/frontend-service.yaml",
              "C:/dev/DeathStarBench/hotelReservation/openshift/mongodb-geo-deployment.yaml"
            ],
            "subDirectories": [
              {
                "path": "C:/dev/DeathStarBench/hotelReservation/openshift/scripts",
                "instanceName": "C:/dev/DeathStarBench/hotelReservation/openshift/scripts::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/hotelReservation/openshift/scripts/create-configmaps.sh",
                  "C:/dev/DeathStarBench/hotelReservation/openshift/scripts/update-configmaps.sh",
                  "C:/dev/DeathStarBench/hotelReservation/openshift/scripts/zap.sh",
                  "C:/dev/DeathStarBench/hotelReservation/openshift/scripts/softrestart.sh",
                  "C:/dev/DeathStarBench/hotelReservation/openshift/scripts/build-docker-img.sh",
                  "C:/dev/DeathStarBench/hotelReservation/openshift/scripts/deploy.sh",
                  "C:/dev/DeathStarBench/hotelReservation/openshift/scripts/dumplogs.sh"
                ],
                "subDirectories": [],
                "numFiles": 7,
                "package_name": "C:/dev/DeathStarBench/hotelReservation/openshift/scripts::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/hotelReservation/openshift/configmaps",
                "instanceName": "C:/dev/DeathStarBench/hotelReservation/openshift/configmaps::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/hotelReservation/openshift/configmaps/config.json"
                ],
                "subDirectories": [],
                "numFiles": 1,
                "package_name": "C:/dev/DeathStarBench/hotelReservation/openshift/configmaps::PackageName"
              }
            ],
            "numFiles": 48,
            "package_name": "C:/dev/DeathStarBench/hotelReservation/openshift::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/hotelReservation/wrk2_lua_scripts",
            "instanceName": "C:/dev/DeathStarBench/hotelReservation/wrk2_lua_scripts::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/hotelReservation/wrk2_lua_scripts/mixed-workload_type_1.lua"
            ],
            "subDirectories": [],
            "numFiles": 1,
            "package_name": "C:/dev/DeathStarBench/hotelReservation/wrk2_lua_scripts::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/hotelReservation/vendor",
            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org",
              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com",
              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org"
            ],
            "subDirectories": [
              {
                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org",
                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc",
                  "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/genproto"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc",
                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/backoff.go",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/balancer_conn_wrappers.go",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/credentials",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/connectivity",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclb",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/internal",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/stats",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/service_config.go",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/transport",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/go16.go",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/LICENSE",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/AUTHORS",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/README.md",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/keepalive",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/picker_wrapper.go",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/clientconn.go",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/server.go",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/stream.go",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/balancer",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/proxy.go",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/rpc_util.go",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/trace.go",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/status",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/naming",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/peer",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/resolver",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/vet.sh",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclb_picker.go",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclb_remote_balancer.go",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/interceptor.go",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/tap",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/encoding",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/metadata",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/call.go",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/balancer_v1_wrapper.go",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/balancer.go",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/Makefile",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/codes",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/.travis.yml",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/resolver_conn_wrapper.go",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/go17.go",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclb_util.go",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclog",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/codegen.sh",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/doc.go",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/CONTRIBUTING.md",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclb.go",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/codec.go",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/pickfirst.go"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/credentials",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/credentials::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/credentials/credentials_util_pre_go17.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/credentials/credentials_util_go18.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/credentials/credentials_util_go17.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/credentials/credentials.go"
                        ],
                        "subDirectories": [],
                        "numFiles": 4,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/credentials::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/connectivity",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/connectivity::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/connectivity/connectivity.go"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/connectivity::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclb",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclb::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclb/grpc_lb_v1"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclb/grpc_lb_v1",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclb/grpc_lb_v1::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclb/grpc_lb_v1/messages"
                            ],
                            "subDirectories": [
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclb/grpc_lb_v1/messages",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclb/grpc_lb_v1/messages::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclb/grpc_lb_v1/messages/messages.pb.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclb/grpc_lb_v1/messages/messages.proto"
                                ],
                                "subDirectories": [],
                                "numFiles": 2,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclb/grpc_lb_v1/messages::PackageName"
                              }
                            ],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclb/grpc_lb_v1::PackageName"
                          }
                        ],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclb::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/internal",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/internal::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/internal/internal.go"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/internal::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/stats",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/stats::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/stats/stats.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/stats/handlers.go"
                        ],
                        "subDirectories": [],
                        "numFiles": 2,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/stats::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/transport",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/transport::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/transport/http2_server.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/transport/http2_client.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/transport/go16.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/transport/handler_server.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/transport/control.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/transport/log.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/transport/bdp_estimator.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/transport/transport.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/transport/go17.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/transport/http_util.go"
                        ],
                        "subDirectories": [],
                        "numFiles": 10,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/transport::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/keepalive",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/keepalive::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/keepalive/keepalive.go"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/keepalive::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/balancer",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/balancer::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/balancer/base",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/balancer/roundrobin",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/balancer/balancer.go"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/balancer/base",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/balancer/base::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/balancer/base/base.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/balancer/base/balancer.go"
                            ],
                            "subDirectories": [],
                            "numFiles": 2,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/balancer/base::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/balancer/roundrobin",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/balancer/roundrobin::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/balancer/roundrobin/roundrobin.go"
                            ],
                            "subDirectories": [],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/balancer/roundrobin::PackageName"
                          }
                        ],
                        "numFiles": 3,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/balancer::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/status",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/status::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/status/status.go"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/status::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/naming",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/naming::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/naming/go18.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/naming/go17.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/naming/dns_resolver.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/naming/naming.go"
                        ],
                        "subDirectories": [],
                        "numFiles": 4,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/naming::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/peer",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/peer::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/peer/peer.go"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/peer::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/resolver",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/resolver::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/resolver/passthrough",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/resolver/resolver.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/resolver/dns"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/resolver/passthrough",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/resolver/passthrough::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/resolver/passthrough/passthrough.go"
                            ],
                            "subDirectories": [],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/resolver/passthrough::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/resolver/dns",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/resolver/dns::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/resolver/dns/go18.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/resolver/dns/go17.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/resolver/dns/dns_resolver.go"
                            ],
                            "subDirectories": [],
                            "numFiles": 3,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/resolver/dns::PackageName"
                          }
                        ],
                        "numFiles": 3,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/resolver::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/tap",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/tap::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/tap/tap.go"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/tap::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/encoding",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/encoding::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/encoding/proto",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/encoding/encoding.go"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/encoding/proto",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/encoding/proto::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/encoding/proto/proto.go"
                            ],
                            "subDirectories": [],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/encoding/proto::PackageName"
                          }
                        ],
                        "numFiles": 2,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/encoding::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/metadata",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/metadata::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/metadata/metadata.go"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/metadata::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/codes",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/codes::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/codes/codes.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/codes/code_string.go"
                        ],
                        "subDirectories": [],
                        "numFiles": 2,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/codes::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclog",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclog::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclog/loggerv2.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclog/grpclog.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclog/logger.go"
                        ],
                        "subDirectories": [],
                        "numFiles": 3,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc/grpclog::PackageName"
                      }
                    ],
                    "numFiles": 49,
                    "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/grpc::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/genproto",
                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/genproto::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/genproto/LICENSE",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/genproto/googleapis"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/genproto/googleapis",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/genproto/googleapis::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/genproto/googleapis/rpc"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/genproto/googleapis/rpc",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/genproto/googleapis/rpc::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/genproto/googleapis/rpc/status"
                            ],
                            "subDirectories": [
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/genproto/googleapis/rpc/status",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/genproto/googleapis/rpc/status::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/genproto/googleapis/rpc/status/status.pb.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 1,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/genproto/googleapis/rpc/status::PackageName"
                              }
                            ],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/genproto/googleapis/rpc::PackageName"
                          }
                        ],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/genproto/googleapis::PackageName"
                      }
                    ],
                    "numFiles": 2,
                    "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org/genproto::PackageName"
                  }
                ],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/google.golang.org::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com",
                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/mitchellh",
                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/segmentio",
                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp",
                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber",
                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache",
                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem",
                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing",
                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/olivere",
                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hailocab",
                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing-contrib",
                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/codahale",
                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/mitchellh",
                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/mitchellh::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/mitchellh/go-homedir"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/mitchellh/go-homedir",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/mitchellh/go-homedir::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/mitchellh/go-homedir/LICENSE",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/mitchellh/go-homedir/README.md",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/mitchellh/go-homedir/homedir.go"
                        ],
                        "subDirectories": [],
                        "numFiles": 3,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/mitchellh/go-homedir::PackageName"
                      }
                    ],
                    "numFiles": 1,
                    "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/mitchellh::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/segmentio",
                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/segmentio::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/segmentio/ksuid"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/segmentio/ksuid",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/segmentio/ksuid::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/segmentio/ksuid/base62.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/segmentio/ksuid/.gitignore",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/segmentio/ksuid/README.md",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/segmentio/ksuid/ksuid.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/segmentio/ksuid/set.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/segmentio/ksuid/rand.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/segmentio/ksuid/sequence.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/segmentio/ksuid/uint128.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/segmentio/ksuid/LICENSE.md"
                        ],
                        "subDirectories": [],
                        "numFiles": 9,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/segmentio/ksuid::PackageName"
                      }
                    ],
                    "numFiles": 1,
                    "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/segmentio::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp",
                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/go-cleanhttp",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/go-rootcerts"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/go-cleanhttp",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/go-cleanhttp::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/go-cleanhttp/LICENSE",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/go-cleanhttp/README.md",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/go-cleanhttp/cleanhttp.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/go-cleanhttp/doc.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/go-cleanhttp/handlers.go"
                        ],
                        "subDirectories": [],
                        "numFiles": 5,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/go-cleanhttp::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/LICENSE",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/website",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/website",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/website::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/website/LICENSE.md"
                            ],
                            "subDirectories": [],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/website::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api/operator_raft.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api/status.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api/raw.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api/coordinate.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api/README.md",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api/event.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api/operator.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api/api.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api/lock.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api/operator_area.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api/catalog.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api/snapshot.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api/operator_keyring.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api/semaphore.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api/operator_segment.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api/operator_autopilot.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api/acl.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api/kv.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api/agent.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api/health.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api/session.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api/prepared_query.go"
                            ],
                            "subDirectories": [],
                            "numFiles": 22,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul/api::PackageName"
                          }
                        ],
                        "numFiles": 3,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/consul::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/LICENSE",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/ops-misc",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/website",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/coordinate"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/ops-misc",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/ops-misc::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/ops-misc/debian"
                            ],
                            "subDirectories": [
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/ops-misc/debian",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/ops-misc/debian::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/ops-misc/debian/copyright"
                                ],
                                "subDirectories": [],
                                "numFiles": 1,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/ops-misc/debian::PackageName"
                              }
                            ],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/ops-misc::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/website",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/website::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/website/source"
                            ],
                            "subDirectories": [
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/website/source",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/website/source::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/website/source/LICENSE"
                                ],
                                "subDirectories": [],
                                "numFiles": 1,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/website/source::PackageName"
                              }
                            ],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/website::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/coordinate",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/coordinate::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/coordinate/config.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/coordinate/phantom.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/coordinate/coordinate.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/coordinate/client.go"
                            ],
                            "subDirectories": [],
                            "numFiles": 4,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf/coordinate::PackageName"
                          }
                        ],
                        "numFiles": 4,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/serf::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/go-rootcerts",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/go-rootcerts::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/go-rootcerts/rootcerts.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/go-rootcerts/LICENSE",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/go-rootcerts/README.md",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/go-rootcerts/rootcerts_base.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/go-rootcerts/Makefile",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/go-rootcerts/.travis.yml",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/go-rootcerts/rootcerts_darwin.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/go-rootcerts/doc.go"
                        ],
                        "subDirectories": [],
                        "numFiles": 8,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp/go-rootcerts::PackageName"
                      }
                    ],
                    "numFiles": 4,
                    "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hashicorp::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber",
                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-lib",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-lib",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-lib::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-lib/LICENSE",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-lib/metrics"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-lib/metrics",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-lib/metrics::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-lib/metrics/local.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-lib/metrics/metrics.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-lib/metrics/gauge.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-lib/metrics/factory.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-lib/metrics/counter.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-lib/metrics/timer.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-lib/metrics/stopwatch.go"
                            ],
                            "subDirectories": [],
                            "numFiles": 7,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-lib/metrics::PackageName"
                          }
                        ],
                        "numFiles": 2,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-lib::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/jaeger_tag.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/DCO",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/observer.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/transport_udp.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/.gitignore",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/contrib_observer.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/internal",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/LICENSE",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/README.md",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/reporter.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/metrics.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/jaeger_thrift_span.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/interop.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/config",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/glide.yaml",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/constants.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/logger.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/Gopkg.toml",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/sampler.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/.gitmodules",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/utils",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/reporter_options.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/propagation.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/sampler_options.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/RELEASE.md",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/glide.lock",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/span.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/header.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/Makefile",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/.travis.yml",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/zipkin_thrift_span.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/transport.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/tracer.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/tracer_options.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/rpcmetrics",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/doc.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/Gopkg.lock",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/baggage_setter.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/CONTRIBUTING.md",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/reference.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/context.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/zipkin.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/log",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/CHANGELOG.md"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/internal",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/internal::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/internal/spanlog",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/internal/baggage"
                            ],
                            "subDirectories": [
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/internal/spanlog",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/internal/spanlog::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/internal/spanlog/json.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 1,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/internal/spanlog::PackageName"
                              },
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/internal/baggage",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/internal/baggage::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/internal/baggage/remote",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/internal/baggage/restriction_manager.go"
                                ],
                                "subDirectories": [
                                  {
                                    "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/internal/baggage/remote",
                                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/internal/baggage/remote::DirectoryComponent",
                                    "instanceType": "DIRECTORYCOMPONENT",
                                    "files": [
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/internal/baggage/remote/options.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/internal/baggage/remote/restriction_manager.go"
                                    ],
                                    "subDirectories": [],
                                    "numFiles": 2,
                                    "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/internal/baggage/remote::PackageName"
                                  }
                                ],
                                "numFiles": 2,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/internal/baggage::PackageName"
                              }
                            ],
                            "numFiles": 2,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/internal::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/config",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/config::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/config/config.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/config/options.go"
                            ],
                            "subDirectories": [],
                            "numFiles": 2,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/config::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/jaeger",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/agent",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/sampling",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/baggage",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/zipkincore"
                            ],
                            "subDirectories": [
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/jaeger",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/jaeger::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/jaeger/constants.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/jaeger/agent.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/jaeger/ttypes.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 3,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/jaeger::PackageName"
                              },
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/agent",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/agent::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/agent/constants.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/agent/agent.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/agent/ttypes.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 3,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/agent::PackageName"
                              },
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/sampling",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/sampling::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/sampling/samplingmanager.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/sampling/constants.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/sampling/ttypes.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 3,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/sampling::PackageName"
                              },
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/baggage",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/baggage::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/baggage/baggagerestrictionmanager.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/baggage/constants.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/baggage/ttypes.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 3,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/baggage::PackageName"
                              },
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/zipkincore",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/zipkincore::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/zipkincore/constants.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/zipkincore/zipkincollector.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/zipkincore/ttypes.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 3,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen/zipkincore::PackageName"
                              }
                            ],
                            "numFiles": 5,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/thrift-gen::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/utils",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/utils::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/utils/udp_client.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/utils/rand.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/utils/rate_limiter.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/utils/utils.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/utils/localip.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/utils/http_json.go"
                            ],
                            "subDirectories": [],
                            "numFiles": 6,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/utils::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/rpcmetrics",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/rpcmetrics::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/rpcmetrics/observer.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/rpcmetrics/README.md",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/rpcmetrics/metrics.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/rpcmetrics/endpoints.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/rpcmetrics/normalizer.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/rpcmetrics/doc.go"
                            ],
                            "subDirectories": [],
                            "numFiles": 6,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/rpcmetrics::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/log",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/log::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/log/logger.go"
                            ],
                            "subDirectories": [],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go/log::PackageName"
                          }
                        ],
                        "numFiles": 45,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber/jaeger-client-go::PackageName"
                      }
                    ],
                    "numFiles": 2,
                    "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/uber::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache",
                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/LICENSE",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/contrib",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/NOTICE",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/debian"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/contrib",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/contrib::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/contrib/fb303"
                            ],
                            "subDirectories": [
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/contrib/fb303",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/contrib/fb303::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/contrib/fb303/LICENSE"
                                ],
                                "subDirectories": [],
                                "numFiles": 1,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/contrib/fb303::PackageName"
                              }
                            ],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/contrib::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/dart",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/hs"
                            ],
                            "subDirectories": [
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift"
                                ],
                                "subDirectories": [
                                  {
                                    "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift",
                                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift::DirectoryComponent",
                                    "instanceType": "DIRECTORYCOMPONENT",
                                    "files": [
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/field.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/simple_json_protocol.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/framed_transport.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/protocol_exception.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/application_exception.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/transport_exception.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/compact_protocol.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/exception.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/multiplexed_protocol.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/messagetype.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/iostream_transport.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/ssl_socket.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/buffered_transport.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/http_transport.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/zlib_transport.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/server.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/http_client.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/memory_buffer.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/deserializer.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/rich_transport.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/serializer.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/pointerize.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/transport_factory.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/ssl_server_socket.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/server_transport.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/debug_protocol.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/json_protocol.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/numeric.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/type.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/server_socket.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/protocol.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/processor.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/protocol_factory.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/binary_protocol.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/processor_factory.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/transport.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/socket.go",
                                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift/simple_server.go"
                                    ],
                                    "subDirectories": [],
                                    "numFiles": 38,
                                    "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go/thrift::PackageName"
                                  }
                                ],
                                "numFiles": 1,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/go::PackageName"
                              },
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/dart",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/dart::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/dart/LICENSE_HEADER"
                                ],
                                "subDirectories": [],
                                "numFiles": 1,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/dart::PackageName"
                              },
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/hs",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/hs::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/hs/LICENSE"
                                ],
                                "subDirectories": [],
                                "numFiles": 1,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib/hs::PackageName"
                              }
                            ],
                            "numFiles": 3,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/lib::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/debian",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/debian::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/debian/copyright"
                            ],
                            "subDirectories": [],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift/debian::PackageName"
                          }
                        ],
                        "numFiles": 5,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache/thrift::PackageName"
                      }
                    ],
                    "numFiles": 1,
                    "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/apache::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem",
                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem/grpc-opentracing"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem/grpc-opentracing",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem/grpc-opentracing::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem/grpc-opentracing/go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem/grpc-opentracing/PATENTS",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem/grpc-opentracing/LICENSE"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem/grpc-opentracing/go",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem/grpc-opentracing/go::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem/grpc-opentracing/go/otgrpc"
                            ],
                            "subDirectories": [
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem/grpc-opentracing/go/otgrpc",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem/grpc-opentracing/go/otgrpc::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem/grpc-opentracing/go/otgrpc/shared.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem/grpc-opentracing/go/otgrpc/README.md",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem/grpc-opentracing/go/otgrpc/client.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem/grpc-opentracing/go/otgrpc/options.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem/grpc-opentracing/go/otgrpc/errors.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem/grpc-opentracing/go/otgrpc/server.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem/grpc-opentracing/go/otgrpc/package.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 7,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem/grpc-opentracing/go/otgrpc::PackageName"
                              }
                            ],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem/grpc-opentracing/go::PackageName"
                          }
                        ],
                        "numFiles": 3,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem/grpc-opentracing::PackageName"
                      }
                    ],
                    "numFiles": 1,
                    "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/grpc-ecosystem::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing",
                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go/ext",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go/.gitignore",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go/LICENSE",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go/README.md",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go/noop.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go/gocontext.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go/propagation.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go/span.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go/Makefile",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go/.travis.yml",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go/globaltracer.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go/tracer.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go/log",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go/CHANGELOG.md"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go/ext",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go/ext::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go/ext/tags.go"
                            ],
                            "subDirectories": [],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go/ext::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go/log",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go/log::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go/log/field.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go/log/util.go"
                            ],
                            "subDirectories": [],
                            "numFiles": 2,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go/log::PackageName"
                          }
                        ],
                        "numFiles": 14,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing/opentracing-go::PackageName"
                      }
                    ],
                    "numFiles": 1,
                    "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/olivere",
                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/olivere::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/olivere/grpc"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/olivere/grpc",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/olivere/grpc::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/olivere/grpc/LICENSE",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/olivere/grpc/lb"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/olivere/grpc/lb",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/olivere/grpc/lb::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/olivere/grpc/lb/consul"
                            ],
                            "subDirectories": [
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/olivere/grpc/lb/consul",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/olivere/grpc/lb/consul::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/olivere/grpc/lb/consul/consul.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 1,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/olivere/grpc/lb/consul::PackageName"
                              }
                            ],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/olivere/grpc/lb::PackageName"
                          }
                        ],
                        "numFiles": 2,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/olivere/grpc::PackageName"
                      }
                    ],
                    "numFiles": 1,
                    "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/olivere::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hailocab",
                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hailocab::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hailocab/go-geoindex"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hailocab/go-geoindex",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hailocab/go-geoindex::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hailocab/go-geoindex/count-index.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hailocab/go-geoindex/testing.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hailocab/go-geoindex/sets.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hailocab/go-geoindex/LICENSE",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hailocab/go-geoindex/README.md",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hailocab/go-geoindex/queue.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hailocab/go-geoindex/point.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hailocab/go-geoindex/points-index.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hailocab/go-geoindex/counters.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hailocab/go-geoindex/geo-index.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hailocab/go-geoindex/benchmarks.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hailocab/go-geoindex/clustering-index.go"
                        ],
                        "subDirectories": [],
                        "numFiles": 12,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hailocab/go-geoindex::PackageName"
                      }
                    ],
                    "numFiles": 1,
                    "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/hailocab::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing-contrib",
                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing-contrib::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing-contrib/go-stdlib"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing-contrib/go-stdlib",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing-contrib/go-stdlib::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing-contrib/go-stdlib/nethttp",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing-contrib/go-stdlib/LICENSE"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing-contrib/go-stdlib/nethttp",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing-contrib/go-stdlib/nethttp::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing-contrib/go-stdlib/nethttp/client.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing-contrib/go-stdlib/nethttp/server.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing-contrib/go-stdlib/nethttp/doc.go"
                            ],
                            "subDirectories": [],
                            "numFiles": 3,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing-contrib/go-stdlib/nethttp::PackageName"
                          }
                        ],
                        "numFiles": 2,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing-contrib/go-stdlib::PackageName"
                      }
                    ],
                    "numFiles": 1,
                    "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/opentracing-contrib::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/codahale",
                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/codahale::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/codahale/hdrhistogram"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/codahale/hdrhistogram",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/codahale/hdrhistogram::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/codahale/hdrhistogram/LICENSE",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/codahale/hdrhistogram/hdr.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/codahale/hdrhistogram/README.md",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/codahale/hdrhistogram/window.go",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/codahale/hdrhistogram/.travis.yml"
                        ],
                        "subDirectories": [],
                        "numFiles": 5,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/codahale/hdrhistogram::PackageName"
                      }
                    ],
                    "numFiles": 1,
                    "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/codahale::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang",
                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/LICENSE",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/AUTHORS",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/proto",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/CONTRIBUTORS"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes/timestamp",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes/regen.sh",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes/duration",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes/duration.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes/any.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes/timestamp.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes/any",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes/doc.go"
                            ],
                            "subDirectories": [
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes/timestamp",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes/timestamp::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes/timestamp/timestamp.pb.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes/timestamp/timestamp.proto"
                                ],
                                "subDirectories": [],
                                "numFiles": 2,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes/timestamp::PackageName"
                              },
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes/duration",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes/duration::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes/duration/duration.proto",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes/duration/duration.pb.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 2,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes/duration::PackageName"
                              },
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes/any",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes/any::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes/any/any.proto",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes/any/any.pb.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 2,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes/any::PackageName"
                              }
                            ],
                            "numFiles": 8,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/ptypes::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/proto",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/proto::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/proto/properties.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/proto/pointer_unsafe.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/proto/text_parser.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/proto/discard.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/proto/text.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/proto/message_set.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/proto/lib.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/proto/pointer_reflect.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/proto/decode.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/proto/extensions.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/proto/Makefile",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/proto/clone.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/proto/equal.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/proto/encode.go"
                            ],
                            "subDirectories": [],
                            "numFiles": 14,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf/proto::PackageName"
                          }
                        ],
                        "numFiles": 5,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang/protobuf::PackageName"
                      }
                    ],
                    "numFiles": 1,
                    "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com/golang::PackageName"
                  }
                ],
                "numFiles": 12,
                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/github.com::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org",
                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x",
                    "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net",
                      "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/internal",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/PATENTS",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/LICENSE",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/AUTHORS",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/lex",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/CONTRIBUTORS",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/idna",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/trace",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/context"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/internal",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/internal::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/internal/timeseries"
                            ],
                            "subDirectories": [
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/internal/timeseries",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/internal/timeseries::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/internal/timeseries/timeseries.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 1,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/internal/timeseries::PackageName"
                              }
                            ],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/internal::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/lex",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/lex::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/lex/httplex"
                            ],
                            "subDirectories": [
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/lex/httplex",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/lex/httplex::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/lex/httplex/httplex.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 1,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/lex/httplex::PackageName"
                              }
                            ],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/lex::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/idna",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/idna::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/idna/trieval.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/idna/tables.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/idna/punycode.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/idna/idna.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/idna/trie.go"
                            ],
                            "subDirectories": [],
                            "numFiles": 5,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/idna::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/trace",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/trace::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/trace/trace_go17.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/trace/trace_go16.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/trace/trace.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/trace/events.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/trace/histogram.go"
                            ],
                            "subDirectories": [],
                            "numFiles": 5,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/trace::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/hpack",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/not_go19.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/go17_not18.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/client_conn_pool.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/.gitignore",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/write.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/go16.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/writesched.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/gotrack.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/errors.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/server.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/go19.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/writesched_random.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/frame.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/http2.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/ciphers.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/not_go18.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/not_go17.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/writesched_priority.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/configure_transport.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/pipe.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/Dockerfile",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/not_go16.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/Makefile",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/go18.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/transport.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/go17.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/flow.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/README",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/headermap.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/databuffer.go"
                            ],
                            "subDirectories": [
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/hpack",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/hpack::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/hpack/tables.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/hpack/hpack.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/hpack/huffman.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/hpack/encode.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 4,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2/hpack::PackageName"
                              }
                            ],
                            "numFiles": 31,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/http2::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/context",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/context::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/context/pre_go19.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/context/pre_go17.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/context/go19.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/context/go17.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/context/context.go"
                            ],
                            "subDirectories": [],
                            "numFiles": 5,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net/context::PackageName"
                          }
                        ],
                        "numFiles": 10,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/net::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text",
                        "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/transform",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/PATENTS",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/LICENSE",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/AUTHORS",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/language",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/collate",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/CONTRIBUTORS",
                          "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/secure"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/ucd",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/tag",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/colltab",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/gen",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/triegen"
                            ],
                            "subDirectories": [
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/ucd",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/ucd::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/ucd/ucd.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 1,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/ucd::PackageName"
                              },
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/tag",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/tag::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/tag/tag.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 1,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/tag::PackageName"
                              },
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/colltab",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/colltab::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/colltab/iter.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/colltab/colltab.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/colltab/collelem.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/colltab/numeric.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/colltab/trie.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/colltab/weighter.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/colltab/contract.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/colltab/table.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 8,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/colltab::PackageName"
                              },
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/gen",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/gen::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/gen/gen.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/gen/code.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 2,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/gen::PackageName"
                              },
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/triegen",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/triegen::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/triegen/compact.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/triegen/print.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/triegen/triegen.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 3,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal/triegen::PackageName"
                              }
                            ],
                            "numFiles": 5,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/internal::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/transform",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/transform::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/transform/transform.go"
                            ],
                            "subDirectories": [],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/transform::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/language",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/language::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/language/coverage.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/language/gen_index.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/language/go1_2.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/language/index.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/language/common.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/language/language.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/language/parse.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/language/tags.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/language/tables.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/language/gen.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/language/lookup.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/language/gen_common.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/language/match.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/language/Makefile",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/language/go1_1.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/language/doc.go"
                            ],
                            "subDirectories": [],
                            "numFiles": 16,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/language::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/collate",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/collate::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/collate/index.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/collate/tables.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/collate/collate.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/collate/option.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/collate/sort.go",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/collate/build",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/collate/maketables.go"
                            ],
                            "subDirectories": [
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/collate/build",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/collate/build::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/collate/build/order.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/collate/build/colelem.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/collate/build/trie.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/collate/build/contract.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/collate/build/builder.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/collate/build/table.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 6,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/collate/build::PackageName"
                              }
                            ],
                            "numFiles": 7,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/collate::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/cldr",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/norm",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/rangetable",
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/bidi"
                            ],
                            "subDirectories": [
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/cldr",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/cldr::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/cldr/slice.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/cldr/cldr.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/cldr/decode.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/cldr/xml.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/cldr/base.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/cldr/collate.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/cldr/makexml.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/cldr/resolve.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 8,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/cldr::PackageName"
                              },
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/norm",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/norm::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/norm/iter.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/norm/tables9.0.0.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/norm/input.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/norm/readwriter.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/norm/transform.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/norm/forminfo.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/norm/triegen.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/norm/normalize.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/norm/trie.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/norm/composition.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/norm/maketables.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/norm/tables10.0.0.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 12,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/norm::PackageName"
                              },
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/rangetable",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/rangetable::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/rangetable/tables9.0.0.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/rangetable/gen.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/rangetable/merge.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/rangetable/rangetable.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/rangetable/tables10.0.0.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 5,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/rangetable::PackageName"
                              },
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/bidi",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/bidi::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/bidi/tables9.0.0.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/bidi/gen_trieval.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/bidi/trieval.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/bidi/bidi.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/bidi/core.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/bidi/gen.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/bidi/prop.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/bidi/gen_ranges.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/bidi/bracket.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/bidi/tables10.0.0.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 10,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode/bidi::PackageName"
                              }
                            ],
                            "numFiles": 4,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/unicode::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/secure",
                            "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/secure::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/secure/bidirule"
                            ],
                            "subDirectories": [
                              {
                                "path": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/secure/bidirule",
                                "instanceName": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/secure/bidirule::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/secure/bidirule/bidirule9.0.0.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/secure/bidirule/bidirule.go",
                                  "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/secure/bidirule/bidirule10.0.0.go"
                                ],
                                "subDirectories": [],
                                "numFiles": 3,
                                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/secure/bidirule::PackageName"
                              }
                            ],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text/secure::PackageName"
                          }
                        ],
                        "numFiles": 10,
                        "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x/text::PackageName"
                      }
                    ],
                    "numFiles": 2,
                    "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org/x::PackageName"
                  }
                ],
                "numFiles": 1,
                "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor/golang.org::PackageName"
              }
            ],
            "numFiles": 3,
            "package_name": "C:/dev/DeathStarBench/hotelReservation/vendor::PackageName"
          }
        ],
        "numFiles": 17,
        "package_name": "C:/dev/DeathStarBench/hotelReservation::PackageName"
      },
      {
        "path": "C:/dev/DeathStarBench/mediaMicroservices",
        "instanceName": "C:/dev/DeathStarBench/mediaMicroservices::DirectoryComponent",
        "instanceType": "DIRECTORYCOMPONENT",
        "files": [
          "C:/dev/DeathStarBench/mediaMicroservices/docker",
          "C:/dev/DeathStarBench/mediaMicroservices/third_party",
          "C:/dev/DeathStarBench/mediaMicroservices/src",
          "C:/dev/DeathStarBench/mediaMicroservices/datasets",
          "C:/dev/DeathStarBench/mediaMicroservices/wrk2",
          "C:/dev/DeathStarBench/mediaMicroservices/docker-compose.yml",
          "C:/dev/DeathStarBench/mediaMicroservices/README.md",
          "C:/dev/DeathStarBench/mediaMicroservices/CMakeLists.txt",
          "C:/dev/DeathStarBench/mediaMicroservices/gen-py",
          "C:/dev/DeathStarBench/mediaMicroservices/.dockerignore",
          "C:/dev/DeathStarBench/mediaMicroservices/scripts",
          "C:/dev/DeathStarBench/mediaMicroservices/cmake",
          "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server",
          "C:/dev/DeathStarBench/mediaMicroservices/config",
          "C:/dev/DeathStarBench/mediaMicroservices/media_service.thrift",
          "C:/dev/DeathStarBench/mediaMicroservices/gen-lua",
          "C:/dev/DeathStarBench/mediaMicroservices/test",
          "C:/dev/DeathStarBench/mediaMicroservices/Dockerfile",
          "C:/dev/DeathStarBench/mediaMicroservices/docker-compose-sharding.yml",
          "C:/dev/DeathStarBench/mediaMicroservices/openshift",
          "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp"
        ],
        "subDirectories": [
          {
            "path": "C:/dev/DeathStarBench/mediaMicroservices/docker",
            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/docker::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/mediaMicroservices/docker/mcrouter",
              "C:/dev/DeathStarBench/mediaMicroservices/docker/wrk2-opentracing",
              "C:/dev/DeathStarBench/mediaMicroservices/docker/thrift-microservice-deps",
              "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift"
            ],
            "subDirectories": [
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/docker/mcrouter",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/docker/mcrouter::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/docker/mcrouter/Dockerfile"
                ],
                "subDirectories": [],
                "numFiles": 1,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/docker/mcrouter::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/docker/wrk2-opentracing",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/docker/wrk2-opentracing::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/docker/wrk2-opentracing/Dockerfile"
                ],
                "subDirectories": [],
                "numFiles": 1,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/docker/wrk2-opentracing::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/docker/thrift-microservice-deps",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/docker/thrift-microservice-deps::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/docker/thrift-microservice-deps/cpp",
                  "C:/dev/DeathStarBench/mediaMicroservices/docker/thrift-microservice-deps/python"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/mediaMicroservices/docker/thrift-microservice-deps/cpp",
                    "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/docker/thrift-microservice-deps/cpp::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/thrift-microservice-deps/cpp/README.md",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/thrift-microservice-deps/cpp/Dockerfile"
                    ],
                    "subDirectories": [],
                    "numFiles": 2,
                    "package_name": "C:/dev/DeathStarBench/mediaMicroservices/docker/thrift-microservice-deps/cpp::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/mediaMicroservices/docker/thrift-microservice-deps/python",
                    "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/docker/thrift-microservice-deps/python::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/thrift-microservice-deps/python/Dockerfile-py"
                    ],
                    "subDirectories": [],
                    "numFiles": 1,
                    "package_name": "C:/dev/DeathStarBench/mediaMicroservices/docker/thrift-microservice-deps/python::PackageName"
                  }
                ],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/docker/thrift-microservice-deps::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer",
                  "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/travis.yml.etlua",
                  "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/README.md",
                  "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/appveyor.yml",
                  "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/.dockerignore",
                  "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/COPYRIGHT",
                  "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-json",
                  "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/nginx.conf",
                  "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/gen_travis.lua",
                  "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/AUTHORS.md",
                  "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/nginx.vh.default.conf",
                  "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/.travis.yml",
                  "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/xenial",
                  "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/CHANGELOG.md",
                  "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer",
                    "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/example",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/src",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/.circleci",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/LICENSE",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/README.md",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/CMakeLists.txt",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/scripts",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/.clang-format",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/test",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/ci"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/example",
                        "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/example::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/example/hello_server",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/example/tutorial"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/example/hello_server",
                            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/example/hello_server::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/example/hello_server/jaeger",
                              "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/example/hello_server/README.md"
                            ],
                            "subDirectories": [
                              {
                                "path": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/example/hello_server/jaeger",
                                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/example/hello_server/jaeger::DirectoryComponent",
                                "instanceType": "DIRECTORYCOMPONENT",
                                "files": [
                                  "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/example/hello_server/jaeger/server_hello.lua",
                                  "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/example/hello_server/jaeger/docker-compose.yaml",
                                  "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/example/hello_server/jaeger/jaeger-config.json",
                                  "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/example/hello_server/jaeger/server.lua",
                                  "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/example/hello_server/jaeger/Dockerfile"
                                ],
                                "subDirectories": [],
                                "numFiles": 5,
                                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/example/hello_server/jaeger::PackageName"
                              }
                            ],
                            "numFiles": 2,
                            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/example/hello_server::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/example/tutorial",
                            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/example/tutorial::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/example/tutorial/README.md",
                              "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/example/tutorial/tutorial.lua"
                            ],
                            "subDirectories": [],
                            "numFiles": 2,
                            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/example/tutorial::PackageName"
                          }
                        ],
                        "numFiles": 2,
                        "package_name": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/example::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/src",
                        "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/src::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/src/lua_tracer.cpp",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/src/lua_tracer.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/src/carrier.cpp",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/src/lua_span.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/src/dynamic_tracer.cpp",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/src/lua_span_context.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/src/lua_span.cpp",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/src/dynamic_tracer.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/src/lua_class_description.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/src/lua_span_context.cpp",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/src/module.cpp",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/src/carrier.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/src/utility.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/src/utility.cpp"
                        ],
                        "subDirectories": [],
                        "numFiles": 14,
                        "package_name": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/src::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/.circleci",
                        "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/.circleci::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/.circleci/config.yml"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/.circleci::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/scripts",
                        "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/scripts::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/scripts/run_clang_format.sh"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/scripts::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/test",
                        "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/test::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/test/tracer.lua"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/test::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/ci",
                        "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/ci::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/ci/setup_build_environment.sh",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/ci/install_lua.sh",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/ci/install_opentracing.sh",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/ci/install_rocks.sh",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/ci/Dockerfile",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/ci/do_ci.sh",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/ci/run_docker.sh"
                        ],
                        "subDirectories": [],
                        "numFiles": 7,
                        "package_name": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer/ci::PackageName"
                      }
                    ],
                    "numFiles": 10,
                    "package_name": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-bridge-tracer::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-json",
                    "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-json::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-json/json.lua"
                    ],
                    "subDirectories": [],
                    "numFiles": 1,
                    "package_name": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-json::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/xenial",
                    "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/xenial::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/xenial/Dockerfile"
                    ],
                    "subDirectories": [],
                    "numFiles": 1,
                    "package_name": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/xenial::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift",
                    "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/src",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/TBufferedTransport.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/RpcClientFactory.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/TFramedTransport.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/TJsonProtocol.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/TCompactProtocol.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/GenericObjectPool.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/Thrift.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/TSocket.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/RpcClient.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/Object.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/THttpTransport.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/TTransport.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/Makefile.am",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/TMemoryBuffer.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/TProtocol.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/coding_standards.md",
                      "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/TBinaryProtocol.lua"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/src",
                        "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/src::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/src/longnumberutils.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/src/usocket.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/src/luasocket.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/src/lualongnumber.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/src/luabitwise.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/src/Makefile",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/src/socket.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/src/luabpack.c"
                        ],
                        "subDirectories": [],
                        "numFiles": 8,
                        "package_name": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift/src::PackageName"
                      }
                    ],
                    "numFiles": 18,
                    "package_name": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift/lua-thrift::PackageName"
                  }
                ],
                "numFiles": 15,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/docker/openresty-thrift::PackageName"
              }
            ],
            "numFiles": 4,
            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/docker::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/mediaMicroservices/third_party",
            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/third_party::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/mediaMicroservices/third_party/PicoSHA2"
            ],
            "subDirectories": [
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/third_party/PicoSHA2",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/third_party/PicoSHA2::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/third_party/PicoSHA2/picosha2.h",
                  "C:/dev/DeathStarBench/mediaMicroservices/third_party/PicoSHA2/LICENSE",
                  "C:/dev/DeathStarBench/mediaMicroservices/third_party/PicoSHA2/README.md"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/third_party/PicoSHA2::PackageName"
              }
            ],
            "numFiles": 1,
            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/third_party::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/mediaMicroservices/src",
            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/src::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/mediaMicroservices/src/UserReviewService",
              "C:/dev/DeathStarBench/mediaMicroservices/src/tracing.h",
              "C:/dev/DeathStarBench/mediaMicroservices/src/PlotService",
              "C:/dev/DeathStarBench/mediaMicroservices/src/RatingService",
              "C:/dev/DeathStarBench/mediaMicroservices/src/utils_mongodb.h",
              "C:/dev/DeathStarBench/mediaMicroservices/src/utils.h",
              "C:/dev/DeathStarBench/mediaMicroservices/src/UniqueIdService",
              "C:/dev/DeathStarBench/mediaMicroservices/src/PageService",
              "C:/dev/DeathStarBench/mediaMicroservices/src/logger.h",
              "C:/dev/DeathStarBench/mediaMicroservices/src/CMakeLists.txt",
              "C:/dev/DeathStarBench/mediaMicroservices/src/utils_memcached.h",
              "C:/dev/DeathStarBench/mediaMicroservices/src/UserService",
              "C:/dev/DeathStarBench/mediaMicroservices/src/GenericClient.h",
              "C:/dev/DeathStarBench/mediaMicroservices/src/MovieReviewService",
              "C:/dev/DeathStarBench/mediaMicroservices/src/MovieInfoService",
              "C:/dev/DeathStarBench/mediaMicroservices/src/ThriftClient.h",
              "C:/dev/DeathStarBench/mediaMicroservices/src/MovieIdService",
              "C:/dev/DeathStarBench/mediaMicroservices/src/TextService",
              "C:/dev/DeathStarBench/mediaMicroservices/src/ReviewStorageService",
              "C:/dev/DeathStarBench/mediaMicroservices/src/ComposeReviewService",
              "C:/dev/DeathStarBench/mediaMicroservices/src/RedisClient.h",
              "C:/dev/DeathStarBench/mediaMicroservices/src/CastInfoService",
              "C:/dev/DeathStarBench/mediaMicroservices/src/ClientPool.h"
            ],
            "subDirectories": [
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/src/UserReviewService",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/src/UserReviewService::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/src/UserReviewService/CMakeLists.txt",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/UserReviewService/UserReviewHandler.h",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/UserReviewService/UserReviewService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/src/UserReviewService::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/src/PlotService",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/src/PlotService::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/src/PlotService/PlotHandler.h",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/PlotService/CMakeLists.txt",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/PlotService/PlotService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/src/PlotService::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/src/RatingService",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/src/RatingService::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/src/RatingService/CMakeLists.txt",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/RatingService/RatingService.cpp",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/RatingService/RatingHandler.h"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/src/RatingService::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/src/UniqueIdService",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/src/UniqueIdService::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/src/UniqueIdService/CMakeLists.txt",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/UniqueIdService/UniqueIdHandler.h",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/UniqueIdService/UniqueIdService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/src/UniqueIdService::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/src/PageService",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/src/PageService::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/src/PageService/CMakeLists.txt",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/PageService/PageService.cpp",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/PageService/PageHandler.h"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/src/PageService::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/src/UserService",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/src/UserService::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/src/UserService/UserHandler.h",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/UserService/CMakeLists.txt",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/UserService/UserService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/src/UserService::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/src/MovieReviewService",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/src/MovieReviewService::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/src/MovieReviewService/MovieReviewService.cpp",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/MovieReviewService/CMakeLists.txt",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/MovieReviewService/MovieReviewHandler.h"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/src/MovieReviewService::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/src/MovieInfoService",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/src/MovieInfoService::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/src/MovieInfoService/CMakeLists.txt",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/MovieInfoService/MovieInfoService.cpp",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/MovieInfoService/MovieInfoHandler.h"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/src/MovieInfoService::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/src/MovieIdService",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/src/MovieIdService::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/src/MovieIdService/CMakeLists.txt",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/MovieIdService/MovieIdHandler.h",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/MovieIdService/MovieIdService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/src/MovieIdService::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/src/TextService",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/src/TextService::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/src/TextService/TextService.cpp",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/TextService/CMakeLists.txt",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/TextService/TextHandler.h"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/src/TextService::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/src/ReviewStorageService",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/src/ReviewStorageService::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/src/ReviewStorageService/CMakeLists.txt",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/ReviewStorageService/ReviewStorageHandler.h",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/ReviewStorageService/ReviewStorageService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/src/ReviewStorageService::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/src/ComposeReviewService",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/src/ComposeReviewService::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/src/ComposeReviewService/CMakeLists.txt",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/ComposeReviewService/ComposeReviewHandler.h",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/ComposeReviewService/ComposeReviewService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/src/ComposeReviewService::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/src/CastInfoService",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/src/CastInfoService::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/src/CastInfoService/CastInfoService.cpp",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/CastInfoService/CastInfoHandler.h",
                  "C:/dev/DeathStarBench/mediaMicroservices/src/CastInfoService/CMakeLists.txt"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/src/CastInfoService::PackageName"
              }
            ],
            "numFiles": 23,
            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/src::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/mediaMicroservices/datasets",
            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/datasets::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/mediaMicroservices/datasets/tmdb"
            ],
            "subDirectories": [
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/datasets/tmdb",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/datasets/tmdb::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/datasets/tmdb/casts.json",
                  "C:/dev/DeathStarBench/mediaMicroservices/datasets/tmdb/movies.json",
                  "C:/dev/DeathStarBench/mediaMicroservices/datasets/tmdb/get_casts.py",
                  "C:/dev/DeathStarBench/mediaMicroservices/datasets/tmdb/get_movies.py"
                ],
                "subDirectories": [],
                "numFiles": 4,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/datasets/tmdb::PackageName"
              }
            ],
            "numFiles": 1,
            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/datasets::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/mediaMicroservices/wrk2",
            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/wrk2::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/SCRIPTING",
              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src",
              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/wrk",
              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/LICENSE",
              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/obj",
              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/README.md",
              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/CoordinatedOmission",
              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/gen_path.py",
              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/scripts",
              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps",
              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/run.sh",
              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/Makefile",
              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/NOTICE"
            ],
            "subDirectories": [
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/wrk.c",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/zmalloc.h",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/aprintf.c",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/tinymt64.c",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/main.h",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/config.h",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/hdr_histogram.h",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/ae_kqueue.c",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/ssl.c",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/zmalloc.c",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/script.h",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/wrk.h",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/stats.c",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/units.c",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/ae_evport.c",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/http_parser.c",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/hdr_histogram.c",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/ae.h",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/http_parser.h",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/units.h",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/wrk.lua",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/tinymt64.h",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/ae_epoll.c",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/script.c",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/aprintf.h",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/ssl.h",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/ae.c",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/net.c",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/stats.h",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/ae_select.c",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src/net.h"
                ],
                "subDirectories": [],
                "numFiles": 31,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/src::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/obj",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/obj::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/obj/net.o",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/obj/script.o",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/obj/hdr_histogram.o",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/obj/bytecode.o",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/obj/tinymt64.o",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/obj/units.o",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/obj/stats.o",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/obj/ssl.o",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/obj/http_parser.o",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/obj/ae.o",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/obj/zmalloc.o",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/obj/wrk.o",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/obj/aprintf.o"
                ],
                "subDirectories": [],
                "numFiles": 13,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/obj::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/CoordinatedOmission",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/CoordinatedOmission::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/CoordinatedOmission/wrk2_CleanVsCO.png"
                ],
                "subDirectories": [],
                "numFiles": 1,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/CoordinatedOmission::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/scripts",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/scripts::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/scripts/pipeline.lua",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/scripts/media-microservices",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/scripts/setup.lua",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/scripts/addr.lua",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/scripts/post.lua",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/scripts/stop.lua",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/scripts/report.lua",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/scripts/auth.lua",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/scripts/multiplepaths.lua",
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/scripts/counter.lua"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/scripts/media-microservices",
                    "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/scripts/media-microservices::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/mediaMicroservices/wrk2/scripts/media-microservices/compose-review.lua"
                    ],
                    "subDirectories": [],
                    "numFiles": 1,
                    "package_name": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/scripts/media-microservices::PackageName"
                  }
                ],
                "numFiles": 10,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/scripts::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit",
                    "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src",
                      "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc",
                      "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/dynasm",
                      "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/etc",
                      "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/COPYRIGHT",
                      "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/Makefile",
                      "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/README"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src",
                        "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_parse.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj.supp",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_jit.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_ffi.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_load.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_trace.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_ccallback.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_opt_mem.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_snap.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/luaconf.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_asm_x86.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_traceerr.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_obj.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_target_mips.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_opt_dce.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_cparse.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_ir.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_asm.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_lex.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_udata.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_obj.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_def.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_alloc.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_opt_fold.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_errmsg.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_ffi.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_clib.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_state.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_table.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_cconv.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_init.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_record.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_err.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_string.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/vm_ppc.dasc",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/host",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/msvcbuild.bat",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_err.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_base.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_dispatch.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_ff.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_vmevent.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_debug.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_emit_x86.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_emit_arm.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_cparse.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_strscan.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/luajit",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_ffdef.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_trace.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_base.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/ljamalg.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_char.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/xedkbuild.bat",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_lex.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_func.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_ircall.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_obj.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_bcwrite.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_parse.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_target.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_ffrecord.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_meta.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_char.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_gdbjit.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_gdbjit.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_clib.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_ffrecord.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_load.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_debug.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_bcread.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_meta.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_lex.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_io.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_record.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_emit_mips.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_bc.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_alloc.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_iropt.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_bcread.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_libdef.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_vmmath.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_err.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_carith.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_func.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_gdbjit.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_api.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_parse.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_os.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_gc.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_debug.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_lib.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_state.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_table.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/Makefile.dep",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_meta.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_opt_fold.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/vm_mips.dasc",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_target_ppc.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_crecord.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_io.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_lib.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_bcdef.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_target_x86.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_api.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_ccall.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_opt_split.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_char.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_folddef.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_cconv.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_vm.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_bc.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_asm.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_bc.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_alloc.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_cdata.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_string.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_vmmath.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_ccall.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_ffrecord.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_jit.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lua.hpp",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_clib.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/vm_ppcspe.dasc",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_opt_narrow.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_cparse.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_ccallback.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_strscan.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_cdata.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_bcdump.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_asm.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_gc.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/luajit.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_vmevent.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_func.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_udata.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_bit.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_target_arm.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_opt_mem.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_math.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lauxlib.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_lib.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_frame.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_cconv.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_gc.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_recdef.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_bcwrite.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_opt_loop.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_asm_mips.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/libluajit.a",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_crecord.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_mcode.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_package.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_vm.s",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_asm_ppc.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_package.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_opt_dce.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_vmevent.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_str.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_str.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_str.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_ctype.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_snap.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_ccallback.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_jit.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lualib.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_aux.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_asm_arm.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_ir.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_cdata.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_debug.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_debug.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_udata.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/Makefile",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_tab.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/luajit.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_tab.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_snap.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_opt_loop.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_arch.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_dispatch.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_vm.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/luajit.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_opt_sink.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_strscan.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_ctype.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_mcode.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_aux.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_state.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_dispatch.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_ctype.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lua.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_record.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_opt_sink.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/vm_arm.dasc",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_emit_ppc.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_carith.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_ir.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_ccall.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/vm_x86.dasc",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/ps4build.bat",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/jit",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_bit.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_trace.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_carith.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_mcode.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_init.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_math.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_tab.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_opt_split.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lib_os.o",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_opt_narrow.c",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/lj_crecord.h"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/host",
                            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/host::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/host/buildvm.c",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/host/buildvm_fold.o",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/host/minilua.c",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/host/minilua",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/host/buildvm_fold.c",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/host/minilua.o",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/host/buildvm_asm.o",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/host/buildvm_peobj.o",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/host/buildvm_arch.h",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/host/buildvm_asm.c",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/host/buildvm_lib.c",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/host/genminilua.lua",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/host/buildvm",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/host/buildvm.o",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/host/buildvm.h",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/host/buildvm_peobj.c",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/host/buildvm_lib.o",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/host/README"
                            ],
                            "subDirectories": [],
                            "numFiles": 18,
                            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/host::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/jit",
                            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/jit::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/jit/dis_ppc.lua",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/jit/v.lua",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/jit/dis_mips.lua",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/jit/bc.lua",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/jit/dis_arm.lua",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/jit/bcsave.lua",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/jit/dis_x86.lua",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/jit/dis_mipsel.lua",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/jit/vmdef.lua",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/jit/dump.lua",
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/jit/dis_x64.lua"
                            ],
                            "subDirectories": [],
                            "numFiles": 11,
                            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src/jit::PackageName"
                          }
                        ],
                        "numFiles": 213,
                        "package_name": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/src::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc",
                        "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc/bluequad-print.css",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc/extensions.html",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc/ext_ffi.html",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc/changes.html",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc/ext_c_api.html",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc/install.html",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc/ext_jit.html",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc/running.html",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc/status.html",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc/faq.html",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc/contact.html",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc/ext_ffi_tutorial.html",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc/luajit.html",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc/ext_ffi_api.html",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc/ext_ffi_semantics.html",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc/img",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc/bluequad.css"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc/img",
                            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc/img::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc/img/contact.png"
                            ],
                            "subDirectories": [],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc/img::PackageName"
                          }
                        ],
                        "numFiles": 17,
                        "package_name": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/doc::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/dynasm",
                        "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/dynasm::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/dynasm/dasm_mips.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/dynasm/dasm_x86.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/dynasm/dasm_x86.lua",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/dynasm/dynasm.lua",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/dynasm/dasm_x64.lua",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/dynasm/dasm_ppc.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/dynasm/dasm_ppc.lua",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/dynasm/dasm_proto.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/dynasm/dasm_arm.h",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/dynasm/dasm_mips.lua",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/dynasm/dasm_arm.lua"
                        ],
                        "subDirectories": [],
                        "numFiles": 11,
                        "package_name": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/dynasm::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/etc",
                        "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/etc::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/etc/luajit.1",
                          "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/etc/luajit.pc"
                        ],
                        "subDirectories": [],
                        "numFiles": 2,
                        "package_name": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit/etc::PackageName"
                      }
                    ],
                    "numFiles": 7,
                    "package_name": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps/luajit::PackageName"
                  }
                ],
                "numFiles": 1,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/wrk2/deps::PackageName"
              }
            ],
            "numFiles": 13,
            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/wrk2::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/mediaMicroservices/gen-py",
            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/gen-py::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-py/__init__.py"
            ],
            "subDirectories": [
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/TextService-remote",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/MovieReviewService-remote",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/MovieIdService-remote",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/UserService-remote",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/UniqueIdService.py",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/CastInfoService.py",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/RatingService.py",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/ReviewStorageService-remote",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/MovieIdService.py",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/TextService.py",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/UserService.py",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/UserReviewService.py",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/MovieInfoService.py",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/PageService-remote",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/PlotService-remote",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/PageService.py",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/UserReviewService-remote",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/constants.py",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/UniqueIdService-remote",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/MovieInfoService-remote",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/PlotService.py",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/ComposeReviewService-remote",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/ReviewStorageService.py",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/ttypes.py",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/MovieReviewService.py",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/ComposeReviewService.py",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/__init__.py",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/CastInfoService-remote",
                  "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service/RatingService-remote"
                ],
                "subDirectories": [],
                "numFiles": 29,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/gen-py/media_service::PackageName"
              }
            ],
            "numFiles": 2,
            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/gen-py::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/mediaMicroservices/scripts",
            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/scripts::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/mediaMicroservices/scripts/compose_review.sh",
              "C:/dev/DeathStarBench/mediaMicroservices/scripts/register_users.sh",
              "C:/dev/DeathStarBench/mediaMicroservices/scripts/write_movie_info.py",
              "C:/dev/DeathStarBench/mediaMicroservices/scripts/mongodb_bootstrap.sh",
              "C:/dev/DeathStarBench/mediaMicroservices/scripts/register_movies.sh",
              "C:/dev/DeathStarBench/mediaMicroservices/scripts/gen_mongo_configs.py"
            ],
            "subDirectories": [],
            "numFiles": 6,
            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/scripts::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/mediaMicroservices/cmake",
            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/cmake::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/mediaMicroservices/cmake/Findlibmemcached.cmake",
              "C:/dev/DeathStarBench/mediaMicroservices/cmake/Findthrift.cmake"
            ],
            "subDirectories": [],
            "numFiles": 2,
            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/cmake::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server",
            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/jaeger-config.json",
              "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/conf",
              "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts"
            ],
            "subDirectories": [
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/conf",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/conf::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/conf/nginx.conf"
                ],
                "subDirectories": [],
                "numFiles": 1,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/conf::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/test.lua",
                  "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api",
                    "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/review",
                      "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/plot",
                      "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/movie-info",
                      "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/movie",
                      "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/user",
                      "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/cast-info"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/review",
                        "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/review::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/review/compose.lua"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/review::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/plot",
                        "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/plot::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/plot/write.lua"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/plot::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/movie-info",
                        "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/movie-info::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/movie-info/write.lua"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/movie-info::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/movie",
                        "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/movie::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/movie/register.lua"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/movie::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/user",
                        "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/user::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/user/register.lua"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/user::PackageName"
                      },
                      {
                        "path": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/cast-info",
                        "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/cast-info::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/cast-info/write.lua"
                        ],
                        "subDirectories": [],
                        "numFiles": 1,
                        "package_name": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api/cast-info::PackageName"
                      }
                    ],
                    "numFiles": 6,
                    "package_name": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts/wrk2-api::PackageName"
                  }
                ],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server/lua-scripts::PackageName"
              }
            ],
            "numFiles": 3,
            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/nginx-web-server::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/mediaMicroservices/config",
            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/config::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/mediaMicroservices/config/jaeger-config.yml",
              "C:/dev/DeathStarBench/mediaMicroservices/config/mongodb",
              "C:/dev/DeathStarBench/mediaMicroservices/config/service-config.json"
            ],
            "subDirectories": [
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/config/mongodb",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/config/mongodb::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/config/mongodb/movie-id",
                  "C:/dev/DeathStarBench/mediaMicroservices/config/mongodb/user"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/mediaMicroservices/config/mongodb/movie-id",
                    "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/config/mongodb/movie-id::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/mediaMicroservices/config/mongodb/movie-id/init-shard_1.js",
                      "C:/dev/DeathStarBench/mediaMicroservices/config/mongodb/movie-id/init-shard_0.js",
                      "C:/dev/DeathStarBench/mediaMicroservices/config/mongodb/movie-id/init-config.js",
                      "C:/dev/DeathStarBench/mediaMicroservices/config/mongodb/movie-id/init-router.js",
                      "C:/dev/DeathStarBench/mediaMicroservices/config/mongodb/movie-id/init-shard_2.js"
                    ],
                    "subDirectories": [],
                    "numFiles": 5,
                    "package_name": "C:/dev/DeathStarBench/mediaMicroservices/config/mongodb/movie-id::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/mediaMicroservices/config/mongodb/user",
                    "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/config/mongodb/user::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/mediaMicroservices/config/mongodb/user/init-shard_3.js",
                      "C:/dev/DeathStarBench/mediaMicroservices/config/mongodb/user/init-shard_1.js",
                      "C:/dev/DeathStarBench/mediaMicroservices/config/mongodb/user/init-config.js",
                      "C:/dev/DeathStarBench/mediaMicroservices/config/mongodb/user/init-router.js",
                      "C:/dev/DeathStarBench/mediaMicroservices/config/mongodb/user/init-shard_2.js"
                    ],
                    "subDirectories": [],
                    "numFiles": 5,
                    "package_name": "C:/dev/DeathStarBench/mediaMicroservices/config/mongodb/user::PackageName"
                  }
                ],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/config/mongodb::PackageName"
              }
            ],
            "numFiles": 3,
            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/config::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/mediaMicroservices/gen-lua",
            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/gen-lua::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/mediaMicroservices/gen-lua/media_service_MovieInfoService.lua",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-lua/media_service_ttypes.lua",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-lua/media_service_TextService.lua",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-lua/media_service_RatingService.lua",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-lua/media_service_PageService.lua",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-lua/media_service_PlotService.lua",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-lua/media_service_CastInfoService.lua",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-lua/media_service_UserReviewService.lua",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-lua/media_service_MovieIdService.lua",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-lua/media_service_constants.lua",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-lua/media_service_UniqueIdService.lua",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-lua/media_service_UserService.lua",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-lua/media_service_MovieReviewService.lua",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-lua/media_service_ComposeReviewService.lua",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-lua/media_service_ReviewStorageService.lua"
            ],
            "subDirectories": [],
            "numFiles": 15,
            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/gen-lua::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/mediaMicroservices/test",
            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/test::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/mediaMicroservices/test/testComposeReviewServiceE2E.py",
              "C:/dev/DeathStarBench/mediaMicroservices/test/testMovieInfoService.py",
              "C:/dev/DeathStarBench/mediaMicroservices/test/testMovieIdService.py",
              "C:/dev/DeathStarBench/mediaMicroservices/test/testMovieReviewService.py",
              "C:/dev/DeathStarBench/mediaMicroservices/test/CMakeLists.txt",
              "C:/dev/DeathStarBench/mediaMicroservices/test/testPlotService.py",
              "C:/dev/DeathStarBench/mediaMicroservices/test/testPageService.py",
              "C:/dev/DeathStarBench/mediaMicroservices/test/testComposeReviewService.py",
              "C:/dev/DeathStarBench/mediaMicroservices/test/testTextService.py",
              "C:/dev/DeathStarBench/mediaMicroservices/test/testUserReviewService.py",
              "C:/dev/DeathStarBench/mediaMicroservices/test/testMemcachedAtomicIncrement.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/test/testReviewStorageService.py",
              "C:/dev/DeathStarBench/mediaMicroservices/test/testRatingService.py",
              "C:/dev/DeathStarBench/mediaMicroservices/test/testClientPool.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/test/testUniqueIdService.py",
              "C:/dev/DeathStarBench/mediaMicroservices/test/testUserService.py",
              "C:/dev/DeathStarBench/mediaMicroservices/test/testCastInfoService.py"
            ],
            "subDirectories": [],
            "numFiles": 17,
            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/test::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/mediaMicroservices/openshift",
            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/openshift::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/media-microsvc-ns.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/mms-client.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/plot-memcached.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/cast-info-mongodb.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/networking",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/unique-id-service.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/compose-review-service.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/movie-info-service.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/rating-redis.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/user-review-mongodb.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/user-mongodb.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/README.md",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/review-storage-service.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/movie-review-mongodb.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/movie-info-memcached.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/jaeger.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/user-review-service.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/plot-service.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/movie-id-service.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/movie-id-memcached.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/movie-review-service.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/plot-mongodb.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/cast-info-service.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/cast-info-memcached.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/review-storage-memcached.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/movie-id-mongodb.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/compose-review-memcached.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/rating-service.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/nginx-web-server.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/user-service.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/review-storage-mongodb.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/user-memcached.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/movie-info-mongodb.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/movie-review-redis.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/text-service.yaml",
              "C:/dev/DeathStarBench/mediaMicroservices/openshift/user-review-redis.yaml"
            ],
            "subDirectories": [
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/openshift/networking",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/openshift/networking::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/openshift/networking/istio-gateway",
                  "C:/dev/DeathStarBench/mediaMicroservices/openshift/networking/destination-rule-all.yaml"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/mediaMicroservices/openshift/networking/istio-gateway",
                    "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/openshift/networking/istio-gateway::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/networking/istio-gateway/mediamicrosvc-gateway.yaml"
                    ],
                    "subDirectories": [],
                    "numFiles": 1,
                    "package_name": "C:/dev/DeathStarBench/mediaMicroservices/openshift/networking/istio-gateway::PackageName"
                  }
                ],
                "numFiles": 2,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/openshift/networking::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/deploy-all-services-and-configurations.sh",
                  "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/zap.sh",
                  "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/update-all.sh",
                  "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/create-destination-rule-all.sh",
                  "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/create-all-micro-services.sh",
                  "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/create-istio-gateways.sh",
                  "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/update-micro-service.sh",
                  "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/util",
                  "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/restartall.sh",
                  "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/configmaps",
                  "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/update-all-micro-services.sh",
                  "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/streamlogs.sh",
                  "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/create-all-configmap.sh",
                  "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/update-destination-rule-all.sh",
                  "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/update-all-configmap.sh",
                  "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/dumplogs.sh"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/util",
                    "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/util::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/util/make-dra.sh"
                    ],
                    "subDirectories": [],
                    "numFiles": 1,
                    "package_name": "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/util::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/configmaps",
                    "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/configmaps::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/configmaps/update-configmap-gen-lua.sh",
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/configmaps/create-configmap-lua-scripts.sh",
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/configmaps/update-configmap-jaeger-config-json.sh",
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/configmaps/update-configmap-lua-scripts.sh",
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/configmaps/update-configmap-nginx-conf.sh",
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/configmaps/create-configmap-nginx-conf.sh",
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/configmaps/create-configmap-gen-lua.sh",
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/configmaps/create-configmap-jaeger-config-json.sh"
                    ],
                    "subDirectories": [],
                    "numFiles": 8,
                    "package_name": "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts/configmaps::PackageName"
                  }
                ],
                "numFiles": 16,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/openshift/scripts::PackageName"
              },
              {
                "path": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps",
                "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps::DirectoryComponent",
                "instanceType": "DIRECTORYCOMPONENT",
                "files": [
                  "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/jaeger-config.json",
                  "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/nginx.conf",
                  "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts",
                  "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/gen-lua"
                ],
                "subDirectories": [
                  {
                    "path": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts",
                    "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/test.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api"
                    ],
                    "subDirectories": [
                      {
                        "path": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api",
                        "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api::DirectoryComponent",
                        "instanceType": "DIRECTORYCOMPONENT",
                        "files": [
                          "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/review",
                          "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/plot",
                          "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/movie-info",
                          "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/movie",
                          "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/user",
                          "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/cast-info"
                        ],
                        "subDirectories": [
                          {
                            "path": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/review",
                            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/review::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/review/compose.lua"
                            ],
                            "subDirectories": [],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/review::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/plot",
                            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/plot::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/plot/write.lua"
                            ],
                            "subDirectories": [],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/plot::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/movie-info",
                            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/movie-info::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/movie-info/write.lua"
                            ],
                            "subDirectories": [],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/movie-info::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/movie",
                            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/movie::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/movie/register.lua"
                            ],
                            "subDirectories": [],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/movie::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/user",
                            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/user::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/user/register.lua"
                            ],
                            "subDirectories": [],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/user::PackageName"
                          },
                          {
                            "path": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/cast-info",
                            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/cast-info::DirectoryComponent",
                            "instanceType": "DIRECTORYCOMPONENT",
                            "files": [
                              "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/cast-info/write.lua"
                            ],
                            "subDirectories": [],
                            "numFiles": 1,
                            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api/cast-info::PackageName"
                          }
                        ],
                        "numFiles": 6,
                        "package_name": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts/wrk2-api::PackageName"
                      }
                    ],
                    "numFiles": 2,
                    "package_name": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/lua-scripts::PackageName"
                  },
                  {
                    "path": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/gen-lua",
                    "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/gen-lua::DirectoryComponent",
                    "instanceType": "DIRECTORYCOMPONENT",
                    "files": [
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/gen-lua/media_service_MovieInfoService.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/gen-lua/media_service_ttypes.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/gen-lua/media_service_TextService.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/gen-lua/media_service_RatingService.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/gen-lua/media_service_PageService.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/gen-lua/media_service_PlotService.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/gen-lua/media_service_CastInfoService.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/gen-lua/media_service_UserReviewService.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/gen-lua/media_service_MovieIdService.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/gen-lua/media_service_constants.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/gen-lua/media_service_UniqueIdService.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/gen-lua/media_service_UserService.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/gen-lua/media_service_MovieReviewService.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/gen-lua/media_service_ComposeReviewService.lua",
                      "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/gen-lua/media_service_ReviewStorageService.lua"
                    ],
                    "subDirectories": [],
                    "numFiles": 15,
                    "package_name": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps/gen-lua::PackageName"
                  }
                ],
                "numFiles": 4,
                "package_name": "C:/dev/DeathStarBench/mediaMicroservices/openshift/configmaps::PackageName"
              }
            ],
            "numFiles": 38,
            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/openshift::PackageName"
          },
          {
            "path": "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp",
            "instanceName": "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp::DirectoryComponent",
            "instanceType": "DIRECTORYCOMPONENT",
            "files": [
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/RatingService.h",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/UniqueIdService.h",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/media_service_types.h",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/media_service_constants.h",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/ReviewStorageService.h",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/PageService_server.skeleton.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/UniqueIdService_server.skeleton.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/MovieReviewService.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/CastInfoService.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/UserReviewService.h",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/TextService.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/MovieInfoService_server.skeleton.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/CastInfoService_server.skeleton.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/UserReviewService_server.skeleton.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/MovieInfoService.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/MovieReviewService.h",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/MovieIdService.h",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/PlotService.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/MovieInfoService.h",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/RatingService.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/media_service_types.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/PageService.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/PlotService.h",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/MovieIdService_server.skeleton.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/UserService_server.skeleton.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/ReviewStorageService_server.skeleton.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/PlotService_server.skeleton.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/MovieReviewService_server.skeleton.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/media_service_constants.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/UserService.h",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/ComposeReviewService_server.skeleton.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/ComposeReviewService.h",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/CastInfoService.h",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/ReviewStorageService.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/UserService.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/UserReviewService.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/TextService_server.skeleton.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/RatingService_server.skeleton.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/UniqueIdService.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/TextService.h",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/PageService.h",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/MovieIdService.cpp",
              "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp/ComposeReviewService.cpp"
            ],
            "subDirectories": [],
            "numFiles": 43,
            "package_name": "C:/dev/DeathStarBench/mediaMicroservices/gen-cpp::PackageName"
          }
        ],
        "numFiles": 21,
        "package_name": "C:/dev/DeathStarBench/mediaMicroservices::PackageName"
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
    c.bench_function("LAAST", move |b| {
        b.iter(|| {
            epoch.advance().unwrap();
            eprintln!("{} alloced bytes", allocated.read().unwrap());
            let _ctx = black_box(parse_project_context(&dir)).unwrap();
            // println!("{}", x);
            // black_box(Box::leak(Box::new(1)));
            epoch.advance().unwrap();
            eprintln!("{} alloced bytes after", allocated.read().unwrap());
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
