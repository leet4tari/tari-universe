use crate::database::schema::*;
use crate::interface::TappletManifest;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = installed_tapplet)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct InstalledTapplet {
  pub id: Option<i32>,
  pub tapplet_id: Option<i32>,
  pub is_dev_mode: bool,
  pub dev_mode_endpoint: Option<String>,
  pub path_to_dist: Option<String>,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = installed_tapplet)]
pub struct CreateInstalledTapplet<'a> {
  pub is_dev_mode: bool,
  pub dev_mode_endpoint: &'a str,
  pub path_to_dist: &'a str,
}

impl<'a> From<&CreateInstalledTapplet<'a>> for UpdateInstalledTapplet {
  fn from(create_installed_tapplet: &CreateInstalledTapplet) -> Self {
    UpdateInstalledTapplet {
      is_dev_mode: create_installed_tapplet.is_dev_mode,
      dev_mode_endpoint: Some(create_installed_tapplet.dev_mode_endpoint.to_string()),
      path_to_dist: Some(create_installed_tapplet.path_to_dist.to_string()),
    }
  }
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = installed_tapplet)]
pub struct UpdateInstalledTapplet {
  pub is_dev_mode: bool,
  pub dev_mode_endpoint: Option<String>,
  pub path_to_dist: Option<String>,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = tapplet)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tapplet {
  pub id: Option<i32>,
  pub registry_id: String,
  pub display_name: String,
  pub author_name: String,
  pub author_website: String,
  pub about_summary: String,
  pub about_description: String,
  pub category: String,
  pub package_name: String,
  pub registry_url: String,
  pub image_id: Option<i32>,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = tapplet)]
pub struct CreateTapplet<'a> {
  pub registry_id: &'a str,
  pub display_name: &'a str,
  pub author_name: &'a str,
  pub author_website: &'a str,
  pub about_summary: &'a str,
  pub about_description: &'a str,
  pub category: &'a str,
  pub package_name: &'a str,
  pub registry_url: &'a str,
  pub image_id: Option<i32>,
}

impl<'a> From<&'a TappletManifest> for CreateTapplet<'a> {
  fn from(tapplet_manifest: &'a TappletManifest) -> Self {
    CreateTapplet {
      registry_id: &tapplet_manifest.id,
      display_name: &tapplet_manifest.metadata.display_name,
      author_name: &tapplet_manifest.metadata.author.name,
      author_website: &tapplet_manifest.metadata.author.website,
      about_summary: &tapplet_manifest.metadata.about.summary,
      about_description: &tapplet_manifest.metadata.about.description,
      category: &tapplet_manifest.metadata.category,
      package_name: &tapplet_manifest.metadata.source.location.npm.package_name,
      registry_url: &tapplet_manifest.metadata.source.location.npm.registry,
      image_id: None,
    }
  }
}

impl<'a> From<&CreateTapplet<'a>> for UpdateTapplet {
  fn from(create_tapplet: &CreateTapplet) -> Self {
    UpdateTapplet {
      registry_id: create_tapplet.registry_id.to_string(),
      display_name: create_tapplet.display_name.to_string(),
      author_name: create_tapplet.author_name.to_string(),
      author_website: create_tapplet.author_website.to_string(),
      about_summary: create_tapplet.about_summary.to_string(),
      about_description: create_tapplet.about_description.to_string(),
      category: create_tapplet.category.to_string(),
      package_name: create_tapplet.package_name.to_string(),
      registry_url: create_tapplet.registry_url.to_string(),
      image_id: create_tapplet.image_id,
    }
  }
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = tapplet)]
pub struct UpdateTapplet {
  pub registry_id: String,
  pub display_name: String,
  pub author_name: String,
  pub author_website: String,
  pub about_summary: String,
  pub about_description: String,
  pub category: String,
  pub package_name: String,
  pub registry_url: String,
  pub image_id: Option<i32>,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = asset)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Asset {
  pub id: Option<i32>,
  pub rel_path: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = asset)]
pub struct CreateAsset<'a> {
  pub rel_path: &'a str,
}

impl<'a> From<&CreateAsset<'a>> for UpdateAsset {
  fn from(create_asset: &CreateAsset) -> Self {
    UpdateAsset {
      rel_path: create_asset.rel_path.to_string(),
    }
  }
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = asset)]
pub struct UpdateAsset {
  pub rel_path: String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = tapplet_version)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TappletVersion {
  pub id: Option<i32>,
  pub tapplet_id: Option<i32>,
  pub version: String,
  pub checksum: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = tapplet_version)]
pub struct CreateTappletVersion<'a> {
  pub tapplet_id: Option<i32>,
  pub version: &'a str,
  pub checksum: &'a str,
}

impl<'a> From<&CreateTappletVersion<'a>> for UpdateTappletVersion {
  fn from(create_tapplet_version: &CreateTappletVersion) -> Self {
    UpdateTappletVersion {
      tapplet_id: create_tapplet_version.tapplet_id,
      version: create_tapplet_version.version.to_string(),
      checksum: create_tapplet_version.checksum.to_string(),
    }
  }
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = tapplet_version)]
pub struct UpdateTappletVersion {
  pub tapplet_id: Option<i32>,
  pub version: String,
  pub checksum: String,
}