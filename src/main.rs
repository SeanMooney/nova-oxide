use std::io::Write;

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
            .with_struct_builder(true)
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

    // create a flavor data object
    let flavor_inst = gen::flavor::FlavorVariant0NovaObjectData::builder()
        .id(42).flavorid("m1.small")
        .name("m1.small".to_string())
        .memory_mb(2048).vcpus(2).root_gb(20)
        .ephemeral_gb(0).swap(0).projects(vec![]).is_public(true).rxtx_factor(1.0);
    println!("{:?}", flavor_inst);
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

