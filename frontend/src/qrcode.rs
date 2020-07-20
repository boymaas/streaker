use qrcode_generator::QrCodeEcc;
use url::Url;

type SvgCode = String;

// &format!("https://{}.monetashi.io", anode)
pub fn generate(url: &str) -> SvgCode {
    qrcode_generator::to_svg_to_string(url, QrCodeEcc::Low, 400, None).unwrap()
}

pub fn generate_url(name: &str, url: &str, source: &str) -> String {
    let anode_url = Url::parse(url).unwrap();

    let url = Url::parse_with_params(
        "https://mobile.opes.pe/opesapp/check-in",
        &[
            ("name", name),
            ("url", &anode_url.to_string()),
            ("source", source),
        ],
    )
    .unwrap();

    url.to_string()
}
