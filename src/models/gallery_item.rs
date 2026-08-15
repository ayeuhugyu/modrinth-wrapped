use serde::{Deserialize, Serialize};

use crate::models::project::DateField;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
/// An image that has been uploaded to a project’s gallery
pub struct GalleryItem {
    /// The URL of the gallery image
    url: String,
    /// Whether the image is featured in the gallery
    featured: bool,
    /// The title of the gallery image
    title: Option<String>,
    /// The description of the gallery image
    description: Option<String>,
    /// The date and time the gallery image was created.  
    /// Disabling the `parse-dates` feature will leave this as a ISO-8601 string.
    created: DateField,
    /// The order of the gallery image.
    /// Gallery images are sorted by this field and then alphabetically by title.
    ordering: Option<i32>,
}
