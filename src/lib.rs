#![crate_name = "compsdk"]


extern crate serde;
extern crate serde_json;
extern crate reqwest;

pub mod sdk {

    #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
    pub struct LicenseCustomer {
        enterprise_name: String,
        contact_email: String,
        contact_name: String,
        contact_phone: String
    }

    #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
    pub struct LicenseData {
        license_code: String,
        customer: Option<LicenseCustomer>
    }

    #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
    pub struct ComponentRegister {
        name: String,
        #[serde(rename = "showName")]
        show_name: String,
        description: String,
        #[serde(rename = "versionCode")]
        version_code: i32,
        #[serde(rename = "versionName")]
        version_name: String,
        #[serde(rename = "isOpenSource")]
        is_open_source: i8,
        #[serde(rename = "isEnabled")]
        is_enabled: i8,
        #[serde(rename = "isUnderCarriage")]
        is_under_carriage: i8,
        #[serde(rename = "compactOsVersion")]
        compact_os_version: Option<String>,
        #[serde(rename = "producerCompany")]
        producer_company: String,
        #[serde(rename = "producerContact")]
        producer_contact: String,
        #[serde(rename = "producerEmail")]
        producer_email: String,
        #[serde(rename = "producerPhone")]
        producer_phone: String,
        #[serde(rename = "producerUrl")]
        producer_url: Option<String>
    }

    #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
    pub struct ResultComponentRegister {
        code: i32,
        message: String,
        data: Option<ComponentRegister>
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct ComponentLicensed {
        #[serde(rename = "isRevoked")]
        is_revoked: i8,
        #[serde(rename = "isTrial")]
        is_trial: i8,
        #[serde(rename = "trialStartDate")]
        trial_start_date: i64,
        #[serde(rename = "trialEndDate")]
        trial_end_date: i64
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct ResultComponentLicensed {
        code: i32,
        message: String,
        data: Option<ComponentLicensed>
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct ComponentLicense {
        #[serde(rename = "licenseName")]
        pub license_name: String,
        #[serde(rename = "licenseText")]
        pub license_text: String
    }

    impl std::fmt::Display for ComponentLicense {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut str = format!("license_name: {}\n", self.license_name);
            str += &format!("license_text: {}\n", self.license_text);
            write!(f, "{}", str)
        }
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct ResultComponentLicense {
        code: i32,
        message: String,
        data: Option<ComponentLicense>
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct ComponentProducer {
        company: String,
        contact: String,
        email: String,
        phone: String,
        url: String
    }

    impl std::fmt::Display for ComponentProducer {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut str = format!("company: {}\n", self.company);
            str += &format!("contact: {}\n", self.contact);
            str += &format!("email: {}\n", self.email);
            str += &format!("phone: {}\n", self.phone);
            str += &format!("url: {}\n", self.url);
            write!(f, "{}", str)
        }
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct ResultComponentProducer {
        code: i32,
        message: String,
        data: Option<ComponentProducer>
    }

    #[allow(dead_code)]
    pub struct ComponentSDK {
        lic_info: Option<LicenseData>,
        comp_info: Option<ResultComponentRegister>,
        lic_host: String,
        lic_port: i32,
        component_name: String,
        component_key: String,

        pub is_valid: bool,
        pub invalid_message: String,
        pub license: Option<ComponentLicense>,
        pub producer: Option<ComponentProducer>
    }

    impl ComponentSDK {
        pub fn new(component_name: String, component_key: String, host: String, port: i32) -> ComponentSDK {
            let mut sdk = ComponentSDK {
                lic_info: None,
                comp_info: None,
                lic_host: host,
                lic_port: port,
                component_name,
                component_key,
                is_valid: false,
                invalid_message: "".to_string(),
                license: None,
                producer: None
            };
            sdk.load();
            return sdk;
        }

        fn load(&mut self) {
            let param = format!("{{\"compName\":\"{}\", \"compKey\":\"{}\"}}", self.component_name, self.component_key);
            let url_lic = format!("http://{}:{}/api/license/read", self.lic_host, self.lic_port);
            let ret = self.http_get(&url_lic);
            self.lic_info = serde_json::from_str(&ret).unwrap();
            // 从云端获取组件的注册信息
            let url_comp = format!("http://license.isyscore.com:9990/api/license/cloud/component/one2?compName={}&compKey={}", self.component_name, self.component_key);
            let ret2 = self.http_get(&url_comp);
            self.comp_info = serde_json::from_str(&ret2).unwrap();

            let tmp_comp_info = self.comp_info.clone();
            let tmp_comp_un = self.comp_info.clone().unwrap();
            let tmp_lic_info = self.lic_info.clone();
            match tmp_comp_info {   // move tmp_comp_info
                None => {
                    self.is_valid = false;
                    self.invalid_message = "无法顺利请求云端服务器".to_string();
                }
                Some(tmpcompinfo) => {
                    let tmp_data = tmpcompinfo.data.clone();
                    match tmp_data {    // move tmp_data
                        None => {
                            self.is_valid = false;
                            self.invalid_message = if tmp_comp_un.message == "" { "无法从云端获取数据".to_string() } else { tmp_comp_un.message }
                        }
                        Some(_) => {
                            if tmp_comp_un.code == 200 {    // move tmp_comp_info2
                                match tmp_lic_info {    // tmp_lic_info
                                    None => {
                                        self.is_valid = false;
                                        self.invalid_message = "OS 未授权".to_string();
                                    }
                                    Some(tmplic) => {
                                        let tmpcustomer = tmplic.customer.unwrap();
                                        let tmpdataun = tmp_comp_un.data.clone().unwrap();
                                        // let tmpcompdata = self.comp_info.un
                                        if tmpcustomer.enterprise_name == tmpdataun.producer_company && tmpcustomer.contact_name == tmpdataun.producer_contact {
                                            // 直接授权给自己
                                            self.is_valid = true;
                                            self.invalid_message = "".to_string();
                                        } else {
                                            // 查授权
                                            let url_comp_valid = format!("http://{}:{}/api/core/license/component/valid", self.lic_host, self.lic_port);
                                            let ret3 = self.http_post(&url_comp_valid, &param);
                                            let r: ResultComponentLicensed = serde_json::from_str(&ret3).unwrap();
                                            if r.code == 200 {
                                                self.is_valid = true;
                                                self.invalid_message = r.message;
                                            } else {
                                                self.is_valid = false;
                                                self.invalid_message = if r.message == "" { "".to_string() } else { r.message }
                                            }
                                        }
                                    }
                                }
                            } else {
                                // 云端返回的异常信息
                                self.is_valid = false;
                                self.invalid_message = tmp_comp_un.message;
                            }
                        }
                    }
                }
            }

            // load license
            let url_comp_lic = format!("http://{}:{}/api/core/license/component/license", self.lic_host, self.lic_port);
            let ret4 = self.http_post(&url_comp_lic, &param);
            let r_lic: ResultComponentLicense = serde_json::from_str(&ret4).unwrap();
            self.license = r_lic.data;
            // load producer
            let url_comp_lic = format!("http://{}:{}/api/core/license/component/producer", self.lic_host, self.lic_port);
            let ret4 = self.http_post(&url_comp_lic, &param);
            let r_producer: ResultComponentProducer = serde_json::from_str(&ret4).unwrap();
            self.producer = r_producer.data;
        }

        fn http_get(&self, url: &str) -> String {
            let resp = reqwest::blocking::get(url);
            return if resp.is_ok() {
                resp.unwrap().text().unwrap_or(String::from(""))
            } else {
                "".to_string()
            }
        }

        fn http_post(&self, url: &str, json: &str) -> String {
            let resp = reqwest::blocking::Client::new().post(url).header("Content-Type", "application/json").body(String::from(json)).send();
            return if resp.is_ok() {
                resp.unwrap().text().unwrap_or(String::from(""))
            } else {
                String::from("")
            };
        }
    }


}
