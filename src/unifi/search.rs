
use crate::{
    gui::{CancelSignal, ChannelsSearchThread},
    mac_address::MacAddress,
    unifi::{
        api::{UnifiAPIError, UnifiClient},
        devices::UnifiDeviceBasic,
    },
};
use multiversion::multiversion;
use zeroize::Zeroize;

/// Input parameters for searching a UniFi device by MAC address.
#[derive(Default, Debug, Clone)]
pub struct UnifiSearchInfo {
    pub username: String,
    pub password: String,
    pub server_url: String,
    pub mac_to_search: MacAddress,
    pub accept_invalid_certs: bool,
}

/// Search result type, wrapping an optional device or an API error.
pub type UnifiSearchResult = Result<Option<UnifiDeviceBasic>, UnifiAPIError>;

/// Logs into the UniFi controller and returns a client instance.
fn get_client_and_login<'a>(
    username: &mut str,
    password: &mut str,
    server_url: &'a str,
    accept_invalid_certs: bool,
) -> Result<UnifiClient<'a>, UnifiAPIError> {
    let mut client = UnifiClient::new(server_url, accept_invalid_certs)?;
    let login_result = client.login(username, password);

    password.zeroize();
    username.zeroize();

    login_result?;
    debug_assert!(client.is_logged_in());
    Ok(client)
}

/// SIMD-accelerated device search by MAC address within a site.
#[multiversion(targets = "simd")]
fn find_device_simd(
    site_devices: Vec<UnifiDeviceBasic>,
    mac_to_search: MacAddress,
) -> Option<UnifiDeviceBasic> {
    site_devices.into_iter().find(|device| device.mac == mac_to_search)
}

/// Searches all UniFi sites for a device matching the provided MAC address.
pub fn find_unifi_device(
    search_info: &mut UnifiSearchInfo,
    search_thread_channels: &mut ChannelsSearchThread,
) -> UnifiSearchResult {
    let UnifiSearchInfo {
        username,
        password,
        server_url,
        mac_to_search,
        accept_invalid_certs,
    } = search_info;

    let mut client = get_client_and_login(username, password, server_url, *accept_invalid_certs)?;

    if try_cancelled(search_thread_channels)? {
        return Ok(None);
    }

    let mac_to_search = *mac_to_search;
    let mut unifi_sites = client.get_sites()?;
    let total_sites = unifi_sites.len() as f32;

    for (index, site) in unifi_sites.iter_mut().enumerate() {
        if try_cancelled(search_thread_channels)? {
            return Ok(None);
        }

        let _ = search_thread_channels
            .percentage_tx
            .try_send(index as f32 / total_sites);

        let site_devices = client.get_site_devices_basic(&site.code)?;
        if let Some(mut found_device) = find_device_simd(site_devices, mac_to_search) {
            let _ = search_thread_channels.percentage_tx.try_send(1.0);

            found_device.create_device_label();
            found_device.site = std::mem::take(&mut site.desc);
            return Ok(Some(found_device));
        }
    }

    Ok(None)
}

/// Checks for cancellation signal from the GUI thread.
fn try_cancelled(channels: &mut ChannelsSearchThread) -> Result<bool, UnifiAPIError> {
    match channels.signal_rx.try_recv() {
        Ok(signal) if signal == CancelSignal => Ok(true),
        _ => Ok(false),
    }
}
