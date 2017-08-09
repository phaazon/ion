use serde_json::from_reader;
use std::path::Path;
use std::fs::File;

use linear::{M44, Quat, V3};
use model::{ObjVertex, Model};
use resource::{Load, LoadError, LoadResult, Res, ResCache};
use scale::Scale;
use transform::{Transform, Transformable};

type ObjObject = Object<ObjVertex>;

#[derive(Clone, Debug)]
pub struct Object<Vertex> {
  pub model: Res<Model<Vertex>>,
  pub position: V3<f32>,
  pub orientation: Quat<f32>,
  pub scale: Scale
}

impl<Vertex> Object<Vertex> {
  pub fn new(model: Res<Model<Vertex>>, position: V3<f32>, orientation: Quat<f32>, scale: Scale) -> Self {
    Object {
      model: model,
      position: position,
      orientation: orientation,
      scale: scale
    }
  }
}

impl<Vertex> Transformable for Object<Vertex> {
  fn transform(&self) -> Transform {
    (M44::from_translation(-self.position) * M44::from(self.scale) * M44::from(self.orientation)).into()
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectManifest {
  model: String,
  #[serde(default = "def_position")]
  position: [f32; 3],
  #[serde(default = "def_orientation")]
  orientation: [f32; 4],
  #[serde(default = "def_scale")]
  scale: [f32; 3]
}

impl Load for ObjObject {
  type Args = ();

  const TY_STR: &'static str = "objects";

  fn load<P>(path: P, cache: &mut ResCache, _: Self::Args) -> Result<LoadResult<Self>, LoadError> where P: AsRef<Path> {
    let path = path.as_ref();

    // read the manifest
    let manifest: ObjectManifest = {
      let file = File::open(path).map_err(|_| LoadError::FileNotFound(path.to_path_buf()))?;
      from_reader(file).map_err(|e| LoadError::ParseFailed(format!("{:?}", e)))?
    };

    let model = cache.get(&manifest.model, ()).ok_or(LoadError::ConversionFailed(format!("unable to find model {} for object at {:?}", manifest.model, path)))?;

    Ok((Object {
      model: model,
      position: manifest.position.into(),
      orientation: manifest.orientation.into(),
      scale: manifest.scale.into()
    }).into())
  }
}

fn def_position() -> [f32; 3] { [0., 0., 0.] }
fn def_orientation() -> [f32; 4] { [1., 0., 0., 0.] }
fn def_scale() -> [f32; 3] { [1., 1., 1.] }
