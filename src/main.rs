use std::{io::Write, vec};

use pyo3::{prelude::*, types::{PyModule, PyType}};
use serde_json;

use schemars::schema::RootSchema;
use typify::{TypeSpace, TypeSpacePatch, TypeSpaceSettings};

// import the flavor module from gen folder
mod gen;


fn init_nova_objects() -> Result<(), PyErr> {
   Python::with_gil(|py| -> Result<(), PyErr> {
        let nova_obj_module: &PyModule = PyModule::import(py, "nova.objects")?;
        nova_obj_module.getattr("register_all")?.call0()?;
        Ok(())
    })?;
    Ok(())
}

fn get_python_class(qualified_class_name: &str) -> PyResult<Py<PyType>> {
    Python::with_gil(|py| -> PyResult<Py<PyType>> {
    // assuming  qualified_class_name is a valid python class in the form of "module.name:class_name"
    // split the qualified_class_name into module and class_name
    let parts: Vec<&str> = qualified_class_name.split(':').collect();
    let module_name = parts[0];
    let class_name = parts[1];
    let module: &PyModule = PyModule::import(py, module_name)?;
    let class_obj:  &PyType= module.getattr(class_name)?.extract()?;
    Ok(class_obj.into())
    })
}

// a main function that retruns a result of either a PyError or a serde_json::Errors
fn main() -> Result<(), Box<dyn std::error::Error>> {
    pyo3::prepare_freethreaded_python();
    init_nova_objects()?;     
    let flavor_schema = get_json_schema("nova.objects.flavor:Flavor")?; 
    let mut type_space = TypeSpace::new(
        TypeSpaceSettings::default()
            .with_derive("JsonSchema".to_string())
            .with_type_mod("types")
            .with_struct_builder(false)
            .with_patch(
                "AllTheTraits",
                TypeSpacePatch::default()
                    .with_derive("Hash")
                    .with_derive("Ord")
                    .with_derive("PartialOrd")
                    .with_derive("Eq")
                    .with_derive("PartialEq"),
            ),
    );

    let mut schema: RootSchema = serde_json::from_str(&flavor_schema).unwrap();
    schema.schema.metadata().title = Some("Flavor".to_string());
    type_space.add_root_schema(schema).unwrap();
    let iostream = type_space.to_stream();
    let contents = format!(
        "{}\n{}",
        "use serde::{Deserialize, Serialize};
        use schemars::JsonSchema;
        ",
        rustfmt_wrapper::rustfmt(iostream.to_string()).unwrap()
    );
    let mut file = std::fs::File::create("src/gen/flavor.rs")?;
    file.write_all(contents.as_bytes())?;
    file.sync_all()?;

    let flavor = gen::flavor::Flavor::Variant0 {
        nova_object_changes: vec![], 
        nova_object_data: Default::default(),
        nova_object_name: "Flavor".to_string(), 
        nova_object_namespace: "nova".to_string(), 
        nova_object_version: "1.0".to_string(),
    };
    println!("Flavor: ");
    serde_json::to_writer(std::io::stdout(), &flavor)?;
    println!("\n\n\n");

    println!("FlavorVariant0 form Flavor: ");
    let data = r#"{"nova_object_changes":[], "nova_object.data":{"deleted":false,"description":"tiny flavor","disabled":false,"ephemeral_gb":0,"extra_specs":{},"flavorid":"1","id":0,"is_public":true,"memory_mb":512,"name":"m1.tiny","projects":[],"root_gb":10,"rxtx_factor":1.0,"swap":0,"vcpu_weight":1,"vcpus":1},"nova_object.name":"Flavor","nova_object.namespace":"nova","nova_object.version":"1.0"}"#;
    let flavor_ovo: gen::flavor::Flavor = serde_json::from_str(data)?;
    let flavor_variant0: gen::FlavorVariant0 = flavor_ovo.into();
    serde_json::to_writer(std::io::stdout(), &flavor_variant0)?;

    println!("\n\n\n");
    println!("FlavorVariant0: ");
    let data = r#"{"nova_object_changes":[], "nova_object.data":{"deleted":false,"description":"tiny flavor","disabled":false,"ephemeral_gb":0,"extra_specs":{},"flavorid":"1","id":0,"is_public":true,"memory_mb":512,"name":"m1.tiny","projects":[],"root_gb":10,"rxtx_factor":1.0,"swap":0,"vcpu_weight":1,"vcpus":1},"nova_object.name":"Flavor","nova_object.namespace":"nova","nova_object.version":"1.0"}"#;
    let flavor_ovo: gen::FlavorVariant0 = serde_json::from_str(data)?;
    serde_json::to_writer(std::io::stdout(), &flavor_ovo)?;
    println!("\n\n\n");
    Ok(())
}




fn get_json_schema(qualified_class_name: &str) -> Result<String, PyErr> {
    Python::with_gil(|py| {
        let flavor: Py<PyType>=get_python_class(qualified_class_name)?;
        let result: Py<PyAny> = call_class_function_0(py, flavor, "to_json_schema")?;
        let json_module: &PyModule = PyModule::import(py, "json")?;
        let json_dumps:&PyAny  = json_module.getattr("dumps")?;
        let flavor_schema: String = json_dumps.call1((result,))?.extract::<&str>()?.to_string();
        Ok(flavor_schema)
    })
}

fn call_class_function_0(py: Python<'_>, cls: Py<PyType>, name: &str) -> Result<Py<PyAny>, PyErr> {
    Ok(cls.getattr(py, name)?.call0(py)?)
}

