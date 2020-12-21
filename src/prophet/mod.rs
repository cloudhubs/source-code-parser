mod component;
pub use component::*;

mod context;
pub use context::*;

mod model;
pub use model::*;

// mod tests {
//     use super::*;
//     use serde_json::json;

//     #[test]
//     pub fn serialization_transformation() {
//         let sub_components = vec![ComponentType::MethodComponent(MethodComponent {
//             component: ComponentInfo {
//                 path: "path",
//                 package_name: "pkg",
//                 instance_name: "inst",
//                 instance_type: InstanceType::MethodComponent,
//                 // sub_components: vec![],
//             },
//             method_name: "method_name"
//         })];
//         let actual = json!(JSSAContext {
//             component: ComponentInfo {
//                 path: "path",
//                 package_name: "pkg",
//                 instance_name: "inst",
//                 instance_type: InstanceType::ClassComponent,
//                 sub_components,
//             }
//         });

//         let expected = json!({
//             "path": "path",
//             "package_name": "pkg",
//             "instance_name": "inst",
//             "instance_type": "CLASSCOMPONENT",
//             "sub_components": [
//                 {
//                     "path": "path",
//                     "package_name": "pkg",
//                     "instance_name": "inst",
//                     "instance_type": "METHODCOMPONENT",
//                     "sub_components": [],
//                     "method_name": "method_name",
//                 },
//             ],
//         });

//         assert_eq!(expected, actual);
//     }
// }