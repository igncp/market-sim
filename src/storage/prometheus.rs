use crate::storage_interface::{PrometheusMetrics, StoragePrometheus};

pub struct StoragePrometheusImpl {
    pub job_name: String,
    pub url: String,
}

impl StoragePrometheus for StoragePrometheusImpl {
    fn get_metrics_text(
        &self,
        prefix: &str,
        metrics: &PrometheusMetrics,
    ) -> Result<String, String> {
        let mut metrics_text = String::new();

        for metric in metrics.iter() {
            let mut labels_text = String::new();

            if !metric.labels.is_empty() {
                labels_text.push_str("{");

                for (label_name, label_value) in metric.labels.iter() {
                    labels_text.push_str(&format!(r#"{}="{}","#, label_name, label_value));
                }

                labels_text.pop();
                labels_text.push_str("}");
            }
            metrics_text.push_str(&format!(
                "{}_{}{} {}\n",
                prefix, metric.name, labels_text, metric.value
            ));
        }

        Ok(metrics_text)
    }

    async fn flush_metrics(&self) -> Result<(), String> {
        let client = reqwest::Client::new();

        client
            .post(format!(
                r#"{}/api/v1/admin/tsdb/delete_series?match[]={{job="{}"}}"#,
                self.url, self.job_name
            ))
            .send()
            .await
            .map_err(|e| format!("Failed to flush metrics: {}", e))?;

        Ok(())
    }
}
