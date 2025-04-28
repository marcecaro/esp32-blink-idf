use esp_idf_svc::wifi::EspWifi;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_hal::peripherals::Peripherals;
// The imports should be from esp_idf_svc instead of esp_idf_hal
use esp_idf_svc::wifi::ClientConfiguration;
use esp_idf_svc::wifi::Configuration;

pub fn wifi_init<'a >(ssid: &'a str, password: &'a str) -> anyhow::Result<EspWifi<'a>> {
    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    // Initialize Wi-Fi driver
    let mut wifi = EspWifi::new(peripherals.modem, sys_loop, Some(nvs)).unwrap();

    // Set Wi-Fi configuration (SSID and password)
    let mut wifi_config = Configuration::Client(ClientConfiguration::default());
    
    // Convert strings to heapless::String with proper capacity
    if let Configuration::Client(client_config) = &mut wifi_config {
        // Copy SSID characters into fixed-length array
        for (i, c) in ssid.chars().enumerate() {
            if i < client_config.ssid.capacity() {
                client_config.ssid.push(c).unwrap();
            }
        }
        
        // Copy password characters into fixed-length array
        for (i, c) in password.chars().enumerate() {
            if i < client_config.password.capacity() {
                client_config.password.push(c).unwrap();
            }
        }
    }
    wifi.set_configuration(&wifi_config).unwrap();

    wifi.start().unwrap();
    wifi.connect().unwrap();
    while !wifi.is_connected().unwrap() {
        // Wait until connected
        std::thread::sleep(core::time::Duration::from_millis(100));
    }
    println!("Wi-Fi connected, IP info: {:?}", wifi.sta_netif().get_ip_info().unwrap());

    Ok(wifi)
}
