use qrcode_generator::QrCodeEcc;
use url::Url;

type SvgCode = String;

// &format!("https://{}.monetashi.io", anode)
pub fn generate(name: &str, url: &str, source: &str) -> SvgCode {
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

    qrcode_generator::to_svg_to_string(url.to_string(), QrCodeEcc::Low, 400, None).unwrap()
}
