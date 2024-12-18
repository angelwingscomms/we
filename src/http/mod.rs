use anyhow::Context;
use serde::Serialize;

pub fn post<T: Serialize, S: 'static + Send + FnMut(&str), E: 'static + Send + FnMut(&str)>(
    url: &str,
    body: T,
    mut on_success: S,
    mut on_error: E,
) {
    if let Ok(v) = serde_json::to_vec(&body) {
        let mut request = ehttp::Request::post(url, v);
        request.headers.insert("Content-Type", "application/json");
        ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
            let r = |result: ehttp::Result<ehttp::Response>| -> anyhow::Result<String> {
                let res = result
                    .map_err(|e| anyhow::anyhow!(e.to_string()))
                    .context("ehttp result")?;
                Ok(res
                    .text()
                    .ok_or(anyhow::anyhow!("empty ehttp result"))
                    .context("response.text()")?
                    .to_string())
                // *aa.lock() = serde_json::from_str(
                //     res_text_result
                //         .ok_or(anyhow!("empty ehttp result"))
                //         .context("response.text()")?,
                // )
                // .context("serde_json from string")?;
            };
            match r(result) {
                Err(e) => {
                    on_error(&e.to_string());
                }
                Ok(r) => {
                    on_success(&r);
                }
            };
        });
    } else {
        on_error("body to vec failed");
    }
}

pub fn put<T: Serialize, S: 'static + Send + FnMut(&str), E: 'static + Send + FnMut(&str)>(
    url: &str,
    body: T,
    mut on_success: S,
    mut on_error: E,
) {
    if let Ok(v) = serde_json::to_vec(&body) {
        let mut request = ehttp::Request::post(url, v);
        request.headers.insert("Content-Type", "application/json");
        ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
            let r = |result: ehttp::Result<ehttp::Response>| -> anyhow::Result<String> {
                let res = result
                    .map_err(|e| anyhow::anyhow!(e.to_string()))
                    .context("ehttp result")?;
                Ok(res
                    .text()
                    .ok_or(anyhow::anyhow!("empty ehttp result"))
                    .context("response.text()")?
                    .to_string())
                // *aa.lock() = serde_json::from_str(
                //     res_text_result
                //         .ok_or(anyhow!("empty ehttp result"))
                //         .context("response.text()")?,
                // )
                // .context("serde_json from string")?;
            };
            match r(result) {
                Err(e) => {
                    on_error(&e.to_string());
                }
                Ok(r) => {
                    on_success(&r);
                }
            };
        });
    } else {
        on_error("body to vec failed");
    }
}

pub fn get<S: 'static + Send + FnMut(&str), E: 'static + Send + FnMut(&str)>(
    url: &str,
    mut on_success: S,
    mut on_error: E,
) {
    let request = ehttp::Request::get(url);
    ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
        let r = |result: ehttp::Result<ehttp::Response>| -> anyhow::Result<String> {
            let res = result
                .map_err(|e| anyhow::anyhow!(e.to_string()))
                .context("ehttp result")?;
            Ok(res
                .text()
                .ok_or(anyhow::anyhow!("empty ehttp result"))
                .context("response.text()")?
                .to_string())
        };
        match r(result) {
            Err(e) => {
                on_error(&e.to_string());
            }
            Ok(r) => {
                on_success(&r);
            }
        };
    });
}
