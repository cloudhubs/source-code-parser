use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use statistical::*;

extern crate source_code_parser;
use source_code_parser::{
    msd::{run_msd_parse, Executor, NodePattern, ParserContext},
    *,
};

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

const msds_json_endpoint_simple: &'static str = r##"
[
  {
    "identifier": "ClassOrInterface",
    "pattern": "#{service_name}Handler",
    "subpatterns": [
      {
        "identifier": "Method",
        "pattern": "#{endpoint}(^[a-zA-Z]*$)",
        "subpatterns": [],
        "callback": "let endpoint = ctx.get_variable(\"endpoint\").unwrap();let service = ctx.get_variable(\"service_name\").unwrap();if (!endpoint.ends_with(\"Handler\")) { ctx.make_attribute(service, endpoint, None); }",
        "essential": true
      }
    ],
    "essential": true
  }
]
"##;

const msds_json_entity: &'static str = r##"
[
  {
    "identifier": "Method",
    "pattern": "#{container}",
    "subpatterns": [
      {
        "identifier": "CallExpr",
        "pattern": "mongoc_client_get_collection",
        "subpatterns": [
          {
            "identifier": "Literal",
            "pattern": "\"#{collection}\"",
            "subpatterns": [],
            "essential": true
          }
        ],
        "essential": true
      }
    ],
    "callback": "let container = ctx.get_variable(\"container\").unwrap();let collection = ctx.get_variable(\"collection\").unwrap();ctx.make_object(collection);ctx.make_transient(container);",
    "essential": true
  },
  {
    "identifier": "Method",
    "pattern": "#&{_method_name}",
    "subpatterns": [
      {
        "identifier": "CallExpr",
        "pattern": "mongoc_client_get_collection",
        "subpatterns": [
          {
            "identifier": "Literal",
            "pattern": "\"#{collection_name}\"",
            "subpatterns": [],
            "essential": true
          }
        ],
        "essential": true
      },
      {
        "identifier": "CallExpr",
        "pattern": "BCON_NEW",
        "subpatterns": [
          {
            "identifier": "Literal",
            "pattern": "\"#{token}(\\$?.+)\"",
            "subpatterns": [],
            "callback": "let token = ctx.get_variable(\"token\").unwrap();ctx.make_transient(\"tokens\");let tokens = ctx.get_object(\"tokens\").unwrap();let ndx = 0;while tokens.contains_key(`${ndx}`) {match tokens.get(`${ndx}`).iter().next() {Some(Some(_)) => { ndx = ndx + 1; },_ => { break; }}}ctx.make_attribute(\"tokens\", `${ndx}`, Some(token));ctx.make_attribute(\"tokens\", `${ndx+1}`, None);",
            "essential": true
          },
          {
            "identifier": "CallExpr",
            "pattern": "BCON_#{token}(.+)",
            "subpatterns": [],
            "callback": "let token = ctx.get_variable(\"token\").unwrap();ctx.make_transient(\"tokens\");let tokens = ctx.get_object(\"tokens\").unwrap();let ndx = 0;while tokens.contains_key(`${ndx}`) {match tokens.get(`${ndx}`).iter().next() {Some(Some(_)) => {ndx =  ndx + 1;},_ => { break; }}}ctx.make_attribute(\"tokens\", `${ndx}`, Some(token));ctx.make_attribute(\"tokens\", `${ndx+1}`, None);",
            "essential": false
          }
        ],
        "callback": "fn done(ndx, tokens) {match tokens.get(`${ndx}`).iter().next() {Some(Some(_)) => false,_ => true}}fn parse_pair(parent_tag, tokens, ndx, ctx) {loop {ndx = do_parse_pair(parent_tag, tokens, ndx, ctx);if done(ndx, tokens) { break; }match tokens.get(`${ndx}`).iter().next() {Some(Some(token)) => {if token == \"}\" { break; }}_ => {},}}ndx}fn do_parse_pair(parent_tag, tokens, ndx, ctx) {ndx = choose_action(parent_tag, tokens, ndx, ctx);loop {    match tokens.get(`${ndx}`).iter().next() {    Some(Some(token)) => {    if token == \"{\" {    ndx = choose_action(parent_tag, tokens, ndx + 1, ctx);    } else { break; }    }_ => return -100,    }    }let lhs = tokens.get(`${ndx}`).iter().next().unwrap().iter().next().unwrap();ndx = choose_action(`${parent_tag}.${lhs}`, tokens, ndx + 1, ctx);if done(ndx, tokens) { return -100; }let rhs = tokens.get(`${ndx}`).iter().next().unwrap().iter().next().unwrap();if rhs == \"}\" || rhs == \"]\" { return ndx; }if rhs == \"[\" { return parse_array_literal(parent_tag, tokens, ndx - 1, ctx); }ctx.make_attribute(parent_tag, lhs, Some(rhs));ndx + 1}fn do_and(parent_tag, tokens, ndx, ctx) {loop {    match tokens.get(`${ndx}`).iter().next() {    Some(Some(token)) => {    if token != \"]\" {    ndx = choose_action(parent_tag, tokens, ndx + 1, ctx);    } else {    break;    }    }_ => return -100,    }    }ndx}fn parse_array(parent_tag, tokens, ndx, ctx) {let array_name = tokens.get(`${ndx + 1}`).iter().next();array_name = match array_name {Some(Some(val)) => val,_ => return -100,};ctx.make_attribute(parent_tag, array_name, Some(\"[]\"));choose_action(`${parent_tag}.${array_name}`, tokens, ndx + 2, ctx) + 1}fn parse_array_literal(parent_tag, tokens, ndx, ctx) {let array_name = tokens.get(`${ndx}`).iter().next();array_name = match array_name {Some(Some(val)) => val,_ => return -100,};ctx.make_attribute(parent_tag, array_name, Some(\"[]\"));do_each(`${parent_tag}.${array_name}`, tokens, ndx + 2, ctx) + 1}fn do_each(parent_tag, tokens, ndx, ctx) {loop {    match tokens.get(`${ndx}`).iter().next() {    Some(Some(token)) => {    if token != \"]\" {    ndx = parse_pair(parent_tag, tokens, ndx + 1, ctx);    } else {    break;    }    }_ => return -100,    }    }    ndx}fn do_elemMatch(parent_tag, tokens, ndx, ctx) { parse_pair(parent_tag, tokens, ndx, ctx) }fn choose_action(parent, tokens, ndx, ctx) {if done(ndx, tokens) { return -1; }    match tokens.get(`${ndx}`).iter().next() {    Some(Some(token)) => match token {    \"$and\" => do_and(parent, tokens, ndx + 1, ctx),    \"$not\" => parse_pair(parent, tokens, ndx + 1, ctx),    \"$push\" => parse_array(parent, tokens, ndx + 1, ctx),    \"$pull\" => parse_array(parent, tokens, ndx + 1, ctx),    \"$each\" => do_each(parent, tokens, ndx + 1, ctx),    \"projection\" => panic(\"Unhandled\"),    \"$elemMatch\" => do_elemMatch(parent, tokens, ndx + 1, ctx),    \"{\" => parse_pair(parent, tokens, ndx + 1, ctx),    \"}\" => ndx + 1,    \"$position\" => ndx + 2,    \"$set\" => choose_action(parent, tokens, ndx + 1, ctx),    other => {    if !other.starts_with(\"$\") {    return ndx;    } else {    panic(\"Unknown command\");    }    }    }    _ => ndx    }}fn cleanup(ctx) {ctx.make_transient(\"tokens\");let tokens = ctx.get_object(\"tokens\").unwrap();let ndx = 0;while tokens.contains_key(`${ndx}`) {ctx.make_attribute(\"tokens\", `${ndx}`, None);ndx = ndx + 1;}}let tokens = ctx.get_object(\"tokens\").unwrap();let coll = ctx.get_variable(\"collection_name\").unwrap();match tokens.get(\"0\").iter().next() {Some(Some(token)) => {if token.starts_with(\"$\") || token == \"{\" || token == \"[\" {choose_action(coll, tokens, 0, ctx);} else if token != \"projection\" {parse_pair(coll, tokens, 0, ctx);} else {cleanup(ctx);panic(\"unhandled\");}}_ => {cleanup(ctx);panic(\"No tokens\");}}cleanup(ctx);",
        "essential": true
      }
    ],
    "essential": true
  }
]
"##;

const msds_json_endpoint: &'static str = r##"
[
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

"##;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn laast_benchmark(c: &mut Criterion) {
    let epoch = jemalloc_ctl::epoch::mib().unwrap();
    let allocated = jemalloc_ctl::stats::allocated::mib().unwrap();

    let dir = serde_json::from_str::<Directory>(directory_json).unwrap();
    let mut mem = vec![];
    c.bench_function("LAAST", |b| {
        b.iter(|| {
            epoch.advance().unwrap();
            let before = allocated.read().unwrap();
            let _ctx = black_box(parse_project_context(&dir)).unwrap();
            epoch.advance().unwrap();
            mem.push((allocated.read().unwrap() - before) as f64);
        })
    });
    let mean = mean(&mem);
    println!(
        "{} +/- {} ({})",
        mean,
        standard_deviation(&mem, Some(mean)),
        median(&mem)
    );
}

fn ressa_benchmark(c: &mut Criterion, name: &str, msds_json: &str) {
    let epoch = jemalloc_ctl::epoch::mib().unwrap();
    let allocated = jemalloc_ctl::stats::allocated::mib().unwrap();

    let dir = serde_json::from_str::<Directory>(directory_json).unwrap();
    let ctx = parse_project_context(&dir).unwrap();
    let msds = serde_json::from_str::<Vec<NodePattern>>(msds_json).unwrap();
    let mut mem = vec![];
    c.bench_function(name, |b| {
        b.iter(|| {
            epoch.advance().unwrap();
            let before = allocated.read().unwrap();
            let _ctx = black_box(run_msd_parse(&mut ctx.modules.clone(), msds.clone()));
            epoch.advance().unwrap();
            let after = allocated.read().unwrap();
            println!("{} - {}", after, before);
            mem.push((after - before) as f64);
        })
    });
    let mean = mean(&mem);
    println!(
        "{} +/- {} ({})",
        mean,
        standard_deviation(&mem, Some(mean)),
        median(&mem)
    );
}

fn ressa_benchmark_endpoint_simple(c: &mut Criterion) {
    ressa_benchmark(c, "RESSA Endpoint Simple", msds_json_endpoint_simple)
}

fn ressa_benchmark_endpoint(c: &mut Criterion) {
    ressa_benchmark(c, "RESSA Endpint (Call Graph)", msds_json_endpoint)
}

fn ressa_benchmark_entity(c: &mut Criterion) {
    ressa_benchmark(c, "RESSA Entity", msds_json_entity)
}

fn rune_benchmark(c: &mut Criterion) {
    let epoch = jemalloc_ctl::epoch::mib().unwrap();
    let allocated = jemalloc_ctl::stats::allocated::mib().unwrap();

    let msds = serde_json::from_str::<Vec<NodePattern>>(msds_json_endpoint_simple).unwrap();
    let msd = msds
        .get(0)
        .unwrap()
        .clone()
        .subpatterns
        .get(0)
        .unwrap()
        .clone();
    let ex = Executor::new().unwrap();
    let mut mem = vec![];
    c.bench_function("Rune", |b| {
        b.iter(|| {
            epoch.advance().unwrap();
            let before = allocated.read().unwrap();

            let _ctx = black_box(ex.execute(&msd, ParserContext::default()));
            epoch.advance().unwrap();
            let after = allocated.read().unwrap();
            println!("{} - {}", after, before);
            mem.push((after - before) as f64);
        })
    });
    let mean = mean(&mem);
    println!(
        "{} +/- {} ({})",
        mean,
        standard_deviation(&mem, Some(mean)),
        median(&mem)
    );
}

criterion_group!(
    benches,
    // laast_benchmark,
    // ressa_benchmark_endpoint_simple,
    // ressa_benchmark_endpoint,
    // ressa_benchmark_entity
    rune_benchmark
);
criterion_main!(benches);
