use crate::error::map_response_error;
#[allow(deprecated)]
use crate::models::tags::{
    CategoryData, DonationPlatformData, GameVersionData, LicenseData, LicenseTextData, LoaderData,
};
use crate::{ModrinthClient, ModrinthError};

impl ModrinthClient {
    /// List possible project categories.
    ///
    /// This calls:
    /// `GET /tag/category`
    ///
    /// From <https://docs.modrinth.com/api/operations/categorylist/>
    ///
    /// # Errors
    /// - [`ModrinthError::ApiError`] for any API errors
    /// - [`ModrinthError::ReqwestError`] for request/response failures
    pub async fn list_tag_categories(&self) -> Result<Vec<CategoryData>, ModrinthError> {
        let url = format!("{}/tag/category", self.base_url);
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let categories = resp
            .json::<Vec<CategoryData>>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(categories);
    }

    /// List possible loaders.  
    /// Does not use the `Loader` enum.
    ///
    /// This calls:
    /// `GET /tag/loader`
    ///
    /// From <https://docs.modrinth.com/api/operations/loaderlist/>
    ///
    /// # Errors
    /// - [`ModrinthError::ApiError`] for any API errors
    /// - [`ModrinthError::ReqwestError`] for request/response failures
    pub async fn list_tag_loaders(&self) -> Result<Vec<LoaderData>, ModrinthError> {
        let url = format!("{}/tag/loader", self.base_url);
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let loaders = resp
            .json::<Vec<LoaderData>>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(loaders);
    }

    /// List possible game versions.
    ///
    /// This calls:
    /// `GET /tag/game_version`
    ///
    /// From <https://docs.modrinth.com/api/operations/versionlist/>
    ///
    /// # Errors
    /// - [`ModrinthError::ApiError`] for any API errors
    /// - [`ModrinthError::ReqwestError`] for request/response failures
    pub async fn list_tag_game_versions(&self) -> Result<Vec<GameVersionData>, ModrinthError> {
        let url = format!("{}/tag/game_version", self.base_url);
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let versions = resp
            .json::<Vec<GameVersionData>>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(versions);
    }

    /// List possible licenses.
    ///
    /// This calls:
    /// `GET /tag/license`
    ///
    /// From <https://docs.modrinth.com/api/operations/licenselist/>
    ///
    /// # Errors
    /// - [`ModrinthError::ApiError`] for any API errors
    /// - [`ModrinthError::ReqwestError`] for request/response failures
    #[deprecated(note = "simply use SPDX IDs.")]
    #[allow(deprecated)]
    pub async fn list_tag_licenses(&self) -> Result<Vec<LicenseData>, ModrinthError> {
        let url = format!("{}/tag/license", self.base_url);
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let licenses = resp
            .json::<Vec<LicenseData>>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(licenses);
    }

    /// List possible project categories.
    ///
    /// This calls:
    /// `GET /tag/license/{license_id}`
    ///
    /// From <https://docs.modrinth.com/api/operations/licensetext/>
    ///
    /// # Errors
    /// - [`ModrinthError::ApiError`] for any API errors
    /// - [`ModrinthError::ReqwestError`] for request/response failures
    pub async fn get_license_text<S: Into<String>>(
        &self,
        license_id: S,
    ) -> Result<LicenseTextData, ModrinthError> {
        let url = format!("{}/tag/license/{}", self.base_url, license_id.into());
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let license_text = resp
            .json::<LicenseTextData>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(license_text);
    }

    /// List possible donation platforms.  
    /// Does not use the `DonationPlatform` enum.
    ///
    /// This calls:
    /// `GET /tag/donation_platform`
    ///
    /// From <https://docs.modrinth.com/api/operations/donationplatformlist/>
    ///
    /// # Errors
    /// - [`ModrinthError::ApiError`] for any API errors
    /// - [`ModrinthError::ReqwestError`] for request/response failures
    pub async fn list_tag_donation_platforms(
        &self,
    ) -> Result<Vec<DonationPlatformData>, ModrinthError> {
        // TODO: donation-platform-enums feature
        let url = format!("{}/tag/donation_platform", self.base_url);
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let donation_platforms = resp
            .json::<Vec<DonationPlatformData>>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(donation_platforms);
    }

    /// List possible report types.
    ///
    /// This calls:
    /// `GET /tag/report_type`
    ///
    /// From <https://docs.modrinth.com/api/operations/reporttypelist/>
    ///
    /// # Errors
    /// - [`ModrinthError::ApiError`] for any API errors
    /// - [`ModrinthError::ReqwestError`] for request/response failures
    pub async fn list_tag_report_types(&self) -> Result<Vec<String>, ModrinthError> {
        let url = format!("{}/tag/report_type", self.base_url);
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let report_types = resp
            .json::<Vec<String>>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(report_types);
    }

    /// List possible project types.  
    /// Does not use the `ProjectType` enum.
    ///
    /// This calls:
    /// `GET /tag/project_type`
    ///
    /// From <https://docs.modrinth.com/api/operations/projecttypelist/>
    ///
    /// # Errors
    /// - [`ModrinthError::ApiError`] for any API errors
    /// - [`ModrinthError::ReqwestError`] for request/response failures
    pub async fn list_tag_project_types(&self) -> Result<Vec<String>, ModrinthError> {
        let url = format!("{}/tag/project_type", self.base_url);
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let project_types = resp
            .json::<Vec<String>>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(project_types);
    }

    /// List possible side types.
    ///
    /// This calls:
    /// `GET /tag/side_type`
    ///
    /// From <https://docs.modrinth.com/api/operations/sidetypelist/>
    ///
    /// # Errors
    /// - [`ModrinthError::ApiError`] for any API errors
    /// - [`ModrinthError::ReqwestError`] for request/response failures
    #[deprecated(note = "use [`Environment`](crate::Environment) instead of side types.")]
    pub async fn list_tag_side_types(&self) -> Result<Vec<String>, ModrinthError> {
        let url = format!("{}/tag/side_type", self.base_url);
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let side_types = resp
            .json::<Vec<String>>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(side_types);
    }
}
