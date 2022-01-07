pub const DEATHSTARBENCH_ROOT: &str = "/home/jacob/dev/DeathStarBench/mediaMicroservices";
pub const TRAINTICKET_ROOT: &str = "/home/jacob/dev/train-ticket";

pub fn directory_json_dsb() -> String {
    format!(
        r#"
{{
    "instanceType": "DirectoryComponent",
    "path": "",
    "files": [],
    "subDirectories": [
      {{
        "instanceType": "DIRECTORYCOMPONENT",
        "path": "{dir}",
        "instanceName": "{dir}::DirectoryComponent",
        "files": [
          "{dir}/docker",
          "{dir}/third_party",
          "{dir}/src",
          "{dir}/datasets",
          "{dir}/wrk2",
          "{dir}/docker-compose.yml",
          "{dir}/README.md",
          "{dir}/CMakeLists.txt",
          "{dir}/gen-py",
          "{dir}/.dockerignore",
          "{dir}/scripts",
          "{dir}/cmake",
          "{dir}/nginx-web-server",
          "{dir}/config",
          "{dir}/media_service.thrift",
          "{dir}/gen-lua",
          "{dir}/test",
          "{dir}/Dockerfile",
          "{dir}/docker-compose-sharding.yml",
          "{dir}/openshift",
          "{dir}/gen-cpp"
        ],
        "subDirectories": [
          {{
            "instanceType": "DIRECTORYCOMPONENT",
            "path": "{dir}/src",
            "instanceName": "{dir}/src::DirectoryComponent",
            "files": [
              "{dir}/src/UserReviewService",
              "{dir}/src/tracing.h",
              "{dir}/src/PlotService",
              "{dir}/src/RatingService",
              "{dir}/src/utils_mongodb.h",
              "{dir}/src/utils.h",
              "{dir}/src/UniqueIdService",
              "{dir}/src/PageService",
              "{dir}/src/logger.h",
              "{dir}/src/CMakeLists.txt",
              "{dir}/src/utils_memcached.h",
              "{dir}/src/UserService",
              "{dir}/src/GenericClient.h",
              "{dir}/src/MovieReviewService",
              "{dir}/src/MovieInfoService",
              "{dir}/src/ThriftClient.h",
              "{dir}/src/MovieIdService",
              "{dir}/src/TextService",
              "{dir}/src/ReviewStorageService",
              "{dir}/src/ComposeReviewService",
              "{dir}/src/RedisClient.h",
              "{dir}/src/CastInfoService",
              "{dir}/src/ClientPool.h"
            ],
            "subDirectories": [
              {{
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "{dir}/src/UserReviewService",
                "instanceName": "{dir}/src/UserReviewService::DirectoryComponent",
                "files": [
                  "{dir}/src/UserReviewService/CMakeLists.txt",
                  "{dir}/src/UserReviewService/UserReviewHandler.h",
                  "{dir}/src/UserReviewService/UserReviewService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "{dir}/src/UserReviewService::PackageName"
              }},
              {{
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "{dir}/src/PlotService",
                "instanceName": "{dir}/src/PlotService::DirectoryComponent",
                "files": [
                  "{dir}/src/PlotService/PlotHandler.h",
                  "{dir}/src/PlotService/CMakeLists.txt",
                  "{dir}/src/PlotService/PlotService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "{dir}/src/PlotService::PackageName"
              }},
              {{
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "{dir}/src/RatingService",
                "instanceName": "{dir}/src/RatingService::DirectoryComponent",
                "files": [
                  "{dir}/src/RatingService/CMakeLists.txt",
                  "{dir}/src/RatingService/RatingService.cpp",
                  "{dir}/src/RatingService/RatingHandler.h"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "{dir}/src/RatingService::PackageName"
              }},
              {{
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "{dir}/src/UniqueIdService",
                "instanceName": "{dir}/src/UniqueIdService::DirectoryComponent",
                "files": [
                  "{dir}/src/UniqueIdService/CMakeLists.txt",
                  "{dir}/src/UniqueIdService/UniqueIdHandler.h",
                  "{dir}/src/UniqueIdService/UniqueIdService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "{dir}/src/UniqueIdService::PackageName"
              }},
              {{
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "{dir}/src/PageService",
                "instanceName": "{dir}/src/PageService::DirectoryComponent",
                "files": [
                  "{dir}/src/PageService/CMakeLists.txt",
                  "{dir}/src/PageService/PageService.cpp",
                  "{dir}/src/PageService/PageHandler.h"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "{dir}/src/PageService::PackageName"
              }},
              {{
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "{dir}/src/UserService",
                "instanceName": "{dir}/src/UserService::DirectoryComponent",
                "files": [
                  "{dir}/src/UserService/UserHandler.h",
                  "{dir}/src/UserService/CMakeLists.txt",
                  "{dir}/src/UserService/UserService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "{dir}/src/UserService::PackageName"
              }},
              {{
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "{dir}/src/MovieReviewService",
                "instanceName": "{dir}/src/MovieReviewService::DirectoryComponent",
                "files": [
                  "{dir}/src/MovieReviewService/MovieReviewService.cpp",
                  "{dir}/src/MovieReviewService/CMakeLists.txt",
                  "{dir}/src/MovieReviewService/MovieReviewHandler.h"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "{dir}/src/MovieReviewService::PackageName"
              }},
              {{
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "{dir}/src/MovieInfoService",
                "instanceName": "{dir}/src/MovieInfoService::DirectoryComponent",
                "files": [
                  "{dir}/src/MovieInfoService/CMakeLists.txt",
                  "{dir}/src/MovieInfoService/MovieInfoService.cpp",
                  "{dir}/src/MovieInfoService/MovieInfoHandler.h"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "{dir}/src/MovieInfoService::PackageName"
              }},
              {{
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "{dir}/src/MovieIdService",
                "instanceName": "{dir}/src/MovieIdService::DirectoryComponent",
                "files": [
                  "{dir}/src/MovieIdService/CMakeLists.txt",
                  "{dir}/src/MovieIdService/MovieIdHandler.h",
                  "{dir}/src/MovieIdService/MovieIdService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "{dir}/src/MovieIdService::PackageName"
              }},
              {{
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "{dir}/src/TextService",
                "instanceName": "{dir}/src/TextService::DirectoryComponent",
                "files": [
                  "{dir}/src/TextService/TextService.cpp",
                  "{dir}/src/TextService/CMakeLists.txt",
                  "{dir}/src/TextService/TextHandler.h"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "{dir}/src/TextService::PackageName"
              }},
              {{
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "{dir}/src/ReviewStorageService",
                "instanceName": "{dir}/src/ReviewStorageService::DirectoryComponent",
                "files": [
                  "{dir}/src/ReviewStorageService/CMakeLists.txt",
                  "{dir}/src/ReviewStorageService/ReviewStorageHandler.h",
                  "{dir}/src/ReviewStorageService/ReviewStorageService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "{dir}/src/ReviewStorageService::PackageName"
              }},
              {{
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "{dir}/src/ComposeReviewService",
                "instanceName": "{dir}/src/ComposeReviewService::DirectoryComponent",
                "files": [
                  "{dir}/src/ComposeReviewService/CMakeLists.txt",
                  "{dir}/src/ComposeReviewService/ComposeReviewHandler.h",
                  "{dir}/src/ComposeReviewService/ComposeReviewService.cpp"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "{dir}/src/ComposeReviewService::PackageName"
              }},
              {{
                "instanceType": "DIRECTORYCOMPONENT",
                "path": "{dir}/src/CastInfoService",
                "instanceName": "{dir}/src/CastInfoService::DirectoryComponent",
                "files": [
                  "{dir}/src/CastInfoService/CastInfoService.cpp",
                  "{dir}/src/CastInfoService/CastInfoHandler.h",
                  "{dir}/src/CastInfoService/CMakeLists.txt"
                ],
                "subDirectories": [],
                "numFiles": 3,
                "package_name": "{dir}/src/CastInfoService::PackageName"
              }}
            ],
            "numFiles": 23,
            "package_name": "{dir}/src::PackageName"
          }}
        ],
        "numFiles": 21,
        "package_name": "{dir}::PackageName"
      }}
    ],
    "numFiles": 0
  }}
"#,
        dir = DEATHSTARBENCH_ROOT
    )
}

pub fn directory_json_tt() -> String {
    format!(
        r#"{{
  "path": "{dir}",
  "files": [],
  "subDirectories": [
      {{
          "path": "{dir}/ts-contacts-service/src/main/java/contacts",
          "files": [
              "{dir}/ts-contacts-service/src/main/java/contacts/init/InitData.java",
              "{dir}/ts-contacts-service/src/main/java/contacts/repository/ContactsRepository.java",
              "{dir}/ts-contacts-service/src/main/java/contacts/config/SecurityConfig.java",
              "{dir}/ts-contacts-service/src/main/java/contacts/ContactsApplication.java",
              "{dir}/ts-contacts-service/src/main/java/contacts/service/ContactsService.java",
              "{dir}/ts-contacts-service/src/main/java/contacts/service/ContactsServiceImpl.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-contacts-service/src/main/java/contacts/controller",
                  "files": [
                      "{dir}/ts-contacts-service/src/main/java/contacts/controller/ContactsController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-contacts-service/src/main/java/contacts/service",
                  "files": [
                      "{dir}/ts-contacts-service/src/main/java/contacts/service/ContactsServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-contacts-service/src/main/java/contacts/entity",
                  "files": [
                      "{dir}/ts-contacts-service/src/main/java/contacts/entity/Contacts.java",
                      "{dir}/ts-contacts-service/src/main/java/contacts/entity/DocumentType.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-food-map-service/src/main/java/food",
          "files": [
              "{dir}/ts-food-map-service/src/main/java/food/init/InitData.java",
              "{dir}/ts-food-map-service/src/main/java/food/repository/TrainFoodRepository.java",
              "{dir}/ts-food-map-service/src/main/java/food/repository/FoodStoreRepository.java",
              "{dir}/ts-food-map-service/src/main/java/food/config/SecurityConfig.java",
              "{dir}/ts-food-map-service/src/main/java/food/controller/TrainFoodController.java",
              "{dir}/ts-food-map-service/src/main/java/food/controller/FoodStoreController.java",
              "{dir}/ts-food-map-service/src/main/java/food/service/FoodMapService.java",
              "{dir}/ts-food-map-service/src/main/java/food/service/FoodMapServiceImpl.java",
              "{dir}/ts-food-map-service/src/main/java/food/FoodMapApplication.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-food-map-service/src/main/java/food/controller",
                  "files": [
                      "{dir}/ts-food-map-service/src/main/java/food/controller/TrainFoodController.java",
                      "{dir}/ts-food-map-service/src/main/java/food/controller/FoodStoreController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-food-map-service/src/main/java/food/service",
                  "files": [
                      "{dir}/ts-food-map-service/src/main/java/food/service/FoodMapServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-food-map-service/src/main/java/food/entity",
                  "files": [
                      "{dir}/ts-food-map-service/src/main/java/food/entity/FoodStore.java",
                      "{dir}/ts-food-map-service/src/main/java/food/entity/TrainFood.java",
                      "{dir}/ts-food-map-service/src/main/java/food/entity/Food.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-notification-service/src/main/java/notification",
          "files": [
              "{dir}/ts-notification-service/src/main/java/notification/NotificationApplication.java",
              "{dir}/ts-notification-service/src/main/java/notification/config/SecurityConfig.java",
              "{dir}/ts-notification-service/src/main/java/notification/config/EmailConfig.java",
              "{dir}/ts-notification-service/src/main/java/notification/config/EmailProperties.java",
              "{dir}/ts-notification-service/src/main/java/notification/controller/NotificationController.java",
              "{dir}/ts-notification-service/src/main/java/notification/service/MailService.java",
              "{dir}/ts-notification-service/src/main/java/notification/service/NotificationServiceImpl.java",
              "{dir}/ts-notification-service/src/main/java/notification/service/NotificationService.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-notification-service/src/main/java/notification/controller",
                  "files": [
                      "{dir}/ts-notification-service/src/main/java/notification/controller/NotificationController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-notification-service/src/main/java/notification/service",
                  "files": [
                      "{dir}/ts-notification-service/src/main/java/notification/service/NotificationServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-notification-service/src/main/java/notification/entity",
                  "files": [
                      "{dir}/ts-notification-service/src/main/java/notification/entity/Mail.java",
                      "{dir}/ts-notification-service/src/main/java/notification/entity/NotifyInfo.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-cancel-service/src/main/java/cancel",
          "files": [
              "{dir}/ts-cancel-service/src/main/java/cancel/CancelApplication.java",
              "{dir}/ts-cancel-service/src/main/java/cancel/config/SecurityConfig.java",
              "{dir}/ts-cancel-service/src/main/java/cancel/controller/CancelController.java",
              "{dir}/ts-cancel-service/src/main/java/cancel/service/CancelServiceImpl.java",
              "{dir}/ts-cancel-service/src/main/java/cancel/service/CancelService.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-cancel-service/src/main/java/cancel/controller",
                  "files": [
                      "{dir}/ts-cancel-service/src/main/java/cancel/controller/CancelController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-cancel-service/src/main/java/cancel/service",
                  "files": [
                      "{dir}/ts-cancel-service/src/main/java/cancel/service/CancelServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-cancel-service/src/main/java/cancel/entity",
                  "files": [
                      "{dir}/ts-cancel-service/src/main/java/cancel/entity/Order.java",
                      "{dir}/ts-cancel-service/src/main/java/cancel/entity/GetAccountByIdResult.java",
                      "{dir}/ts-cancel-service/src/main/java/cancel/entity/OrderStatus.java",
                      "{dir}/ts-cancel-service/src/main/java/cancel/entity/User.java",
                      "{dir}/ts-cancel-service/src/main/java/cancel/entity/Account.java",
                      "{dir}/ts-cancel-service/src/main/java/cancel/entity/GetOrderByIdInfo.java",
                      "{dir}/ts-cancel-service/src/main/java/cancel/entity/SeatClass.java",
                      "{dir}/ts-cancel-service/src/main/java/cancel/entity/DocumentType.java",
                      "{dir}/ts-cancel-service/src/main/java/cancel/entity/Gender.java",
                      "{dir}/ts-cancel-service/src/main/java/cancel/entity/NotifyInfo.java",
                      "{dir}/ts-cancel-service/src/main/java/cancel/entity/GetAccountByIdInfo.java",
                      "{dir}/ts-cancel-service/src/main/java/cancel/entity/VerifyResult.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-config-service/src/main/java/config",
          "files": [
              "{dir}/ts-config-service/src/main/java/config/init/InitData.java",
              "{dir}/ts-config-service/src/main/java/config/repository/ConfigRepository.java",
              "{dir}/ts-config-service/src/main/java/config/ConfigApplication.java",
              "{dir}/ts-config-service/src/main/java/config/SecurityConfig.java",
              "{dir}/ts-config-service/src/main/java/config/controller/ConfigController.java",
              "{dir}/ts-config-service/src/main/java/config/service/ConfigServiceImpl.java",
              "{dir}/ts-config-service/src/main/java/config/service/ConfigService.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-config-service/src/main/java/config/controller",
                  "files": [
                      "{dir}/ts-config-service/src/main/java/config/controller/ConfigController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-config-service/src/main/java/config/service",
                  "files": [
                      "{dir}/ts-config-service/src/main/java/config/service/ConfigServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-config-service/src/main/java/config/entity",
                  "files": [
                      "{dir}/ts-config-service/src/main/java/config/entity/Config.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-assurance-service/src/main/java/assurance",
          "files": [
              "{dir}/ts-assurance-service/src/main/java/assurance/init/InitData.java",
              "{dir}/ts-assurance-service/src/main/java/assurance/repository/AssuranceRepository.java",
              "{dir}/ts-assurance-service/src/main/java/assurance/config/SecurityConfig.java",
              "{dir}/ts-assurance-service/src/main/java/assurance/AssuranceApplication.java",
              "{dir}/ts-assurance-service/src/main/java/assurance/controller/AssuranceController.java",
              "{dir}/ts-assurance-service/src/main/java/assurance/service/AssuranceServiceImpl.java",
              "{dir}/ts-assurance-service/src/main/java/assurance/service/AssuranceService.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-assurance-service/src/main/java/assurance/controller",
                  "files": [
                      "{dir}/ts-assurance-service/src/main/java/assurance/controller/AssuranceController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-assurance-service/src/main/java/assurance/service",
                  "files": [
                      "{dir}/ts-assurance-service/src/main/java/assurance/service/AssuranceServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-assurance-service/src/main/java/assurance/entity",
                  "files": [
                      "{dir}/ts-assurance-service/src/main/java/assurance/entity/AssuranceTypeBean.java",
                      "{dir}/ts-assurance-service/src/main/java/assurance/entity/PlainAssurance.java",
                      "{dir}/ts-assurance-service/src/main/java/assurance/entity/AssuranceType.java",
                      "{dir}/ts-assurance-service/src/main/java/assurance/entity/Assurance.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-order-other-service/src/main/java/other",
          "files": [
              "{dir}/ts-order-other-service/src/main/java/other/init/InitData.java",
              "{dir}/ts-order-other-service/src/main/java/other/repository/OrderOtherRepository.java",
              "{dir}/ts-order-other-service/src/main/java/other/config/SecurityConfig.java",
              "{dir}/ts-order-other-service/src/main/java/other/controller/OrderOtherController.java",
              "{dir}/ts-order-other-service/src/main/java/other/OrderOtherApplication.java",
              "{dir}/ts-order-other-service/src/main/java/other/service/OrderOtherService.java",
              "{dir}/ts-order-other-service/src/main/java/other/service/OrderOtherServiceImpl.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-order-other-service/src/main/java/other/controller",
                  "files": [
                      "{dir}/ts-order-other-service/src/main/java/other/controller/OrderOtherController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-order-other-service/src/main/java/other/service",
                  "files": [
                      "{dir}/ts-order-other-service/src/main/java/other/service/OrderOtherServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-order-other-service/src/main/java/other/entity",
                  "files": [
                      "{dir}/ts-order-other-service/src/main/java/other/entity/LeftTicketInfo.java",
                      "{dir}/ts-order-other-service/src/main/java/other/entity/Order.java",
                      "{dir}/ts-order-other-service/src/main/java/other/entity/OrderAlterInfo.java",
                      "{dir}/ts-order-other-service/src/main/java/other/entity/SoldTicket.java",
                      "{dir}/ts-order-other-service/src/main/java/other/entity/OrderStatus.java",
                      "{dir}/ts-order-other-service/src/main/java/other/entity/QueryInfo.java",
                      "{dir}/ts-order-other-service/src/main/java/other/entity/Seat.java",
                      "{dir}/ts-order-other-service/src/main/java/other/entity/SeatClass.java",
                      "{dir}/ts-order-other-service/src/main/java/other/entity/Ticket.java",
                      "{dir}/ts-order-other-service/src/main/java/other/entity/OrderSecurity.java",
                      "{dir}/ts-order-other-service/src/main/java/other/entity/TrainType.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-route-service/src/main/java/route",
          "files": [
              "{dir}/ts-route-service/src/main/java/route/init/InitData.java",
              "{dir}/ts-route-service/src/main/java/route/repository/RouteRepository.java",
              "{dir}/ts-route-service/src/main/java/route/config/SecurityConfig.java",
              "{dir}/ts-route-service/src/main/java/route/controller/RouteController.java",
              "{dir}/ts-route-service/src/main/java/route/RouteApplication.java",
              "{dir}/ts-route-service/src/main/java/route/service/RouteService.java",
              "{dir}/ts-route-service/src/main/java/route/service/RouteServiceImpl.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-route-service/src/main/java/route/controller",
                  "files": [
                      "{dir}/ts-route-service/src/main/java/route/controller/RouteController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-route-service/src/main/java/route/service",
                  "files": [
                      "{dir}/ts-route-service/src/main/java/route/service/RouteServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-route-service/src/main/java/route/entity",
                  "files": [
                      "{dir}/ts-route-service/src/main/java/route/entity/Route.java",
                      "{dir}/ts-route-service/src/main/java/route/entity/RouteInfo.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-price-service/src/main/java/price",
          "files": [
              "{dir}/ts-price-service/src/main/java/price/init/InitData.java",
              "{dir}/ts-price-service/src/main/java/price/repository/PriceConfigRepository.java",
              "{dir}/ts-price-service/src/main/java/price/config/SecurityConfig.java",
              "{dir}/ts-price-service/src/main/java/price/controller/PriceController.java",
              "{dir}/ts-price-service/src/main/java/price/PriceApplication.java",
              "{dir}/ts-price-service/src/main/java/price/service/PriceService.java",
              "{dir}/ts-price-service/src/main/java/price/service/PriceServiceImpl.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-price-service/src/main/java/price/controller",
                  "files": [
                      "{dir}/ts-price-service/src/main/java/price/controller/PriceController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-price-service/src/main/java/price/service",
                  "files": [
                      "{dir}/ts-price-service/src/main/java/price/service/PriceServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-price-service/src/main/java/price/entity",
                  "files": [
                      "{dir}/ts-price-service/src/main/java/price/entity/PriceConfig.java",
                  "{dir}/ts-price-service/src/main/java/price/entity/TrainType.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-preserve-service/src/main/java/preserve",
          "files": [
              "{dir}/ts-preserve-service/src/main/java/preserve/config/SecurityConfig.java",
              "{dir}/ts-preserve-service/src/main/java/preserve/PreserveApplication.java",
              "{dir}/ts-preserve-service/src/main/java/preserve/controller/PreserveController.java",
              "{dir}/ts-preserve-service/src/main/java/preserve/service/PreserveService.java",
              "{dir}/ts-preserve-service/src/main/java/preserve/service/PreserveServiceImpl.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-preserve-service/src/main/java/preserve/controller",
                  "files": [
                      "{dir}/ts-preserve-service/src/main/java/preserve/controller/PreserveController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-preserve-service/src/main/java/preserve/service",
                  "files": [
                      "{dir}/ts-preserve-service/src/main/java/preserve/service/PreserveServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-preserve-service/src/main/java/preserve/entity",
                  "files": [
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/Contacts.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/Order.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/Consign.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/TripResponse.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/Type.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/TripAllDetail.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/OrderStatus.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/User.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/AssuranceType.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/Account.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/Seat.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/SeatClass.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/Ticket.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/OrderTicketsInfo.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/DocumentType.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/Gender.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/Assurance.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/FoodOrder.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/TripAllDetailInfo.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/NotifyInfo.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/Travel.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/TripId.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/OrderTicketsResult.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/TravelResult.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/TrainType.java",
                      "{dir}/ts-preserve-service/src/main/java/preserve/entity/Trip.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-security-service/src/main/java/security",
          "files": [
              "{dir}/ts-security-service/src/main/java/security/SecurityApplication.java",
              "{dir}/ts-security-service/src/main/java/security/init/InitData.java",
              "{dir}/ts-security-service/src/main/java/security/repository/SecurityRepository.java",
              "{dir}/ts-security-service/src/main/java/security/config/SecurityConfig.java",
              "{dir}/ts-security-service/src/main/java/security/controller/SecurityController.java",
              "{dir}/ts-security-service/src/main/java/security/service/SecurityService.java",
              "{dir}/ts-security-service/src/main/java/security/service/SecurityServiceImpl.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-security-service/src/main/java/security/controller",
                  "files": [
                      "{dir}/ts-security-service/src/main/java/security/controller/SecurityController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-security-service/src/main/java/security/service",
                  "files": [
                      "{dir}/ts-security-service/src/main/java/security/service/SecurityServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-security-service/src/main/java/security/entity",
                  "files": [
                      "{dir}/ts-security-service/src/main/java/security/entity/Order.java",
                      "{dir}/ts-security-service/src/main/java/security/entity/SecurityConfig.java",
                      "{dir}/ts-security-service/src/main/java/security/entity/OrderStatus.java",
                      "{dir}/ts-security-service/src/main/java/security/entity/SeatClass.java",
                      "{dir}/ts-security-service/src/main/java/security/entity/OrderSecurity.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-consign-service/src/main/java/consign",
          "files": [
              "{dir}/ts-consign-service/src/main/java/consign/repository/ConsignRepository.java",
              "{dir}/ts-consign-service/src/main/java/consign/ConsignApplication.java",
              "{dir}/ts-consign-service/src/main/java/consign/config/SecurityConfig.java",
              "{dir}/ts-consign-service/src/main/java/consign/controller/ConsignController.java",
              "{dir}/ts-consign-service/src/main/java/consign/service/ConsignServiceImpl.java",
              "{dir}/ts-consign-service/src/main/java/consign/service/ConsignService.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-consign-service/src/main/java/consign/controller",
                  "files": [
                      "{dir}/ts-consign-service/src/main/java/consign/controller/ConsignController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-consign-service/src/main/java/consign/service",
                  "files": [
                      "{dir}/ts-consign-service/src/main/java/consign/service/ConsignServiceImpl.java"
                  ],
                  "subDirectories": []
              }}
              ,
              {{
                  "path": "{dir}/ts-consign-service/src/main/java/consign/entity",
                  "files": [
                      "{dir}/ts-consign-service/src/main/java/consign/entity/Consign.java",
                      "{dir}/ts-consign-service/src/main/java/consign/entity/InsertConsignRecordResult.java",
                      "{dir}/ts-consign-service/src/main/java/consign/entity/GetPriceDomain.java",
                      "{dir}/ts-consign-service/src/main/java/consign/entity/ConsignRecord.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-train-service/src/main/java/train",
          "files": [
              "{dir}/ts-train-service/src/main/java/train/init/InitData.java",
              "{dir}/ts-train-service/src/main/java/train/repository/TrainTypeRepository.java",
              "{dir}/ts-train-service/src/main/java/train/config/SecurityConfig.java",
              "{dir}/ts-train-service/src/main/java/train/controller/TrainController.java",
              "{dir}/ts-train-service/src/main/java/train/TrainApplication.java",
              "{dir}/ts-train-service/src/main/java/train/service/TrainService.java",
              "{dir}/ts-train-service/src/main/java/train/service/TrainServiceImpl.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-train-service/src/main/java/train/controller",
                  "files": [
                      "{dir}/ts-train-service/src/main/java/train/controller/TrainController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-train-service/src/main/java/train/service",
                  "files": [
                      "{dir}/ts-train-service/src/main/java/train/service/TrainServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-train-service/src/main/java/train/entity",
                  "files": [
                      "{dir}/ts-train-service/src/main/java/train/entity/TrainType.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-order-service/src/main/java/order",
          "files": [
              "{dir}/ts-order-service/src/main/java/order/init/InitData.java",
              "{dir}/ts-order-service/src/main/java/order/repository/OrderRepository.java",
              "{dir}/ts-order-service/src/main/java/order/config/SecurityConfig.java",
              "{dir}/ts-order-service/src/main/java/order/OrderApplication.java",
              "{dir}/ts-order-service/src/main/java/order/controller/OrderController.java",
              "{dir}/ts-order-service/src/main/java/order/service/OrderServiceImpl.java",
              "{dir}/ts-order-service/src/main/java/order/service/OrderService.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-order-service/src/main/java/order/controller",
                  "files": [
                      "{dir}/ts-order-service/src/main/java/order/controller/OrderController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-order-service/src/main/java/order/service",
                  "files": [
                      "{dir}/ts-order-service/src/main/java/order/service/OrderServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-order-service/src/main/java/order/entity",
                  "files": [
                      "{dir}/ts-order-service/src/main/java/order/entity/LeftTicketInfo.java",
                      "{dir}/ts-order-service/src/main/java/order/entity/Order.java",
                      "{dir}/ts-order-service/src/main/java/order/entity/OrderAlterInfo.java",
                      "{dir}/ts-order-service/src/main/java/order/entity/SoldTicket.java",
                      "{dir}/ts-order-service/src/main/java/order/entity/OrderStatus.java",
                      "{dir}/ts-order-service/src/main/java/order/entity/Seat.java",
                      "{dir}/ts-order-service/src/main/java/order/entity/OrderInfo.java",
                      "{dir}/ts-order-service/src/main/java/order/entity/SeatClass.java",
                      "{dir}/ts-order-service/src/main/java/order/entity/Ticket.java",
                      "{dir}/ts-order-service/src/main/java/order/entity/OrderSecurity.java",
                      "{dir}/ts-order-service/src/main/java/order/entity/TrainType.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-verification-code-service/src/main/java/verifycode",
          "files": [
              "{dir}/ts-verification-code-service/src/main/java/verifycode/util/CookieUtil.java",
              "{dir}/ts-verification-code-service/src/main/java/verifycode/config/SecurityConfig.java",
              "{dir}/ts-verification-code-service/src/main/java/verifycode/VerifyCodeApplication.java",
              "{dir}/ts-verification-code-service/src/main/java/verifycode/controller/VerifyCodeController.java",
              "{dir}/ts-verification-code-service/src/main/java/verifycode/service/impl/VerifyCodeServiceImpl.java",
              "{dir}/ts-verification-code-service/src/main/java/verifycode/service/VerifyCodeService.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-verification-code-service/src/main/java/verifycode/controller",
                  "files": [
                      "{dir}/ts-verification-code-service/src/main/java/verifycode/controller/VerifyCodeController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-verification-code-service/src/main/java/verifycode/service/impl",
                  "files": [
                      "{dir}/ts-verification-code-service/src/main/java/verifycode/service/impl/VerifyCodeServiceImpl.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-food-service/src/main/java/foodsearch",
          "files": [
              "{dir}/ts-food-service/src/main/java/foodsearch/repository/FoodOrderRepository.java",
              "{dir}/ts-food-service/src/main/java/foodsearch/config/SecurityConfig.java",
              "{dir}/ts-food-service/src/main/java/foodsearch/controller/FoodController.java",
              "{dir}/ts-food-service/src/main/java/foodsearch/FoodApplication.java",
              "{dir}/ts-food-service/src/main/java/foodsearch/service/FoodServiceImpl.java",
              "{dir}/ts-food-service/src/main/java/foodsearch/service/FoodService.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-food-service/src/main/java/foodsearch/controller",
                  "files": [
                      "{dir}/ts-food-service/src/main/java/foodsearch/controller/FoodController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-food-service/src/main/java/foodsearch/service",
                  "files": [
                      "{dir}/ts-food-service/src/main/java/foodsearch/service/FoodServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-food-service/src/main/java/foodsearch/entity",
                  "files": [
                      "{dir}/ts-food-service/src/main/java/foodsearch/entity/FoodStore.java",
                      "{dir}/ts-food-service/src/main/java/foodsearch/entity/TrainFood.java",
                      "{dir}/ts-food-service/src/main/java/foodsearch/entity/Route.java",
                      "{dir}/ts-food-service/src/main/java/foodsearch/entity/AllTripFood.java",
                      "{dir}/ts-food-service/src/main/java/foodsearch/entity/Food.java",
                      "{dir}/ts-food-service/src/main/java/foodsearch/entity/FoodOrder.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-rebook-service/src/main/java/rebook",
          "files": [
              "{dir}/ts-rebook-service/src/main/java/rebook/config/SecurityConfig.java",
              "{dir}/ts-rebook-service/src/main/java/rebook/RebookApplication.java",
              "{dir}/ts-rebook-service/src/main/java/rebook/controller/RebookController.java",
              "{dir}/ts-rebook-service/src/main/java/rebook/service/RebookServiceImpl.java",
              "{dir}/ts-rebook-service/src/main/java/rebook/service/RebookService.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-rebook-service/src/main/java/rebook/controller",
                  "files": [
                      "{dir}/ts-rebook-service/src/main/java/rebook/controller/RebookController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-rebook-service/src/main/java/rebook/service",
                  "files": [
                      "{dir}/ts-rebook-service/src/main/java/rebook/service/RebookServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-rebook-service/src/main/java/rebook/entity",
                  "files": [
                      "{dir}/ts-rebook-service/src/main/java/rebook/entity/Contacts.java",
                      "{dir}/ts-rebook-service/src/main/java/rebook/entity/Order.java",
                      "{dir}/ts-rebook-service/src/main/java/rebook/entity/TripResponse.java",
                      "{dir}/ts-rebook-service/src/main/java/rebook/entity/Type.java",
                      "{dir}/ts-rebook-service/src/main/java/rebook/entity/TripAllDetail.java",
                      "{dir}/ts-rebook-service/src/main/java/rebook/entity/OrderStatus.java",
                      "{dir}/ts-rebook-service/src/main/java/rebook/entity/Seat.java",
                      "{dir}/ts-rebook-service/src/main/java/rebook/entity/SeatClass.java",
                      "{dir}/ts-rebook-service/src/main/java/rebook/entity/Ticket.java",
                      "{dir}/ts-rebook-service/src/main/java/rebook/entity/OrderTicketsInfo.java",
                      "{dir}/ts-rebook-service/src/main/java/rebook/entity/RebookInfo.java",
                      "{dir}/ts-rebook-service/src/main/java/rebook/entity/PaymentDifferenceInfo.java",
                      "{dir}/ts-rebook-service/src/main/java/rebook/entity/TripAllDetailInfo.java",
                      "{dir}/ts-rebook-service/src/main/java/rebook/entity/VerifyResult.java",
                      "{dir}/ts-rebook-service/src/main/java/rebook/entity/TripId.java",
                      "{dir}/ts-rebook-service/src/main/java/rebook/entity/Trip.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-admin-basic-info-service/src/main/java/adminbasic",
          "files": [
              "{dir}/ts-admin-basic-info-service/src/main/java/adminbasic/config/SecurityConfig.java",
              "{dir}/ts-admin-basic-info-service/src/main/java/adminbasic/controller/AdminBasicInfoController.java",
              "{dir}/ts-admin-basic-info-service/src/main/java/adminbasic/service/AdminBasicInfoService.java",
              "{dir}/ts-admin-basic-info-service/src/main/java/adminbasic/service/AdminBasicInfoServiceImpl.java",
              "{dir}/ts-admin-basic-info-service/src/main/java/adminbasic/AdminBasicInfoApplication.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-admin-basic-info-service/src/main/java/adminbasic/entity",
                  "files": [
                      "{dir}/ts-admin-basic-info-service/src/main/java/adminbasic/entity/Contacts.java",
                      "{dir}/ts-admin-basic-info-service/src/main/java/adminbasic/entity/PriceInfo.java",
                      "{dir}/ts-admin-basic-info-service/src/main/java/adminbasic/entity/Station.java",
                      "{dir}/ts-admin-basic-info-service/src/main/java/adminbasic/entity/Config.java",
                      "{dir}/ts-admin-basic-info-service/src/main/java/adminbasic/entity/TrainType.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-admin-basic-info-service/src/main/java/adminbasic/controller",
                  "files": [
                      "{dir}/ts-admin-basic-info-service/src/main/java/adminbasic/controller/AdminBasicInfoController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-admin-basic-info-service/src/main/java/adminbasic/service",
                  "files": [
                      "{dir}/ts-admin-basic-info-service/src/main/java/adminbasic/service/AdminBasicInfoServiceImpl.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-consign-price-service/src/main/java/consignprice",
          "files": [
              "{dir}/ts-consign-price-service/src/main/java/consignprice/init/InitData.java",
              "{dir}/ts-consign-price-service/src/main/java/consignprice/repository/ConsignPriceConfigRepository.java",
              "{dir}/ts-consign-price-service/src/main/java/consignprice/ConsignPriceApplication.java",
              "{dir}/ts-consign-price-service/src/main/java/consignprice/config/SecurityConfig.java",
              "{dir}/ts-consign-price-service/src/main/java/consignprice/controller/ConsignPriceController.java",
              "{dir}/ts-consign-price-service/src/main/java/consignprice/service/ConsignPriceService.java",
              "{dir}/ts-consign-price-service/src/main/java/consignprice/service/ConsignPriceServiceImpl.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-consign-price-service/src/main/java/consignprice/controller",
                  "files": [
                      "{dir}/ts-consign-price-service/src/main/java/consignprice/controller/ConsignPriceController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-consign-price-service/src/main/java/consignprice/service",
                  "files": [
                      "{dir}/ts-consign-price-service/src/main/java/consignprice/service/ConsignPriceServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-consign-price-service/src/main/java/consignprice/entity",
                  "files": [
                      "{dir}/ts-consign-price-service/src/main/java/consignprice/entity/ConsignPrice.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-basic-service/src/main/java/fdse/microservice",
          "files": [
              "{dir}/ts-basic-service/src/main/java/fdse/microservice/BasicApplication.java",
              "{dir}/ts-basic-service/src/main/java/fdse/microservice/config/SecurityConfig.java",
              "{dir}/ts-basic-service/src/main/java/fdse/microservice/controller/BasicController.java",
              "{dir}/ts-basic-service/src/main/java/fdse/microservice/service/BasicServiceImpl.java",
              "{dir}/ts-basic-service/src/main/java/fdse/microservice/service/BasicService.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-basic-service/src/main/java/fdse/microservice/controller",
                  "files": [
                      "{dir}/ts-basic-service/src/main/java/fdse/microservice/controller/BasicController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-basic-service/src/main/java/fdse/microservice/service",
                  "files": [
                      "{dir}/ts-basic-service/src/main/java/fdse/microservice/service/BasicServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-basic-service/src/main/java/fdse/microservice/entity",
                  "files": [
                      "{dir}/ts-basic-service/src/main/java/fdse/microservice/entity/Contacts.java",
                      "{dir}/ts-basic-service/src/main/java/fdse/microservice/entity/Type.java",
                      "{dir}/ts-basic-service/src/main/java/fdse/microservice/entity/Route.java",
                      "{dir}/ts-basic-service/src/main/java/fdse/microservice/entity/Travel.java",
                      "{dir}/ts-basic-service/src/main/java/fdse/microservice/entity/TripId.java",
                      "{dir}/ts-basic-service/src/main/java/fdse/microservice/entity/PriceConfig.java",
                      "{dir}/ts-basic-service/src/main/java/fdse/microservice/entity/TravelResult.java",
                      "{dir}/ts-basic-service/src/main/java/fdse/microservice/entity/TrainType.java",
                      "{dir}/ts-basic-service/src/main/java/fdse/microservice/entity/Trip.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-preserve-other-service/src/main/java/preserveOther",
          "files": [
              "{dir}/ts-preserve-other-service/src/main/java/preserveOther/config/SecurityConfig.java",
              "{dir}/ts-preserve-other-service/src/main/java/preserveOther/controller/PreserveOtherController.java",
              "{dir}/ts-preserve-other-service/src/main/java/preserveOther/PreserveOtherApplication.java",
              "{dir}/ts-preserve-other-service/src/main/java/preserveOther/service/PreserveOtherServiceImpl.java",
              "{dir}/ts-preserve-other-service/src/main/java/preserveOther/service/PreserveOtherService.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-preserve-other-service/src/main/java/preserveOther/controller",
                  "files": [
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/controller/PreserveOtherController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-preserve-other-service/src/main/java/preserveOther/service",
                  "files": [
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/service/PreserveOtherServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity",
                  "files": [
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/Contacts.java",
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/Order.java",
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/Consign.java",
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/TripResponse.java",
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/Type.java",
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/TripAllDetail.java",
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/OrderStatus.java",
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/User.java",
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/AssuranceType.java",
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/Account.java",
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/Seat.java",
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/SeatClass.java",
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/Ticket.java",
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/OrderTicketsInfo.java",
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/DocumentType.java",
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/Gender.java",
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/Assurance.java",
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/FoodOrder.java",
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/TripAllDetailInfo.java",
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/NotifyInfo.java",
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/Travel.java",
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/TripId.java",
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/TravelResult.java",
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/TrainType.java",
                      "{dir}/ts-preserve-other-service/src/main/java/preserveOther/entity/Trip.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-common/src/main/java/edu/fudan/common",
          "files": [
              "{dir}/ts-common/src/main/java/edu/fudan/common/util/Response.java",
              "{dir}/ts-common/src/main/java/edu/fudan/common/util/JsonUtils.java",
              "{dir}/ts-common/src/main/java/edu/fudan/common/config/SwaggerConfig.java",
              "{dir}/ts-common/src/main/java/edu/fudan/common/security/jwt/JWTUtil.java",
              "{dir}/ts-common/src/main/java/edu/fudan/common/security/jwt/JWTFilter.java",
              "{dir}/ts-common/src/main/java/edu/fudan/common/exception/TokenException.java",
              "{dir}/ts-common/src/main/java/edu/fudan/common/exception/BaseException.java"
          ],
          "subDirectories": []
      }},
      {{
          "path": "{dir}/ts-station-service/src/main/java/fdse/microservice",
          "files": [
              "{dir}/ts-station-service/src/main/java/fdse/microservice/init/InitData.java",
              "{dir}/ts-station-service/src/main/java/fdse/microservice/repository/StationRepository.java",
              "{dir}/ts-station-service/src/main/java/fdse/microservice/config/SecurityConfig.java",
              "{dir}/ts-station-service/src/main/java/fdse/microservice/entity/Station.java",
              "{dir}/ts-station-service/src/main/java/fdse/microservice/controller/StationController.java",
              "{dir}/ts-station-service/src/main/java/fdse/microservice/StationApplication.java",
              "{dir}/ts-station-service/src/main/java/fdse/microservice/service/StationService.java",
              "{dir}/ts-station-service/src/main/java/fdse/microservice/service/StationServiceImpl.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-station-service/src/main/java/fdse/microservice/controller",
                  "files": [
                      "{dir}/ts-station-service/src/main/java/fdse/microservice/controller/StationController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-station-service/src/main/java/fdse/microservice/service",
                  "files": [
                      "{dir}/ts-station-service/src/main/java/fdse/microservice/service/StationServiceImpl.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-auth-service/src/main/java/auth",
          "files": [
              "{dir}/ts-auth-service/src/main/java/auth/dto/TokenDto.java",
              "{dir}/ts-auth-service/src/main/java/auth/dto/BasicAuthDto.java",
              "{dir}/ts-auth-service/src/main/java/auth/dto/AuthDto.java",
              "{dir}/ts-auth-service/src/main/java/auth/init/InitUser.java",
              "{dir}/ts-auth-service/src/main/java/auth/repository/UserRepository.java",
              "{dir}/ts-auth-service/src/main/java/auth/config/WebSecurityConfig.java",
              "{dir}/ts-auth-service/src/main/java/auth/security/jwt/JWTProvider.java",
              "{dir}/ts-auth-service/src/main/java/auth/security/UserDetailsServiceImpl.java",
              "{dir}/ts-auth-service/src/main/java/auth/entity/User.java",
              "{dir}/ts-auth-service/src/main/java/auth/controller/AuthController.java",
              "{dir}/ts-auth-service/src/main/java/auth/controller/UserController.java",
              "{dir}/ts-auth-service/src/main/java/auth/AuthApplication.java",
              "{dir}/ts-auth-service/src/main/java/auth/pub constant/InfoConstant.java",
              "{dir}/ts-auth-service/src/main/java/auth/pub constant/AuthConstant.java",
              "{dir}/ts-auth-service/src/main/java/auth/service/impl/TokenServiceImpl.java",
              "{dir}/ts-auth-service/src/main/java/auth/service/impl/UserServiceImpl.java",
              "{dir}/ts-auth-service/src/main/java/auth/service/UserService.java",
              "{dir}/ts-auth-service/src/main/java/auth/service/TokenService.java",
              "{dir}/ts-auth-service/src/main/java/auth/exception/handler/GlobalExceptionHandler.java",
              "{dir}/ts-auth-service/src/main/java/auth/exception/UserOperationException.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-auth-service/src/main/java/auth/controller",
                  "files": [
                      "{dir}/ts-auth-service/src/main/java/auth/controller/AuthController.java",
                      "{dir}/ts-auth-service/src/main/java/auth/controller/UserController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-auth-service/src/main/java/auth/service",
                  "files": [
                  ],
                  "subDirectories": [
                      {{
                          "path": "{dir}/ts-auth-service/src/main/java/auth/service/impl",
                          "files": [
                              "{dir}/ts-auth-service/src/main/java/auth/service/impl/UserServiceImpl.java",
                              "{dir}/ts-auth-service/src/main/java/auth/service/impl/TokenServiceImpl.java"
                          ],
                          "subDirectories": []
                      }}
                  ]
              }},
              {{
                  "path": "{dir}/ts-auth-service/src/main/java/auth/entity",
                  "files": [
                      "{dir}/ts-auth-service/src/main/java/auth/entity/User.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-user-service/src/main/java/user",
          "files": [
              "{dir}/ts-user-service/src/main/java/user/dto/AuthDto.java",
              "{dir}/ts-user-service/src/main/java/user/dto/Gender.java",
              "{dir}/ts-user-service/src/main/java/user/dto/UserDto.java",
              "{dir}/ts-user-service/src/main/java/user/init/InitUser.java",
              "{dir}/ts-user-service/src/main/java/user/repository/UserRepository.java",
              "{dir}/ts-user-service/src/main/java/user/config/SecurityConfig.java",
              "{dir}/ts-user-service/src/main/java/user/entity/User.java",
              "{dir}/ts-user-service/src/main/java/user/controller/UserController.java",
              "{dir}/ts-user-service/src/main/java/user/UserApplication.java",
              "{dir}/ts-user-service/src/main/java/user/service/impl/UserServiceImpl.java",
              "{dir}/ts-user-service/src/main/java/user/service/UserService.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-user-service/src/main/java/user/controller",
                  "files": [
                      "{dir}/ts-user-service/src/main/java/user/controller/UserController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-user-service/src/main/java/user/service",
                  "files": [
                  ],
                  "subDirectories": [
                      {{
                          "path": "{dir}/ts-user-service/src/main/java/user/service/impl",
                          "files": [
                              "{dir}/ts-user-service/src/main/java/user/service/impl/UserServiceImpl.java"
                          ],
                          "subDirectories": []
                      }}
                  ]
              }},
              {{
                  "path": "{dir}/ts-user-service/src/main/java/user/entity",
                  "files": [
                      "{dir}/ts-user-service/src/main/java/user/entity/User.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-route-plan-service/src/main/java/plan",
          "files": [
              "{dir}/ts-route-plan-service/src/main/java/plan/config/SecurityConfig.java",
              "{dir}/ts-route-plan-service/src/main/java/plan/RoutePlanApplication.java",
              "{dir}/ts-route-plan-service/src/main/java/plan/controller/RoutePlanController.java",
              "{dir}/ts-route-plan-service/src/main/java/plan/service/RoutePlanService.java",
              "{dir}/ts-route-plan-service/src/main/java/plan/service/RoutePlanServiceImpl.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-route-plan-service/src/main/java/plan/controller",
                  "files": [
                      "{dir}/ts-route-plan-service/src/main/java/plan/controller/RoutePlanController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-route-plan-service/src/main/java/plan/service",
                  "files": [
                      "{dir}/ts-route-plan-service/src/main/java/plan/service/RoutePlanServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-route-plan-service/src/main/java/plan/entity",
                  "files": [
                      "{dir}/ts-route-plan-service/src/main/java/plan/entity/TripResponse.java",
                      "{dir}/ts-route-plan-service/src/main/java/plan/entity/Type.java",
                      "{dir}/ts-route-plan-service/src/main/java/plan/entity/TripAllDetail.java",
                      "{dir}/ts-route-plan-service/src/main/java/plan/entity/Route.java",
                      "{dir}/ts-route-plan-service/src/main/java/plan/entity/TripInfo.java",
                      "{dir}/ts-route-plan-service/src/main/java/plan/entity/RoutePlanInfo.java",
                      "{dir}/ts-route-plan-service/src/main/java/plan/entity/TripAllDetailInfo.java",
                      "{dir}/ts-route-plan-service/src/main/java/plan/entity/RoutePlanResultUnit.java",
                      "{dir}/ts-route-plan-service/src/main/java/plan/entity/TripId.java",
                      "{dir}/ts-route-plan-service/src/main/java/plan/entity/Trip.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-ticketinfo-service/src/main/java/ticketinfo",
          "files": [
              "{dir}/ts-ticketinfo-service/src/main/java/ticketinfo/config/SecurityConfig.java",
              "{dir}/ts-ticketinfo-service/src/main/java/ticketinfo/TicketInfoApplication.java",
              "{dir}/ts-ticketinfo-service/src/main/java/ticketinfo/controller/TicketInfoController.java",
              "{dir}/ts-ticketinfo-service/src/main/java/ticketinfo/service/TicketInfoServiceImpl.java",
              "{dir}/ts-ticketinfo-service/src/main/java/ticketinfo/service/TicketInfoService.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-ticketinfo-service/src/main/java/ticketinfo/controller",
                  "files": [
                      "{dir}/ts-ticketinfo-service/src/main/java/ticketinfo/controller/TicketInfoController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-ticketinfo-service/src/main/java/ticketinfo/service",
                  "files": [
                      "{dir}/ts-ticketinfo-service/src/main/java/ticketinfo/service/TicketInfoServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-ticketinfo-service/src/main/java/ticketinfo/entity",
                  "files": [
                      "{dir}/ts-ticketinfo-service/src/main/java/ticketinfo/entity/Type.java",
                      "{dir}/ts-ticketinfo-service/src/main/java/ticketinfo/entity/Travel.java",
                      "{dir}/ts-ticketinfo-service/src/main/java/ticketinfo/entity/TripId.java",
                      "{dir}/ts-ticketinfo-service/src/main/java/ticketinfo/entity/TravelResult.java",
                      "{dir}/ts-ticketinfo-service/src/main/java/ticketinfo/entity/TrainType.java",
                      "{dir}/ts-ticketinfo-service/src/main/java/ticketinfo/entity/Trip.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-payment-service/src/main/java/com/trainticket",
          "files": [
              "{dir}/ts-payment-service/src/main/java/com/trainticket/init/InitData.java",
              "{dir}/ts-payment-service/src/main/java/com/trainticket/repository/AddMoneyRepository.java",
              "{dir}/ts-payment-service/src/main/java/com/trainticket/repository/PaymentRepository.java",
              "{dir}/ts-payment-service/src/main/java/com/trainticket/config/SecurityConfig.java",
              "{dir}/ts-payment-service/src/main/java/com/trainticket/controller/PaymentController.java",
              "{dir}/ts-payment-service/src/main/java/com/trainticket/PaymentApplication.java",
              "{dir}/ts-payment-service/src/main/java/com/trainticket/service/PaymentService.java",
              "{dir}/ts-payment-service/src/main/java/com/trainticket/service/PaymentServiceImpl.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-payment-service/src/main/java/com/trainticket/controller",
                  "files": [
                      "{dir}/ts-payment-service/src/main/java/com/trainticket/controller/PaymentController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-payment-service/src/main/java/com/trainticket/service",
                  "files": [
                      "{dir}/ts-payment-service/src/main/java/com/trainticket/service/PaymentServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-payment-service/src/main/java/com/trainticket/entity",
                  "files": [
                      "{dir}/ts-payment-service/src/main/java/com/trainticket/entity/Payment.java",
                      "{dir}/ts-payment-service/src/main/java/com/trainticket/entity/Money.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-inside-payment-service/src/main/java/inside_payment",
          "files": [
              "{dir}/ts-inside-payment-service/src/main/java/inside_payment/init/InitData.java",
              "{dir}/ts-inside-payment-service/src/main/java/inside_payment/repository/AddMoneyRepository.java",
              "{dir}/ts-inside-payment-service/src/main/java/inside_payment/repository/PaymentRepository.java",
              "{dir}/ts-inside-payment-service/src/main/java/inside_payment/util/CookieUtil.java",
              "{dir}/ts-inside-payment-service/src/main/java/inside_payment/config/SecurityConfig.java",
              "{dir}/ts-inside-payment-service/src/main/java/inside_payment/async/ExecutorConfig.java",
              "{dir}/ts-inside-payment-service/src/main/java/inside_payment/async/AsyncTask.java",
              "{dir}/ts-inside-payment-service/src/main/java/inside_payment/controller/InsidePaymentController.java",
              "{dir}/ts-inside-payment-service/src/main/java/inside_payment/InsidePaymentApplication.java",
              "{dir}/ts-inside-payment-service/src/main/java/inside_payment/service/InsidePaymentService.java",
              "{dir}/ts-inside-payment-service/src/main/java/inside_payment/service/InsidePaymentServiceImpl.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-inside-payment-service/src/main/java/inside_payment/controller",
                  "files": [
                      "{dir}/ts-inside-payment-service/src/main/java/inside_payment/controller/InsidePaymentController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-inside-payment-service/src/main/java/inside_payment/service",
                  "files": [
                      "{dir}/ts-inside-payment-service/src/main/java/inside_payment/service/InsidePaymentServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-inside-payment-service/src/main/java/inside_payment/entity",
                  "files": [
                      "{dir}/ts-inside-payment-service/src/main/java/inside_payment/entity/PaymentType.java",
                      "{dir}/ts-inside-payment-service/src/main/java/inside_payment/entity/Order.java",
                      "{dir}/ts-inside-payment-service/src/main/java/inside_payment/entity/Payment.java",
                      "{dir}/ts-inside-payment-service/src/main/java/inside_payment/entity/OutsidePaymentInfo.java",
                      "{dir}/ts-inside-payment-service/src/main/java/inside_payment/entity/OrderStatus.java",
                      "{dir}/ts-inside-payment-service/src/main/java/inside_payment/entity/PaymentInfo.java",
                      "{dir}/ts-inside-payment-service/src/main/java/inside_payment/entity/MoneyType.java",
                      "{dir}/ts-inside-payment-service/src/main/java/inside_payment/entity/Money.java",
                      "{dir}/ts-inside-payment-service/src/main/java/inside_payment/entity/SeatClass.java",
                      "{dir}/ts-inside-payment-service/src/main/java/inside_payment/entity/Balance.java",
                      "{dir}/ts-inside-payment-service/src/main/java/inside_payment/entity/AccountInfo.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-seat-service/src/main/java/seat",
          "files": [
              "{dir}/ts-seat-service/src/main/java/seat/config/SecurityConfig.java",
              "{dir}/ts-seat-service/src/main/java/seat/SeatApplication.java",
              "{dir}/ts-seat-service/src/main/java/seat/controller/SeatController.java",
              "{dir}/ts-seat-service/src/main/java/seat/service/SeatServiceImpl.java",
              "{dir}/ts-seat-service/src/main/java/seat/service/SeatService.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-seat-service/src/main/java/seat/controller",
                  "files": [
                      "{dir}/ts-seat-service/src/main/java/seat/controller/SeatController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-seat-service/src/main/java/seat/service",
                  "files": [
                      "{dir}/ts-seat-service/src/main/java/seat/service/SeatServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-seat-service/src/main/java/seat/entity",
                  "files": [
                      "{dir}/ts-seat-service/src/main/java/seat/entity/LeftTicketInfo.java",
                      "{dir}/ts-seat-service/src/main/java/seat/entity/Route.java",
                      "{dir}/ts-seat-service/src/main/java/seat/entity/Seat.java",
                      "{dir}/ts-seat-service/src/main/java/seat/entity/SeatClass.java",
                      "{dir}/ts-seat-service/src/main/java/seat/entity/Ticket.java",
                      "{dir}/ts-seat-service/src/main/java/seat/entity/Config.java",
                      "{dir}/ts-seat-service/src/main/java/seat/entity/TrainType.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-execute-service/src/main/java/execute",
          "files": [
              "{dir}/ts-execute-service/src/main/java/execute/config/SecurityConfig.java",
              "{dir}/ts-execute-service/src/main/java/execute/serivce/ExecuteServiceImpl.java",
              "{dir}/ts-execute-service/src/main/java/execute/serivce/ExecuteService.java",
              "{dir}/ts-execute-service/src/main/java/execute/ExecuteApplication.java",
              "{dir}/ts-execute-service/src/main/java/execute/controller/ExecuteController.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-execute-service/src/main/java/execute/controller",
                  "files": [
                      "{dir}/ts-execute-service/src/main/java/execute/controller/ExecuteController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-execute-service/src/main/java/execute/controller",
                  "files": [
                      "{dir}/ts-execute-service/src/main/java/execute/controller/ExecuteController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-execute-service/src/main/java/execute/serivce",
                  "files": [
                      "{dir}/ts-execute-service/src/main/java/execute/serivce/ExecuteServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-execute-service/src/main/java/execute/entity",
                  "files": [
                      "{dir}/ts-execute-service/src/main/java/execute/entity/Order.java",
                      "{dir}/ts-execute-service/src/main/java/execute/entity/OrderStatus.java",
                      "{dir}/ts-execute-service/src/main/java/execute/entity/SeatClass.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-travel-plan-service/src/main/java/travelplan",
          "files": [
              "{dir}/ts-travel-plan-service/src/main/java/travelplan/config/SecurityConfig.java",
              "{dir}/ts-travel-plan-service/src/main/java/travelplan/TravelPlanApplication.java",
              "{dir}/ts-travel-plan-service/src/main/java/travelplan/controller/TravelPlanController.java",
              "{dir}/ts-travel-plan-service/src/main/java/travelplan/service/TravelPlanService.java",
              "{dir}/ts-travel-plan-service/src/main/java/travelplan/service/TravelPlanServiceImpl.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-travel-plan-service/src/main/java/travelplan/controller",
                  "files": [
                      "{dir}/ts-travel-plan-service/src/main/java/travelplan/controller/TravelPlanController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-travel-plan-service/src/main/java/travelplan/service",
                  "files": [
                      "{dir}/ts-travel-plan-service/src/main/java/travelplan/service/TravelPlanServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-travel-plan-service/src/main/java/travelplan/entity",
                  "files": [
                      "{dir}/ts-travel-plan-service/src/main/java/travelplan/entity/TripResponse.java",
                      "{dir}/ts-travel-plan-service/src/main/java/travelplan/entity/TrainTypeEnum.java",
                      "{dir}/ts-travel-plan-service/src/main/java/travelplan/entity/TripInfo.java",
                      "{dir}/ts-travel-plan-service/src/main/java/travelplan/entity/TransferTravelResult.java",
                      "{dir}/ts-travel-plan-service/src/main/java/travelplan/entity/TravelAdvanceResultUnit.java",
                      "{dir}/ts-travel-plan-service/src/main/java/travelplan/entity/Seat.java",
                      "{dir}/ts-travel-plan-service/src/main/java/travelplan/entity/SeatClass.java",
                      "{dir}/ts-travel-plan-service/src/main/java/travelplan/entity/TransferTravelInfo.java",
                      "{dir}/ts-travel-plan-service/src/main/java/travelplan/entity/RoutePlanInfo.java",
                      "{dir}/ts-travel-plan-service/src/main/java/travelplan/entity/RoutePlanResultUnit.java",
                      "{dir}/ts-travel-plan-service/src/main/java/travelplan/entity/TripId.java",
                      "{dir}/ts-travel-plan-service/src/main/java/travelplan/entity/Trip.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-travel2-service/src/main/java/travel2",
          "files": [
              "{dir}/ts-travel2-service/src/main/java/travel2/init/InitData.java",
              "{dir}/ts-travel2-service/src/main/java/travel2/repository/TripRepository.java",
              "{dir}/ts-travel2-service/src/main/java/travel2/Travel2Application.java",
              "{dir}/ts-travel2-service/src/main/java/travel2/config/SecurityConfig.java",
              "{dir}/ts-travel2-service/src/main/java/travel2/controller/Travel2Controller.java",
              "{dir}/ts-travel2-service/src/main/java/travel2/service/Travel2Service.java",
              "{dir}/ts-travel2-service/src/main/java/travel2/service/Travel2ServiceImpl.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-travel2-service/src/main/java/travel2/controller",
                  "files": [
                      "{dir}/ts-travel2-service/src/main/java/travel2/controller/Travel2Controller.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-travel2-service/src/main/java/travel2/service",
                  "files": [
                      "{dir}/ts-travel2-service/src/main/java/travel2/service/Travel2ServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-travel2-service/src/main/java/travel2/entity",
                  "files": [
                      "{dir}/ts-travel2-service/src/main/java/travel2/entity/TripResponse.java",
                      "{dir}/ts-travel2-service/src/main/java/travel2/entity/Type.java",
                      "{dir}/ts-travel2-service/src/main/java/travel2/entity/TripAllDetail.java",
                      "{dir}/ts-travel2-service/src/main/java/travel2/entity/SoldTicket.java",
                      "{dir}/ts-travel2-service/src/main/java/travel2/entity/Route.java",
                      "{dir}/ts-travel2-service/src/main/java/travel2/entity/TripInfo.java",
                      "{dir}/ts-travel2-service/src/main/java/travel2/entity/TravelInfo.java",
                      "{dir}/ts-travel2-service/src/main/java/travel2/entity/Seat.java",
                      "{dir}/ts-travel2-service/src/main/java/travel2/entity/SeatClass.java",
                      "{dir}/ts-travel2-service/src/main/java/travel2/entity/TripAllDetailInfo.java",
                      "{dir}/ts-travel2-service/src/main/java/travel2/entity/Travel.java",
                      "{dir}/ts-travel2-service/src/main/java/travel2/entity/AdminTrip.java",
                      "{dir}/ts-travel2-service/src/main/java/travel2/entity/TripId.java",
                      "{dir}/ts-travel2-service/src/main/java/travel2/entity/TravelResult.java",
                      "{dir}/ts-travel2-service/src/main/java/travel2/entity/TrainType.java",
                      "{dir}/ts-travel2-service/src/main/java/travel2/entity/Trip.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-travel-service/src/main/java/travel",
          "files": [
              "{dir}/ts-travel-service/src/main/java/travel/init/InitData.java",
              "{dir}/ts-travel-service/src/main/java/travel/repository/TripRepository.java",
              "{dir}/ts-travel-service/src/main/java/travel/config/SecurityConfig.java",
              "{dir}/ts-travel-service/src/main/java/travel/controller/TravelController.java",
              "{dir}/ts-travel-service/src/main/java/travel/TravelApplication.java",
              "{dir}/ts-travel-service/src/main/java/travel/service/TravelService.java",
              "{dir}/ts-travel-service/src/main/java/travel/service/TravelServiceImpl.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-travel-service/src/main/java/travel/controller",
                  "files": [
                      "{dir}/ts-travel-service/src/main/java/travel/controller/TravelController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-travel-service/src/main/java/travel/service",
                  "files": [
                      "{dir}/ts-travel-service/src/main/java/travel/service/TravelServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-travel-service/src/main/java/travel/entity",
                  "files": [
                      "{dir}/ts-travel-service/src/main/java/travel/entity/TripResponse.java",
                      "{dir}/ts-travel-service/src/main/java/travel/entity/Type.java",
                      "{dir}/ts-travel-service/src/main/java/travel/entity/TripAllDetail.java",
                      "{dir}/ts-travel-service/src/main/java/travel/entity/SoldTicket.java",
                      "{dir}/ts-travel-service/src/main/java/travel/entity/Route.java",
                      "{dir}/ts-travel-service/src/main/java/travel/entity/TripInfo.java",
                      "{dir}/ts-travel-service/src/main/java/travel/entity/TravelInfo.java",
                      "{dir}/ts-travel-service/src/main/java/travel/entity/Seat.java",
                      "{dir}/ts-travel-service/src/main/java/travel/entity/SeatClass.java",
                      "{dir}/ts-travel-service/src/main/java/travel/entity/TripAllDetailInfo.java",
                      "{dir}/ts-travel-service/src/main/java/travel/entity/Travel.java",
                      "{dir}/ts-travel-service/src/main/java/travel/entity/AdminTrip.java",
                      "{dir}/ts-travel-service/src/main/java/travel/entity/TripId.java",
                      "{dir}/ts-travel-service/src/main/java/travel/entity/TravelResult.java",
                      "{dir}/ts-travel-service/src/main/java/travel/entity/TrainType.java",
                      "{dir}/ts-travel-service/src/main/java/travel/entity/Trip.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-admin-order-service/src/main/java/adminorder",
          "files": [
              "{dir}/ts-admin-order-service/src/main/java/adminorder/config/SecurityConfig.java",
              "{dir}/ts-admin-order-service/src/main/java/adminorder/AdminOrderApplication.java",
              "{dir}/ts-admin-order-service/src/main/java/adminorder/controller/AdminOrderController.java",
              "{dir}/ts-admin-order-service/src/main/java/adminorder/service/AdminOrderServiceImpl.java",
              "{dir}/ts-admin-order-service/src/main/java/adminorder/service/AdminOrderService.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-admin-order-service/src/main/java/adminorder/controller",
                  "files": [
                      "{dir}/ts-admin-order-service/src/main/java/adminorder/controller/AdminOrderController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-admin-order-service/src/main/java/adminorder/service",
                  "files": [
                      "{dir}/ts-admin-order-service/src/main/java/adminorder/service/AdminOrderServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-admin-order-service/src/main/java/adminorder/entity",
                  "files": [
                      "{dir}/ts-admin-order-service/src/main/java/adminorder/entity/Order.java",
                      "{dir}/ts-admin-order-service/src/main/java/adminorder/entity/OrderStatus.java",
                      "{dir}/ts-admin-order-service/src/main/java/adminorder/entity/SeatClass.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-admin-travel-service/src/main/java/admintravel",
          "files": [
              "{dir}/ts-admin-travel-service/src/main/java/admintravel/config/SecurityConfig.java",
              "{dir}/ts-admin-travel-service/src/main/java/admintravel/controller/AdminTravelController.java",
              "{dir}/ts-admin-travel-service/src/main/java/admintravel/service/AdminTravelService.java",
              "{dir}/ts-admin-travel-service/src/main/java/admintravel/service/AdminTravelServiceImpl.java",
              "{dir}/ts-admin-travel-service/src/main/java/admintravel/AdminTravelApplication.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-admin-travel-service/src/main/java/admintravel/controller",
                  "files": [
                      "{dir}/ts-admin-travel-service/src/main/java/admintravel/controller/AdminTravelController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-admin-travel-service/src/main/java/admintravel/service",
                  "files": [
                      "{dir}/ts-admin-travel-service/src/main/java/admintravel/service/AdminTravelServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-admin-travel-service/src/main/java/admintravel/entity",
                  "files": [
                      "{dir}/ts-admin-travel-service/src/main/java/admintravel/entity/Type.java",
                      "{dir}/ts-admin-travel-service/src/main/java/admintravel/entity/Route.java",
                      "{dir}/ts-admin-travel-service/src/main/java/admintravel/entity/TravelInfo.java",
                      "{dir}/ts-admin-travel-service/src/main/java/admintravel/entity/AdminTrip.java",
                      "{dir}/ts-admin-travel-service/src/main/java/admintravel/entity/TripId.java",
                      "{dir}/ts-admin-travel-service/src/main/java/admintravel/entity/TrainType.java",
                      "{dir}/ts-admin-travel-service/src/main/java/admintravel/entity/Trip.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-admin-user-service/src/main/java/adminuser",
          "files": [],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-admin-user-service/src/main/java/adminuser/controller",
                  "files": [
                      "{dir}/ts-admin-user-service/src/main/java/adminuser/controller/AdminUserController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-admin-user-service/src/main/java/adminuser/service",
                  "files": [
                      "{dir}/ts-admin-user-service/src/main/java/adminuser/service/AdminUserServiceImpl.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }},
      {{
          "path": "{dir}/ts-admin-route-service/src/main/java/adminroute",
          "files": [
              "{dir}/ts-admin-route-service/src/main/java/adminroute/config/SecurityConfig.java",
              "{dir}/ts-admin-route-service/src/main/java/adminroute/controller/AdminRouteController.java",
              "{dir}/ts-admin-route-service/src/main/java/adminroute/AdminRouteApplication.java",
              "{dir}/ts-admin-route-service/src/main/java/adminroute/service/AdminRouteService.java",
              "{dir}/ts-admin-route-service/src/main/java/adminroute/service/AdminRouteServiceImpl.java"
          ],
          "subDirectories": [
              {{
                  "path": "{dir}/ts-admin-route-service/src/main/java/adminroute/controller",
                  "files": [
                      "{dir}/ts-admin-route-service/src/main/java/adminroute/controller/AdminRouteController.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-admin-route-service/src/main/java/adminroute/service",
                  "files": [
                      "{dir}/ts-admin-route-service/src/main/java/adminroute/service/AdminRouteServiceImpl.java"
                  ],
                  "subDirectories": []
              }},
              {{
                  "path": "{dir}/ts-admin-route-service/src/main/java/adminroute/entity",
                  "files": [
                      "{dir}/ts-admin-route-service/src/main/java/adminroute/entity/Route.java",
                      "{dir}/ts-admin-route-service/src/main/java/adminroute/entity/RouteInfo.java"
                  ],
                  "subDirectories": []
              }}
          ]
      }}
  ]
}}"#,
        dir = TRAINTICKET_ROOT
    )
}

pub const RESSA_JSON_ENDPOINT_SIMPLE_DSB: &str = r##"
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

pub const RESSA_JSON_ENTITY_DSB: &str = r##"
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

pub const RESSA_JSON_ENDPOINT_DSB: &str = r##"
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

pub const RESSA_JSON_ENDPOINT_TT: &str = r##"
[
  {
    "identifier": "ClassOrInterface",
    "pattern": ".*",
    "auxiliary_pattern": "#{pkg}",
    "subpatterns": [
      {
        "identifier": "Method",
        "pattern": "^main$",
        "auxiliary_pattern": "^void$",
        "subpatterns": [],
        "essential": true
      }
    ],
    "callback": "let pkg = ctx.get_variable(\"pkg\").unwrap();let controller = `API - ${pkg}`;ctx.make_object(controller);ctx.make_tag(pkg, controller);ctx.make_transient(\"allServices\");let array = ctx.get_object(\"allServices\").unwrap();let i = 0;while array.contains_key(`${i}`) { i = i + 1; }ctx.make_attribute(`allServices`, `${i}`, Some(pkg));",
    "essential": true
  },
  {
    "identifier": "ClassOrInterface",
    "pattern": "#{controller}",
    "auxiliary_pattern": "#{package}",
    "subpatterns": [
      {
        "identifier": "Annotation",
        "pattern": "@RequestMapping",
        "auxiliary_pattern": "\"#{endpoint_url_base}\"",
        "subpatterns": [],
        "callback": "",
        "essential": true
      },
      {
        "identifier": "Method",
        "pattern": "#{endpoint_method}",
        "auxiliary_pattern": "#{return_type}",
        "subpatterns": [
          {
            "identifier": "Annotation",
            "pattern": "@#{method_type}Mapping",
            "auxiliary_pattern": "#{endpoint_url_part}",
            "subpatterns": [
              {
                "identifier": "AnnotationValuePair",
                "pattern": "path",
                "auxiliary_pattern": "\"#{endpoint_url_part_path}\"",
                "subpatterns": [],
                "essential": false
              },
              {
                "identifier": "AnnotationValuePair",
                "pattern": "value",
                "auxiliary_pattern": "\"#{endpoint_url_part_path}\"",
                "subpatterns": [],
                "essential": false
              }
            ],
            "callback": "let endpoint_url_part_path = ctx.get_variable(\"endpoint_url_part_path\").unwrap_or(\"\");let endpoint_url_part_value = ctx.get_variable(\"endpoint_url_part_value\").unwrap_or(\"\");let endpoint_url_part = ctx.get_variable(\"endpoint_url_part\").unwrap_or(\"\");endpoint_url_part = endpoint_url_part.replace(\"\\\"\", \"\");let endpoint_url_base = ctx.get_variable(\"endpoint_url_base\").unwrap_or(\"\");if (endpoint_url_part.len()>0){    endpoint_url_base.push_str(endpoint_url_part.clone());}else if (endpoint_url_part_path.len()>0){    endpoint_url_base.push_str(endpoint_url_part_path.clone());}else if (endpoint_url_part_value.len()>0){    endpoint_url_base.push_str(endpoint_url_part_value.clone());}ctx.make_variable(\"endpoint_url\", endpoint_url_base);",
            "essential": true
          }
        ],
        "callback": "let package = ctx.get_variable(\"package\").unwrap();let allServices = ctx.get_object(\"allServices\").unwrap();let i = 0;let controller = ``;while allServices.contains_key(`${i}`) { controller = allServices.get(`${i}`).unwrap().unwrap(); if package == controller || package.starts_with(`${controller}.`) { break; } else { i = i + 1; }}if !allServices.contains_key(`${i}`) { panic(\"Unknown service matched\");}let endpoint_method = `${package}.${ctx.get_variable(\"controller\").unwrap()}#${ctx.get_variable(\"endpoint_method\").unwrap()}`;let return_type = ctx.get_variable(\"return_type\").unwrap_or(\"\");let endpoint_url = ctx.get_variable(\"endpoint_url\").unwrap();if !endpoint_url.starts_with(\"/\") { endpoint_url = \"/\" + endpoint_url;}let method_type = ctx.get_variable(\"method_type\").unwrap_or(\"\");match method_type { \"Post\" => method_type = String::from_str(\"POST\"),\"Get\" => method_type = String::from_str(\"GET\"),\"Put\" => method_type = String::from_str(\"PUT\"),\"Delete\" => method_type = String::from_str(\"DELETE\"),\"Patch\" => method_type = String::from_str(\"PATCH\"),_ => method_type = method_type}let full_endpoint = method_type.clone();full_endpoint.push_str(\" \");full_endpoint.push_str(endpoint_url.clone());full_endpoint.push_str(\" \");let url_itr = endpoint_url.split(\"{\");let stripped_url = url_itr.next().unwrap();for arg_part in url_itr { let arg_part_itr = arg_part.split(\"}\"); let arg_part2 = arg_part_itr.skip(1).next().unwrap(); stripped_url.push_str(\"{}\"); stripped_url.push_str(arg_part2.clone());}let full_path = method_type.clone() + \" \" + stripped_url.clone();ctx.make_transient(full_path);ctx.make_attribute(full_path, \"Controller\", Some(controller));ctx.make_attribute(full_path, \"Endpoint method\", Some(endpoint_method));full_endpoint.push_str(return_type.clone());controller = \"API - \" + controller;ctx.make_object(controller);ctx.make_attribute(controller, endpoint_method, Some(full_endpoint));ctx.make_variable(`endpoint_url_part_path`, ``);ctx.make_variable(`endpoint_url_part`, ``);",
        "essential": true
      }
    ],
    "essential": true
  },
  {
    "identifier": "ClassOrInterface",
    "pattern": "#{service}",
    "auxiliary_pattern": "#{package}",
    "subpatterns": [
      {
        "identifier": "Annotation",
        "pattern": "@Service",
        "subpatterns": [],
        "callback": "",
        "essential": true
      },
      {
        "identifier": "Field",
        "pattern": "#{path_var_name}",
        "subpatterns": [
          {
            "identifier": "Literal",
            "pattern": "(https?://.*):(\\d*)#{path_var_val}/?\"",
            "subpatterns": [],
            "callback": "let path_var_name = ctx.get_variable(\"path_var_name\").unwrap_or(\"\");let path_var_val = ctx.get_variable(\"path_var_val\").unwrap_or(\"\");if path_var_name.len() > 0 && path_var_val.len() > 0 { ctx.make_variable(path_var_name, path_var_val); ctx.make_variable(\"path_var_name\", \"\"); ctx.make_variable(\"path_var_val\", \"\"); }",
            "essential": false
          }
        ],
        "essential": false
      },
      {
        "identifier": "Method",
        "pattern": "#{calling_method}",
        "subpatterns": [
          {
            "identifier": "CallExpr",
            "pattern": "exchange",
            "auxiliary_pattern": "#{somename}",
            "subpatterns": [
              {
                "identifier": "Literal",
                "transparent": true,
                "pattern": "",
                "auxiliary_pattern": "",
                "subpatterns": [
                  {
                    "identifier": "Literal",
                    "pattern": "(https?://.*):(\\d*)#{path_root}/?\"",
                    "subpatterns": [],
                    "callback": "ctx.make_variable(\"found_path\", \"true\");ctx.make_variable(\"lit_is_path\", \"true\");let path_root = ctx.get_variable(\"path_root\").unwrap();ctx.make_variable(\"curr_path\", path_root);",
                    "essential": true
                  },
                  {
                    "identifier": "Ident",
                    "pattern": "#{path_root_ident}",
                    "subpatterns": [],
                    "callback": "let path_root_ident = ctx.get_variable(\"path_root_ident\").unwrap();let found_path = ctx.get_variable(\"found_path\").unwrap_or(\"\");let call_finished = ctx.get_variable(\"call_finished\").unwrap_or(\"\");if found_path.len() == 0 && call_finished.len() == 0 {let path_root_val = ctx.get_variable(path_root_ident.clone()).unwrap_or(\"\");if path_root_val.len() > 0 {ctx.make_variable(\"path_root\", path_root_val);ctx.make_variable(\"curr_path\", path_root_val);ctx.make_variable(\"found_path\", \"true\");ctx.make_variable(\"ident_is_path\", \"true\");}}",
                    "essential": true
                  },
                  {
                    "identifier": "Ident",
                    "pattern": "#{path_ident}",
                    "subpatterns": [],
                    "essential": false,
                    "callback": "let path_root = ctx.get_variable(\"path_root\").unwrap_or(\"\");let found_path = ctx.get_variable(\"found_path\").unwrap_or(\"\");let ident_is_path = ctx.get_variable(\"ident_is_path\").unwrap_or(\"\");let found_path_ident = ctx.get_variable(\"found_path_ident\").unwrap_or(\"\");if (path_root.len() > 0 && found_path.len() > 0 && ident_is_path.len() == 0 && found_path_ident.len() == 0) {    let curr_path = ctx.get_variable(\"curr_path\").unwrap();    let path_ident = ctx.get_variable(\"path_ident\").unwrap();    curr_path.push_str(\"{}\");    ctx.make_variable(\"curr_path\", curr_path);    ctx.make_variable(\"found_path_ident\", \"true\");}if(ident_is_path.len() > 0){ctx.make_variable(\"ident_is_path\", \"\");}"
                  },
                  {
                    "identifier": "Literal",
                    "pattern": "\"#{path_lit}\"",
                    "subpatterns": [],
                    "essential": false,
                    "callback": "let path_root = ctx.get_variable(\"path_root\").unwrap_or(\"\");let found_path = ctx.get_variable(\"found_path\").unwrap_or(\"\");let lit_is_path = ctx.get_variable(\"lit_is_path\").unwrap_or(\"\"); if (path_root.len() > 0 && found_path.len() > 0 && lit_is_path.len() == 0) {    let curr_path = ctx.get_variable(\"curr_path\").unwrap();    let path_lit = ctx.get_variable(\"path_lit\").unwrap_or(\"\");    if path_lit.len() > 0 {curr_path.push_str(path_lit);    ctx.make_variable(\"curr_path\", curr_path);ctx.make_variable(\"found_path_ident\", \"\");}    }ctx.make_variable(\"path_lit\", \"\");"
                  }
                ],
                "essential": true,
                "callback": "ctx.make_variable(\"lit_is_path\", \"\");"
              },
              {
                "identifier": "Ident",
                "pattern": "#{method_ident}(^[A-Z]+$)",
                "subpatterns": [],
                "essential": true,
                "callback": "let path_root = ctx.get_variable(\"path_root\").unwrap_or(\"\"); let found_path = ctx.get_variable(\"found_path\").unwrap_or(\"\"); if (path_root.len() > 0 && found_path.len() > 0) { let pkg = ctx.get_variable(\"package\").unwrap(); let allServices = ctx.get_object(\"allServices\").unwrap(); let i = 0; let service = ``; while allServices.contains_key(`${i}`) { service = allServices.get(`${i}`).unwrap().unwrap(); if pkg == service || pkg.starts_with(`${service}.`) { break; } else { i = i + 1; } } if !allServices.contains_key(`${i}`) { panic(\"Unknown service matched\"); } let method_ident = ctx.get_variable(\"method_ident\").unwrap(); let curr_path = ctx.get_variable(\"curr_path\").unwrap(); let service_calls = \"Calls - \" + service; let full_call = method_ident + \" \" + curr_path; let source_obj = ctx.get_object(full_call).unwrap(); let call_info = source_obj.get(\"Endpoint method\").unwrap().unwrap(); ctx.make_object(service_calls); ctx.make_tag(service, service_calls); ctx.make_attribute(service, full_call, Some(call_info)); } ctx.make_variable(\"found_path\", \"\"); ctx.make_variable(\"found_path_ident\", \"\"); ctx.make_variable(\"path_root\", \"\"); ctx.make_variable(\"curr_path\", \"\"); ctx.make_variable(\"path_root_ident\", \"\"); ctx.make_variable(\"found_path\", \"\"); ctx.make_variable(\"call_finished\", \"\"); ctx.make_variable(\"ident_is_path\", \"\"); ctx.make_variable(\"path_ident\", \"\");"
              }
            ],
            "callback": "ctx.make_variable(\"call_finished\", \"\");",
            "essential": false
          }
        ],
        "essential": false
      }
    ],
    "essential": true
  }
]
"##;

pub const RESSA_JSON_ENTITY_TT: &str = r##"
[
  {
    "identifier": "ClassOrInterface",
    "pattern": ".*",
    "auxiliary_pattern": "#{package_name}",
    "subpatterns": [
      {
        "identifier": "Annotation",
        "pattern": "@Document",
        "subpatterns": [
          {
            "identifier": "AnnotationValuePair",
            "pattern": "collection",
            "auxiliary_pattern": "\"#{entity_name}\"",
            "subpatterns": [],
            "essential": true
          }
        ],
        "essential": true
      },
      {
        "identifier": "Field",
        "pattern": "#{field_name}",
        "auxiliary_pattern": "#{field_type}",
        "subpatterns": [],
        "callback": "let entity_name = ctx.get_variable(\"package_name\").unwrap() + \": \" + ctx.get_variable(\"entity_name\").unwrap();ctx.make_object(entity_name);let field_name = ctx.get_variable(\"field_name\").unwrap();let field_type = ctx.get_variable(\"field_type\").unwrap();ctx.make_attribute(entity_name, field_name, Some(field_type));",
        "essential": true
      }
    ],
    "essential": true
  }
]
"##;
