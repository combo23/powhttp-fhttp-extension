use powhttp_sdk::{run, ContextMenuItemSingle, Error, ExtensionHandle, SingleEntryContext};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(async |handle: ExtensionHandle| {
        handle
            .extend_context_menu_single(ContextMenuItemSingle::new(
                "copy-fhttp-headers",
                "Copy to fhttp",
                async |ctx: SingleEntryContext, handle: ExtensionHandle| {
                    let entry = handle
                        .get_session_entry(ctx.session_id, ctx.entry_id)
                        .await?;

                    let Some(entry) = entry else {
                        return Ok(());
                    };

                    let all_headers: Vec<(String, String)> = entry
                        .request
                        .headers
                        .iter()
                        .map(|(name, value)| (name.to_lowercase(), value.clone()))
                        .collect();

                    // Split into pseudo headers (starting with ':') and regular headers
                    let pseudo_headers: Vec<&(String, String)> = all_headers
                        .iter()
                        .filter(|(name, _)| name.starts_with(':'))
                        .collect();
                    let regular_headers: Vec<&(String, String)> = all_headers
                        .iter()
                        .filter(|(name, _)| !name.starts_with(':'))
                        .collect();

                    // Find the max header name length for alignment (consider both regular + pseudo + order keys)
                    let max_name_len = regular_headers
                        .iter()
                        .map(|(name, _)| name.len() + 2) // +2 for quotes
                        .chain(std::iter::once("http.HeaderOrderKey".len()))
                        .chain(std::iter::once("http.PHeaderOrderKey".len()))
                        .max()
                        .unwrap_or(0);

                    let mut lines = Vec::new();
                    lines.push("req.Header = http.Header{".to_string());

                    for (name, value) in &regular_headers {
                        let quoted_name = format!("\"{}\"", name);
                        let padding = max_name_len - quoted_name.len();
                        let escaped_value = value.replace('\\', "\\\\").replace('"', "\\\"");
                        lines.push(format!(
                            "    {}{}: {{\"{}\"}},",
                            quoted_name,
                            " ".repeat(padding),
                            escaped_value
                        ));
                    }

                    // Add HeaderOrderKey (regular headers, deduplicated)
                    let mut seen = std::collections::HashSet::new();
                    let order_values: Vec<String> = regular_headers
                        .iter()
                        .filter(|(name, _)| seen.insert(name.as_str()))
                        .map(|(name, _)| format!("\"{}\"", name))
                        .collect();
                    let order_key = "http.HeaderOrderKey";
                    let padding = max_name_len - order_key.len();
                    lines.push(format!(
                        "    {}{}: {{{}}},",
                        order_key,
                        " ".repeat(padding),
                        order_values.join(", ")
                    ));

                    // Add PHeaderOrderKey (pseudo headers)
                    if !pseudo_headers.is_empty() {
                        let porder_values: Vec<String> = pseudo_headers
                            .iter()
                            .map(|(name, _)| format!("\"{}\"", name))
                            .collect();
                        let porder_key = "http.PHeaderOrderKey";
                        let padding = max_name_len - porder_key.len();
                        lines.push(format!(
                            "    {}{}: {{{}}},",
                            porder_key,
                            " ".repeat(padding),
                            porder_values.join(", ")
                        ));
                    }

                    lines.push("}".to_string());

                    let output = lines.join("\n");
                    handle.write_text_to_clipboard(&output).await?;

                    Ok(())
                },
            ))
            .await?;

        Ok(())
    })
    .await
}
